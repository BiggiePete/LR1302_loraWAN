//! SX1250 radio driver (C: `loragw_sx1250.c` / `sx1250_defs.h`).
//!
//! Only the SX1250 radio used by the LR1302/SX1302 design is supported;
//! the SX1255/1257 (`loragw_sx125x.c`) and SX1261 (LBT) paths of the C HAL
//! are not ported.

use crate::error::{Error, Result};
use crate::spi::Com;
use crate::util::take_bits;

/// SX1250 op codes (C: `sx1250_op_code_t`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum Op {
    SetSleep = 0x84,
    SetStandby = 0x80,
    SetFs = 0xC1,
    SetTx = 0x83,
    SetRx = 0x82,
    SetRfFrequency = 0x86,
    SetPacketType = 0x8A,
    SetTxParams = 0x8E,
    SetModulationParams = 0x8B,
    SetPacketParams = 0x8C,
    SetBufferBaseAddress = 0x8F,
    SetPaConfig = 0x95,
    SetRegulatorMode = 0x96,
    SetDioIrqParams = 0x08,
    Calibrate = 0x89,
    CalibrateImage = 0x98,
    SetRfSwitchMode = 0x9D,
    StopTimerOnPreamble = 0x9F,
    GetStatus = 0xC0,
    GetIrqStatus = 0x12,
    GetRxBufferStatus = 0x13,
    GetPacketStatus = 0x14,
    GetDeviceErrors = 0x17,
    ClrIrqStatus = 0x02,
    WriteRegister = 0x0D,
    ReadRegister = 0x1D,
    WriteBuffer = 0x0E,
    ReadBuffer = 0x1E,
    SetTxCw = 0xD1,
    SetTxContinuousPreamble = 0xD2,
}

/// Standby modes (C: `sx1250_standby_modes_t`).
pub const STDBY_RC: u8 = 0x00;
pub const STDBY_XOSC: u8 = 0x01;

/// Convert a frequency in Hz to the SX1302/SX1250 `SET_RF_FREQUENCY`
/// argument (C: `SX1250_FREQ_TO_REG`).
pub const fn freq_to_reg(freq_hz: u32) -> u32 {
    // (f * 2^25 / 32_000_000), evaluated with u64 to avoid overflow
    ((freq_hz as u64 * (1u64 << 25)) / 32_000_000) as u32
}

impl<SPI, D> Com<SPI, D>
where
    SPI: embedded_hal_async::spi::SpiDevice<u8>,
    D: embedded_hal_async::delay::DelayNs,
{
    /// Write an SX1250 register/command frame (C: `sx1250_reg_w`).
    pub async fn sx1250_w(&mut self, rf_chain: u8, op: Op, data: &[u8]) -> Result<()> {
        self.radio_w(rf_chain, op as u8, data).await
    }

    /// Read from an SX1250 register/command (C: `sx1250_reg_r`).
    ///
    /// `data` must be pre-filled with the bytes to clock out (the C HAL does
    /// the same: for `READ_REGISTER` the caller puts the address in it).
    pub async fn sx1250_r(&mut self, rf_chain: u8, op: Op, data: &mut [u8]) -> Result<()> {
        self.radio_r(rf_chain, op as u8, data).await
    }

    /// Image calibration for the given frequency (C: `sx1250_calibrate`).
    pub async fn sx1250_calibrate(&mut self, rf_chain: u8, freq_hz: u32) -> Result<()> {
        let mut buff = [0u8; 3];
        self.sx1250_r(rf_chain, Op::GetStatus, &mut buff[..1]).await?;

        // image filter calibration words per frequency range
        let (lo, hi): (u8, u8) = if freq_hz > 430_000_000 && freq_hz < 440_000_000 {
            (0x6B, 0x6F)
        } else if freq_hz > 470_000_000 && freq_hz < 510_000_000 {
            (0x75, 0x81)
        } else if freq_hz > 779_000_000 && freq_hz < 787_000_000 {
            (0xC1, 0xC5)
        } else if freq_hz > 863_000_000 && freq_hz < 870_000_000 {
            (0xD7, 0xDB)
        } else if freq_hz > 902_000_000 && freq_hz < 928_000_000 {
            (0xE1, 0xE9)
        } else {
            return Err(Error::InvalidConfig("sx1250 calibration frequency"));
        };
        buff[0] = lo;
        buff[1] = hi;
        self.sx1250_w(rf_chain, Op::CalibrateImage, &buff[..2]).await?;

        self.wait_ms(10).await;

        buff[0] = 0x00;
        buff[1] = 0x00;
        buff[2] = 0x00;
        self.sx1250_r(rf_chain, Op::GetDeviceErrors, &mut buff[..3])
            .await?;
        if take_bits(buff[2], 4, 1) != 0 {
            return Err(Error::Radio(buff[2]));
        }
        Ok(())
    }

    /// Full radio bring-up: standby, calibrations, frequency, RX continuous
    /// (C: `sx1250_setup`).
    pub async fn sx1250_setup(
        &mut self,
        rf_chain: u8,
        freq_hz: u32,
        single_input_mode: bool,
    ) -> Result<()> {
        let mut buff = [0u8; 16];

        // standby for calibrations
        buff[0] = STDBY_RC;
        self.sx1250_w(rf_chain, Op::SetStandby, &buff[..1]).await?;
        self.wait_ms(10).await;

        buff[0] = 0x00;
        self.sx1250_r(rf_chain, Op::GetStatus, &mut buff[..1]).await?;
        if take_bits(buff[0], 4, 3) != 0x02 {
            return Err(Error::Radio(buff[0]));
        }

        // run all calibrations (TCXO)
        buff[0] = 0x7F;
        self.sx1250_w(rf_chain, Op::Calibrate, &buff[..1]).await?;
        self.wait_ms(10).await;

        // standby with XOSC on
        buff[0] = STDBY_XOSC;
        self.sx1250_w(rf_chain, Op::SetStandby, &buff[..1]).await?;
        self.wait_ms(10).await;

        buff[0] = 0x00;
        self.sx1250_r(rf_chain, Op::GetStatus, &mut buff[..1]).await?;
        if take_bits(buff[0], 4, 3) != 0x03 {
            return Err(Error::Radio(buff[0]));
        }

        // maximum bitrate, to lower the TX -> FS switch time
        for (hi, lo, val) in [(0x06u8, 0xA1u8, 0x01u8), (0x06, 0xA2, 0x00), (0x06, 0xA3, 0x00)] {
            self.sx1250_w(rf_chain, Op::WriteRegister, &[hi, lo, val])
                .await?;
        }

        // configure DIOs for RX: drive strength, inputs, pulls, outputs
        for addr in [0x0582u16, 0x0583, 0x0584, 0x0585, 0x0580] {
            self.sx1250_w(
                rf_chain,
                Op::WriteRegister,
                &[(addr >> 8) as u8, (addr & 0xFF) as u8, 0x00],
            )
            .await?;
        }

        // fixed gain
        self.sx1250_w(rf_chain, Op::WriteRegister, &[0x08, 0xB6, 0x2A])
            .await?;

        // frequency
        let freq_reg = freq_to_reg(freq_hz);
        buff[0] = (freq_reg >> 24) as u8;
        buff[1] = (freq_reg >> 16) as u8;
        buff[2] = (freq_reg >> 8) as u8;
        buff[3] = (freq_reg >> 0) as u8;
        self.sx1250_w(rf_chain, Op::SetRfFrequency, &buff[..4]).await?;

        // frequency offset = 0
        self.sx1250_w(rf_chain, Op::WriteRegister, &[0x08, 0x8F, 0x00, 0x00, 0x00])
            .await?;

        // RX continuous: gives a clock to the SX1302
        self.sx1250_w(rf_chain, Op::SetRx, &[0xFF, 0xFF, 0xFF]).await?;

        // single or differential input mode
        if single_input_mode {
            self.sx1250_w(rf_chain, Op::WriteRegister, &[0x08, 0xE2, 0x0D])
                .await?;
        }

        // FPGA_MODE_RX
        self.sx1250_w(rf_chain, Op::WriteRegister, &[0x05, 0x87, 0x0B])
            .await?;

        Ok(())
    }
}
