//! LoRa packet time-on-air computation (C: `lora_packet_time_on_air` in
//! `loragw_aux.c`).
//!
//! The C implementation uses `double`; the port evaluates the same formulas
//! in integer arithmetic (the payload symbol count is an exact ceiling, and
//! the symbol count has an exact 1/4 symbol granularity), which reproduces
//! the C results bit-for-bit for all legal inputs.

use crate::consts::*;
use crate::util::{ceil_div, is_lora_bw, is_lora_cr, is_lora_dr};

/// Details of a computed time-on-air.
#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Toa {
    /// Total time on air, in microseconds.
    pub toa_us: u32,
    /// Duration of one symbol, in microseconds.
    pub t_symbol_us: u16,
    /// Number of symbols in the payload (without preamble/header).
    pub nb_symbols_payload: u32,
    /// Total number of symbols, in 1/4 symbol units.
    pub nb_symbols_quarters: u32,
}

impl Toa {
    /// Total number of symbols as a float (C returns a `double`).
    pub fn nb_symbols(&self) -> f32 {
        self.nb_symbols_quarters as f32 / 4.0
    }
}

/// Compute the time on air of a LoRa packet (C: `lora_packet_time_on_air`).
///
/// * `bw` - [`BW_125KHZ`], [`BW_250KHZ`] or [`BW_500KHZ`]
/// * `sf` - spreading factor 5..=12
/// * `cr` - [`CR_LORA_4_5`] .. [`CR_LORA_4_8`]
/// * `n_symbol_preamble` - preamble length in symbols
/// * `no_header` - implicit header mode
/// * `no_crc` - do not send a CRC
/// * `size` - payload size in bytes
///
/// Returns `None` when a parameter is out of range (the C function prints
/// an error and returns 0).
pub fn lora_packet_time_on_air(
    bw: u8,
    sf: u8,
    cr: u8,
    n_symbol_preamble: u16,
    no_header: bool,
    no_crc: bool,
    size: u8,
) -> Option<Toa> {
    if !is_lora_dr(sf as u32) || !is_lora_bw(bw) || !is_lora_cr(cr) {
        return None;
    }

    // bandwidth divider relative to 125 kHz
    let bw_pow: u64 = match bw {
        BW_125KHZ => 1,
        BW_250KHZ => 2,
        BW_500KHZ => 4,
        _ => return None,
    };

    // duration of 1 symbol: 2^SF / BW, in microseconds
    let t_symbol_us = ((1u32 << sf) * 8 / bw_pow as u32) as u16;

    let h: u64 = if no_header { 0 } else { 1 };
    let de: u64 = if sf >= 11 { 1 } else { 0 };
    let n_bit_crc: u64 = if no_crc { 0 } else { 16 };

    // number of symbols in the payload:
    //   ceil(max(8*size + crc - 4*sf + (sf>=7 ? 8 : 0) + 20*H, 0)
    //        / (4 * (sf - 2*DE))) * (CR + 4)
    let numerator = (8i64 * size as i64
        + n_bit_crc as i64
        - 4 * sf as i64
        + if sf >= 7 { 8 } else { 0 }
        + 20 * h as i64)
        .max(0) as u64;
    let denominator = 4 * (sf as u64 - 2 * de);
    let n_symbol_payload = ceil_div(numerator, denominator) * (cr as u64 + 4);

    // number of symbols in packet, in quarter symbols
    //   preamble + (sf >= 7 ? 4.25 : 6.25) + 8 + payload
    let n_symbol_quarters = 4 * n_symbol_preamble as u64
        + if sf >= 7 { 17 } else { 25 }
        + 32
        + 4 * n_symbol_payload;

    let toa_us = (n_symbol_quarters * t_symbol_us as u64 / 4) as u32;

    Some(Toa {
        toa_us,
        t_symbol_us,
        nb_symbols_payload: n_symbol_payload as u32,
        nb_symbols_quarters: n_symbol_quarters as u32,
    })
}
