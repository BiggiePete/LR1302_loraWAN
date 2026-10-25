//! Top-level concentrator API — a port of the C HAL `loragw_hal.c`.
//!
//! This module implements the `lgw_*` entry points on top of the driver
//! modules ([`crate::sx1302`], [`crate::fw`], [`crate::rx`], [`crate::tx`],
//! [`crate::timestamp`]):
//!
//! | C | Rust |
//! |---|------|
//! | `lgw_board_setconf` | [`Lr1302::board_setconf`] |
//! | `lgw_rxrf_setconf` | [`Lr1302::rxrf_setconf`] |
//! | `lgw_rxif_setconf` | [`Lr1302::rxif_setconf`] |
//! | `lgw_demod_setconf` | [`Lr1302::demod_setconf`] |
//! | `lgw_txgain_setconf` | [`Lr1302::txgain_setconf`] |
//! | `lgw_ftime_setconf` | [`Lr1302::ftime_setconf`] |
//! | `lgw_start` / `lgw_stop` | [`Lr1302::start`] / [`Lr1302::stop`] |
//! | `lgw_receive` | [`Lr1302::receive`] |
//! | `lgw_send` | [`Lr1302::send`] |
//! | `lgw_status` | [`Lr1302::status`] |
//! | `lgw_abort_tx` | [`Lr1302::abort_tx`] |
//! | `lgw_get_trigcnt` / `lgw_get_instcnt` | [`Lr1302::get_trigcnt`] / [`Lr1302::get_instcnt`] |
//! | `lgw_get_eui` | [`Lr1302::get_eui`] |
//! | `lgw_get_temperature` | [`Lr1302::get_temperature`] (unsupported, see [`Lr1302::set_temperature`]) |
//! | `lgw_version_info` | [`Lr1302::version_info`] |
//! | `lgw_time_on_air` | [`Lr1302::time_on_air`] |
//!
//! Differences with the C API:
//!
//! * Configuration functions return [`Result`] instead of `LGW_HAL_ERROR`.
//! * No COM port handling (`lgw_connect` just checks the chip version — the
//!   SPI device is owned by the caller), no log file, no I2C temperature
//!   sensor, no AD5338R DAC, no SX1261 LBT / spectral scan.
//! * The RSSI temperature compensation uses [`Lr1302::set_temperature`]
//!   instead of the STTS751 I2C sensor, and is skipped while no temperature
//!   is provided.
//! * Extra range checks were added where the C code would index arrays out
//!   of bounds (they return [`Error::InvalidConfig`]).

use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::spi::SpiDevice;

use crate::consts::*;
use crate::error::{Error, Result};
use crate::log::{debug, trace};
use crate::sx1302_defs::{
    IF_FSK_STD, IF_LORA_MULTI, IF_LORA_STD, SX1302_AGC_RADIO_GAIN_AUTO, FW_VERSION_AGC_SX1250,
    FW_VERSION_ARB,
};
use crate::types::*;
use crate::util::{bw_getval, is_fsk_bw, is_fsk_dr, is_lora_bw, is_lora_cr, is_lora_dr, is_tx_mode};
use crate::{Lr1302, AGC_FW, ARB_FW};

// C: LGW_RF_RX_BANDWIDTH_* (`loragw_hal.c`)
const LGW_RF_RX_BANDWIDTH_125KHZ: u32 = 1_600_000;
const LGW_RF_RX_BANDWIDTH_250KHZ: u32 = 1_600_000;
const LGW_RF_RX_BANDWIDTH_500KHZ: u32 = 1_600_000;
// C: LGW_RF_RX_FREQ_MIN / MAX
const LGW_RF_RX_FREQ_MIN: u32 = 100_000_000;
const LGW_RF_RX_FREQ_MAX: u32 = 1_000_000_000;

/// C: `lgw_version_string` (`"Version: " LIBLORAGW_VERSION ";"`).
pub const VERSION_STRING: &str = "Version: 2.1.0;";

// ---------------------------------------------------------------------------
// Packet de-duplication helpers (C: is_same_pkt / remove_pkt / merge_packets)
// ---------------------------------------------------------------------------

/// C: `is_same_pkt` — two packets are identical when their timestamps differ
/// by at most 24 µs and channel, datarate, size and payload match.
///
/// `pub(crate)` so the C-vs-Rust conformance tests can drive it directly.
pub(crate) fn is_same_pkt(p1: &RxPacket, p2: &RxPacket) -> bool {
    let dt = (p1.count_us as i64 - p2.count_us as i64).abs();
    dt <= 24
        && p1.if_chain == p2.if_chain
        && p1.datarate == p2.datarate
        && p1.size == p2.size
        && p1.payload[..p1.size as usize] == p2.payload[..p2.size as usize]
}

/// C: `remove_pkt` — replace the packet at `pkt_index` with the last one.
pub(crate) fn remove_pkt(p: &mut [RxPacket], nb_pkt: &mut usize, pkt_index: usize) {
    if pkt_index != *nb_pkt - 1 {
        p[pkt_index] = p[*nb_pkt - 1];
    }
    *nb_pkt -= 1;
}

/// C: `merge_packets` - remove duplicates (keeping the packet with a valid
/// CRC / fine timestamp) then sort by ascending `count_us`.
pub(crate) fn merge_packets(p: &mut [RxPacket], nb_pkt: &mut usize) {
    let mut cpt = *nb_pkt;

    // remove duplicates
    let mut j = 0usize;
    'outer: while j < cpt {
        let mut k = j + 1;
        while k < cpt {
            if is_same_pkt(&p[j], &p[k]) {
                // keep the packet which has CRC checked
                let dup_idx = if p[j].status == STAT_CRC_OK && p[k].status == STAT_CRC_BAD {
                    k
                } else if p[j].status == STAT_CRC_BAD && p[k].status == STAT_CRC_OK {
                    j
                } else if p[j].ftime_received {
                    // we keep the packet which has a fine timestamp
                    k
                } else {
                    j
                };
                remove_pkt(p, &mut cpt, dup_idx);
                // C restarts the search from the first element
                j = 0;
                continue 'outer;
            }
            k += 1;
        }
        j += 1;
    }

    // sort by ascending count_us (C: qsort_r with compare_pkt_tmst)
    for i in 1..cpt {
        let mut k = i;
        while k > 0 && p[k].count_us < p[k - 1].count_us {
            p.swap(k, k - 1);
            k -= 1;
        }
    }

    *nb_pkt = cpt;
}

// ---------------------------------------------------------------------------
// implementation
// ---------------------------------------------------------------------------

impl<SPI, D> Lr1302<SPI, D>
where
    SPI: SpiDevice<u8>,
    D: DelayNs,
{
    // -------------------------------------------------------------------------
    // connection (C: lgw_connect / lgw_disconnect)
    // -------------------------------------------------------------------------

    /// Check the concentrator is present by reading the chip version
    /// register (C: `lgw_connect`, minus the COM port opening).
    ///
    /// The C HAL only prints the version; this port additionally rejects the
    /// bus-idle patterns `0x00` / `0xFF` with [`Error::ChipVersion`], which
    /// indicate a mis-wired SPI chip-select or an unpowered chip.
    pub async fn connect(&mut self) -> Result<()> {
        let version = self.chip_version().await?;
        if version == 0x00 || version == 0xFF {
            debug!("chip version {=u8}: SPI bus idle or mis-wired", version);
            return Err(Error::ChipVersion(version));
        }
        debug!("SX1302 detected, COMMON_VERSION={=u8}", version);
        Ok(())
    }

    /// Read the `COMMON_VERSION` register (C: `lgw_connect`'s version read).
    pub async fn chip_version(&mut self) -> Result<u8> {
        let v = self
            .com
            .reg_r(crate::reg::SX1302_REG_COMMON_VERSION_VERSION)
            .await?;
        Ok(v as u8)
    }

    // -------------------------------------------------------------------------
    // configuration (C: lgw_*_setconf)
    // -------------------------------------------------------------------------

    /// Configure the gateway board (C: `lgw_board_setconf`).
    ///
    /// The C `com_type`/`com_path` fields do not exist here: the SPI device
    /// is passed to [`Lr1302::new`].
    pub fn board_setconf(&mut self, conf: BoardConf) -> Result<()> {
        if self.ctx.is_started {
            return Err(Error::InvalidConfig("concentrator is running"));
        }
        // C indexes CONTEXT_RF_CHAIN[clksrc] later on, so guard the index
        if conf.clksrc >= LGW_RF_CHAIN_NB as u8 {
            return Err(Error::InvalidConfig("clksrc"));
        }
        trace!(
            "board_setconf: public={=bool} clksrc={=u8} full_duplex={=bool}",
            conf.lorawan_public,
            conf.clksrc,
            conf.full_duplex
        );
        self.ctx.board = conf;
        Ok(())
    }

    /// Configure an RF chain (C: `lgw_rxrf_setconf`).
    ///
    /// As in the C HAL, a disabled chain stores nothing (the call is a
    /// no-op). Only [`RadioType::Sx1250`] is supported by this port.
    pub fn rxrf_setconf(&mut self, rf_chain: u8, conf: RfConf) -> Result<()> {
        if self.ctx.is_started {
            return Err(Error::InvalidConfig("concentrator is running"));
        }
        if !conf.enable {
            return Ok(());
        }
        if rf_chain >= LGW_RF_CHAIN_NB as u8 {
            return Err(Error::InvalidConfig("rf_chain"));
        }
        // C also accepts SX1255/SX1257, which this port does not implement
        if conf.type_ != RadioType::Sx1250 {
            return Err(Error::InvalidConfig("radio type"));
        }
        if conf.freq_hz < LGW_RF_RX_FREQ_MIN || conf.freq_hz > LGW_RF_RX_FREQ_MAX {
            return Err(Error::InvalidConfig("freq_hz"));
        }
        trace!(
            "rxrf_setconf chain={=u8} freq={=u32} rssi_offset={=i32}",
            rf_chain,
            conf.freq_hz,
            conf.rssi_offset as i32
        );
        self.ctx.rf_chain[rf_chain as usize] = conf;
        Ok(())
    }

    /// Configure an IF chain + modem (C: `lgw_rxif_setconf`).
    ///
    /// Default values (`BW_UNDEFINED` / `DR_UNDEFINED`) are written back into
    /// `conf` exactly like the C function fills the caller's structure.
    pub fn rxif_setconf(&mut self, if_chain: u8, conf: &mut IfConf) -> Result<()> {
        if self.ctx.is_started {
            return Err(Error::InvalidConfig("concentrator is running"));
        }
        if if_chain >= LGW_IF_CHAIN_NB as u8 {
            return Err(Error::InvalidConfig("if_chain"));
        }
        if !conf.enable {
            self.ctx.if_chain[if_chain as usize].enable = false;
            self.ctx.if_chain[if_chain as usize].freq_hz = 0;
            return Ok(());
        }
        // Note: as in the C HAL, an undefined IF chain type here is not
        // rejected immediately; the `match` below rejects it.
        if conf.rf_chain >= LGW_RF_CHAIN_NB as u8 {
            return Err(Error::InvalidConfig("rf_chain"));
        }

        // check that the IF frequency fits in the radio bandwidth
        let rf_rx_bandwidth: u32 = match conf.bandwidth {
            BW_250KHZ => LGW_RF_RX_BANDWIDTH_250KHZ,
            BW_500KHZ => LGW_RF_RX_BANDWIDTH_500KHZ,
            _ => LGW_RF_RX_BANDWIDTH_125KHZ,
        };
        // C: (bw_hz == -1) ? LGW_REF_BW : bw_hz
        let bw_hz: i32 = match bw_getval(conf.bandwidth) {
            Some(hz) => hz as i32,
            None => LGW_REF_BW as i32,
        };
        if conf.freq_hz + (bw_hz / 2) > (rf_rx_bandwidth as i32) / 2 {
            return Err(Error::InvalidConfig("freq_hz too high"));
        }
        if conf.freq_hz - (bw_hz / 2) < -((rf_rx_bandwidth as i32) / 2) {
            return Err(Error::InvalidConfig("freq_hz too low"));
        }

        match crate::sx1302::sx1302_get_ifmod_config(if_chain) {
            IF_LORA_STD => {
                if conf.bandwidth == BW_UNDEFINED {
                    conf.bandwidth = BW_250KHZ;
                }
                if conf.datarate == DR_UNDEFINED {
                    conf.datarate = DR_LORA_SF7;
                }
                if !is_lora_bw(conf.bandwidth) {
                    return Err(Error::InvalidConfig("bandwidth"));
                }
                if !is_lora_dr(conf.datarate) {
                    return Err(Error::InvalidConfig("datarate"));
                }
                let i = if_chain as usize;
                self.ctx.if_chain[i].enable = conf.enable;
                self.ctx.if_chain[i].rf_chain = conf.rf_chain;
                self.ctx.if_chain[i].freq_hz = conf.freq_hz;
                self.ctx.lora_service.bandwidth = conf.bandwidth;
                self.ctx.lora_service.datarate = conf.datarate;
                self.ctx.lora_service.implicit_hdr = conf.implicit_hdr;
                self.ctx.lora_service.implicit_payload_length = conf.implicit_payload_length;
                self.ctx.lora_service.implicit_crc_en = conf.implicit_crc_en;
                self.ctx.lora_service.implicit_coderate = conf.implicit_coderate;
            }
            IF_LORA_MULTI => {
                if conf.bandwidth == BW_UNDEFINED {
                    conf.bandwidth = BW_125KHZ;
                }
                if conf.datarate == DR_UNDEFINED {
                    conf.datarate = DR_LORA_SF7;
                }
                if conf.bandwidth != BW_125KHZ {
                    return Err(Error::InvalidConfig("bandwidth"));
                }
                if !is_lora_dr(conf.datarate) {
                    return Err(Error::InvalidConfig("datarate"));
                }
                let i = if_chain as usize;
                self.ctx.if_chain[i].enable = conf.enable;
                self.ctx.if_chain[i].rf_chain = conf.rf_chain;
                self.ctx.if_chain[i].freq_hz = conf.freq_hz;
            }
            IF_FSK_STD => {
                if conf.bandwidth == BW_UNDEFINED {
                    conf.bandwidth = BW_250KHZ;
                }
                if conf.datarate == DR_UNDEFINED {
                    conf.datarate = 64_000;
                }
                if !is_fsk_bw(conf.bandwidth) {
                    return Err(Error::InvalidConfig("bandwidth"));
                }
                if !is_fsk_dr(conf.datarate) {
                    return Err(Error::InvalidConfig("datarate"));
                }
                let i = if_chain as usize;
                self.ctx.if_chain[i].enable = conf.enable;
                self.ctx.if_chain[i].rf_chain = conf.rf_chain;
                self.ctx.if_chain[i].freq_hz = conf.freq_hz;
                self.ctx.fsk.bandwidth = conf.bandwidth;
                self.ctx.fsk.datarate = conf.datarate;
                if conf.sync_word > 0 {
                    self.ctx.fsk.sync_word_size = conf.sync_word_size;
                    self.ctx.fsk.sync_word = conf.sync_word;
                }
            }
            _ => return Err(Error::InvalidConfig("if chain type not supported")),
        }
        trace!(
            "rxif_setconf if={=u8} rf={=u8} freq={=i32} bw={=u32} dr={=u32}",
            if_chain,
            conf.rf_chain,
            conf.freq_hz,
            conf.bandwidth as u32,
            conf.datarate as u32
        );
        Ok(())
    }

    /// Configure the LoRa/FSK demodulators (C: `lgw_demod_setconf`).
    pub fn demod_setconf(&mut self, conf: DemodConf) -> Result<()> {
        trace!("demod_setconf");
        self.ctx.demod = conf;
        Ok(())
    }

    /// Configure the TX gain LUT of an RF chain (C: `lgw_txgain_setconf`).
    pub fn txgain_setconf(&mut self, rf_chain: u8, conf: TxGainLut) -> Result<()> {
        // C does not check the RF chain index here (it would overflow the
        // context array); this port does.
        if rf_chain >= LGW_RF_CHAIN_NB as u8 {
            return Err(Error::InvalidConfig("rf_chain"));
        }
        // C: 1 <= size <= TX_GAIN_LUT_SIZE_MAX
        if conf.size < 1 || conf.size as usize > TX_GAIN_LUT_SIZE_MAX {
            return Err(Error::InvalidConfig("tx gain LUT size"));
        }
        for g in &conf.lut[..conf.size as usize] {
            if g.dig_gain > 3 {
                return Err(Error::InvalidConfig("tx gain: dig_gain"));
            }
            if g.dac_gain > 3 {
                return Err(Error::InvalidConfig("tx gain: dac_gain"));
            }
            if g.mix_gain < 5 || g.mix_gain > 15 {
                return Err(Error::InvalidConfig("tx gain: mix_gain"));
            }
            if g.pa_gain > 3 {
                return Err(Error::InvalidConfig("tx gain: pa_gain"));
            }
            if g.pwr_idx > 22 {
                return Err(Error::InvalidConfig("tx gain: pwr_idx"));
            }
        }
        self.ctx.tx_gain_lut[rf_chain as usize] = conf;
        trace!(
            "txgain_setconf chain={=u8} size={=u32}",
            rf_chain,
            conf.size as u32
        );
        Ok(())
    }

    /// Configure the fine timestamping (C: `lgw_ftime_setconf`).
    ///
    /// Enabling it is accepted for API compatibility, but this port never
    /// computes fine timestamps: [`RxPacket::ftime_received`] stays `false`.
    pub fn ftime_setconf(&mut self, conf: FtimeConf) -> Result<()> {
        trace!("ftime_setconf");
        self.ctx.ftime = conf;
        Ok(())
    }

    // -------------------------------------------------------------------------
    // lifecycle (C: lgw_start / lgw_stop)
    // -------------------------------------------------------------------------

    /// Connect to the concentrator and configure it from the previously set
    /// parameters (C: `lgw_start`).
    pub async fn start(&mut self) -> Result<()> {
        debug!("starting concentrator (clksrc={=u8})", self.ctx.board.clksrc);
        match self.start_inner().await {
            Ok(()) => {
                debug!("concentrator started");
                Ok(())
            }
            Err(e) => {
                debug!("concentrator start failed: {}", e);
                Err(e)
            }
        }
    }

    /// Body of [`Lr1302::start`] (C: `lgw_start`), split out so the entry
    /// point can log the outcome.
    async fn start_inner(&mut self) -> Result<()> {
        // C: "restarting it now" note when already started
        self.connect().await?;

        // the C HAL derives the AGC firmware and TX configuration from the
        // radio type of the clock source chain, which must be configured
        let clksrc = self.ctx.board.clksrc;
        if !self.ctx.rf_chain[clksrc as usize].enable {
            return Err(Error::InvalidConfig("clksrc"));
        }
        let clock_type = self.ctx.rf_chain[clksrc as usize].type_;

        // Set all GPIOs to 0
        self.sx1302_set_gpio(0x00).await?;

        // Calibrate radios
        self.sx1302_radio_calibrate().await?;

        // Setup radios for RX
        for i in 0..LGW_RF_CHAIN_NB as u8 {
            let conf = self.ctx.rf_chain[i as usize];
            if !conf.enable {
                continue;
            }
            self.sx1302_radio_reset(i, conf.type_).await?;
            match conf.type_ {
                RadioType::Sx1250 => {
                    self.com
                        .sx1250_setup(i, conf.freq_hz, conf.single_input_mode)
                        .await?;
                }
                // sx1255/sx1257 are not ported
                _ => return Err(Error::Unsupported),
            }
            self.sx1302_radio_set_mode(i, conf.type_).await?;
        }

        // Select the radio which provides the clock to the sx1302
        self.sx1302_radio_clock_select(clksrc).await?;

        // Release host control on radio (will be controlled by AGC)
        self.sx1302_radio_host_ctrl(false).await?;

        // Basic initialization of the sx1302
        self.sx1302_init().await?;

        // Configure PA/LNA LUTs
        self.sx1302_pa_lna_lut_configure().await?;

        // Configure Radio FE
        self.sx1302_radio_fe_configure().await?;

        // Configure the Channelizer
        self.sx1302_channelizer_configure(false).await?;

        // configure LoRa 'multi-sf' modems
        self.sx1302_lora_correlator_configure().await?;
        let radio_freq_hz = self.ctx.rf_chain[0].freq_hz;
        self.sx1302_lora_modem_configure(radio_freq_hz).await?;

        // configure LoRa 'single-sf' modem
        if self.ctx.if_chain[8].enable {
            self.sx1302_lora_service_correlator_configure().await?;
            self.sx1302_lora_service_modem_configure(radio_freq_hz)
                .await?;
        }

        // configure FSK modem
        if self.ctx.if_chain[9].enable {
            self.sx1302_fsk_configure().await?;
        }

        // configure syncword
        let lora_service_sf = self.ctx.lora_service.datarate as u8;
        self.sx1302_lora_syncword(self.ctx.board.lorawan_public, lora_service_sf)
            .await?;

        // enable demodulators - to be done before starting AGC/ARB
        self.sx1302_modem_enable().await?;

        // Load AGC firmware
        match clock_type {
            RadioType::Sx1250 => self.sx1302_agc_load_firmware(AGC_FW).await?,
            _ => return Err(Error::Unsupported),
        }
        self.sx1302_agc_start(
            FW_VERSION_AGC_SX1250,
            clock_type,
            SX1302_AGC_RADIO_GAIN_AUTO,
            SX1302_AGC_RADIO_GAIN_AUTO,
            self.ctx.board.full_duplex,
            false, // LBT (sx1261) is not ported
        )
        .await?;

        // Load ARB firmware
        self.sx1302_arb_load_firmware(ARB_FW).await?;
        self.sx1302_arb_start(FW_VERSION_ARB).await?;

        // static TX configuration
        self.sx1302_tx_configure(clock_type).await?;

        // enable GPS (PPS trigger and counter sampling; the GPS UART itself
        // is not ported)
        self.sx1302_gps_enable(true).await?;

        // C also probes the I2C temperature sensor and, in full duplex
        // boards, the AD5338R DAC — neither is ported (see `set_temperature`).

        // Set CONFIG_DONE GPIO to 1 (turn on the corresponding LED)
        self.sx1302_set_gpio(0x01).await?;

        self.ctx.is_started = true;
        Ok(())
    }

    /// Stop the concentrator (C: `lgw_stop`).
    pub async fn stop(&mut self) -> Result<()> {
        if !self.ctx.is_started {
            return Ok(());
        }
        debug!("stopping concentrator");

        // Abort current TX if needed, on both chains (C continues on error)
        let mut result = Ok(());
        for i in 0..LGW_RF_CHAIN_NB as u8 {
            if let Err(e) = self.abort_tx(i).await {
                result = Err(e);
            }
        }

        // C: lgw_disconnect() closes the COM device; here the SPI device
        // stays owned by the caller, so there is nothing to close.

        self.ctx.is_started = false;
        if result.is_err() {
            debug!("stop reported an error while aborting TX");
        } else {
            debug!("concentrator stopped");
        }
        result
    }

    // -------------------------------------------------------------------------
    // RX / TX (C: lgw_receive / lgw_send / lgw_status / lgw_abort_tx)
    // -------------------------------------------------------------------------

    /// Fetch and parse up to `pkt_data.len()` packets from the concentrator
    /// (C: `lgw_receive`), returning the number of packets written.
    ///
    /// The RSSI temperature compensation is applied only when a temperature
    /// was provided with [`Lr1302::set_temperature`]; the static
    /// `rssi_offset` of the RF chain is always applied.
    pub async fn receive(&mut self, pkt_data: &mut [RxPacket]) -> Result<usize> {
        // Get packets from SX1302, if any
        let nb_pkt_fetched = self.sx1302_fetch().await? as usize;
        trace!("rx fetched {=u32} raw packet(s)", nb_pkt_fetched as u32);

        // Update internal counter — C: "needs to be called regularly by the
        // upper layer"
        self.sx1302_update().await?;

        // Exit now if no packet fetched
        if nb_pkt_fetched == 0 {
            return Ok(0);
        }

        let max_pkt = pkt_data.len();
        // C prints a warning when packets have to be left in the RX buffer;
        // they are fetched again on the next call.

        let temperature = self.ctx.temperature;
        let mut nb_pkt_found = 0usize;

        for _ in 0..nb_pkt_fetched.min(max_pkt) {
            let mut pkt = match self.sx1302_parse().await? {
                Some(p) => p,
                // C: LGW_REG_WARNING -> stop parsing, keep what we have
                None => return Ok(nb_pkt_found),
            };

            // Apply the RSSI offset calibrated for the board
            let rf = pkt.rf_chain as usize;
            let rssi_offset = self.ctx.rf_chain[rf].rssi_offset;
            pkt.rssic += rssi_offset;
            pkt.rssis += rssi_offset;

            // ... and the offset calibrated for the current temperature
            if let Some(temp) = temperature {
                let tcomp = self.ctx.rf_chain[rf].rssi_tcomp;
                let t_offset = crate::rx::sx1302_rssi_get_temperature_offset(&tcomp, temp);
                pkt.rssic += t_offset;
                pkt.rssis += t_offset;
            }

            pkt_data[nb_pkt_found] = pkt;
            debug!(
                "rx if={=u8} rf={=u8} tmst={=u32} dr={=u32} size={=u16} rssi={=i32} snr={=i32}",
                pkt.if_chain,
                pkt.rf_chain,
                pkt.count_us,
                pkt.datarate,
                pkt.size,
                (pkt.rssis * 100.0) as i32,
                (pkt.snr * 100.0) as i32
            );
            nb_pkt_found += 1;
        }

        // Remove duplicated packets generated by double demod when precision
        // timestamp is enabled
        if nb_pkt_found > 0 && self.ctx.ftime.enable {
            merge_packets(&mut pkt_data[..nb_pkt_found], &mut nb_pkt_found);
        }

        if nb_pkt_found > 0 {
            debug!("rx {=u32} packet(s) ready", nb_pkt_found as u32);
        }

        Ok(nb_pkt_found)
    }

    /// Schedule a packet to be transmitted (C: `lgw_send`).
    pub async fn send(&mut self, pkt: &TxPacket) -> Result<()> {
        if !self.ctx.is_started {
            return Err(Error::InvalidConfig("concentrator not started"));
        }
        if pkt.rf_chain >= LGW_RF_CHAIN_NB as u8 {
            return Err(Error::InvalidConfig("rf_chain"));
        }
        let rf = pkt.rf_chain as usize;
        if !self.ctx.rf_chain[rf].tx_enable {
            return Err(Error::InvalidConfig("tx_enable"));
        }
        if !self.ctx.rf_chain[rf].enable {
            return Err(Error::InvalidConfig("rf chain disabled"));
        }
        if !is_tx_mode(pkt.tx_mode) {
            return Err(Error::InvalidConfig("tx_mode"));
        }

        match pkt.modulation {
            MOD_LORA => {
                if !is_lora_bw(pkt.bandwidth) {
                    return Err(Error::InvalidConfig("bandwidth"));
                }
                if !is_lora_dr(pkt.datarate) {
                    return Err(Error::InvalidConfig("datarate"));
                }
                if !is_lora_cr(pkt.coderate) {
                    return Err(Error::InvalidConfig("coderate"));
                }
                if pkt.size > 255 {
                    return Err(Error::InvalidConfig("payload size"));
                }
            }
            MOD_FSK => {
                if pkt.f_dev < 1 || pkt.f_dev > 200 {
                    return Err(Error::InvalidConfig("f_dev"));
                }
                if !is_fsk_dr(pkt.datarate) {
                    return Err(Error::InvalidConfig("datarate"));
                }
                if pkt.size > 255 {
                    return Err(Error::InvalidConfig("payload size"));
                }
            }
            MOD_CW => {
                // no extra check
            }
            _ => return Err(Error::InvalidConfig("modulation")),
        }

        // C additionally drives the AD5338R DAC in full duplex mode and runs
        // Listen-Before-Talk through an SX1261 — neither is ported.

        debug!(
            "tx rf={=u8} mode={=u8} freq={=u32} dr={=u32} size={=u16} power={=i8}",
            pkt.rf_chain,
            pkt.tx_mode,
            pkt.freq_hz,
            pkt.datarate,
            pkt.size,
            pkt.rf_power
        );
        match self.sx1302_send(pkt).await {
            Ok(()) => {
                trace!("tx packet scheduled");
                Ok(())
            }
            Err(e) => {
                debug!("tx failed: {}", e);
                Err(e)
            }
        }
    }

    /// Give the status of the TX or RX modem (C: `lgw_status`).
    pub async fn status(&mut self, rf_chain: u8, select: u8) -> Result<u8> {
        if rf_chain >= LGW_RF_CHAIN_NB as u8 {
            return Err(Error::InvalidConfig("rf_chain"));
        }
        match select {
            TX_STATUS => {
                if !self.ctx.is_started {
                    Ok(TX_OFF)
                } else {
                    self.sx1302_tx_status(rf_chain).await
                }
            }
            RX_STATUS => {
                if !self.ctx.is_started {
                    Ok(RX_OFF)
                } else {
                    self.sx1302_rx_status(rf_chain).await
                }
            }
            _ => Err(Error::InvalidConfig("select")),
        }
    }

    /// Abort a currently scheduled or ongoing TX (C: `lgw_abort_tx`).
    pub async fn abort_tx(&mut self, rf_chain: u8) -> Result<()> {
        if rf_chain >= LGW_RF_CHAIN_NB as u8 {
            return Err(Error::InvalidConfig("rf_chain"));
        }
        self.sx1302_tx_abort(rf_chain).await
    }

    // -------------------------------------------------------------------------
    // counters and misc (C: lgw_get_* / lgw_version_info / lgw_time_on_air)
    // -------------------------------------------------------------------------

    /// Value of the internal counter when the latest event (GPS pulse) was
    /// captured, in µs (C: `lgw_get_trigcnt`).
    pub async fn get_trigcnt(&mut self) -> Result<u32> {
        self.sx1302_timestamp_counter(true).await
    }

    /// Instantaneous value of the internal counter, in µs
    /// (C: `lgw_get_instcnt`).
    pub async fn get_instcnt(&mut self) -> Result<u32> {
        self.sx1302_timestamp_counter(false).await
    }

    /// Return the concentrator EUI (C: `lgw_get_eui`).
    pub async fn get_eui(&mut self) -> Result<u64> {
        self.sx1302_get_eui().await
    }

    /// Return the temperature measured by the concentrator sensor
    /// (C: `lgw_get_temperature`).
    ///
    /// The STTS751 I2C temperature sensor is not ported, so this always
    /// returns [`Error::Unsupported`]; provide the ambient temperature
    /// yourself with [`Lr1302::set_temperature`] instead.
    pub fn get_temperature(&self) -> Result<f32> {
        Err(Error::Unsupported)
    }

    /// Provide the ambient temperature (°C) used for the RSSI temperature
    /// compensation in [`Lr1302::receive`].
    ///
    /// This is the replacement for the C HAL's I2C STTS751 reads. Passing
    /// `None` disables the compensation (only `rssi_offset` is applied).
    pub fn set_temperature(&mut self, temperature: Option<f32>) {
        self.ctx.temperature = temperature;
    }

    /// Library version string (C: `lgw_version_info`).
    pub const fn version_info() -> &'static str {
        VERSION_STRING
    }

    /// Time on air of a packet, in milliseconds (C: `lgw_time_on_air`).
    pub fn time_on_air(&self, packet: &TxPacket) -> u32 {
        if packet.modulation == MOD_LORA {
            match crate::toa::lora_packet_time_on_air(
                packet.bandwidth,
                packet.datarate as u8,
                packet.coderate,
                packet.preamble,
                packet.no_header,
                packet.no_crc,
                packet.size as u8,
            ) {
                Some(toa) => (toa.toa_us as f64 / 1000.0 + 0.5) as u32,
                None => 0,
            }
        } else if packet.modulation == MOD_FSK {
            // PREAMBLE + SYNC_WORD + PKT_LEN + PKT_PAYLOAD + CRC
            let sync_word_size = self.ctx.fsk.sync_word_size as f64;
            let crc = if packet.no_crc { 0.0 } else { 2.0 };
            let t_fsk = (8.0
                * (packet.preamble as f64
                    + sync_word_size
                    + 1.0
                    + packet.size as f64
                    + crc)
                / packet.datarate as f64)
                * 1E3;
            // C: add margin for rounding
            t_fsk as u32 + 1
        } else {
            0
        }
    }
}
