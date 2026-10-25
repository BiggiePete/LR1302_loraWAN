//! Constants of the C HAL (`loragw_hal.h`, `loragw_com.h`).

/// Concentrator return codes (C: `LGW_HAL_SUCCESS` / `LGW_HAL_ERROR`).
pub const LGW_HAL_SUCCESS: i32 = 0;
pub const LGW_HAL_ERROR: i32 = -1;

/// RF reference oscillator frequency (C: `LGW_XTAL_FREQU`).
pub const LGW_XTAL_FREQU: u32 = 32_000_000;
/// Number of RF chains (C: `LGW_RF_CHAIN_NB`).
pub const LGW_RF_CHAIN_NB: usize = 2;
/// Number of IF + modem RX chains (C: `LGW_IF_CHAIN_NB`).
pub const LGW_IF_CHAIN_NB: usize = 10;
/// Reference bandwidth of a data channel (C: `LGW_REF_BW`).
pub const LGW_REF_BW: u32 = 125_000;
/// Number of LoRa "multi SF" chains (C: `LGW_MULTI_NB`).
pub const LGW_MULTI_NB: usize = 8;
/// Bitmask enabling the multi-SF correlators, SF12..SF5 (C: `LGW_MULTI_SF_EN`).
pub const LGW_MULTI_SF_EN: u8 = 0xFF;
/// Maximum size of a TX gain LUT (C: `TX_GAIN_LUT_SIZE_MAX`).
pub const TX_GAIN_LUT_SIZE_MAX: usize = 16;

// modulation ----------------------------------------------------------------

/// Modulation not set (C: `MOD_UNDEFINED`).
pub const MOD_UNDEFINED: u8 = 0;
/// CW modulation (C: `MOD_CW`).
pub const MOD_CW: u8 = 0x08;
/// LoRa modulation (C: `MOD_LORA`).
pub const MOD_LORA: u8 = 0x10;
/// FSK modulation (C: `MOD_FSK`).
pub const MOD_FSK: u8 = 0x20;

// bandwidth -----------------------------------------------------------------

/// Bandwidth not set (C: `BW_UNDEFINED`).
pub const BW_UNDEFINED: u8 = 0;
/// 125 kHz (C: `BW_125KHZ`).
pub const BW_125KHZ: u8 = 0x04;
/// 250 kHz (C: `BW_250KHZ`).
pub const BW_250KHZ: u8 = 0x05;
/// 500 kHz (C: `BW_500KHZ`).
pub const BW_500KHZ: u8 = 0x06;

// datarate ------------------------------------------------------------------

/// Datarate not set (C: `DR_UNDEFINED`).
pub const DR_UNDEFINED: u32 = 0;
/// LoRa SF5 (C: `DR_LORA_SF5`).
pub const DR_LORA_SF5: u32 = 5;
/// LoRa SF6 (C: `DR_LORA_SF6`).
pub const DR_LORA_SF6: u32 = 6;
/// LoRa SF7 (C: `DR_LORA_SF7`).
pub const DR_LORA_SF7: u32 = 7;
/// LoRa SF8 (C: `DR_LORA_SF8`).
pub const DR_LORA_SF8: u32 = 8;
/// LoRa SF9 (C: `DR_LORA_SF9`).
pub const DR_LORA_SF9: u32 = 9;
/// LoRa SF10 (C: `DR_LORA_SF10`).
pub const DR_LORA_SF10: u32 = 10;
/// LoRa SF11 (C: `DR_LORA_SF11`).
pub const DR_LORA_SF11: u32 = 11;
/// LoRa SF12 (C: `DR_LORA_SF12`).
pub const DR_LORA_SF12: u32 = 12;
/// Minimum FSK baudrate (C: `DR_FSK_MIN`).
pub const DR_FSK_MIN: u32 = 500;
/// Maximum FSK baudrate (C: `DR_FSK_MAX`).
pub const DR_FSK_MAX: u32 = 250_000;

// coderate ------------------------------------------------------------------

/// Coding rate not set (C: `CR_UNDEFINED`).
pub const CR_UNDEFINED: u8 = 0;
/// LoRa coding rate 4/5 (C: `CR_LORA_4_5`).
pub const CR_LORA_4_5: u8 = 0x01;
/// LoRa coding rate 4/6 (C: `CR_LORA_4_6`).
pub const CR_LORA_4_6: u8 = 0x02;
/// LoRa coding rate 4/7 (C: `CR_LORA_4_7`).
pub const CR_LORA_4_7: u8 = 0x03;
/// LoRa coding rate 4/8 (C: `CR_LORA_4_8`).
pub const CR_LORA_4_8: u8 = 0x04;

// RX packet status ----------------------------------------------------------

/// CRC status unknown (C: `STAT_UNDEFINED`).
pub const STAT_UNDEFINED: u8 = 0x00;
/// Packet has no CRC (C: `STAT_NO_CRC`).
pub const STAT_NO_CRC: u8 = 0x01;
/// Packet CRC is wrong (C: `STAT_CRC_BAD`).
pub const STAT_CRC_BAD: u8 = 0x11;
/// Packet CRC is correct (C: `STAT_CRC_OK`).
pub const STAT_CRC_OK: u8 = 0x10;

// TX ------------------------------------------------------------------------

/// Transmit immediately (C: `IMMEDIATE`).
pub const IMMEDIATE: u8 = 0;
/// Transmit at a timestamp (C: `TIMESTAMPED`).
pub const TIMESTAMPED: u8 = 1;
/// Transmit on GPS trigger (C: `ON_GPS`).
pub const ON_GPS: u8 = 2;

/// Select TX status (C: `TX_STATUS`).
pub const TX_STATUS: u8 = 1;
/// Select RX status (C: `RX_STATUS`).
pub const RX_STATUS: u8 = 2;

/// TX status unknown (C: `TX_STATUS_UNKNOWN`).
pub const TX_STATUS_UNKNOWN: u8 = 0;
/// TX modem disabled (C: `TX_OFF`).
pub const TX_OFF: u8 = 1;
/// TX modem ready (C: `TX_FREE`).
pub const TX_FREE: u8 = 2;
/// TX modem loaded, waiting for trigger (C: `TX_SCHEDULED`).
pub const TX_SCHEDULED: u8 = 3;
/// TX modem emitting (C: `TX_EMITTING`).
pub const TX_EMITTING: u8 = 4;

/// RX status unknown (C: `RX_STATUS_UNKNOWN`).
pub const RX_STATUS_UNKNOWN: u8 = 0;
/// RX modem disabled (C: `RX_OFF`).
pub const RX_OFF: u8 = 1;
/// RX modem receiving (C: `RX_ON`).
pub const RX_ON: u8 = 2;
/// RX suspended during a TX (C: `RX_SUSPENDED`).
pub const RX_SUSPENDED: u8 = 3;

// radio types ----------------------------------------------------------------

/// Radio type of a RF chain (C: `lgw_radio_type_t`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum RadioType {
    /// No radio (C: `LGW_RADIO_TYPE_NONE`).
    None = 0,
    /// SX1255 (not supported by this port).
    Sx1255 = 1,
    /// SX1257 (not supported by this port).
    Sx1257 = 2,
    /// SX1272 (not supported by this port).
    Sx1272 = 3,
    /// SX1276 (not supported by this port).
    Sx1276 = 4,
    /// SX1250 (C: `LGW_RADIO_TYPE_SX1250`), the LR1302 radio.
    Sx1250 = 5,
}
