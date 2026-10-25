//! Driver configuration context and per-instance state.
//!
//! Port of `lgw_context_t` (C: `loragw_hal.h`) with the default values set
//! by the static initializer in `loragw_hal.c`, plus the two stateful
//! instances that live as C globals in `loragw_sx1302.c`: the RX buffer
//! ([`RxBuffer`]) and the timestamp counter ([`TimestampCounter`]).

use crate::consts::*;
use crate::types::*;

/// Packet staging buffer fed from the SX1302 internal RX buffer
/// (C: `rx_buffer_t`, instance `rx_buffer` in `loragw_sx1302.c`).
#[derive(Debug, Clone, Copy)]
pub struct RxBuffer {
    /// Byte array holding data fetched from the SX1302 RX buffer
    /// (C: `buffer[4096]`).
    pub buffer: [u8; 4096],
    /// Number of bytes currently stored in the buffer (C: `buffer_size`).
    pub buffer_size: u16,
    /// Current parsing index in the buffer (C: `buffer_index`).
    pub buffer_index: i32,
    /// Number of complete packets contained in the buffer
    /// (C: `buffer_pkt_nb`).
    pub buffer_pkt_nb: u8,
}

impl Default for RxBuffer {
    fn default() -> Self {
        Self {
            buffer: [0u8; 4096],
            buffer_size: 0,
            buffer_index: 0,
            buffer_pkt_nb: 0,
        }
    }
}

/// Wrap-tracking info of one 27-bit 1 MHz counter
/// (C: `struct timestamp_info_s`).
#[derive(Debug, Clone, Copy, Default)]
pub struct TimestampInfo {
    /// Reference value, last read (C: `counter_us_27bits_ref`).
    pub counter_us_27bits_ref: u32,
    /// Rollover/wrap status (C: `counter_us_27bits_wrap`).
    pub counter_us_27bits_wrap: u8,
}

/// Context maintaining the rollover status of the internal counters
/// (C: `timestamp_counter_t`, instance `counter_us` in `loragw_sx1302.c`).
#[derive(Debug, Clone, Copy, Default)]
pub struct TimestampCounter {
    /// Instantaneous freerun counter state (C: `inst`).
    pub inst: TimestampInfo,
    /// PPS-triggered counter state (C: `pps`).
    pub pps: TimestampInfo,
}

/// Gateway configuration shared by every module, mirroring the C
/// `lgw_context_t` with the defaults of the static initializer in
/// `loragw_hal.c`.
#[derive(Debug, Clone, Copy)]
pub struct Context {
    /// `lgw_start()` ran successfully (C: `is_started`).
    pub is_started: bool,
    /// Board configuration (C: `board_cfg`).
    pub board: BoardConf,
    /// RF chain configurations (C: `rf_chain_cfg`).
    pub rf_chain: [RfConf; LGW_RF_CHAIN_NB],
    /// IF chain configurations (C: `if_chain_cfg`).
    pub if_chain: [IfConf; LGW_IF_CHAIN_NB],
    /// Demodulator configuration (C: `demod_cfg`).
    pub demod: DemodConf,
    /// LoRa "std" service channel parameters (C: `lora_service_cfg`).
    pub lora_service: IfConf,
    /// FSK channel parameters (C: `fsk_cfg`).
    pub fsk: IfConf,
    /// TX gain LUTs, one per RF chain (C: `tx_gain_lut`).
    pub tx_gain_lut: [TxGainLut; LGW_RF_CHAIN_NB],
    /// Fine timestamping configuration (C: `ftime_cfg`).
    pub ftime: FtimeConf,
    /// Last known ambient temperature in °C, used for the RSSI temperature
    /// compensation in [`crate::Lr1302::receive`].
    ///
    /// The C HAL reads it from an STTS751 sensor over I2C, which is not
    /// ported; set it yourself with
    /// [`crate::Lr1302::set_temperature`] (e.g. from your own sensor or an
    /// external probe). When `None`, the temperature compensation is skipped
    /// and only the static `rssi_offset` is applied.
    pub temperature: Option<f32>,
    /// RX packet staging buffer (C: global `rx_buffer`).
    pub rx_buffer: RxBuffer,
    /// Timestamp counter wrap tracking (C: global `counter_us`).
    pub counter_us: TimestampCounter,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            is_started: false,
            board: BoardConf::default(),
            // C: `rf_chain_cfg = {{0}}` / `if_chain_cfg = {{0}}`
            rf_chain: [RfConf::default(); LGW_RF_CHAIN_NB],
            if_chain: [IfConf::default(); LGW_IF_CHAIN_NB],
            // C: `.multisf_datarate = LGW_MULTI_SF_EN`
            demod: DemodConf::default(),
            // C: bandwidth = BW_250KHZ, datarate = DR_LORA_SF7, implicit off
            lora_service: IfConf {
                bandwidth: BW_250KHZ,
                datarate: DR_LORA_SF7,
                ..IfConf::default()
            },
            // C: bandwidth = BW_125KHZ, datarate = 50000, sync word 3 x 0xC194C1
            fsk: IfConf {
                bandwidth: BW_125KHZ,
                datarate: 50_000,
                sync_word_size: 3,
                sync_word: 0xC194_C1,
                ..IfConf::default()
            },
            // C: both chains get a single-entry LUT {14,0,2,3,10,0,0,0}
            tx_gain_lut: [TxGainLut::default(); LGW_RF_CHAIN_NB],
            // C: enable = false, mode = LGW_FTIME_MODE_ALL_SF
            ftime: FtimeConf::default(),
            // not part of the C context (I2C sensor), see `set_temperature`
            temperature: None,
            rx_buffer: RxBuffer::default(),
            counter_us: TimestampCounter::default(),
        }
    }
}
