//! SX1302 TX path (C: `sx1302_tx_*`, `sx1302_send`, `sx1302_set_gpio` of
//! `libloragw/src/loragw_sx1302.c`).
//!
//! Everything USB-specific of the C TX path (`lgw_com_set_write_mode`,
//! `lgw_com_flush`), the `printf`/`DEBUG_*` traces and the performance
//! timers are dropped; the SX1255/1257, SX1261 LBT and AD5338R full duplex
//! branches are not ported and report [`Error::Unsupported`] instead.

use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::spi::SpiDevice;

use crate::consts::*;
use crate::error::{Error, Result};
use crate::reg;
use crate::sx1302_defs::{
    reg_select, set_ppm_on, sx1302_freq_to_reg, MIN_FSK_PREAMBLE, MIN_LORA_PREAMBLE,
    STD_FSK_PREAMBLE, STD_LORA_PREAMBLE, TX_START_DELAY_DEFAULT,
};
use crate::types::TxPacket;
use crate::util;
use crate::Lr1302;

impl<SPI, D> Lr1302<SPI, D>
where
    SPI: SpiDevice<u8>,
    D: DelayNs,
{
    /// Compute the TX start delay and program it in the concentrator
    /// (C: `sx1302_tx_set_start_delay`).
    ///
    /// Returns the delay that was written (0 when the modulation is not
    /// LoRa), mirroring the C `uint16_t * delay` output argument.
    pub(crate) async fn sx1302_tx_set_start_delay(
        &mut self,
        rf_chain: u8,
        radio_type: RadioType,
        modulation: u8,
        bandwidth: u8,
        chirp_lowpass: u8,
    ) -> Result<u16> {
        let mut tx_start_delay: u16 = TX_START_DELAY_DEFAULT * 32;
        let radio_bw_delay: u16;

        // tx start delay only necessary for beaconing (LoRa)
        if modulation != MOD_LORA {
            return Ok(0);
        }

        // Adjust with radio type and bandwidth
        // (C also handles LGW_RADIO_TYPE_SX1255/SX1257; those radios are out
        // of scope for this port.)
        match radio_type {
            RadioType::Sx1250 => {
                radio_bw_delay = match bandwidth {
                    BW_125KHZ => 19,
                    BW_250KHZ => 24,
                    BW_500KHZ => 21,
                    _ => return Err(Error::InvalidConfig("bandwidth")),
                };
            }
            _ => return Err(Error::Unsupported),
        }

        // Adjust with modulation
        let bw_hz = util::bw_getval(bandwidth).ok_or(Error::InvalidConfig("bandwidth"))?;
        if chirp_lowpass >= 31 {
            return Err(Error::InvalidConfig("chirp_lowpass"));
        }
        // C: `((1 << chirp_lowpass) - 1) * 1e6 / bw_hz` (double, truncated
        // to uint16_t)
        let filter_delay = (((1i32 << chirp_lowpass) - 1) as f64 * 1e6 / bw_hz as f64) as u16;
        // C: `8 * (32e6 / (32 * bw_hz))` (double, truncated to uint16_t)
        let modem_delay = (8.0 * (32e6 / (32.0 * bw_hz as f64))) as u16;

        // Compute total delay (C: uint16_t subtraction, wrapping)
        tx_start_delay = (tx_start_delay as u32)
            .wrapping_sub(radio_bw_delay as u32 + filter_delay as u32 + modem_delay as u32)
            as u16;

        // write the 16 bits delay register, MSB first
        let buff = [(tx_start_delay >> 8) as u8, (tx_start_delay >> 0) as u8];
        self.com
            .reg_wb(
                reg::tx_top_tx_start_delay_msb_tx_start_delay(rf_chain),
                &buff,
            )
            .await?;

        Ok(tx_start_delay)
    }

    /// Read the status of the TX modem (C: `sx1302_tx_status`).
    ///
    /// Returns one of [`TX_STATUS_UNKNOWN`], [`TX_FREE`], [`TX_EMITTING`] or
    /// [`TX_SCHEDULED`]. As in C, a failed register read (and any value that
    /// is not a known FSM state) reports [`TX_STATUS_UNKNOWN`].
    pub(crate) async fn sx1302_tx_status(&mut self, rf_chain: u8) -> Result<u8> {
        let read_value = match self
            .com
            .reg_r(reg::tx_top_tx_fsm_status_tx_status(rf_chain))
            .await
        {
            Ok(v) => v,
            Err(_) => return Ok(TX_STATUS_UNKNOWN),
        };

        if read_value == 0x80 {
            Ok(TX_FREE)
        } else if matches!(read_value, 0x30 | 0x50 | 0x60 | 0x70) {
            Ok(TX_EMITTING)
        } else if matches!(read_value, 0x91 | 0x92) {
            Ok(TX_SCHEDULED)
        } else {
            Ok(TX_STATUS_UNKNOWN)
        }
    }

    /// Read the status of the RX modem (C: `sx1302_rx_status`).
    ///
    /// Not implemented in the C HAL either: it always reports
    /// [`RX_STATUS_UNKNOWN`].
    pub(crate) async fn sx1302_rx_status(&mut self, rf_chain: u8) -> Result<u8> {
        let _ = rf_chain; /* dummy for compilation, as in C */
        Ok(RX_STATUS_UNKNOWN)
    }

    /// Stop any scheduled or running transmission (C: `sx1302_tx_abort`).
    pub(crate) async fn sx1302_tx_abort(&mut self, rf_chain: u8) -> Result<()> {
        self.com
            .reg_w(reg::tx_top_tx_trig_tx_trig_immediate(rf_chain), 0x00)
            .await?;
        self.com
            .reg_w(reg::tx_top_tx_trig_tx_trig_delayed(rf_chain), 0x00)
            .await?;
        self.com
            .reg_w(reg::tx_top_tx_trig_tx_trig_gps(rf_chain), 0x00)
            .await?;

        // C: polls the TX status every 1 ms with a 1000 ms timeout
        for _ in 0..1000 {
            let tx_status = self.sx1302_tx_status(rf_chain).await?;
            if tx_status == TX_FREE {
                return Ok(());
            }
            self.com.wait_ms(1).await;
        }
        Err(Error::Timeout)
    }

    /// Configure the static TX destination interface (C: `sx1302_tx_configure`).
    pub(crate) async fn sx1302_tx_configure(&mut self, radio_type: RadioType) -> Result<()> {
        // Select the TX destination interface
        match radio_type {
            RadioType::Sx1250 => {
                // Let AGC control PLL DIV (sx1250 only)
                self.com
                    .reg_w(reg::SX1302_REG_TX_TOP_A_TX_RFFE_IF_CTRL2_PLL_DIV_CTRL_AGC, 1)
                    .await?;
                self.com
                    .reg_w(reg::SX1302_REG_TX_TOP_B_TX_RFFE_IF_CTRL2_PLL_DIV_CTRL_AGC, 1)
                    .await?;

                // SX126x Tx RFFE
                self.com
                    .reg_w(reg::SX1302_REG_TX_TOP_A_TX_RFFE_IF_CTRL_TX_IF_DST, 0x01)
                    .await?;
                self.com
                    .reg_w(reg::SX1302_REG_TX_TOP_B_TX_RFFE_IF_CTRL_TX_IF_DST, 0x01)
                    .await?;
            }
            // C: LGW_RADIO_TYPE_SX1255 / LGW_RADIO_TYPE_SX1257 are not ported
            _ => return Err(Error::Unsupported),
        }

        // Configure the TX mode of operation (modulation)
        self.com
            .reg_w(reg::SX1302_REG_TX_TOP_A_TX_RFFE_IF_CTRL_TX_MODE, 0x01)
            .await?;
        self.com
            .reg_w(reg::SX1302_REG_TX_TOP_B_TX_RFFE_IF_CTRL_TX_MODE, 0x01)
            .await?;

        // Configure the output data clock edge (data on rising edge)
        self.com
            .reg_w(reg::SX1302_REG_TX_TOP_A_TX_RFFE_IF_CTRL_TX_CLK_EDGE, 0x00)
            .await?;
        self.com
            .reg_w(reg::SX1302_REG_TX_TOP_B_TX_RFFE_IF_CTRL_TX_CLK_EDGE, 0x00)
            .await?;

        Ok(())
    }

    /// Load a packet in the TX buffer and trigger its transmission
    /// (C: `sx1302_send`).
    ///
    /// The C arguments `radio_type`, `tx_lut`, `lwan_public` and
    /// `context_fsk` are read from the context (see PORTING.md §6) and copied
    /// into locals before the first `.await`.
    pub(crate) async fn sx1302_send(&mut self, pkt: &TxPacket) -> Result<()> {
        let rf_chain = pkt.rf_chain;
        if rf_chain as usize >= LGW_RF_CHAIN_NB {
            return Err(Error::InvalidConfig("rf_chain"));
        }
        if pkt.size as usize > pkt.payload.len() {
            return Err(Error::InvalidConfig("size"));
        }

        // C: `radio_type`, `tx_lut`, `lwan_public` and `context_fsk`
        // arguments, taken from the driver context
        let radio_type = self.ctx.rf_chain[rf_chain as usize].type_;
        let tx_lut = self.ctx.tx_gain_lut[rf_chain as usize];
        let lwan_public = self.ctx.board.lorawan_public;
        let context_fsk = self.ctx.fsk;

        // Select the proper modem
        match pkt.modulation {
            MOD_CW => {
                self.com
                    .reg_w(reg::tx_top_gen_cfg_0_modulation_type(rf_chain), 0x00)
                    .await?;
                self.com
                    .reg_w(reg::tx_top_tx_rffe_if_ctrl_tx_if_src(rf_chain), 0x00)
                    .await?;
            }
            MOD_LORA => {
                self.com
                    .reg_w(reg::tx_top_gen_cfg_0_modulation_type(rf_chain), 0x00)
                    .await?;
                self.com
                    .reg_w(reg::tx_top_tx_rffe_if_ctrl_tx_if_src(rf_chain), 0x01)
                    .await?;
            }
            MOD_FSK => {
                self.com
                    .reg_w(reg::tx_top_gen_cfg_0_modulation_type(rf_chain), 0x01)
                    .await?;
                self.com
                    .reg_w(reg::tx_top_tx_rffe_if_ctrl_tx_if_src(rf_chain), 0x02)
                    .await?;
            }
            _ => return Err(Error::InvalidConfig("modulation")),
        }

        // Find the proper index in the TX gain LUT according to requested rf_power
        if tx_lut.size == 0 || tx_lut.size as usize > TX_GAIN_LUT_SIZE_MAX {
            return Err(Error::InvalidConfig("tx_gain_lut"));
        }
        let mut pow_index = tx_lut.size as usize - 1;
        while pow_index > 0 && tx_lut.lut[pow_index].rf_power > pkt.rf_power {
            pow_index -= 1;
        }
        let gain = tx_lut.lut[pow_index];

        // loading calibrated Tx DC offsets
        self.com
            .reg_w(
                reg::tx_top_tx_rffe_if_i_offset_i_offset(rf_chain),
                gain.offset_i as i32,
            )
            .await?;
        self.com
            .reg_w(
                reg::tx_top_tx_rffe_if_q_offset_q_offset(rf_chain),
                gain.offset_q as i32,
            )
            .await?;

        // Set the power parameters to be used for TX
        // (C also handles LGW_RADIO_TYPE_SX1255/SX1257, not ported here)
        let power: u8 = match radio_type {
            RadioType::Sx1250 => {
                // only 1 bit used to control the external PA
                let pa_en: u8 = if gain.pa_gain > 0 { 1 } else { 0 };
                (pa_en << 6) | gain.pwr_idx
            }
            _ => return Err(Error::Unsupported),
        };
        self.com
            .reg_w(reg::tx_top_agc_tx_pwr_agc_tx_pwr(rf_chain), power as i32)
            .await?;

        // Set digital gain
        self.com
            .reg_w(
                reg::tx_top_tx_rffe_if_iq_gain_iq_gain(rf_chain),
                gain.dig_gain as i32,
            )
            .await?;

        // Set Tx frequency (the C SX1255 branch doubles it; not ported)
        let freq_reg = sx1302_freq_to_reg(pkt.freq_hz as u64);
        self.com
            .reg_w(
                reg::tx_top_tx_rffe_if_freq_rf_h_freq_rf(rf_chain),
                ((freq_reg >> 16) & 0xFF) as i32,
            )
            .await?;
        self.com
            .reg_w(
                reg::tx_top_tx_rffe_if_freq_rf_m_freq_rf(rf_chain),
                ((freq_reg >> 8) & 0xFF) as i32,
            )
            .await?;
        self.com
            .reg_w(
                reg::tx_top_tx_rffe_if_freq_rf_l_freq_rf(rf_chain),
                ((freq_reg >> 0) & 0xFF) as i32,
            )
            .await?;

        // Set AGC bandwidth and modulation type
        let mod_bw: u8 = match pkt.modulation {
            MOD_LORA => pkt.bandwidth,
            MOD_CW | MOD_FSK => (0x01 << 7) | pkt.bandwidth,
            _ => return Err(Error::InvalidConfig("modulation")),
        };
        self.com
            .reg_w(reg::tx_top_agc_tx_bw_agc_tx_bw(rf_chain), mod_bw as i32)
            .await?;

        // Chirp lowpass filter, only set by the LoRa branch (C initializes it
        // to 0 and `sx1302_tx_set_start_delay` only uses it for LoRa)
        let mut chirp_lowpass: u8 = 0;

        // Configure modem
        match pkt.modulation {
            MOD_CW => {
                // Set frequency deviation
                // C: `ceil(fabs((float)freq_offset / 10)) * 10e3`
                // (`no_std` has no libm `ceil`, so the round-up of the
                // already non-negative value is done explicitly)
                let offset_ratio = (pkt.freq_offset as f32) / 10.0f32;
                let offset_abs = if offset_ratio < 0.0 {
                    -offset_ratio
                } else {
                    offset_ratio
                };
                let offset_trunc = offset_abs as i64 as f32; // truncates toward zero
                let offset_ceil = if offset_abs > offset_trunc {
                    offset_trunc + 1.0
                } else {
                    offset_trunc
                };
                let freq_dev: u32 = (offset_ceil as f64 * 10e3) as u32;
                let fdev_reg = sx1302_freq_to_reg(freq_dev as u64);
                self.com
                    .reg_w(
                        reg::tx_top_tx_rffe_if_freq_dev_h_freq_dev(rf_chain),
                        ((fdev_reg >> 8) & 0xFF) as i32,
                    )
                    .await?;
                self.com
                    .reg_w(
                        reg::tx_top_tx_rffe_if_freq_dev_l_freq_dev(rf_chain),
                        ((fdev_reg >> 0) & 0xFF) as i32,
                    )
                    .await?;

                // Send frequency deviation to AGC fw for radio config
                let fdev_reg = crate::sx1250::freq_to_reg(freq_dev);
                self.com
                    .reg_w(
                        reg::SX1302_REG_AGC_MCU_MCU_MAIL_BOX_WR_DATA_BYTE2_MCU_MAIL_BOX_WR_DATA,
                        ((fdev_reg >> 16) & 0xFF) as i32,
                    )
                    .await?;
                self.com
                    .reg_w(
                        reg::SX1302_REG_AGC_MCU_MCU_MAIL_BOX_WR_DATA_BYTE1_MCU_MAIL_BOX_WR_DATA,
                        ((fdev_reg >> 8) & 0xFF) as i32,
                    )
                    .await?;
                self.com
                    .reg_w(
                        reg::SX1302_REG_AGC_MCU_MCU_MAIL_BOX_WR_DATA_BYTE0_MCU_MAIL_BOX_WR_DATA,
                        ((fdev_reg >> 0) & 0xFF) as i32,
                    )
                    .await?;

                // Set the frequency offset (ratio of the frequency deviation)
                // C: `(int)(((float)freq_offset * 1e3 * 64) / (float)freq_dev)`
                let test_mod_freq =
                    ((pkt.freq_offset as f32 as f64 * 1e3 * 64.0) / (freq_dev as f32 as f64))
                        as i32;
                self.com
                    .reg_w(
                        reg::tx_top_tx_rffe_if_test_mod_freq(rf_chain),
                        test_mod_freq,
                    )
                    .await?;
            }
            MOD_LORA => {
                // Set bandwidth
                let bw_hz =
                    util::bw_getval(pkt.bandwidth).ok_or(Error::InvalidConfig("bandwidth"))?;
                let freq_dev = bw_hz / 2;
                let fdev_reg = sx1302_freq_to_reg(freq_dev as u64);
                self.com
                    .reg_w(
                        reg::tx_top_tx_rffe_if_freq_dev_h_freq_dev(rf_chain),
                        ((fdev_reg >> 8) & 0xFF) as i32,
                    )
                    .await?;
                self.com
                    .reg_w(
                        reg::tx_top_tx_rffe_if_freq_dev_l_freq_dev(rf_chain),
                        ((fdev_reg >> 0) & 0xFF) as i32,
                    )
                    .await?;
                self.com
                    .reg_w(
                        reg::tx_top_txrx_cfg0_0_modem_bw(rf_chain),
                        pkt.bandwidth as i32,
                    )
                    .await?;

                // Preamble length (0: use the recommended LoRa preamble size,
                // then enforce the minimum preamble size)
                let mut preamble = pkt.preamble;
                if preamble == 0 {
                    preamble = STD_LORA_PREAMBLE;
                } else if preamble < MIN_LORA_PREAMBLE {
                    preamble = MIN_LORA_PREAMBLE;
                }
                self.com
                    .reg_w(
                        reg::tx_top_txrx_cfg1_3_preamble_symb_nb(rf_chain),
                        ((preamble >> 8) & 0xFF) as i32,
                    )
                    .await?;
                self.com
                    .reg_w(
                        reg::tx_top_txrx_cfg1_2_preamble_symb_nb(rf_chain),
                        ((preamble >> 0) & 0xFF) as i32,
                    )
                    .await?;

                // LoRa datarate
                self.com
                    .reg_w(
                        reg::tx_top_txrx_cfg0_0_modem_sf(rf_chain),
                        pkt.datarate as i32,
                    )
                    .await?;

                // Chirp filtering
                chirp_lowpass = if pkt.datarate < 10 { 6 } else { 7 };
                self.com
                    .reg_w(
                        reg::tx_top_tx_cfg0_0_chirp_lowpass(rf_chain),
                        chirp_lowpass as i32,
                    )
                    .await?;

                // Coding Rate
                self.com
                    .reg_w(
                        reg::tx_top_txrx_cfg0_1_coding_rate(rf_chain),
                        pkt.coderate as i32,
                    )
                    .await?;

                // Start LoRa modem
                self.com
                    .reg_w(reg::tx_top_txrx_cfg0_2_modem_en(rf_chain), 1)
                    .await?;
                self.com
                    .reg_w(reg::tx_top_txrx_cfg0_2_cadrxtx(rf_chain), 2)
                    .await?;
                self.com
                    .reg_w(reg::tx_top_txrx_cfg1_1_modem_start(rf_chain), 1)
                    .await?;
                self.com
                    .reg_w(reg::tx_top_tx_cfg0_0_continuous(rf_chain), 0)
                    .await?;

                // Modulation options
                self.com
                    .reg_w(
                        reg::tx_top_tx_cfg0_0_chirp_invert(rf_chain),
                        if pkt.invert_pol { 1 } else { 0 },
                    )
                    .await?;
                self.com
                    .reg_w(
                        reg::tx_top_txrx_cfg0_2_implicit_header(rf_chain),
                        if pkt.no_header { 1 } else { 0 },
                    )
                    .await?;
                self.com
                    .reg_w(
                        reg::tx_top_txrx_cfg0_2_crc_en(rf_chain),
                        if pkt.no_crc { 0 } else { 1 },
                    )
                    .await?;

                // Syncword (0x12 private / 0x34 public, forced private for
                // SF5 and SF6)
                if !lwan_public
                    || pkt.datarate == DR_LORA_SF5
                    || pkt.datarate == DR_LORA_SF6
                {
                    self.com
                        .reg_w(reg::tx_top_frame_synch_0_peak1_pos(rf_chain), 2)
                        .await?;
                    self.com
                        .reg_w(reg::tx_top_frame_synch_1_peak2_pos(rf_chain), 4)
                        .await?;
                } else {
                    self.com
                        .reg_w(reg::tx_top_frame_synch_0_peak1_pos(rf_chain), 6)
                        .await?;
                    self.com
                        .reg_w(reg::tx_top_frame_synch_1_peak2_pos(rf_chain), 8)
                        .await?;
                }

                // Set Fine Sync for SF5/SF6
                if pkt.datarate == DR_LORA_SF5 || pkt.datarate == DR_LORA_SF6 {
                    self.com
                        .reg_w(reg::tx_top_txrx_cfg0_2_fine_synch_en(rf_chain), 1)
                        .await?;
                } else {
                    self.com
                        .reg_w(reg::tx_top_txrx_cfg0_2_fine_synch_en(rf_chain), 0)
                        .await?;
                }

                // Set Payload length
                self.com
                    .reg_w(
                        reg::tx_top_txrx_cfg0_3_payload_length(rf_chain),
                        pkt.size as i32,
                    )
                    .await?;

                // Set PPM offset (low datarate optimization)
                self.com
                    .reg_w(reg::tx_top_txrx_cfg0_1_ppm_offset_hdr_ctrl(rf_chain), 0)
                    .await?;
                if set_ppm_on(pkt.bandwidth, pkt.datarate) {
                    self.com
                        .reg_w(reg::tx_top_txrx_cfg0_1_ppm_offset(rf_chain), 1)
                        .await?;
                } else {
                    self.com
                        .reg_w(reg::tx_top_txrx_cfg0_1_ppm_offset(rf_chain), 0)
                        .await?;
                }
            }
            MOD_FSK => {
                // Set frequency deviation
                let freq_dev = (pkt.f_dev as f64 * 1e3) as u32;
                let fdev_reg = sx1302_freq_to_reg(freq_dev as u64);
                self.com
                    .reg_w(
                        reg::tx_top_tx_rffe_if_freq_dev_h_freq_dev(rf_chain),
                        ((fdev_reg >> 8) & 0xFF) as i32,
                    )
                    .await?;
                self.com
                    .reg_w(
                        reg::tx_top_tx_rffe_if_freq_dev_l_freq_dev(rf_chain),
                        ((fdev_reg >> 0) & 0xFF) as i32,
                    )
                    .await?;

                // Send frequency deviation to AGC fw for radio config
                let fdev_reg = crate::sx1250::freq_to_reg(freq_dev);
                self.com
                    .reg_w(
                        reg::SX1302_REG_AGC_MCU_MCU_MAIL_BOX_WR_DATA_BYTE2_MCU_MAIL_BOX_WR_DATA,
                        ((fdev_reg >> 16) & 0xFF) as i32,
                    )
                    .await?;
                self.com
                    .reg_w(
                        reg::SX1302_REG_AGC_MCU_MCU_MAIL_BOX_WR_DATA_BYTE1_MCU_MAIL_BOX_WR_DATA,
                        ((fdev_reg >> 8) & 0xFF) as i32,
                    )
                    .await?;
                self.com
                    .reg_w(
                        reg::SX1302_REG_AGC_MCU_MCU_MAIL_BOX_WR_DATA_BYTE0_MCU_MAIL_BOX_WR_DATA,
                        ((fdev_reg >> 0) & 0xFF) as i32,
                    )
                    .await?;

                // Modulation parameters
                self.com
                    .reg_w(reg::tx_top_fsk_cfg_0_pkt_mode(rf_chain), 1) // Variable length
                    .await?;
                self.com
                    .reg_w(
                        reg::tx_top_fsk_cfg_0_crc_en(rf_chain),
                        if pkt.no_crc { 0 } else { 1 },
                    )
                    .await?;
                self.com
                    .reg_w(reg::tx_top_fsk_cfg_0_crc_ibm(rf_chain), 0) // CCITT CRC
                    .await?;
                self.com
                    .reg_w(reg::tx_top_fsk_cfg_0_dcfree_enc(rf_chain), 2) // Whitening encoding
                    .await?;
                self.com
                    .reg_w(reg::tx_top_fsk_mod_fsk_gaussian_en(rf_chain), 1)
                    .await?;
                self.com
                    .reg_w(reg::tx_top_fsk_mod_fsk_gaussian_select_bt(rf_chain), 2)
                    .await?;
                self.com
                    .reg_w(reg::tx_top_fsk_mod_fsk_ref_pattern_en(rf_chain), 1)
                    .await?;
                if context_fsk.sync_word_size == 0 || context_fsk.sync_word_size > 8 {
                    return Err(Error::InvalidConfig("sync_word_size"));
                }
                self.com
                    .reg_w(
                        reg::tx_top_fsk_mod_fsk_ref_pattern_size(rf_chain),
                        (context_fsk.sync_word_size - 1) as i32,
                    )
                    .await?;

                // Syncword
                let fsk_sync_word_reg =
                    context_fsk.sync_word << (8 * (8 - context_fsk.sync_word_size));
                self.com
                    .reg_w(
                        reg::tx_top_fsk_ref_pattern_byte0_fsk_ref_pattern(rf_chain),
                        (fsk_sync_word_reg >> 0) as u8 as i32,
                    )
                    .await?;
                self.com
                    .reg_w(
                        reg::tx_top_fsk_ref_pattern_byte1_fsk_ref_pattern(rf_chain),
                        (fsk_sync_word_reg >> 8) as u8 as i32,
                    )
                    .await?;
                self.com
                    .reg_w(
                        reg::tx_top_fsk_ref_pattern_byte2_fsk_ref_pattern(rf_chain),
                        (fsk_sync_word_reg >> 16) as u8 as i32,
                    )
                    .await?;
                self.com
                    .reg_w(
                        reg::tx_top_fsk_ref_pattern_byte3_fsk_ref_pattern(rf_chain),
                        (fsk_sync_word_reg >> 24) as u8 as i32,
                    )
                    .await?;
                self.com
                    .reg_w(
                        reg::tx_top_fsk_ref_pattern_byte4_fsk_ref_pattern(rf_chain),
                        (fsk_sync_word_reg >> 32) as u8 as i32,
                    )
                    .await?;
                self.com
                    .reg_w(
                        reg::tx_top_fsk_ref_pattern_byte5_fsk_ref_pattern(rf_chain),
                        (fsk_sync_word_reg >> 40) as u8 as i32,
                    )
                    .await?;
                self.com
                    .reg_w(
                        reg::tx_top_fsk_ref_pattern_byte6_fsk_ref_pattern(rf_chain),
                        (fsk_sync_word_reg >> 48) as u8 as i32,
                    )
                    .await?;
                self.com
                    .reg_w(
                        reg::tx_top_fsk_ref_pattern_byte7_fsk_ref_pattern(rf_chain),
                        (fsk_sync_word_reg >> 56) as u8 as i32,
                    )
                    .await?;
                self.com
                    .reg_w(reg::tx_top_fsk_mod_fsk_preamble_seq(rf_chain), 0)
                    .await?;

                // Set datarate
                if pkt.datarate == 0 {
                    return Err(Error::InvalidConfig("datarate"));
                }
                let fsk_br_reg = 32_000_000 / pkt.datarate;
                let buff = [(fsk_br_reg >> 8) as u8, (fsk_br_reg >> 0) as u8];
                self.com
                    .reg_wb(reg::tx_top_fsk_bit_rate_msb_bit_rate(rf_chain), &buff)
                    .await?;

                // Preamble length (0: use the LoRaWAN preamble size, then
                // enforce the minimum preamble size)
                let mut preamble = pkt.preamble;
                if preamble == 0 {
                    preamble = STD_FSK_PREAMBLE;
                } else if preamble < MIN_FSK_PREAMBLE {
                    preamble = MIN_FSK_PREAMBLE;
                }
                let buff = [(preamble >> 8) as u8, (preamble >> 0) as u8];
                self.com
                    .reg_wb(
                        reg::tx_top_fsk_preamble_size_msb_preamble_size(rf_chain),
                        &buff,
                    )
                    .await?;

                // Set Payload length
                self.com
                    .reg_w(
                        reg::tx_top_fsk_pkt_len_pkt_length(rf_chain),
                        pkt.size as i32,
                    )
                    .await?;
            }
            _ => return Err(Error::InvalidConfig("modulation")),
        }

        // Set TX start delay
        let tx_start_delay = self
            .sx1302_tx_set_start_delay(
                rf_chain,
                radio_type,
                pkt.modulation,
                pkt.bandwidth,
                chirp_lowpass,
            )
            .await?;

        // Write payload in transmit buffer
        self.com
            .reg_w(reg::tx_top_tx_ctrl_write_buffer(rf_chain), 0x01)
            .await?;
        let mem_addr = reg_select(rf_chain, 0x5300, 0x5500);
        let size = pkt.size as usize;
        if pkt.modulation == MOD_FSK {
            // insert payload size in the packet for FSK variable mode (1 byte)
            self.com.mem_wb(mem_addr, &[pkt.size as u8]).await?;
            if size > 0 {
                self.com
                    .mem_wb(mem_addr + 1, &pkt.payload[..size])
                    .await?;
            }
        } else if size > 0 {
            self.com.mem_wb(mem_addr, &pkt.payload[..size]).await?;
        }
        self.com
            .reg_w(reg::tx_top_tx_ctrl_write_buffer(rf_chain), 0x00)
            .await?;

        // Trigger transmit
        match pkt.tx_mode {
            IMMEDIATE => {
                self.com
                    .reg_w(reg::tx_top_tx_trig_tx_trig_immediate(rf_chain), 0x00) // reset state machine
                    .await?;
                self.com
                    .reg_w(reg::tx_top_tx_trig_tx_trig_immediate(rf_chain), 0x01)
                    .await?;
            }
            TIMESTAMPED => {
                // C: `count_us * 32 - tx_start_delay` in uint32_t (wrapping)
                let count_us = pkt
                    .count_us
                    .wrapping_mul(32)
                    .wrapping_sub(tx_start_delay as u32);
                self.com
                    .reg_w(
                        reg::tx_top_timer_trig_byte0_timer_delayed_trig(rf_chain),
                        ((count_us >> 0) & 0x0000_00FF) as i32,
                    )
                    .await?;
                self.com
                    .reg_w(
                        reg::tx_top_timer_trig_byte1_timer_delayed_trig(rf_chain),
                        ((count_us >> 8) & 0x0000_00FF) as i32,
                    )
                    .await?;
                self.com
                    .reg_w(
                        reg::tx_top_timer_trig_byte2_timer_delayed_trig(rf_chain),
                        ((count_us >> 16) & 0x0000_00FF) as i32,
                    )
                    .await?;
                self.com
                    .reg_w(
                        reg::tx_top_timer_trig_byte3_timer_delayed_trig(rf_chain),
                        ((count_us >> 24) & 0x0000_00FF) as i32,
                    )
                    .await?;

                self.com
                    .reg_w(reg::tx_top_tx_trig_tx_trig_delayed(rf_chain), 0x00) // reset state machine
                    .await?;
                self.com
                    .reg_w(reg::tx_top_tx_trig_tx_trig_delayed(rf_chain), 0x01)
                    .await?;
            }
            ON_GPS => {
                self.com
                    .reg_w(reg::tx_top_tx_trig_tx_trig_gps(rf_chain), 0x00) // reset state machine
                    .await?;
                self.com
                    .reg_w(reg::tx_top_tx_trig_tx_trig_gps(rf_chain), 0x01)
                    .await?;
            }
            _ => return Err(Error::InvalidConfig("tx_mode")),
        }

        // C also flushes the USB BULK write mode here (`lgw_com_flush` +
        // `lgw_com_set_write_mode(SINGLE)`); the SPI transport of this port
        // has no such modes.

        Ok(())
    }

    /// Set all GPIO outputs at once, 1 bit per GPIO (C: `sx1302_set_gpio`).
    pub(crate) async fn sx1302_set_gpio(&mut self, gpio_reg_val: u8) -> Result<()> {
        self.com
            .reg_w(
                reg::SX1302_REG_GPIO_GPIO_OUT_L_OUT_VALUE,
                gpio_reg_val as i32,
            )
            .await
    }
}
