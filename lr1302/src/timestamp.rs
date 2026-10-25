//! SX1302 timestamp counters (C: `loragw_sx1302_timestamp.c`).
//!
//! Keeps the rollover state of the two 27-bit 1 MHz counters of the SX1302
//! (PPS-triggered and free-running/`inst`) in [`crate::context::TimestampCounter`],
//! expands them onto a continuous 32-bit microsecond timeline, and computes
//! the demodulation processing delay to subtract from packet timestamps
//! (C: `timestamp_counter_correction`).
//!
//! Not ported (fine/"precision" timestamps are out of scope, see the crate
//! docs), documented here as omissions:
//!
//! * `precise_timestamp_calculate` and its private helpers
//!   `timestamp_pps_history_save` (the C file-static PPS history has no Rust
//!   state; its call in `timestamp_counter_get` is dropped) and
//!   `sx1302_dc_notch_delay`.
//! * `precision_timestamp_correction`, i.e. the `ftime_cfg.enable == true`
//!   branch of `timestamp_counter_correction` — that function therefore
//!   always returns the legacy (`legacy_timestamp_correction`) value.
//! * `timestamp_counter_delete` is folded into `timestamp_counter_new` (both
//!   C functions are a `memset(0)`, and the C HAL never calls `delete`); a
//!   `timestamp_counter_delete` alias is still provided for parity.
//!
//! All `printf` / `DEBUG_*` output of the C code is dropped.

use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::spi::SpiDevice;

use crate::context::TimestampCounter;
use crate::consts::{BW_125KHZ, BW_250KHZ, BW_500KHZ};
use crate::error::Result;
use crate::sx1302_defs::set_ppm_on;
use crate::util::{is_lora_bw, is_lora_cr, is_lora_dr};
use crate::Lr1302;

/// DFT peak detection disabled (C: `RX_DFT_PEAK_MODE_DISABLED`,
/// `sx1302_rx_dft_peak_mode_t`).
pub const RX_DFT_PEAK_MODE_DISABLED: u8 = 0x00;
/// DFT peak detection full mode (C: `RX_DFT_PEAK_MODE_FULL`,
/// `sx1302_rx_dft_peak_mode_t`).
pub const RX_DFT_PEAK_MODE_FULL: u8 = 0x01;
/// DFT peak detection track mode (C: `RX_DFT_PEAK_MODE_TRACK`,
/// `sx1302_rx_dft_peak_mode_t`).
pub const RX_DFT_PEAK_MODE_TRACK: u8 = 0x02;
/// DFT peak detection automatic mode (C: `RX_DFT_PEAK_MODE_AUTO`,
/// `sx1302_rx_dft_peak_mode_t`).
pub const RX_DFT_PEAK_MODE_AUTO: u8 = 0x03;

/// Number of timestamp metrics in precision timestamp mode
/// (C: `PRECISION_TIMESTAMP_TS_METRICS_MAX`).
const PRECISION_TIMESTAMP_TS_METRICS_MAX: i32 = 32;
/// Number of symbols in precision timestamp mode
/// (C: `PRECISION_TIMESTAMP_NB_SYMBOLS`).
const PRECISION_TIMESTAMP_NB_SYMBOLS: i32 = 0;

impl<SPI, D> Lr1302<SPI, D>
where
    SPI: SpiDevice<u8>,
    D: DelayNs,
{
    /// Reset the counter wrap-tracking state (C: `timestamp_counter_new`).
    ///
    /// The C `timestamp_counter_delete` performs the exact same `memset(0)`
    /// and has no callers in the C HAL; it is folded into this reset (an
    /// alias is provided below).
    pub(crate) fn timestamp_counter_new(&mut self) {
        self.ctx.counter_us = TimestampCounter::default();
    }

    /// Reset the counter wrap-tracking state (C: `timestamp_counter_delete`).
    ///
    /// Identical to [`Self::timestamp_counter_new`] (see above).
    #[allow(dead_code)] // the C HAL never calls this either; kept for API parity
    pub(crate) fn timestamp_counter_delete(&mut self) {
        self.timestamp_counter_new();
    }

    /// Track a counter rollover and update the reference values
    /// (C: `timestamp_counter_update`).
    pub(crate) fn timestamp_counter_update(&mut self, pps: u32, inst: u32) {
        /* Check if counter has wrapped, and update wrap status if necessary */
        let counter = &mut self.ctx.counter_us;
        if pps < counter.pps.counter_us_27bits_ref {
            // C: `wrap += 1; wrap %= 32;` on a `uint8_t`
            counter.pps.counter_us_27bits_wrap =
                counter.pps.counter_us_27bits_wrap.wrapping_add(1) % 32;
        }
        if inst < counter.inst.counter_us_27bits_ref {
            counter.inst.counter_us_27bits_wrap =
                counter.inst.counter_us_27bits_wrap.wrapping_add(1) % 32;
        }

        /* Update counter reference */
        counter.pps.counter_us_27bits_ref = pps;
        counter.inst.counter_us_27bits_ref = inst;
    }

    /// Read both raw counters from the SX1302, update the rollover state and
    /// expand the values onto the 32-bit timeline (C: `timestamp_counter_get`).
    ///
    /// Returns `(inst, pps)` — the free-running counter first, matching the
    /// order of the C output parameters.
    pub(crate) async fn timestamp_counter_get(&mut self) -> Result<(u32, u32)> {
        let mut buff = [0u8; 8];
        let mut buff_wa = [0u8; 8];

        /* Get the freerun and pps 32MHz timestamp counters - 8 bytes
                0 -> 3 : PPS counter
                4 -> 7 : Freerun counter (inst)
        */
        self.com
            .reg_rb(
                crate::reg::SX1302_REG_TIMESTAMP_TIMESTAMP_PPS_MSB2_TIMESTAMP_PPS,
                &mut buff,
            )
            .await?;

        /* Workaround concentrator chip issue:
            - read MSB again
            - if MSB changed, read the full counter again
         */
        self.com
            .reg_rb(
                crate::reg::SX1302_REG_TIMESTAMP_TIMESTAMP_PPS_MSB2_TIMESTAMP_PPS,
                &mut buff_wa,
            )
            .await?;
        if (buff[0] != buff_wa[0]) || (buff[4] != buff_wa[4]) {
            self.com
                .reg_rb(
                    crate::reg::SX1302_REG_TIMESTAMP_TIMESTAMP_PPS_MSB2_TIMESTAMP_PPS,
                    &mut buff_wa,
                )
                .await?;
            buff = buff_wa; /* use the new read value */
        }

        let counter_pps_us_raw_27bits_now: u32 = ((buff[0] as u32) << 24)
            | ((buff[1] as u32) << 16)
            | ((buff[2] as u32) << 8)
            | (buff[3] as u32);
        let counter_inst_us_raw_27bits_now: u32 = ((buff[4] as u32) << 24)
            | ((buff[5] as u32) << 16)
            | ((buff[6] as u32) << 8)
            | (buff[7] as u32);

        // C also calls `timestamp_pps_history_save()` here; dropped because the
        // PPS history only serves `precise_timestamp_calculate` (not ported).

        /* Scale to 1MHz */
        let counter_pps_us_raw_27bits_now = counter_pps_us_raw_27bits_now / 32;
        let counter_inst_us_raw_27bits_now = counter_inst_us_raw_27bits_now / 32;

        /* Update counter wrapping status */
        self.timestamp_counter_update(
            counter_pps_us_raw_27bits_now,
            counter_inst_us_raw_27bits_now,
        );

        /* Convert 27-bits counter to 32-bits counter */
        let inst = self.timestamp_counter_expand(false, counter_inst_us_raw_27bits_now);
        let pps = self.timestamp_counter_expand(true, counter_pps_us_raw_27bits_now);

        Ok((inst, pps))
    }

    /// Expand a raw 27-bit 1 MHz counter value onto the 32-bit timeline
    /// (C: `timestamp_counter_expand`).
    pub(crate) fn timestamp_counter_expand(&self, pps: bool, cnt_us: u32) -> u32 {
        let tinfo = if pps {
            &self.ctx.counter_us.pps
        } else {
            &self.ctx.counter_us.inst
        };
        (tinfo.counter_us_27bits_wrap as u32) << 27 | cnt_us
    }

    /// Expand a packet timestamp read from the SX1302 internal FIFO onto the
    /// 32-bit timeline (C: `timestamp_pkt_expand`).
    pub(crate) fn timestamp_pkt_expand(&self, pkt_cnt_us: u32) -> u32 {
        let tinfo = &self.ctx.counter_us.inst;

        /* Check if counter has wrapped since the packet has been received in
            the sx1302 internal FIFO:
            case 1: current wrap status counter
            case 2: previous wrap status counter
        */

        /* Use current wrap counter or previous ? */
        // C: `wrap - ((ref >= pkt) ? 0 : 1)` in `int`, then `& 0x1F`;
        // `u8::wrapping_sub` + mask gives the same [0..31] result.
        let wrap_status = tinfo.counter_us_27bits_wrap.wrapping_sub(
            if tinfo.counter_us_27bits_ref >= pkt_cnt_us {
                0
            } else {
                1
            },
        ) & 0x1F; /* [0..31] */

        /* Expand packet counter */
        (wrap_status as u32) << 27 | pkt_cnt_us
    }

    /// Select legacy or precision timestamp output (C: `timestamp_counter_mode`).
    pub(crate) async fn timestamp_counter_mode(&mut self, ftime_enable: bool) -> Result<()> {
        if !ftime_enable {
            /* Latch end-of-packet timestamp (sx1301 compatibility) */
            self.com
                .reg_w(crate::reg::SX1302_REG_RX_TOP_RX_BUFFER_LEGACY_TIMESTAMP, 0x01)
                .await?;
        } else {
            /* Latch end-of-preamble timestamp */
            self.com
                .reg_w(crate::reg::SX1302_REG_RX_TOP_RX_BUFFER_LEGACY_TIMESTAMP, 0x00)
                .await?;
            self.com
                .reg_w(
                    crate::reg::SX1302_REG_RX_TOP_RX_BUFFER_TIMESTAMP_CFG_MAX_TS_METRICS,
                    PRECISION_TIMESTAMP_TS_METRICS_MAX,
                )
                .await?;

            /* LoRa multi-SF modems */
            self.com
                .reg_w(crate::reg::SX1302_REG_RX_TOP_TIMESTAMP_ENABLE, 0x01)
                .await?;
            self.com
                .reg_w(
                    crate::reg::SX1302_REG_RX_TOP_TIMESTAMP_NB_SYMB,
                    PRECISION_TIMESTAMP_NB_SYMBOLS,
                )
                .await?;
        }

        Ok(())
    }

    /// Compute the correction to apply to a packet timestamp for the
    /// demodulation processing delay (C: `timestamp_counter_correction`).
    ///
    /// Reads no registers, so it stays a synchronous `&self` method. Input
    /// validation mirrors the C `IS_LORA_DR` / `IS_LORA_BW` / `IS_LORA_CR`
    /// checks (which print an error and return 0 in C).
    ///
    /// Deviation: the C function dispatches on `context->ftime_cfg.enable`
    /// and calls `precision_timestamp_correction` when fine timestamping is
    /// on. Precision timestamps are not ported (see module docs), so the
    /// legacy correction is returned in both cases.
    pub(crate) fn timestamp_counter_correction(
        &self,
        bandwidth: u8,
        datarate: u32,
        coderate: u8,
        crc_en: bool,
        payload_length: u8,
        dft_peak_mode: u8,
    ) -> i32 {
        /* Check input parameters */
        if !is_lora_dr(datarate) {
            return 0;
        }
        if !is_lora_bw(bandwidth) {
            return 0;
        }
        if !is_lora_cr(coderate) {
            return 0;
        }

        legacy_timestamp_correction(
            bandwidth,
            datarate as u8,
            coderate,
            crc_en,
            payload_length,
            dft_peak_mode,
        )
    }
}

/// Correction for the legacy (end-of-packet) timestamp path
/// (C: `legacy_timestamp_correction`).
///
/// Pure arithmetic: all delays are computed on a 32 MHz clock and summed in
/// `uint64_t`. The C `(sum + 500E3) / 1E6` goes through `double`; because
/// every operand is an exactly representable integer below 2^53 and the true
/// quotient is at least 1/1e6 away from any integer boundary, the integer
/// division below truncates identically.
fn legacy_timestamp_correction(
    bandwidth: u8,
    sf: u8,
    coderate: u8,
    crc_en: bool,
    payload_length: u8,
    dft_peak_mode: u8,
) -> i32 {
    let mut dft_peak_en: u8 = if dft_peak_mode == RX_DFT_PEAK_MODE_DISABLED {
        0
    } else {
        1
    };
    let ppm: u8 = if set_ppm_on(bandwidth, sf as u32) {
        1
    } else {
        0
    };

    let bw_pow: u8 = match bandwidth {
        BW_125KHZ => 1,
        BW_250KHZ => 2,
        BW_500KHZ => 4,
        // C: prints an error and returns 0
        _ => return 0,
    };

    /* Prepare variables for delay computing */
    // C: `clk_period = 250E3 / bw_pow;` (double division, exact here)
    let clk_period: u64 = 250_000 / bw_pow as u64;

    let nb_nibble: u32 = (payload_length as u32 + 2 * crc_en as u32) * 2 + 5;

    let nb_nibble_in_hdr: u32 = if (sf == 5) || (sf == 6) {
        sf as u32
    } else {
        sf as u32 - 2
    };

    // All of this stays in `uint32_t` wrapping arithmetic, exactly like C
    // (for legal inputs the first subtraction can wrap for SF5/SF6).
    let step: u32 = sf as u32 - 2 * ppm as u32;
    let diff: u32 = nb_nibble.wrapping_sub(nb_nibble_in_hdr);
    let mut nb_nibble_in_last_block: u32 =
        diff.wrapping_sub(step.wrapping_mul(diff.wrapping_div(step)));
    if nb_nibble_in_last_block == 0 {
        nb_nibble_in_last_block = step;
    }

    let nb_iter: u8 = (sf + 1) / 2; /* intended to be truncated */

    let mut payload_fits_in_header = false;
    let mut cr_local: u8 = coderate;

    /* Update some variables if payload fits entirely in the header */
    if (2 * (payload_length as i32 + 2 * crc_en as i32) - (sf as i32 - 7)) <= 0
        || ((payload_length == 0) && !crc_en)
    {
        /* Payload fits entirely in first 8 symbols (header):
            - not possible for SF5/SF6, unless payload length is 0 and no CRC
        */
        payload_fits_in_header = true;

        /* overwrite some variables accordingly */
        dft_peak_en = 0;

        cr_local = 4; /* header coding rate is 4 */

        if sf > 6 {
            nb_nibble_in_last_block = sf as u32 - 2;
        } else {
            nb_nibble_in_last_block = sf as u32;
        }
    }

    /* Filtering delay : I/Q 32Mhz -> 4Mhz */
    // C: `16000E3 / bw_pow + 2031250` (double division, exact here)
    let filtering_delay: u64 = 16_000_000 / bw_pow as u64 + 2_031_250;

    /* demap delay */
    let demap_delay: u64 = if payload_fits_in_header {
        clk_period
            + (1u64 << sf) * clk_period * 3 / 4
            + 3 * clk_period
            + (sf as u64 - 2) * clk_period
    } else {
        // C: `(1 - ppm / 4)` is an `int` expression (ppm/4 == 0)
        clk_period
            + (1u64 << sf) * clk_period * (1 - ppm as u64 / 4)
            + 3 * clk_period
            + (sf as u64 - 2 * ppm as u64) * clk_period
    };

    /* FFT delays */
    let fft_delay_state3: u64 = clk_period
        * (((1u64 << sf) - 6) + 2 * ((1u64 << sf) * (nb_iter as u64 - 1) + 6))
        + 4 * clk_period;

    let fft_delay: u64 = if dft_peak_en != 0 {
        (5 - 2 * ppm as i32) as u64
            * ((1u64 << sf) * clk_period + 7 * clk_period)
            + 2 * clk_period
    } else {
        (1u64 << sf) * 2 * clk_period + 3 * clk_period
    };

    /* Decode delay */
    let decode_delay: u64 = 5 * clk_period
        + (9 * clk_period + clk_period * cr_local as u64) * nb_nibble_in_last_block as u64
        + 3 * clk_period;

    /* Cumulated delays (C: integer result of `(sum + 500E3) / 1E6`) */
    let total_delay: u64 = (filtering_delay
        + fft_delay_state3
        + fft_delay
        + demap_delay
        + decode_delay
        + 500_000)
        / 1_000_000;

    // C: prints a diagnostic and `assert(0)` — "SHOULD NOT HAPPEN" (the
    // delays stay far below `INT32_MAX` for all legal inputs).
    debug_assert!(
        total_delay <= i32::MAX as u64,
        "overflow error for timestamp correction (SHOULD NOT HAPPEN)"
    );

    -(total_delay as i32) /* compensate all decoding processing delays */
}
