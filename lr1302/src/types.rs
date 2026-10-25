//! Public configuration and packet structures.
//!
//! Faithful port of the structures declared in the C HAL `loragw_hal.h`
//! (`lgw_conf_board_s`, `lgw_conf_rxrf_s`, `lgw_conf_rxif_s`,
//! `lgw_conf_demod_s`, `lgw_pkt_rx_s`, `lgw_pkt_tx_s`, `lgw_tx_gain_s`,
//! `lgw_tx_gain_lut_s`, `lgw_conf_ftime_s`).
//!
//! Differences with the C API:
//!
//! * `com_type` / `com_path` of the board configuration are gone: the SPI
//!   device is passed to [`crate::Lr1302::new`] instead.
//! * RSSI values are `f32` like in C (`float`).
//! * Payloads are inline fixed-size arrays (`no_std`, no allocator).

use crate::consts::*;

/// Board configuration (C: `lgw_conf_board_s`, without the COM fields).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct BoardConf {
    /// Enable ONLY for *public* networks using the LoRa MAC protocol
    /// (C: `lorawan_public`).
    pub lorawan_public: bool,
    /// Index of the RF chain which provides the clock to the concentrator
    /// (C: `clksrc`, 0 or 1).
    pub clksrc: u8,
    /// Indicates if the gateway operates in full duplex mode (C: `full_duplex`).
    pub full_duplex: bool,
}

impl Default for BoardConf {
    fn default() -> Self {
        Self {
            lorawan_public: true,
            clksrc: 0,
            full_duplex: false,
        }
    }
}

/// Coefficients to compute the RSSI offset for the current temperature
/// (C: `lgw_rssi_tcomp_s`).
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RssiTComp {
    /// C: `coeff_a`
    pub coeff_a: f32,
    /// C: `coeff_b`
    pub coeff_b: f32,
    /// C: `coeff_c`
    pub coeff_c: f32,
    /// C: `coeff_d`
    pub coeff_d: f32,
    /// C: `coeff_e`
    pub coeff_e: f32,
}

impl Default for RssiTComp {
    fn default() -> Self {
        Self {
            coeff_a: 0.0,
            coeff_b: 0.0,
            coeff_c: 0.0,
            coeff_d: 0.0,
            coeff_e: 0.0,
        }
    }
}

/// Configuration of one RF chain (C: `lgw_conf_rxrf_s`).
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RfConf {
    /// Enable or disable that RF chain (C: `enable`).
    pub enable: bool,
    /// Center frequency of the radio in Hz (C: `freq_hz`, 100 MHz .. 1 GHz).
    pub freq_hz: u32,
    /// Board-specific RSSI correction factor (C: `rssi_offset`).
    pub rssi_offset: f32,
    /// Board-specific RSSI temperature compensation (C: `rssi_tcomp`).
    pub rssi_tcomp: RssiTComp,
    /// Radio type of that RF chain (C: `type`).
    ///
    /// This port only supports [`RadioType::Sx1250`], which is why it is the
    /// default value (the C default of `LGW_RADIO_TYPE_NONE` would always be
    /// rejected by `lgw_rxrf_setconf`).
    pub type_: RadioType,
    /// Enable or disable TX on that RF chain (C: `tx_enable`).
    pub tx_enable: bool,
    /// Configure the radio in single (true) or differential (false) input
    /// mode (C: `single_input_mode`, SX1250 only).
    pub single_input_mode: bool,
}

impl Default for RfConf {
    fn default() -> Self {
        Self {
            enable: false,
            freq_hz: 0,
            rssi_offset: 0.0,
            rssi_tcomp: RssiTComp::default(),
            type_: RadioType::Sx1250,
            tx_enable: false,
            single_input_mode: false,
        }
    }
}

/// Configuration of one IF chain + modem (C: `lgw_conf_rxif_s`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct IfConf {
    /// Enable or disable that IF chain (C: `enable`).
    pub enable: bool,
    /// RF chain that IF chain is associated with (C: `rf_chain`).
    pub rf_chain: u8,
    /// Center frequency of the IF chain, relative to the RF chain frequency,
    /// in Hz (C: `freq_hz`).
    pub freq_hz: i32,
    /// RX bandwidth code (C: `bandwidth`, [`BW_UNDEFINED`] for default).
    pub bandwidth: u8,
    /// RX datarate (C: `datarate`, [`DR_UNDEFINED`] for default; for the
    /// multi-SF chains this is a bitmask of enabled spreading factors).
    pub datarate: u32,
    /// Size of the FSK sync word in bytes (C: `sync_word_size`, 0 default).
    pub sync_word_size: u8,
    /// FSK sync word, aligned right (C: `sync_word`, e.g. `0xC194C1`).
    pub sync_word: u64,
    /// LoRa Service implicit header enable (C: `implicit_hdr`).
    pub implicit_hdr: bool,
    /// LoRa Service implicit header payload length (C: `implicit_payload_length`).
    pub implicit_payload_length: u8,
    /// LoRa Service implicit header CRC enable (C: `implicit_crc_en`).
    pub implicit_crc_en: bool,
    /// LoRa Service implicit header coding rate (C: `implicit_coderate`).
    pub implicit_coderate: u8,
}

impl Default for IfConf {
    fn default() -> Self {
        Self {
            enable: false,
            rf_chain: 0,
            freq_hz: 0,
            bandwidth: BW_UNDEFINED,
            datarate: DR_UNDEFINED,
            sync_word_size: 0,
            sync_word: 0,
            implicit_hdr: false,
            implicit_payload_length: 0,
            implicit_crc_en: false,
            implicit_coderate: 0,
        }
    }
}

/// Configuration of the LoRa/FSK demodulators (C: `lgw_conf_demod_s`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DemodConf {
    /// Bitmask to enable spreading factors SF12..SF5 of the multi-SF
    /// correlators (C: `multisf_datarate`, [`LGW_MULTI_SF_EN`] default).
    pub multisf_datarate: u8,
}

impl Default for DemodConf {
    fn default() -> Self {
        Self {
            multisf_datarate: LGW_MULTI_SF_EN,
        }
    }
}

/// Fine timestamping mode (C: `lgw_ftime_mode_t`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FtimeMode {
    /// Fine timestamps for SF5 -> SF10 (C: `LGW_FTIME_MODE_HIGH_CAPACITY`).
    HighCapacity,
    /// Fine timestamps for SF5 -> SF12 (C: `LGW_FTIME_MODE_ALL_SF`).
    AllSf,
}

impl Default for FtimeMode {
    fn default() -> Self {
        Self::AllSf
    }
}

/// Fine timestamping configuration (C: `lgw_conf_ftime_s`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct FtimeConf {
    /// Enable / disable fine timestamping (C: `enable`).
    ///
    /// Note: this port does not compute fine timestamps
    /// ([`crate::Error::Unsupported`]); the flag is only carried in the
    /// configuration.
    pub enable: bool,
    /// Fine timestamping mode (C: `mode`).
    pub mode: FtimeMode,
}

impl Default for FtimeConf {
    fn default() -> Self {
        Self {
            enable: false,
            mode: FtimeMode::AllSf,
        }
    }
}

/// Gains of one TX chain entry (C: `lgw_tx_gain_s`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TxGain {
    /// Measured TX power at the board connector, in dBm (C: `rf_power`).
    pub rf_power: i8,
    /// 2 bits: digital gain of the SX1302 (C: `dig_gain`).
    pub dig_gain: u8,
    /// 1 bit (sx1250): enable/disable the external PA (C: `pa_gain`).
    pub pa_gain: u8,
    /// 2 bits: control of the radio DAC, sx125x only (C: `dac_gain`).
    pub dac_gain: u8,
    /// 4 bits: control of the radio mixer, sx125x only (C: `mix_gain`).
    pub mix_gain: u8,
    /// Calibrated I offset, sx125x only (C: `offset_i`).
    pub offset_i: i8,
    /// Calibrated Q offset, sx125x only (C: `offset_q`).
    pub offset_q: i8,
    /// 6 bits: radio power index used for configuration (C: `pwr_idx`).
    pub pwr_idx: u8,
}

impl Default for TxGain {
    fn default() -> Self {
        Self {
            rf_power: 14,
            dig_gain: 0,
            pa_gain: 2,
            dac_gain: 3,
            mix_gain: 10,
            offset_i: 0,
            offset_q: 0,
            pwr_idx: 0,
        }
    }
}

/// TX gain lookup table (C: `lgw_tx_gain_lut_s`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TxGainLut {
    /// LUT entries (C: `lut[TX_GAIN_LUT_SIZE_MAX]`).
    pub lut: [TxGain; TX_GAIN_LUT_SIZE_MAX],
    /// Number of valid LUT entries, 1..=[`TX_GAIN_LUT_SIZE_MAX`] (C: `size`).
    pub size: u8,
}

impl Default for TxGainLut {
    fn default() -> Self {
        Self {
            lut: [TxGain::default(); TX_GAIN_LUT_SIZE_MAX],
            size: 1,
        }
    }
}

/// Metadata + payload of a received packet (C: `lgw_pkt_rx_s`).
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RxPacket {
    /// Central frequency of the IF chain (C: `freq_hz`).
    pub freq_hz: u32,
    /// Frequency offset from the RF chain center (C: `freq_offset`).
    pub freq_offset: i32,
    /// IF chain the packet was received on (C: `if_chain`).
    pub if_chain: u8,
    /// Packet status, `STAT_*` (C: `status`).
    pub status: u8,
    /// Internal concentrator timestamp, 1 µs resolution (C: `count_us`).
    pub count_us: u32,
    /// RF chain the packet was received on (C: `rf_chain`).
    pub rf_chain: u8,
    /// Modem that received the packet (C: `modem_id`).
    pub modem_id: u8,
    /// Modulation of the packet, `MOD_*` (C: `modulation`).
    pub modulation: u8,
    /// Modulation bandwidth, LoRa only (C: `bandwidth`).
    pub bandwidth: u8,
    /// Datarate: SF for LoRa, baudrate for FSK (C: `datarate`).
    pub datarate: u32,
    /// Coding rate of the packet, LoRa only (C: `coderate`).
    pub coderate: u8,
    /// Average RSSI of the channel in dB (C: `rssic`).
    pub rssic: f32,
    /// Average RSSI of the signal in dB (C: `rssis`).
    pub rssis: f32,
    /// Average packet SNR in dB, LoRa only (C: `snr`).
    pub snr: f32,
    /// Minimum packet SNR in dB, LoRa only (C: `snr_min`).
    pub snr_min: f32,
    /// Maximum packet SNR in dB, LoRa only (C: `snr_max`).
    pub snr_max: f32,
    /// CRC received in the payload (C: `crc`).
    pub crc: u16,
    /// Payload size in bytes (C: `size`).
    pub size: u16,
    /// Payload buffer (C: `payload[256]`).
    pub payload: [u8; 256],
    /// A fine timestamp has been received (C: `ftime_received`).
    pub ftime_received: bool,
    /// Packet fine timestamp, ns since last PPS (C: `ftime`).
    pub ftime: u32,
}

impl Default for RxPacket {
    fn default() -> Self {
        Self {
            freq_hz: 0,
            freq_offset: 0,
            if_chain: 0,
            status: STAT_UNDEFINED,
            count_us: 0,
            rf_chain: 0,
            modem_id: 0,
            modulation: MOD_UNDEFINED,
            bandwidth: BW_UNDEFINED,
            datarate: DR_UNDEFINED,
            coderate: CR_UNDEFINED,
            rssic: 0.0,
            rssis: 0.0,
            snr: 0.0,
            snr_min: 0.0,
            snr_max: 0.0,
            crc: 0,
            size: 0,
            payload: [0u8; 256],
            ftime_received: false,
            ftime: 0,
        }
    }
}

/// Configuration + payload of a packet to transmit (C: `lgw_pkt_tx_s`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TxPacket {
    /// Center frequency of TX (C: `freq_hz`).
    pub freq_hz: u32,
    /// When the TX is triggered, `IMMEDIATE` / `TIMESTAMPED` / `ON_GPS`
    /// (C: `tx_mode`).
    pub tx_mode: u8,
    /// Timestamp or delay in µs for the TX trigger (C: `count_us`).
    pub count_us: u32,
    /// RF chain the packet will be sent through (C: `rf_chain`).
    pub rf_chain: u8,
    /// TX power in dBm (C: `rf_power`).
    pub rf_power: i8,
    /// Modulation to use, `MOD_*` (C: `modulation`).
    pub modulation: u8,
    /// Frequency offset from the radio TX frequency, CW mode only
    /// (C: `freq_offset`).
    pub freq_offset: i8,
    /// Modulation bandwidth, LoRa only (C: `bandwidth`).
    pub bandwidth: u8,
    /// TX datarate: baudrate for FSK, SF for LoRa (C: `datarate`).
    pub datarate: u32,
    /// Coding rate of the packet, LoRa only (C: `coderate`).
    pub coderate: u8,
    /// Invert signal polarity, for orthogonal downlinks, LoRa only
    /// (C: `invert_pol`).
    pub invert_pol: bool,
    /// Frequency deviation in kHz, FSK only (C: `f_dev`).
    pub f_dev: u8,
    /// Preamble length, 0 for default (C: `preamble`).
    pub preamble: u16,
    /// Do not send a CRC in the packet (C: `no_crc`).
    pub no_crc: bool,
    /// Implicit header mode (LoRa) / fixed length (FSK) (C: `no_header`).
    pub no_header: bool,
    /// Payload size in bytes (C: `size`).
    pub size: u16,
    /// Payload buffer (C: `payload[256]`).
    pub payload: [u8; 256],
}

impl TxPacket {
    /// Convenience constructor for an immediate LoRa transmission.
    pub const fn new(freq_hz: u32, rf_chain: u8, payload: [u8; 256], size: u16) -> Self {
        Self {
            freq_hz,
            tx_mode: IMMEDIATE,
            count_us: 0,
            rf_chain,
            rf_power: 14,
            modulation: MOD_LORA,
            freq_offset: 0,
            bandwidth: BW_125KHZ,
            datarate: DR_LORA_SF7,
            coderate: CR_LORA_4_5,
            invert_pol: false,
            f_dev: 0,
            preamble: 0,
            no_crc: false,
            no_header: false,
            size,
            payload,
        }
    }
}

impl Default for TxPacket {
    fn default() -> Self {
        Self {
            freq_hz: 0,
            tx_mode: IMMEDIATE,
            count_us: 0,
            rf_chain: 0,
            rf_power: 0,
            modulation: MOD_UNDEFINED,
            freq_offset: 0,
            bandwidth: BW_UNDEFINED,
            datarate: DR_UNDEFINED,
            coderate: CR_UNDEFINED,
            invert_pol: false,
            f_dev: 0,
            preamble: 0,
            no_crc: false,
            no_header: false,
            size: 0,
            payload: [0u8; 256],
        }
    }
}
