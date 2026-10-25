//! Constants and macros shared by the SX1302 driver modules.
//!
//! Port of the private macros of `loragw_sx1302.c` and the public defines of
//! `loragw_sx1302.h` / `loragw_agc_params.h`.

use crate::consts::LGW_IF_CHAIN_NB;

// ---------------------------------------------------------------------------
// IF chain + modem types (C: `loragw_sx1302.h`)
// ---------------------------------------------------------------------------

/// IF chain not configurable (C: `IF_UNDEFINED`).
pub const IF_UNDEFINED: u8 = 0x00;
/// IF + standard single-SF LoRa modem (C: `IF_LORA_STD`).
pub const IF_LORA_STD: u8 = 0x10;
/// IF + LoRa receiver with multi-SF capability (C: `IF_LORA_MULTI`).
pub const IF_LORA_MULTI: u8 = 0x11;
/// IF + standard FSK modem (C: `IF_FSK_STD`).
pub const IF_FSK_STD: u8 = 0x20;

/// Automatic radio gain control (C: `SX1302_AGC_RADIO_GAIN_AUTO`).
pub const SX1302_AGC_RADIO_GAIN_AUTO: u8 = 0xFF;
/// Default calibrated TX start delay, for 500 kHz BW (C: `TX_START_DELAY_DEFAULT`).
pub const TX_START_DELAY_DEFAULT: u16 = 1500;

/// Hardware capability: type of each of the 10 IF chains + modems
/// (C: `ifmod_config[LGW_IF_CHAIN_NB]`, `LGW_IFMODEM_CONFIG`).
pub const IFMODEM_CONFIG: [u8; LGW_IF_CHAIN_NB] = [
    IF_LORA_MULTI,
    IF_LORA_MULTI,
    IF_LORA_MULTI,
    IF_LORA_MULTI,
    IF_LORA_MULTI,
    IF_LORA_MULTI,
    IF_LORA_MULTI,
    IF_LORA_MULTI,
    IF_LORA_STD,
    IF_FSK_STD,
];

/// Chip model ID of an SX1302 (C: `CHIP_MODEL_ID_SX1302`, may also be 0x00).
pub const CHIP_MODEL_ID_SX1302: u8 = 0x02;
/// Chip model ID of an SX1303 (C: `CHIP_MODEL_ID_SX1303`).
pub const CHIP_MODEL_ID_SX1303: u8 = 0x03;

// ---------------------------------------------------------------------------
// Macros
// ---------------------------------------------------------------------------

/// Select register `a` for RF chain 0 and `b` for RF chain 1
/// (C: `REG_SELECT(rf_chain, a, b)`).
pub const fn reg_select(rf_chain: u8, a: u16, b: u16) -> u16 {
    if rf_chain == 0 {
        a
    } else {
        b
    }
}

/// PPM lock needed for this bandwidth / datarate combination
/// (C: `SET_PPM_ON(bw, dr)`).
pub const fn set_ppm_on(bw: u8, dr: u32) -> bool {
    use crate::consts::*;
    (bw == BW_125KHZ && (dr == DR_LORA_SF11 || dr == DR_LORA_SF12))
        || (bw == BW_250KHZ && dr == DR_LORA_SF12)
}

/// Convert an IF frequency in Hz to its register value
/// (C: `IF_HZ_TO_REG(f)`).
pub const fn if_hz_to_reg(f: i32) -> i32 {
    (f * 32) / 15625
}

/// Convert a frequency in Hz to the SX1302 `SX1302_FREQ_TO_REG` value.
pub const fn sx1302_freq_to_reg(f: u64) -> u32 {
    ((f * (1 << 18)) / 32_000_000) as u32
}

// ---------------------------------------------------------------------------
// Firmware / mailbox constants (C: `loragw_sx1302.c` private constants)
// ---------------------------------------------------------------------------

/// Radio A calibration done flag (C: `AGC_RADIO_A_INIT_DONE`).
pub const AGC_RADIO_A_INIT_DONE: u8 = 0x80;
/// Radio B calibration done flag (C: `AGC_RADIO_B_INIT_DONE`).
pub const AGC_RADIO_B_INIT_DONE: u8 = 0x20;

/// AGC MCU identifier (C: `MCU_AGC`).
pub const MCU_AGC: u8 = 0x01;
/// Arbiter MCU identifier (C: `MCU_ARB`).
pub const MCU_ARB: u8 = 0x02;

/// Load address of the AGC firmware (C: `AGC_MEM_ADDR`).
pub const AGC_MEM_ADDR: u16 = 0x0000;
/// Load address of the arbiter firmware (C: `ARB_MEM_ADDR`).
pub const ARB_MEM_ADDR: u16 = 0x2000;

/// Firmware image size in bytes (C: `MCU_FW_SIZE`).
pub const MCU_FW_SIZE: usize = 8192;

/// Expected AGC firmware version for SX1250 based gateways
/// (C: `FW_VERSION_AGC_SX1250` in `loragw_hal.c`).
pub const FW_VERSION_AGC_SX1250: u8 = 10;
/// Expected arbiter firmware version (C: `FW_VERSION_ARB` in `loragw_hal.c`).
pub const FW_VERSION_ARB: u8 = 2;

// ---------------------------------------------------------------------------
// Preambles (C: `loragw_sx1302.c`)
// ---------------------------------------------------------------------------

/// Minimum LoRa preamble length (C: `MIN_LORA_PREAMBLE`).
pub const MIN_LORA_PREAMBLE: u16 = 6;
/// Standard LoRa preamble length (C: `STD_LORA_PREAMBLE`).
pub const STD_LORA_PREAMBLE: u16 = 8;
/// Minimum FSK preamble length (C: `MIN_FSK_PREAMBLE`).
pub const MIN_FSK_PREAMBLE: u16 = 3;
/// Standard FSK preamble length (C: `STD_FSK_PREAMBLE`).
pub const STD_FSK_PREAMBLE: u16 = 5;

// ---------------------------------------------------------------------------
// GPIO configuration codes (C: `loragw_sx1302.c`)
// ---------------------------------------------------------------------------

/// GPIO function: general register (C: `GPIO_CFG_REGISTER`).
pub const GPIO_CFG_REGISTER: u8 = 0x00;
/// GPIO function: AGC (C: `GPIO_CFG_AGC`).
pub const GPIO_CFG_AGC: u8 = 0x01;
/// GPIO function: arbiter (C: `GPIO_CFG_ARB`).
pub const GPIO_CFG_ARB: u8 = 0x02;
/// GPIO function: SPI expander 1 (C: `GPIO_CFG_SPI_EXP_1`).
pub const GPIO_CFG_SPI_EXP_1: u8 = 0x03;
/// GPIO function: SPI expander CSN (C: `GPIO_CFG_CSN_SPI_EXP`).
pub const GPIO_CFG_CSN_SPI_EXP: u8 = 0x04;
/// GPIO function: SPI expander 2 (C: `GPIO_CFG_SPI_EXP_2`).
pub const GPIO_CFG_SPI_EXP_2: u8 = 0x05;
/// GPIO function: UART (C: `GPIO_CFG_UART`).
pub const GPIO_CFG_UART: u8 = 0x06;
/// GPIO function: SX1255 IQ (C: `GPIO_CFG_SX1255_IQ`).
pub const GPIO_CFG_SX1255_IQ: u8 = 0x07;
/// GPIO function: SX1261 IQ (C: `GPIO_CFG_SX1261_IQ`).
pub const GPIO_CFG_SX1261_IQ: u8 = 0x08;
/// GPIO function: status output (C: `GPIO_CFG_STATUS`).
pub const GPIO_CFG_STATUS: u8 = 0x09;
/// GPIO function: MBIST (C: `GPIO_CFG_MBIST`).
pub const GPIO_CFG_MBIST: u8 = 0x0A;
/// GPIO function: OTP (C: `GPIO_CFG_OTP`).
pub const GPIO_CFG_OTP: u8 = 0x0B;

// ---------------------------------------------------------------------------
// RSSI / frequency constants (C: `loragw_sx1302.c`)
// ---------------------------------------------------------------------------

/// FSK RSSI linearization polynomial coefficient 0 (C: `RSSI_FSK_POLY_0`).
pub const RSSI_FSK_POLY_0: f64 = 90.636423;
/// FSK RSSI linearization polynomial coefficient 1 (C: `RSSI_FSK_POLY_1`).
pub const RSSI_FSK_POLY_1: f64 = 0.420835;
/// FSK RSSI linearization polynomial coefficient 2 (C: `RSSI_FSK_POLY_2`).
pub const RSSI_FSK_POLY_2: f64 = 0.007129;
/// FSK RSSI linearization polynomial coefficient 3 (C: `RSSI_FSK_POLY_3`).
pub const RSSI_FSK_POLY_3: f64 = -0.000026;

/// Frequency offset LSB, 125 kHz channels (C: `FREQ_OFFSET_LSB_125KHZ`).
pub const FREQ_OFFSET_LSB_125KHZ: f32 = 0.119_209_29;
/// Frequency offset LSB, 250 kHz channels (C: `FREQ_OFFSET_LSB_250KHZ`).
pub const FREQ_OFFSET_LSB_250KHZ: f32 = 0.238_418_579;
/// Frequency offset LSB, 500 kHz channels (C: `FREQ_OFFSET_LSB_500KHZ`).
pub const FREQ_OFFSET_LSB_500KHZ: f32 = 0.476_837_158;

// ---------------------------------------------------------------------------
// AGC gain parameters (C: `loragw_agc_params.h`, `agc_params_sx1250`)
// ---------------------------------------------------------------------------

/// AGC gain parameters of the SX1250 radio (C: `struct agc_gain_params_s`).
#[derive(Debug, Clone, Copy)]
pub struct AgcGainParams {
    /// C: `ana_min`
    pub ana_min: u8,
    /// C: `ana_max`
    pub ana_max: u8,
    /// C: `ana_thresh_l`
    pub ana_thresh_l: u8,
    /// C: `ana_thresh_h`
    pub ana_thresh_h: u8,
    /// C: `dec_attn_min`
    pub dec_attn_min: u8,
    /// C: `dec_attn_max`
    pub dec_attn_max: u8,
    /// C: `dec_thresh_l`
    pub dec_thresh_l: u8,
    /// C: `dec_thresh_h1`
    pub dec_thresh_h1: u8,
    /// C: `dec_thresh_h2`
    pub dec_thresh_h2: u8,
    /// C: `chan_attn_min`
    pub chan_attn_min: u8,
    /// C: `chan_attn_max`
    pub chan_attn_max: u8,
    /// C: `chan_thresh_l`
    pub chan_thresh_l: u8,
    /// C: `chan_thresh_h`
    pub chan_thresh_h: u8,
    /// C: `deviceSel` (sx1250 only)
    pub device_sel: u8,
    /// C: `hpMax` (sx1250 only)
    pub hp_max: u8,
    /// C: `paDutyCycle` (sx1250 only)
    pub pa_duty_cycle: u8,
}

/// AGC gain parameters for SX1250 radios (C: `agc_params_sx1250`).
pub const AGC_PARAMS_SX1250: AgcGainParams = AgcGainParams {
    ana_min: 1,
    ana_max: 13,
    ana_thresh_l: 3,
    ana_thresh_h: 12,
    dec_attn_min: 4,
    dec_attn_max: 15,
    dec_thresh_l: 40,
    dec_thresh_h1: 80,
    dec_thresh_h2: 90,
    chan_attn_min: 4,
    chan_attn_max: 14,
    chan_thresh_l: 52,
    chan_thresh_h: 132,
    device_sel: 0,
    hp_max: 7,
    pa_duty_cycle: 4,
};
