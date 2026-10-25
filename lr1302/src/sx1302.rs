//! SX1302 configuration and bring-up (C: `loragw_sx1302.c`).
//!
//! Chip identification, GPIO configuration, radio reset / clock selection /
//! calibration, front-end and channelizer setup, correlator and modem
//! configuration, sync words, timestamp counter access and GPS control.
//!
//! Not ported from this C file: the sx1255/1257 radio calibration firmware
//! (`cal_fw.var` / `loragw_cal.c`, returns [`Error::Unsupported`]), the GPS
//! UART, the USB/debug helpers and the `printf` / `DEBUG_*` logging.

use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::spi::SpiDevice;

use crate::consts::*;
use crate::error::{Error, Result};
use crate::log::{debug, trace};
use crate::reg::*;
use crate::sx1302_defs::*;
use crate::util::bw_getval;
use crate::Lr1302;

/// Frequency tracking mode: disabled (C: `RX_FREQ_TRACK_OFF`).
const RX_FREQ_TRACK_OFF: i32 = 0x00;
/// Frequency tracking mode: automatic (C: `RX_FREQ_TRACK_AUTO`).
const RX_FREQ_TRACK_AUTO: i32 = 0x03;
/// Fine timing mode: linear interpolation (C: `RX_FINE_TIMING_MODE_LINEAR`).
const RX_FINE_TIMING_MODE_LINEAR: i32 = 0x02;
/// DFT peak mode: automatic (C: `RX_DFT_PEAK_MODE_AUTO`).
const RX_DFT_PEAK_MODE_AUTO: i32 = 0x03;

/// Compute the frequency-to-time drift mantissa / exponent pair used by the
/// LoRa modems (C: `calculate_freq_to_time_drift`).
///
/// Returns `(mant, exp)`.
pub(crate) fn calculate_freq_to_time_drift(freq_hz: u32, bw: u8) -> Result<(u16, u8)> {
    let mut exponent: u8 = 0;

    // check input variables
    let bw_hz: i32 = match bw_getval(bw) {
        Some(v) => v as i32,
        None => return Err(Error::InvalidConfig("bandwidth")),
    };
    if bw_hz < 0 {
        return Err(Error::InvalidConfig("bandwidth"));
    }

    // C: mantissa_u64 = (uint64_t)bw_hz * (2 << (20-1)) / freq_hz;
    // (the zero guard only avoids a division by zero, which is UB in C)
    if freq_hz == 0 {
        return Err(Error::InvalidConfig("freq_hz"));
    }
    let mut mantissa_u64: u64 = (bw_hz as u64) * (2u64 << (20 - 1)) / u64::from(freq_hz);
    while mantissa_u64 < 2048 {
        exponent += 1;
        mantissa_u64 <<= 1;
    }

    Ok((mantissa_u64 as u16, exponent))
}

/// Return the modem type of an IF chain (C: `sx1302_get_ifmod_config`).
///
/// The C version indexes `ifmod_config[if_chain]` directly; this guarded
/// version returns [`IF_UNDEFINED`] for an out-of-range chain instead of
/// reading past the array.
pub fn sx1302_get_ifmod_config(if_chain: u8) -> u8 {
    match IFMODEM_CONFIG.get(if_chain as usize) {
        Some(&modem) => modem,
        None => IF_UNDEFINED,
    }
}

impl<SPI, D> Lr1302<SPI, D>
where
    SPI: SpiDevice<u8>,
    D: DelayNs,
{
    /// Configure the SX1302 GPIO functions (C: `sx1302_config_gpio`).
    pub(crate) async fn sx1302_config_gpio(&mut self) -> Result<()> {
        self.com
            .reg_w(SX1302_REG_GPIO_GPIO_SEL_0_SELECTION, GPIO_CFG_REGISTER as i32)
            .await?; /* GPIO_0 => CONFIG_DONE */
        self.com
            .reg_w(SX1302_REG_GPIO_GPIO_SEL_1_SELECTION, GPIO_CFG_REGISTER as i32)
            .await?; /* GPIO_1 => UNUSED */
        self.com
            .reg_w(SX1302_REG_GPIO_GPIO_SEL_2_SELECTION, GPIO_CFG_STATUS as i32)
            .await?; /* GPIO_2 => Tx ON */
        self.com
            .reg_w(SX1302_REG_GPIO_GPIO_SEL_3_SELECTION, GPIO_CFG_REGISTER as i32)
            .await?; /* GPIO_3 => UNUSED */
        self.com
            .reg_w(SX1302_REG_GPIO_GPIO_SEL_4_SELECTION, GPIO_CFG_STATUS as i32)
            .await?; /* GPIO_4 => RX ON (PKT_RECEIVE_TOGGLE_OUT) */
        self.com
            .reg_w(SX1302_REG_GPIO_GPIO_SEL_5_SELECTION, GPIO_CFG_REGISTER as i32)
            .await?; /* GPIO_5 => UNUSED */
        self.com
            .reg_w(SX1302_REG_GPIO_GPIO_SEL_6_SELECTION, GPIO_CFG_REGISTER as i32)
            .await?; /* GPIO_6 => UNUSED */
        self.com
            .reg_w(SX1302_REG_GPIO_GPIO_SEL_7_SELECTION, GPIO_CFG_AGC as i32)
            .await?; /* GPIO_7 => USED FOR LBT (MUST BE INPUT) */
        // GPIO output direction (0 for input and 1 for output)
        self.com
            .reg_w(SX1302_REG_GPIO_GPIO_DIR_L_DIRECTION, 0x7F)
            .await?;
        Ok(())
    }

    /// Initialize the SX1302 and its internal structures (C: `sx1302_init`).
    pub(crate) async fn sx1302_init(&mut self) -> Result<()> {
        trace!("sx1302 basic init");
        // Initialize internal counter
        self.timestamp_counter_new();

        // Initialize RX buffer
        self.rx_buffer_new();

        // Configure timestamping mode
        let ftime_enable = self.ctx.ftime.enable;
        if ftime_enable {
            let model_id = self.sx1302_get_model_id().await?;
            if model_id != CHIP_MODEL_ID_SX1303 {
                // C: "Fine Timestamping is not supported on this Chip Model ID"
                return Err(Error::Unsupported);
            }
        }
        self.timestamp_counter_mode(ftime_enable).await?;

        self.sx1302_config_gpio().await?;

        Ok(())
    }

    /// Read the SX1302 unique identifier (C: `sx1302_get_eui`).
    pub(crate) async fn sx1302_get_eui(&mut self) -> Result<u64> {
        let mut eui: u64 = 0;
        for i in 0..8 {
            self.com.reg_w(SX1302_REG_OTP_BYTE_ADDR_ADDR, i).await?;
            let val: i32 = self.com.reg_r(SX1302_REG_OTP_RD_DATA_RD_DATA).await?;
            eui |= ((val as u8) as u64) << (56 - (i * 8));
        }
        Ok(eui)
    }

    /// Read the SX1302/SX1303 chip model id (C: `sx1302_get_model_id`).
    pub(crate) async fn sx1302_get_model_id(&mut self) -> Result<u8> {
        // Select ChipModelID
        self.com.reg_w(SX1302_REG_OTP_BYTE_ADDR_ADDR, 0xD0).await?;

        // Read Modem ID
        let val: i32 = self.com.reg_r(SX1302_REG_OTP_RD_DATA_RD_DATA).await?;
        Ok(val as u8)
    }

    /// Check the MCUs parity errors and refresh the timestamp counter wrap
    /// status (C: `sx1302_update`).
    ///
    /// Note: the C version has the parity checks inside `#if 0` (disabled
    /// because they add USB latency); here they are executed as described by
    /// the porting contract. A parity error is reported as
    /// [`Error::FirmwareVerify`].
    pub(crate) async fn sx1302_update(&mut self) -> Result<()> {
        // Check MCUs parity errors
        let val: i32 = self.com.reg_r(SX1302_REG_AGC_MCU_CTRL_PARITY_ERROR).await?;
        if val != 0 {
            // C: "Parity error check failed on AGC firmware"
            return Err(Error::FirmwareVerify);
        }
        let val: i32 = self.com.reg_r(SX1302_REG_ARB_MCU_CTRL_PARITY_ERROR).await?;
        if val != 0 {
            // C: "Parity error check failed on ARB firmware"
            return Err(Error::FirmwareVerify);
        }

        // Update internal timestamp counter wrapping status
        let _ = self.timestamp_counter_get().await?;

        Ok(())
    }

    /// Switch the SX1302 clock from SPI clock to the radio clock of the
    /// selected RF chain (C: `sx1302_radio_clock_select`).
    pub(crate) async fn sx1302_radio_clock_select(&mut self, rf_chain: u8) -> Result<()> {
        // Check input parameters
        if rf_chain as usize >= LGW_RF_CHAIN_NB {
            return Err(Error::InvalidConfig("rf_chain"));
        }

        // Switch SX1302 clock from SPI clock to radio clock of the selected RF chain
        match rf_chain {
            0 => {
                self.com
                    .reg_w(SX1302_REG_CLK_CTRL_CLK_SEL_CLK_RADIO_A_SEL, 0x01)
                    .await?;
                self.com
                    .reg_w(SX1302_REG_CLK_CTRL_CLK_SEL_CLK_RADIO_B_SEL, 0x00)
                    .await?;
            }
            1 => {
                self.com
                    .reg_w(SX1302_REG_CLK_CTRL_CLK_SEL_CLK_RADIO_A_SEL, 0x00)
                    .await?;
                self.com
                    .reg_w(SX1302_REG_CLK_CTRL_CLK_SEL_CLK_RADIO_B_SEL, 0x01)
                    .await?;
            }
            _ => return Err(Error::InvalidConfig("rf_chain")),
        }

        // Enable clock dividers
        self.com
            .reg_w(SX1302_REG_CLK_CTRL_CLK_SEL_CLKDIV_EN, 0x01)
            .await?;

        // Set the RIF clock to the 32MHz clock of the radio
        self.com
            .reg_w(SX1302_REG_COMMON_CTRL0_CLK32_RIF_CTRL, 0x01)
            .await?;

        Ok(())
    }

    /// Apply the reset sequence to the radio of the given RF chain
    /// (C: `sx1302_radio_reset`).
    pub(crate) async fn sx1302_radio_reset(&mut self, rf_chain: u8, type_: RadioType) -> Result<()> {
        trace!("radio reset chain={=u8}", rf_chain);
        // Check input parameters
        if rf_chain as usize >= LGW_RF_CHAIN_NB {
            return Err(Error::InvalidConfig("rf_chain"));
        }
        if !matches!(type_, RadioType::Sx1255 | RadioType::Sx1257 | RadioType::Sx1250) {
            return Err(Error::InvalidConfig("radio type"));
        }

        // Switch to SPI clock before reseting the radio
        self.com
            .reg_w(SX1302_REG_COMMON_CTRL0_CLK32_RIF_CTRL, 0x00)
            .await?;

        // Enable the radio
        let reg_radio_en = reg_select(
            rf_chain,
            SX1302_REG_AGC_MCU_RF_EN_A_RADIO_EN,
            SX1302_REG_AGC_MCU_RF_EN_B_RADIO_EN,
        );
        self.com.reg_w(reg_radio_en, 0x01).await?;

        // Select the proper reset sequence depending on the radio type
        let reg_radio_rst = reg_select(
            rf_chain,
            SX1302_REG_AGC_MCU_RF_EN_A_RADIO_RST,
            SX1302_REG_AGC_MCU_RF_EN_B_RADIO_RST,
        );
        self.com.reg_w(reg_radio_rst, 0x01).await?;
        self.com.wait_ms(500).await;
        self.com.reg_w(reg_radio_rst, 0x00).await?;
        self.com.wait_ms(10).await;
        match type_ {
            RadioType::Sx1255 | RadioType::Sx1257 => {
                // Do nothing
            }
            RadioType::Sx1250 => {
                self.com.reg_w(reg_radio_rst, 0x01).await?;
                self.com.wait_ms(10).await; /* wait for auto calibration to complete */
            }
            _ => return Err(Error::InvalidConfig("radio type")),
        }

        Ok(())
    }

    /// Configure the radio mode (sx1250 / sx125x) of an RF chain
    /// (C: `sx1302_radio_set_mode`).
    pub(crate) async fn sx1302_radio_set_mode(
        &mut self,
        rf_chain: u8,
        type_: RadioType,
    ) -> Result<()> {
        // Check input parameters
        if rf_chain as usize >= LGW_RF_CHAIN_NB {
            return Err(Error::InvalidConfig("rf_chain"));
        }
        if !matches!(type_, RadioType::Sx1255 | RadioType::Sx1257 | RadioType::Sx1250) {
            return Err(Error::InvalidConfig("radio type"));
        }

        // Set the radio mode
        let reg = reg_select(
            rf_chain,
            SX1302_REG_COMMON_CTRL0_SX1261_MODE_RADIO_A,
            SX1302_REG_COMMON_CTRL0_SX1261_MODE_RADIO_B,
        );
        let value: i32 = if type_ == RadioType::Sx1250 { 0x01 } else { 0x00 };
        self.com.reg_w(reg, value).await?;

        Ok(())
    }

    /// Give or release the control over the radios to/from the host
    /// (C: `sx1302_radio_host_ctrl`).
    pub(crate) async fn sx1302_radio_host_ctrl(&mut self, host_ctrl: bool) -> Result<()> {
        self.com
            .reg_w(
                SX1302_REG_COMMON_CTRL0_HOST_RADIO_CTRL,
                if host_ctrl { 0x01 } else { 0x00 },
            )
            .await
    }

    /// Reset, clock-select and calibrate the radios (C: `sx1302_radio_calibrate`).
    ///
    /// The C parameters (`context_rf_chain`, `clksrc`, `txgain_lut`) are read
    /// from `self.ctx`. The sx1255/1257 calibration path
    /// (`sx1302_cal_start`, `loragw_cal.c`) is not ported and reports
    /// [`Error::Unsupported`].
    pub(crate) async fn sx1302_radio_calibrate(&mut self) -> Result<()> {
        debug!("calibrating radios");
        let clksrc = self.ctx.board.clksrc;

        // -- Reset radios
        for i in 0..LGW_RF_CHAIN_NB {
            let rf = self.ctx.rf_chain[i];
            if rf.enable {
                self.sx1302_radio_reset(i as u8, rf.type_).await?;
                self.sx1302_radio_set_mode(i as u8, rf.type_).await?;
            }
        }

        // -- Select the radio which provides the clock to the sx1302
        self.sx1302_radio_clock_select(clksrc).await?;

        // -- Ensure PA/LNA are disabled
        self.com
            .reg_w(SX1302_REG_AGC_MCU_CTRL_FORCE_HOST_FE_CTRL, 1)
            .await?;
        self.com.reg_w(SX1302_REG_AGC_MCU_RF_EN_A_PA_EN, 0).await?;
        self.com.reg_w(SX1302_REG_AGC_MCU_RF_EN_A_LNA_EN, 0).await?;

        // -- Start calibration
        let clksrc_rf = self
            .ctx
            .rf_chain
            .get(clksrc as usize)
            .ok_or(Error::InvalidConfig("clksrc"))?;
        if matches!(clksrc_rf.type_, RadioType::Sx1257 | RadioType::Sx1255) {
            // C: load the sx125x CAL firmware then run `sx1302_cal_start`;
            // that calibration firmware is not ported.
            return Err(Error::Unsupported);
        }
        for i in 0..LGW_RF_CHAIN_NB {
            let rf = self.ctx.rf_chain[i];
            if rf.enable {
                self.com.sx1250_calibrate(i as u8, rf.freq_hz).await?;
            }
        }

        // -- Release control over FE
        self.com
            .reg_w(SX1302_REG_AGC_MCU_CTRL_FORCE_HOST_FE_CTRL, 0)
            .await?;

        Ok(())
    }

    /// Configure the PA and LNA LUTs (C: `sx1302_pa_lna_lut_configure`).
    pub(crate) async fn sx1302_pa_lna_lut_configure(&mut self) -> Result<()> {
        let board = self.ctx.board;

        // Configure LUT Table A
        if board.full_duplex {
            // Enable PA: RADIO_CTRL[2] is high when PA_EN=1
            self.com
                .reg_w(SX1302_REG_AGC_MCU_LUT_TABLE_A_PA_LUT, 0x0C)
                .await?;
            // Enable LNA: RADIO_CTRL[1] is always high
            self.com
                .reg_w(SX1302_REG_AGC_MCU_LUT_TABLE_A_LNA_LUT, 0x0F)
                .await?;
        } else {
            // Enable PA: RADIO_CTRL[2] is high when PA_EN=1
            self.com
                .reg_w(SX1302_REG_AGC_MCU_LUT_TABLE_A_PA_LUT, 0x04)
                .await?;
            // Enable LNA: RADIO_CTRL[1] is high when PA_EN=0 & LNA_EN=1
            self.com
                .reg_w(SX1302_REG_AGC_MCU_LUT_TABLE_A_LNA_LUT, 0x02)
                .await?;
        }

        // Configure LUT Table B
        // Enable PA: RADIO_CTRL[8] is high when PA_EN=1 & LNA_EN=0
        self.com
            .reg_w(SX1302_REG_AGC_MCU_LUT_TABLE_B_PA_LUT, 0x04)
            .await?;
        // Enable LNA: RADIO_CTRL[7] is high when PA_EN=0 & LNA_EN=1
        self.com
            .reg_w(SX1302_REG_AGC_MCU_LUT_TABLE_B_LNA_LUT, 0x02)
            .await?;

        Ok(())
    }

    /// Configure the radio front-end stage of the SX1302
    /// (C: `sx1302_radio_fe_configure`).
    pub(crate) async fn sx1302_radio_fe_configure(&mut self) -> Result<()> {
        self.com.reg_w(
            SX1302_REG_RADIO_FE_RSSI_BB_FILTER_ALPHA_RADIO_A_RSSI_BB_FILTER_ALPHA,
            0x03,
        ).await?;
        self.com.reg_w(
            SX1302_REG_RADIO_FE_RSSI_DEC_FILTER_ALPHA_RADIO_A_RSSI_DEC_FILTER_ALPHA,
            0x07,
        ).await?;
        self.com.reg_w(
            SX1302_REG_RADIO_FE_RSSI_BB_FILTER_ALPHA_RADIO_B_RSSI_BB_FILTER_ALPHA,
            0x03,
        ).await?;
        self.com.reg_w(
            SX1302_REG_RADIO_FE_RSSI_DEC_FILTER_ALPHA_RADIO_B_RSSI_DEC_FILTER_ALPHA,
            0x07,
        ).await?;

        self.com.reg_w(
            SX1302_REG_RADIO_FE_RSSI_DB_DEF_RADIO_A_RSSI_DB_DEFAULT_VALUE,
            23,
        ).await?;
        self.com.reg_w(
            SX1302_REG_RADIO_FE_RSSI_DEC_DEF_RADIO_A_RSSI_DEC_DEFAULT_VALUE,
            66,
        ).await?;
        self.com.reg_w(
            SX1302_REG_RADIO_FE_RSSI_DB_DEF_RADIO_B_RSSI_DB_DEFAULT_VALUE,
            23,
        ).await?;
        self.com.reg_w(
            SX1302_REG_RADIO_FE_RSSI_DEC_DEF_RADIO_B_RSSI_DEC_DEFAULT_VALUE,
            66,
        ).await?;

        self.com
            .reg_w(SX1302_REG_RADIO_FE_CTRL0_RADIO_A_DC_NOTCH_EN, 1)
            .await?;
        self.com
            .reg_w(SX1302_REG_RADIO_FE_CTRL0_RADIO_A_HOST_FILTER_GAIN, 0x0b)
            .await?;
        self.com
            .reg_w(SX1302_REG_RADIO_FE_CTRL0_RADIO_B_DC_NOTCH_EN, 1)
            .await?;
        self.com
            .reg_w(SX1302_REG_RADIO_FE_CTRL0_RADIO_B_HOST_FILTER_GAIN, 0x0b)
            .await?;

        Ok(())
    }

    /// Configure the channelizer stage of the SX1302
    /// (C: `sx1302_channelizer_configure`).
    pub(crate) async fn sx1302_channelizer_configure(&mut self, fix_gain: bool) -> Result<()> {
        let if_cfg = self.ctx.if_chain;
        let mut channels_mask: u8 = 0x00;

        // Select which radio is connected to each multi-SF channel
        for i in 0..LGW_MULTI_NB {
            channels_mask |= if_cfg[i].rf_chain << i;
        }
        self.com
            .reg_w(SX1302_REG_RX_TOP_RADIO_SELECT_RADIO_SELECT, channels_mask as i32)
            .await?;

        // Select which radio is connected to the LoRa service channel
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_LORA_SERVICE_RADIO_SEL_RADIO_SELECT,
                if_cfg[8].rf_chain as i32,
            )
            .await?;

        // Select which radio is connected to the FSK channel
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_CFG_3_RADIO_SELECT,
                if_cfg[9].rf_chain as i32,
            )
            .await?;

        // Configure multi-SF channels IF frequencies
        const FREQ_REGS: [(u16, u16); 8] = [
            (
                SX1302_REG_RX_TOP_FREQ_0_MSB_IF_FREQ_0,
                SX1302_REG_RX_TOP_FREQ_0_LSB_IF_FREQ_0,
            ),
            (
                SX1302_REG_RX_TOP_FREQ_1_MSB_IF_FREQ_1,
                SX1302_REG_RX_TOP_FREQ_1_LSB_IF_FREQ_1,
            ),
            (
                SX1302_REG_RX_TOP_FREQ_2_MSB_IF_FREQ_2,
                SX1302_REG_RX_TOP_FREQ_2_LSB_IF_FREQ_2,
            ),
            (
                SX1302_REG_RX_TOP_FREQ_3_MSB_IF_FREQ_3,
                SX1302_REG_RX_TOP_FREQ_3_LSB_IF_FREQ_3,
            ),
            (
                SX1302_REG_RX_TOP_FREQ_4_MSB_IF_FREQ_4,
                SX1302_REG_RX_TOP_FREQ_4_LSB_IF_FREQ_4,
            ),
            (
                SX1302_REG_RX_TOP_FREQ_5_MSB_IF_FREQ_5,
                SX1302_REG_RX_TOP_FREQ_5_LSB_IF_FREQ_5,
            ),
            (
                SX1302_REG_RX_TOP_FREQ_6_MSB_IF_FREQ_6,
                SX1302_REG_RX_TOP_FREQ_6_LSB_IF_FREQ_6,
            ),
            (
                SX1302_REG_RX_TOP_FREQ_7_MSB_IF_FREQ_7,
                SX1302_REG_RX_TOP_FREQ_7_LSB_IF_FREQ_7,
            ),
        ];
        for i in 0..LGW_MULTI_NB {
            let if_freq = if_hz_to_reg(if_cfg[i].freq_hz);
            let (reg_msb, reg_lsb) = FREQ_REGS[i];
            self.com
                .reg_w(reg_msb, (if_freq >> 8) & 0x0000001F)
                .await?;
            self.com
                .reg_w(reg_lsb, (if_freq >> 0) & 0x000000FF)
                .await?;
        }

        // Configure LoRa service channel IF frequency
        let if_freq = if_hz_to_reg(if_cfg[8].freq_hz);
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_LORA_SERVICE_FREQ_MSB_IF_FREQ_0,
                (if_freq >> 8) & 0x0000001F,
            )
            .await?;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_LORA_SERVICE_FREQ_LSB_IF_FREQ_0,
                (if_freq >> 0) & 0x000000FF,
            )
            .await?;

        // Configure FSK channel IF frequency
        let if_freq = if_hz_to_reg(if_cfg[9].freq_hz);
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_FREQ_MSB_IF_FREQ_0,
                (if_freq >> 8) & 0x0000001F,
            )
            .await?;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_FREQ_LSB_IF_FREQ_0,
                (if_freq >> 0) & 0x000000FF,
            )
            .await?;

        // Set the low pass filtering corner frequency for RSSI indicator
        self.com
            .reg_w(SX1302_REG_RX_TOP_RSSI_CONTROL_RSSI_FILTER_ALPHA, 0x05)
            .await?;

        // Set the channelizer RSSI reset value
        self.com
            .reg_w(SX1302_REG_RX_TOP_RSSI_DEF_VALUE_CHAN_RSSI_DEF_VALUE, 85)
            .await?;

        // Force channelizer in fix gain, or let it be controlled by AGC
        if fix_gain {
            self.com
                .reg_w(SX1302_REG_RX_TOP_CHANN_DAGC_CFG5_CHAN_DAGC_MODE, 0x00)
                .await?;
            self.com
                .reg_w(SX1302_REG_RX_TOP_GAIN_CONTROL_CHAN_GAIN, 5)
                .await?;
        } else {
            // Allow the AGC to control gains
            self.com
                .reg_w(SX1302_REG_RX_TOP_CHANN_DAGC_CFG5_CHAN_DAGC_MODE, 0x01)
                .await?;
            // Disable the internal DAGC
            self.com
                .reg_w(
                    SX1302_REG_RX_TOP_CHANN_DAGC_CFG1_CHAN_DAGC_THRESHOLD_HIGH,
                    255,
                )
                .await?;
            self.com
                .reg_w(SX1302_REG_RX_TOP_CHANN_DAGC_CFG2_CHAN_DAGC_THRESHOLD_LOW, 0)
                .await?;
            self.com
                .reg_w(SX1302_REG_RX_TOP_CHANN_DAGC_CFG3_CHAN_DAGC_MAX_ATTEN, 15)
                .await?;
            self.com
                .reg_w(SX1302_REG_RX_TOP_CHANN_DAGC_CFG3_CHAN_DAGC_MIN_ATTEN, 0)
                .await?;
        }

        Ok(())
    }

    /// Configure the FSK modem (C: `sx1302_fsk_configure`).
    pub(crate) async fn sx1302_fsk_configure(&mut self) -> Result<()> {
        let cfg = self.ctx.fsk;

        // C: `cfg->sync_word_size - 1` is computed on promoted `int`s
        let sync_word_size = cfg.sync_word_size;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_CFG_1_PSIZE,
                sync_word_size as i32 - 1,
            )
            .await?;

        // C: `cfg->sync_word << (8 * (8 - cfg->sync_word_size))`; the shift
        // count is kept modulo 64 so that an out-of-range sync word size
        // cannot panic here (C would have undefined behaviour).
        let shift = (8 * (8 - sync_word_size as i32)).rem_euclid(64) as u32;
        let fsk_sync_word_reg = cfg.sync_word << shift;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_REF_PATTERN_BYTE0_FSK_REF_PATTERN,
                (fsk_sync_word_reg >> 0) as u8 as i32,
            )
            .await?;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_REF_PATTERN_BYTE1_FSK_REF_PATTERN,
                (fsk_sync_word_reg >> 8) as u8 as i32,
            )
            .await?;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_REF_PATTERN_BYTE2_FSK_REF_PATTERN,
                (fsk_sync_word_reg >> 16) as u8 as i32,
            )
            .await?;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_REF_PATTERN_BYTE3_FSK_REF_PATTERN,
                (fsk_sync_word_reg >> 24) as u8 as i32,
            )
            .await?;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_REF_PATTERN_BYTE4_FSK_REF_PATTERN,
                (fsk_sync_word_reg >> 32) as u8 as i32,
            )
            .await?;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_REF_PATTERN_BYTE5_FSK_REF_PATTERN,
                (fsk_sync_word_reg >> 40) as u8 as i32,
            )
            .await?;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_REF_PATTERN_BYTE6_FSK_REF_PATTERN,
                (fsk_sync_word_reg >> 48) as u8 as i32,
            )
            .await?;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_REF_PATTERN_BYTE7_FSK_REF_PATTERN,
                (fsk_sync_word_reg >> 56) as u8 as i32,
            )
            .await?;

        let fsk_br_reg = 32_000_000u32
            .checked_div(cfg.datarate)
            .ok_or(Error::InvalidConfig("datarate"))?;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_BIT_RATE_MSB_BIT_RATE,
                (fsk_br_reg >> 8) as u8 as i32,
            )
            .await?;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_BIT_RATE_LSB_BIT_RATE,
                (fsk_br_reg >> 0) as u8 as i32,
            )
            .await?;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_CFG_1_CH_BW_EXPO,
                0x03,
            )
            .await?; /* 125KHz */

        self.com
            .reg_w(SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_CFG_3_RX_INVERT, 0)
            .await?;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_CFG_3_MODEM_INVERT_IQ,
                0,
            )
            .await?;

        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_CFG_4_RSSI_LENGTH,
                4,
            )
            .await?;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_CFG_0_PKT_MODE,
                1,
            )
            .await?; /* variable length */
        self.com
            .reg_w(SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_CFG_0_CRC_EN, 1)
            .await?;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_CFG_0_DCFREE_ENC,
                2,
            )
            .await?;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_CFG_0_CRC_IBM,
                0,
            )
            .await?;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_CFG_4_ERROR_OSR_TOL,
                10,
            )
            .await?;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_PKT_LENGTH_PKT_LENGTH,
                255,
            )
            .await?;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_NODE_ADRS_NODE_ADRS,
                0,
            )
            .await?;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_BROADCAST_BROADCAST,
                0,
            )
            .await?;
        self.com
            .reg_w(SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_CFG_3_AUTO_AFC, 1)
            .await?; /* ?? */
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_TIMEOUT_MSB_TIMEOUT,
                0,
            )
            .await?;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_TIMEOUT_LSB_TIMEOUT,
                128,
            )
            .await?;

        Ok(())
    }

    /// Configure the correlators of the LoRa multi-SF modems
    /// (C: `sx1302_lora_correlator_configure`).
    pub(crate) async fn sx1302_lora_correlator_configure(&mut self) -> Result<()> {
        let if_cfg = self.ctx.if_chain;
        let demod_cfg = self.ctx.demod;
        let mut channels_mask: u8 = 0x00;

        // SF5..SF12: (ACC_PNR, MSP_PNR, MSP_PEAK_NB, MSP2_PEAK_NB) registers
        const SF_CFG: [(u16, u16, u16, u16); 8] = [
            (
                SX1302_REG_RX_TOP_SF5_CFG2_ACC_PNR,
                SX1302_REG_RX_TOP_SF5_CFG4_MSP_PNR,
                SX1302_REG_RX_TOP_SF5_CFG6_MSP_PEAK_NB,
                SX1302_REG_RX_TOP_SF5_CFG7_MSP2_PEAK_NB,
            ),
            (
                SX1302_REG_RX_TOP_SF6_CFG2_ACC_PNR,
                SX1302_REG_RX_TOP_SF6_CFG4_MSP_PNR,
                SX1302_REG_RX_TOP_SF6_CFG6_MSP_PEAK_NB,
                SX1302_REG_RX_TOP_SF6_CFG7_MSP2_PEAK_NB,
            ),
            (
                SX1302_REG_RX_TOP_SF7_CFG2_ACC_PNR,
                SX1302_REG_RX_TOP_SF7_CFG4_MSP_PNR,
                SX1302_REG_RX_TOP_SF7_CFG6_MSP_PEAK_NB,
                SX1302_REG_RX_TOP_SF7_CFG7_MSP2_PEAK_NB,
            ),
            (
                SX1302_REG_RX_TOP_SF8_CFG2_ACC_PNR,
                SX1302_REG_RX_TOP_SF8_CFG4_MSP_PNR,
                SX1302_REG_RX_TOP_SF8_CFG6_MSP_PEAK_NB,
                SX1302_REG_RX_TOP_SF8_CFG7_MSP2_PEAK_NB,
            ),
            (
                SX1302_REG_RX_TOP_SF9_CFG2_ACC_PNR,
                SX1302_REG_RX_TOP_SF9_CFG4_MSP_PNR,
                SX1302_REG_RX_TOP_SF9_CFG6_MSP_PEAK_NB,
                SX1302_REG_RX_TOP_SF9_CFG7_MSP2_PEAK_NB,
            ),
            (
                SX1302_REG_RX_TOP_SF10_CFG2_ACC_PNR,
                SX1302_REG_RX_TOP_SF10_CFG4_MSP_PNR,
                SX1302_REG_RX_TOP_SF10_CFG6_MSP_PEAK_NB,
                SX1302_REG_RX_TOP_SF10_CFG7_MSP2_PEAK_NB,
            ),
            (
                SX1302_REG_RX_TOP_SF11_CFG2_ACC_PNR,
                SX1302_REG_RX_TOP_SF11_CFG4_MSP_PNR,
                SX1302_REG_RX_TOP_SF11_CFG6_MSP_PEAK_NB,
                SX1302_REG_RX_TOP_SF11_CFG7_MSP2_PEAK_NB,
            ),
            (
                SX1302_REG_RX_TOP_SF12_CFG2_ACC_PNR,
                SX1302_REG_RX_TOP_SF12_CFG4_MSP_PNR,
                SX1302_REG_RX_TOP_SF12_CFG6_MSP_PEAK_NB,
                SX1302_REG_RX_TOP_SF12_CFG7_MSP2_PEAK_NB,
            ),
        ];
        for (acc_pnr, msp_pnr, msp_peak_nb, msp2_peak_nb) in SF_CFG {
            self.com.reg_w(acc_pnr, 52).await?;
            self.com.reg_w(msp_pnr, 24).await?;
            self.com.reg_w(msp_peak_nb, 7).await?;
            self.com.reg_w(msp2_peak_nb, 5).await?;
        }

        self.com.reg_w(
            SX1302_REG_RX_TOP_CORRELATOR_ENABLE_ONLY_FIRST_DET_EDGE_ENABLE_ONLY_FIRST_DET_EDGE,
            0xFF,
        ).await?;
        self.com.reg_w(
            SX1302_REG_RX_TOP_CORRELATOR_ENABLE_ACC_CLEAR_ENABLE_CORR_ACC_CLEAR,
            0xFF,
        ).await?;

        // Enabled selected spreading factors
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_CORRELATOR_SF_EN_CORR_SF_EN,
                demod_cfg.multisf_datarate as i32,
            )
            .await?;

        // Enable correlator if channel is enabled (1 correlator per channel)
        for i in 0..LGW_MULTI_NB {
            channels_mask |= (if_cfg[i].enable as u8) << i;
        }
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_CORR_CLOCK_ENABLE_CLK_EN,
                channels_mask as i32,
            )
            .await?;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_CORRELATOR_EN_CORR_EN,
                channels_mask as i32,
            )
            .await?;

        Ok(())
    }

    /// Configure the correlator of the LoRa single-SF (service) modem
    /// (C: `sx1302_lora_service_correlator_configure`).
    pub(crate) async fn sx1302_lora_service_correlator_configure(&mut self) -> Result<()> {
        let cfg = self.ctx.lora_service;

        // Common config for all SF
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DETECT_MSP2_MSP_PEAK_NB,
                7,
            )
            .await?;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DETECT_MSP2_MSP2_PEAK_NB,
                5,
            )
            .await?;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DETECT_ACC1_USE_GAIN_SYMB,
                1,
            )
            .await?;

        let fine_synch_en: i32 = match cfg.datarate {
            DR_LORA_SF5 | DR_LORA_SF6 => 1,
            DR_LORA_SF7
            | DR_LORA_SF8
            | DR_LORA_SF9
            | DR_LORA_SF10
            | DR_LORA_SF11
            | DR_LORA_SF12 => 0,
            // C: "Failed to configure LoRa service modem correlators"
            _ => return Err(Error::InvalidConfig("datarate")),
        };
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_TXRX_CFG2_FINE_SYNCH_EN,
                fine_synch_en,
            )
            .await?;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DETECT_ACC1_ACC_PNR,
                52,
            )
            .await?;

        Ok(())
    }

    /// Configure the LoRa multi-SF modems (C: `sx1302_lora_modem_configure`).
    pub(crate) async fn sx1302_lora_modem_configure(&mut self, radio_freq_hz: u32) -> Result<()> {
        self.com
            .reg_w(SX1302_REG_RX_TOP_DC_NOTCH_CFG1_ENABLE, 0x00)
            .await?;
        self.com
            .reg_w(SX1302_REG_RX_TOP_RX_DFE_AGC1_FORCE_DEFAULT_FIR, 0x01)
            .await?;
        self.com
            .reg_w(SX1302_REG_RX_TOP_DAGC_CFG_GAIN_DROP_COMP, 0x01)
            .await?;
        self.com
            .reg_w(SX1302_REG_RX_TOP_DAGC_CFG_TARGET_LVL, 0x01)
            .await?;

        // Enable full modems
        self.com
            .reg_w(SX1302_REG_OTP_MODEM_EN_0_MODEM_EN, 0xFF)
            .await?;

        // Enable limited modems
        self.com
            .reg_w(SX1302_REG_OTP_MODEM_EN_1_MODEM_EN, 0xFF)
            .await?;

        // Configure coarse sync between correlators and modems
        self.com
            .reg_w(SX1302_REG_RX_TOP_MODEM_SYNC_DELTA_MSB_MODEM_SYNC_DELTA, 0)
            .await?;
        self.com
            .reg_w(SX1302_REG_RX_TOP_MODEM_SYNC_DELTA_LSB_MODEM_SYNC_DELTA, 126)
            .await?;

        // Configure fine sync offset for each channel
        self.com
            .reg_w(SX1302_REG_ARB_MCU_CHANNEL_SYNC_OFFSET_01_CHANNEL_0_OFFSET, 1)
            .await?;
        self.com
            .reg_w(SX1302_REG_ARB_MCU_CHANNEL_SYNC_OFFSET_01_CHANNEL_1_OFFSET, 5)
            .await?;
        self.com
            .reg_w(SX1302_REG_ARB_MCU_CHANNEL_SYNC_OFFSET_23_CHANNEL_2_OFFSET, 9)
            .await?;
        self.com
            .reg_w(SX1302_REG_ARB_MCU_CHANNEL_SYNC_OFFSET_23_CHANNEL_3_OFFSET, 13)
            .await?;
        self.com
            .reg_w(SX1302_REG_ARB_MCU_CHANNEL_SYNC_OFFSET_45_CHANNEL_4_OFFSET, 1)
            .await?;
        self.com
            .reg_w(SX1302_REG_ARB_MCU_CHANNEL_SYNC_OFFSET_45_CHANNEL_5_OFFSET, 5)
            .await?;
        self.com
            .reg_w(SX1302_REG_ARB_MCU_CHANNEL_SYNC_OFFSET_67_CHANNEL_6_OFFSET, 9)
            .await?;
        self.com
            .reg_w(SX1302_REG_ARB_MCU_CHANNEL_SYNC_OFFSET_67_CHANNEL_7_OFFSET, 13)
            .await?;

        // Configure PPM offset
        self.com
            .reg_w(SX1302_REG_RX_TOP_MODEM_PPM_OFFSET1_PPM_OFFSET_SF5, 0x00)
            .await?;
        self.com
            .reg_w(SX1302_REG_RX_TOP_MODEM_PPM_OFFSET1_PPM_OFFSET_SF6, 0x00)
            .await?;
        self.com
            .reg_w(SX1302_REG_RX_TOP_MODEM_PPM_OFFSET1_PPM_OFFSET_SF7, 0x00)
            .await?;
        self.com
            .reg_w(SX1302_REG_RX_TOP_MODEM_PPM_OFFSET1_PPM_OFFSET_SF8, 0x00)
            .await?;
        self.com
            .reg_w(SX1302_REG_RX_TOP_MODEM_PPM_OFFSET2_PPM_OFFSET_SF9, 0x00)
            .await?;
        self.com
            .reg_w(SX1302_REG_RX_TOP_MODEM_PPM_OFFSET2_PPM_OFFSET_SF10, 0x00)
            .await?;
        self.com
            .reg_w(SX1302_REG_RX_TOP_MODEM_PPM_OFFSET2_PPM_OFFSET_SF11, 0x01)
            .await?;
        self.com
            .reg_w(SX1302_REG_RX_TOP_MODEM_PPM_OFFSET2_PPM_OFFSET_SF12, 0x01)
            .await?;

        // Improve SF5 and SF6 performances
        self.com
            .reg_w(SX1302_REG_RX_TOP_FINE_TIMING_A_1_GAIN_P_AUTO, 3)
            .await?; // Default is 1
        self.com
            .reg_w(SX1302_REG_RX_TOP_FINE_TIMING_A_1_GAIN_P_PAYLOAD, 3)
            .await?; // Default is 2

        // Improve SF11/SF12 performances
        self.com
            .reg_w(SX1302_REG_RX_TOP_FINE_TIMING_A_5_GAIN_I_EN_SF11, 1)
            .await?;
        self.com
            .reg_w(SX1302_REG_RX_TOP_FINE_TIMING_A_5_GAIN_I_EN_SF12, 1)
            .await?;

        // Set threshold for 1bin correction (CAN-314)
        self.com
            .reg_w(SX1302_REG_RX_TOP_FREQ_TRACK4_FREQ_SYNCH_THR, 15)
            .await?;

        // Configure modems for best tracking (best demodulation)
        const FREQ_TRACK_A: [u16; 8] = [
            SX1302_REG_RX_TOP_FREQ_TRACK_A_0_FREQ_TRACK_EN_SF5,
            SX1302_REG_RX_TOP_FREQ_TRACK_A_0_FREQ_TRACK_EN_SF6,
            SX1302_REG_RX_TOP_FREQ_TRACK_A_0_FREQ_TRACK_EN_SF7,
            SX1302_REG_RX_TOP_FREQ_TRACK_A_0_FREQ_TRACK_EN_SF8,
            SX1302_REG_RX_TOP_FREQ_TRACK_A_1_FREQ_TRACK_EN_SF9,
            SX1302_REG_RX_TOP_FREQ_TRACK_A_1_FREQ_TRACK_EN_SF10,
            SX1302_REG_RX_TOP_FREQ_TRACK_A_1_FREQ_TRACK_EN_SF11,
            SX1302_REG_RX_TOP_FREQ_TRACK_A_1_FREQ_TRACK_EN_SF12,
        ];
        for reg in FREQ_TRACK_A {
            self.com.reg_w(reg, RX_FREQ_TRACK_AUTO).await?;
        }

        // Configure modems for best timestamping (only valid when double
        // demodulation is enabled)
        const FREQ_TRACK_B: [u16; 8] = [
            SX1302_REG_RX_TOP_FREQ_TRACK_B_0_FREQ_TRACK_EN_SF5,
            SX1302_REG_RX_TOP_FREQ_TRACK_B_0_FREQ_TRACK_EN_SF6,
            SX1302_REG_RX_TOP_FREQ_TRACK_B_0_FREQ_TRACK_EN_SF7,
            SX1302_REG_RX_TOP_FREQ_TRACK_B_0_FREQ_TRACK_EN_SF8,
            SX1302_REG_RX_TOP_FREQ_TRACK_B_1_FREQ_TRACK_EN_SF9,
            SX1302_REG_RX_TOP_FREQ_TRACK_B_1_FREQ_TRACK_EN_SF10,
            SX1302_REG_RX_TOP_FREQ_TRACK_B_1_FREQ_TRACK_EN_SF11,
            SX1302_REG_RX_TOP_FREQ_TRACK_B_1_FREQ_TRACK_EN_SF12,
        ];
        for reg in FREQ_TRACK_B {
            self.com.reg_w(reg, RX_FREQ_TRACK_OFF).await?;
        }

        self.com
            .reg_w(SX1302_REG_RX_TOP_FINE_TIMING_B_5_GAIN_I_EN_SF11, 0)
            .await?;
        self.com
            .reg_w(SX1302_REG_RX_TOP_FINE_TIMING_B_5_GAIN_I_EN_SF12, 0)
            .await?;
        self.com
            .reg_w(SX1302_REG_RX_TOP_FINE_TIMING_B_0_ROUNDING, 1)
            .await?;
        self.com
            .reg_w(SX1302_REG_RX_TOP_FINE_TIMING_B_0_MODE, RX_FINE_TIMING_MODE_LINEAR)
            .await?;
        self.com
            .reg_w(SX1302_REG_RX_TOP_FINE_TIMING_B_1_GAIN_P_AUTO, 0)
            .await?;
        self.com
            .reg_w(SX1302_REG_RX_TOP_FINE_TIMING_B_1_GAIN_P_PREAMB, 6)
            .await?;
        self.com
            .reg_w(SX1302_REG_RX_TOP_FINE_TIMING_B_1_GAIN_P_PAYLOAD, 2)
            .await?;
        self.com
            .reg_w(SX1302_REG_RX_TOP_FINE_TIMING_B_2_GAIN_I_AUTO, 0)
            .await?;
        self.com
            .reg_w(SX1302_REG_RX_TOP_FINE_TIMING_B_2_GAIN_I_PREAMB, 1)
            .await?;
        self.com
            .reg_w(SX1302_REG_RX_TOP_FINE_TIMING_B_2_GAIN_I_PAYLOAD, 0)
            .await?;

        // Set preamble size to 10 (to handle 12 for SF5/SF6 and 8 for SF7->SF12)
        self.com
            .reg_w(SX1302_REG_RX_TOP_TXRX_CFG7_PREAMBLE_SYMB_NB, 0)
            .await?; /* MSB */
        self.com
            .reg_w(SX1302_REG_RX_TOP_TXRX_CFG6_PREAMBLE_SYMB_NB, 10)
            .await?; /* LSB */

        // Freq2TimeDrift computation
        let (mantissa, exponent) = calculate_freq_to_time_drift(radio_freq_hz, BW_125KHZ)?;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_FREQ_TO_TIME0_FREQ_TO_TIME_DRIFT_MANT,
                ((mantissa >> 8) & 0x00FF) as i32,
            )
            .await?;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_FREQ_TO_TIME1_FREQ_TO_TIME_DRIFT_MANT,
                ((mantissa) & 0x00FF) as i32,
            )
            .await?;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_FREQ_TO_TIME2_FREQ_TO_TIME_DRIFT_EXP,
                exponent as i32,
            )
            .await?;

        // Time drift compensation
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_FREQ_TO_TIME3_FREQ_TO_TIME_INVERT_TIME_SYMB,
                1,
            )
            .await?;

        // DFT peak mode : set to AUTO, check timestamp_counter_correction() if changed
        self.com
            .reg_w(SX1302_REG_RX_TOP_RX_CFG0_DFT_PEAK_EN, RX_DFT_PEAK_MODE_AUTO)
            .await?;

        Ok(())
    }

    /// Configure the LoRa single-SF (service) modem
    /// (C: `sx1302_lora_service_modem_configure`).
    pub(crate) async fn sx1302_lora_service_modem_configure(
        &mut self,
        radio_freq_hz: u32,
    ) -> Result<()> {
        let cfg = self.ctx.lora_service;
        let preamble_nb_symb: u8;

        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DC_NOTCH_CFG1_ENABLE,
                0x00,
            )
            .await?;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_DFE_AGC1_FORCE_DEFAULT_FIR,
                0x01,
            )
            .await?;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DAGC_CFG_GAIN_DROP_COMP,
                0x01,
            )
            .await?;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DAGC_CFG_TARGET_LVL,
                0x01,
            )
            .await?;

        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FINE_TIMING1_GAIN_P_AUTO,
                0x03,
            )
            .await?;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FINE_TIMING2_GAIN_I_PAYLOAD,
                0x03,
            )
            .await?;

        match cfg.datarate {
            DR_LORA_SF5 | DR_LORA_SF6 => {
                self.com
                    .reg_w(
                        SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FINE_TIMING1_GAIN_P_PREAMB,
                        0x04,
                    )
                    .await?; // Default value
                self.com
                    .reg_w(
                        SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FINE_TIMING2_GAIN_I_EN,
                        0x00,
                    )
                    .await?; // Default value
            }
            DR_LORA_SF7 | DR_LORA_SF8 | DR_LORA_SF9 | DR_LORA_SF10 => {
                self.com
                    .reg_w(
                        SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FINE_TIMING1_GAIN_P_PREAMB,
                        0x06,
                    )
                    .await?;
                self.com
                    .reg_w(
                        SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FINE_TIMING2_GAIN_I_EN,
                        0x00,
                    )
                    .await?;
            }
            DR_LORA_SF11 | DR_LORA_SF12 => {
                self.com
                    .reg_w(
                        SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FINE_TIMING1_GAIN_P_PREAMB,
                        0x07,
                    )
                    .await?;
                match cfg.bandwidth {
                    BW_125KHZ => {
                        self.com
                            .reg_w(
                                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FINE_TIMING2_GAIN_I_EN,
                                0x01,
                            )
                            .await?;
                    }
                    BW_250KHZ => {
                        self.com
                            .reg_w(
                                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FINE_TIMING2_GAIN_I_EN,
                                0x02,
                            )
                            .await?;
                    }
                    BW_500KHZ => {
                        self.com
                            .reg_w(
                                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FINE_TIMING2_GAIN_I_EN,
                                0x03,
                            )
                            .await?;
                    }
                    // C: unsupported bandwidth message only, execution continues
                    _ => {}
                }
            }
            // C: unsupported datarate message only, execution continues
            _ => {}
        }

        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_TXRX_CFG2_IMPLICIT_HEADER,
                if cfg.implicit_hdr { 1 } else { 0 },
            )
            .await?;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_TXRX_CFG2_CRC_EN,
                if cfg.implicit_crc_en { 1 } else { 0 },
            )
            .await?;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_TXRX_CFG1_CODING_RATE,
                cfg.implicit_coderate as i32,
            )
            .await?;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_TXRX_CFG3_PAYLOAD_LENGTH,
                cfg.implicit_payload_length as i32,
            )
            .await?;

        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_TXRX_CFG0_MODEM_SF,
                cfg.datarate as i32,
            )
            .await?;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_TXRX_CFG0_MODEM_BW,
                cfg.bandwidth as i32,
            )
            .await?;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_TXRX_CFG1_PPM_OFFSET,
                set_ppm_on(cfg.bandwidth, cfg.datarate) as i32,
            )
            .await?;

        // Set preamble size to 8 for SF7->SF12 and to 12 for SF5->SF6
        // (aligned with end-device drivers)
        if (cfg.datarate == DR_LORA_SF5) || (cfg.datarate == DR_LORA_SF6) {
            preamble_nb_symb = 12;
        } else {
            preamble_nb_symb = 8;
        }
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_TXRX_CFG7_PREAMBLE_SYMB_NB,
                ((preamble_nb_symb >> 8) & 0xFF) as i32,
            )
            .await?; /* MSB */
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_TXRX_CFG6_PREAMBLE_SYMB_NB,
                ((preamble_nb_symb >> 0) & 0xFF) as i32,
            )
            .await?; /* LSB */

        // Freq2TimeDrift computation
        let (mantissa, exponent) =
            calculate_freq_to_time_drift(radio_freq_hz, cfg.bandwidth)?;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FREQ_TO_TIME0_FREQ_TO_TIME_DRIFT_MANT,
                ((mantissa >> 8) & 0x00FF) as i32,
            )
            .await?;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FREQ_TO_TIME1_FREQ_TO_TIME_DRIFT_MANT,
                ((mantissa) & 0x00FF) as i32,
            )
            .await?;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FREQ_TO_TIME2_FREQ_TO_TIME_DRIFT_EXP,
                exponent as i32,
            )
            .await?;

        // Time drift compensation
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FREQ_TO_TIME3_FREQ_TO_TIME_INVERT_TIME_SYMB,
                1,
            )
            .await?;

        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_DFE_AGC2_DAGC_IN_COMP,
                1,
            )
            .await?;

        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_TXRX_CFG1_MODEM_EN,
                1,
            )
            .await?;
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_TXRX_CFG2_CADRXTX,
                1,
            )
            .await?;

        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_TXRX_CFG2_MODEM_START,
                1,
            )
            .await?;

        // DFT peak mode : set to AUTO, check timestamp_counter_correction() if changed
        self.com
            .reg_w(
                SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_CFG0_DFT_PEAK_EN,
                RX_DFT_PEAK_MODE_AUTO,
            )
            .await?;

        Ok(())
    }

    /// Enable the modems and RX (C: `sx1302_modem_enable`).
    pub(crate) async fn sx1302_modem_enable(&mut self) -> Result<()> {
        trace!("enabling demodulators");
        // Enable LoRa multi-SF modems
        self.com
            .reg_w(SX1302_REG_COMMON_GEN_CONCENTRATOR_MODEM_ENABLE, 0x01)
            .await?;

        // Enable LoRa service modem
        self.com
            .reg_w(SX1302_REG_COMMON_GEN_MBWSSF_MODEM_ENABLE, 0x01)
            .await?;

        // Enable FSK modem
        self.com
            .reg_w(SX1302_REG_COMMON_GEN_FSK_MODEM_ENABLE, 0x01)
            .await?;

        // Enable RX
        self.com
            .reg_w(SX1302_REG_COMMON_GEN_GLOBAL_EN, 0x01)
            .await?;

        Ok(())
    }

    /// Configure the sync word of the LoRa modems (public 0x34, private
    /// 0x12) (C: `sx1302_lora_syncword`).
    pub(crate) async fn sx1302_lora_syncword(
        &mut self,
        public: bool,
        lora_service_sf: u8,
    ) -> Result<()> {
        // Multi-SF modem configuration: SF5->SF6 private
        self.com
            .reg_w(SX1302_REG_RX_TOP_FRAME_SYNCH0_SF5_PEAK1_POS_SF5, 2)
            .await?;
        self.com
            .reg_w(SX1302_REG_RX_TOP_FRAME_SYNCH1_SF5_PEAK2_POS_SF5, 4)
            .await?;
        self.com
            .reg_w(SX1302_REG_RX_TOP_FRAME_SYNCH0_SF6_PEAK1_POS_SF6, 2)
            .await?;
        self.com
            .reg_w(SX1302_REG_RX_TOP_FRAME_SYNCH1_SF6_PEAK2_POS_SF6, 4)
            .await?;
        if public {
            self.com
                .reg_w(
                    SX1302_REG_RX_TOP_FRAME_SYNCH0_SF7TO12_PEAK1_POS_SF7TO12,
                    6,
                )
                .await?;
            self.com
                .reg_w(
                    SX1302_REG_RX_TOP_FRAME_SYNCH1_SF7TO12_PEAK2_POS_SF7TO12,
                    8,
                )
                .await?;
        } else {
            self.com
                .reg_w(
                    SX1302_REG_RX_TOP_FRAME_SYNCH0_SF7TO12_PEAK1_POS_SF7TO12,
                    2,
                )
                .await?;
            self.com
                .reg_w(
                    SX1302_REG_RX_TOP_FRAME_SYNCH1_SF7TO12_PEAK2_POS_SF7TO12,
                    4,
                )
                .await?;
        }

        // LoRa Service modem configuration
        if (!public)
            || (lora_service_sf == DR_LORA_SF5 as u8)
            || (lora_service_sf == DR_LORA_SF6 as u8)
        {
            self.com
                .reg_w(
                    SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FRAME_SYNCH0_PEAK1_POS,
                    2,
                )
                .await?;
            self.com
                .reg_w(
                    SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FRAME_SYNCH1_PEAK2_POS,
                    4,
                )
                .await?;
        } else {
            self.com
                .reg_w(
                    SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FRAME_SYNCH0_PEAK1_POS,
                    6,
                )
                .await?;
            self.com
                .reg_w(
                    SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FRAME_SYNCH1_PEAK2_POS,
                    8,
                )
                .await?;
        }

        Ok(())
    }

    /// Get the current SX1302 internal counter value
    /// (C: `sx1302_timestamp_counter`).
    pub(crate) async fn sx1302_timestamp_counter(&mut self, pps: bool) -> Result<u32> {
        let (inst_cnt, pps_cnt) = self.timestamp_counter_get().await?;
        let cnt = if pps { pps_cnt } else { inst_cnt };
        Ok(self.timestamp_counter_expand(pps, cnt))
    }

    /// Enable / disable the GPS input for PPS and counter sampling
    /// (C: `sx1302_gps_enable`).
    ///
    /// Only the register writes are ported; the GPS UART itself is out of
    /// scope.
    pub(crate) async fn sx1302_gps_enable(&mut self, enable: bool) -> Result<()> {
        if enable {
            self.com
                .reg_w(SX1302_REG_TIMESTAMP_GPS_CTRL_GPS_EN, 1)
                .await?;
            self.com
                .reg_w(SX1302_REG_TIMESTAMP_GPS_CTRL_GPS_POL, 1)
                .await?; /* invert polarity for PPS */
        } else {
            self.com
                .reg_w(SX1302_REG_TIMESTAMP_GPS_CTRL_GPS_EN, 0)
                .await?;
        }

        Ok(())
    }
}
