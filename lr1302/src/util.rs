//! Small helpers ported from `loragw_aux.h` / `loragw_com.h`.

use crate::consts::*;

/// Get `n` bits starting at position `p` of `b` (C: `TAKE_N_BITS_FROM`).
pub const fn take_bits(b: u8, p: u8, n: u8) -> u8 {
    (b >> p) & ((1u8 << n) - 1)
}

/// C: `IS_LORA_BW`
pub const fn is_lora_bw(bw: u8) -> bool {
    bw == BW_125KHZ || bw == BW_250KHZ || bw == BW_500KHZ
}

/// C: `IS_LORA_DR`
pub const fn is_lora_dr(dr: u32) -> bool {
    matches!(dr, 5..=12)
}

/// C: `IS_LORA_CR`
pub const fn is_lora_cr(cr: u8) -> bool {
    matches!(cr, CR_LORA_4_5..=CR_LORA_4_8)
}

/// C: `IS_FSK_BW`
pub const fn is_fsk_bw(bw: u8) -> bool {
    bw >= 1 && bw <= 7
}

/// C: `IS_FSK_DR`
pub const fn is_fsk_dr(dr: u32) -> bool {
    dr >= DR_FSK_MIN && dr <= DR_FSK_MAX
}

/// C: `IS_TX_MODE`
pub const fn is_tx_mode(mode: u8) -> bool {
    matches!(mode, IMMEDIATE | TIMESTAMPED | ON_GPS)
}

/// Integer ceiling division (used to reproduce `ceil()` of the C ToA code
/// exactly, in integer arithmetic).
pub const fn ceil_div(numerator: u64, denominator: u64) -> u64 {
    (numerator + denominator - 1) / denominator
}

/// Bandwidth code to bandwidth in Hz (C: `lgw_bw_getval`, `None` for the
/// C return value `-1`).
pub const fn bw_getval(bw: u8) -> Option<u32> {
    match bw {
        BW_500KHZ => Some(500_000),
        BW_250KHZ => Some(250_000),
        BW_125KHZ => Some(125_000),
        _ => None,
    }
}

/// Datarate code to spreading factor (C: `lgw_sf_getval`, `None` for the C
/// return value `-1`).
pub const fn sf_getval(dr: u32) -> Option<u8> {
    match dr {
        5 => Some(5),
        6 => Some(6),
        7 => Some(7),
        8 => Some(8),
        9 => Some(9),
        10 => Some(10),
        11 => Some(11),
        12 => Some(12),
        _ => None,
    }
}
