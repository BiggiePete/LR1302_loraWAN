//! C-vs-Rust conformance tests.
//!
//! The point of this module is to *prove* that the ported arithmetic matches
//! the Semtech C HAL bit-for-bit, by running both implementations over the
//! same inputs and comparing their outputs line by line.
//!
//! # How it works
//!
//! * `tools/difftest/harness.c` compiles the *unmodified* vendor sources
//!   (`loragw_hal.c` is `#include`d so its file-static `merge_packets` /
//!   `is_same_pkt` and the global `lgw_context` are reachable) plus
//!   `tools/difftest/stubs.c`, which stubs out everything that would touch
//!   SPI / I2C / GPS. Every function exercised here is pure computation, so
//!   no stub is ever reached (they `abort()` if it is).
//! * The harness reads one command per line from stdin and prints one result
//!   line per command on stdout (see `tools/difftest/README.md`).
//! * Each `#[test]` below builds the same input for the C harness, computes
//!   the expected answer with the ported Rust code, and asserts that the two
//!   agree. `f32` values travel as raw IEEE-754 bit patterns so no precision
//!   is lost between the languages.
//!
//! # Running
//!
//! ```text
//! cargo test -p lr1302
//! ```
//!
//! The first run builds the C harness (MinGW `gcc` on Windows, `gcc` or
//! `clang` elsewhere) via `tools/difftest/build.ps1`; set `LR1302_DIFTEST_SKIP`
//! to skip the comparison (the tests then pass without checking anything —
//! only useful on machines with no C compiler).
//!
//! # What is deliberately *not* compared
//!
//! * Fine ("precision") timestamps: not ported, `timestamp_counter_correction`
//!   always returns the legacy value (`tscorr 1 ...` would diverge by design).
//! * Out-of-range IF chains in `sx1302_get_ifmod_config`: the C version reads
//!   past the end of `ifmod_config[]` (undefined behaviour); this port returns
//!   [`IF_UNDEFINED`](crate::sx1302_defs::IF_UNDEFINED) instead.
//! * Out-of-range parameters to `lora_packet_time_on_air`: the C version
//!   prints an error and returns `0`/`UINT32_MAX` sentinels from
//!   uninitialised locals; the port returns `None` (see
//!   `invalid_time_on_air_inputs_are_rejected`).

use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::{eprintln, format, vec};
use crate::std::string::{String, ToString};
use crate::std::vec::Vec;

use crate::consts::*;
use crate::hal::merge_packets;
use crate::rx;
use crate::sx1302;
use crate::sx1302_defs::IF_UNDEFINED;
use crate::timestamp::{RX_DFT_PEAK_MODE_AUTO, RX_DFT_PEAK_MODE_DISABLED};
use crate::toa;
use crate::types::{RxPacket, RssiTComp, TxPacket};
use crate::Lr1302;

// ---------------------------------------------------------------------------
// C harness plumbing
// ---------------------------------------------------------------------------

/// Absolute path of the built C harness.
fn harness_exe() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("tools")
        .join("difftest")
        .join(if cfg!(windows) { "difftest_c.exe" } else { "difftest_c" })
}

/// Path of the build script for the C harness.
fn harness_build_script() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("tools")
        .join("difftest")
        .join("build.ps1")
}

/// Locate (and, if needed, build) the C harness.
///
/// Returns `None` when the caller asked to skip via `LR1302_DIFTEST_SKIP`.
fn harness() -> Option<PathBuf> {
    if std::env::var_os("LR1302_DIFTEST_SKIP").is_some() {
        eprintln!("LR1302_DIFTEST_SKIP is set: C-vs-Rust comparison skipped");
        return None;
    }
    let exe = harness_exe();
    if exe.exists() {
        return Some(exe);
    }
    eprintln!("building C harness: {}", harness_build_script().display());
    let built = Command::new("powershell")
        .args(["-ExecutionPolicy", "Bypass", "-File"])
        .arg(harness_build_script())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();
    match built {
        Ok(s) if s.success() && exe.exists() => Some(exe),
        other => panic!(
            "failed to build the C harness ({}: {:?}). \
             Install MinGW gcc (C:\\MinGW\\bin\\gcc.exe) or set LR1302_DIFTEST_SKIP=1 \
             to skip the C-vs-Rust comparison.",
            harness_build_script().display(),
            other
        ),
    }
}

/// Feed `input` to the C harness and collect the lines it prints.
///
/// The input goes through a temp file rather than a pipe: the sweeps below
/// are large enough (~100 kB) that a pipe would deadlock against the harness'
/// own stdout once that pipe fills up.
fn run_c(exe: &std::path::Path, input: &[String]) -> Vec<String> {
    let tmp_dir = std::env::temp_dir();
    // A per-process id alone is not enough: cargo test runs the tests in
    // parallel threads of the same process, so add a unique sequence number.
    static SEQ: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);
    let seq = SEQ.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
    let in_path = tmp_dir.join(format!(
        "lr1302_difftest_in_{}_{}.txt",
        std::process::id(),
        seq
    ));
    let mut text = String::new();
    for line in input {
        text.push_str(line);
        text.push('\n');
    }
    std::fs::write(&in_path, &text).expect("failed to write difftest input file");
    let in_file = std::fs::File::open(&in_path).expect("failed to open difftest input file");

    let out = Command::new(exe)
        .stdin(Stdio::from(in_file))
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .output()
        .unwrap_or_else(|e| panic!("failed to run {}: {e}", exe.display()));
    let _ = std::fs::remove_file(&in_path);
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .map(str::to_string)
        .collect()
}

/// Run the C harness and assert that every expected line comes back verbatim.
///
/// The C HAL prints diagnostics (e.g. `ERROR: wrong datarate`) on the same
/// stdout stream, so only lines whose first token matches the next expected
/// response are compared; anything else is treated as noise and ignored.
fn assert_matches_c(input: Vec<String>, expected: Vec<String>) {
    // Most commands map 1:1 input line -> response line; `merge` consumes a
    // variable number of extra lines, hence only the <= comparison.
    assert!(
        input.len() >= expected.len(),
        "internal error: more expected lines ({}) than input commands ({})",
        expected.len(),
        input.len()
    );
    let Some(exe) = harness() else { return };
    let lines = run_c(&exe, &input);

    let mut next = 0usize;
    let mut failures: Vec<String> = Vec::new();
    for line in &lines {
        let token = line.split_whitespace().next().unwrap_or("");
        let want = expected.get(next).map(|e| e.split_whitespace().next().unwrap_or(""));
        if token.is_empty() || want != Some(token) {
            continue; // C diagnostic, not a response line
        }
        if line != &expected[next] {
            failures.push(format!("  case {next}:\n    rust: {}\n    c:    {line}", expected[next]));
        }
        next += 1;
        if failures.len() >= 20 {
            break;
        }
    }

    if next < expected.len() || !failures.is_empty() {
        panic!(
            "C/Rust divergence ({} of {} cases checked)\n{}\n--- full C output ---\n{}",
            next,
            expected.len(),
            if failures.is_empty() {
                "(too few response lines)".to_string()
            } else {
                failures.join("\n")
            },
            lines.join("\n")
        );
    }
}

/// A driver instance whose bus is never touched (the tested functions are
/// pure arithmetic, but a few of them are methods on [`Lr1302`]).
fn dummy_driver() -> Lr1302<Bus, Clock> {
    Lr1302::new(Bus, Clock)
}

struct Bus;
struct Clock;

impl embedded_hal::spi::ErrorType for Bus {
    type Error = core::convert::Infallible;
}

impl embedded_hal_async::spi::SpiDevice<u8> for Bus {
    fn transaction(
        &mut self,
        _transaction: &mut [embedded_hal_async::spi::Operation<'_, u8>],
    ) -> impl core::future::Future<Output = Result<(), Self::Error>> {
        core::future::ready(Ok(()))
    }
}

impl embedded_hal_async::delay::DelayNs for Clock {
    fn delay_ns(&mut self, _ns: u32) -> impl core::future::Future<Output = ()> {
        core::future::ready(())
    }
}

/// Deterministic pseudo-random payload bytes (xorshift32) so both sides see
/// the same data without shipping a big table.
fn pseudo_bytes(seed: &mut u32, len: usize) -> Vec<u8> {
    (0..len)
        .map(|_| {
            *seed ^= *seed << 13;
            *seed ^= *seed >> 17;
            *seed ^= *seed << 5;
            (*seed & 0xFF) as u8
        })
        .collect()
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02X}")).collect()
}

fn f32_bits(f: f32) -> String {
    format!("{:08x}", f.to_bits())
}

// ---------------------------------------------------------------------------
// tests
// ---------------------------------------------------------------------------

/// `lora_packet_time_on_air` (C: `loragw_aux.c`) over the whole legal
/// parameter space.
#[test]
fn lora_time_on_air_matches_c() {
    let mut input = Vec::new();
    let mut expected = Vec::new();

    for &bw in &[BW_125KHZ, BW_250KHZ, BW_500KHZ] {
        for sf in 5u8..=12 {
            for cr in 1u8..=4 {
                for &preamble in &[6u16, 8, 12, 16] {
                    for &no_header in &[false, true] {
                        for &no_crc in &[false, true] {
                            for &size in &[1u8, 12, 33, 200] {
                                input.push(format!(
                                    "toa_lora {bw} {sf} {cr} {preamble} {} {} {size}",
                                    no_header as u8,
                                    no_crc as u8
                                ));
                                let t = toa::lora_packet_time_on_air(
                                    bw, sf, cr, preamble, no_header, no_crc, size,
                                )
                                .expect("all inputs in this sweep are legal");
                                expected.push(format!(
                                    "toa_lora {} {} {} {}",
                                    t.toa_us, t.t_symbol_us, t.nb_symbols_payload, t.nb_symbols_quarters
                                ));
                            }
                        }
                    }
                }
            }
        }
    }

    assert_matches_c(input, expected);
}

/// Out-of-range parameters must be rejected by the port (the C function
/// prints an error and returns `0`, with uninitialised out-parameters).
#[test]
fn invalid_time_on_air_inputs_are_rejected() {
    // sf out of range (C: IS_LORA_DR fails)
    assert!(toa::lora_packet_time_on_air(BW_125KHZ, 4, 1, 8, false, false, 12).is_none());
    assert!(toa::lora_packet_time_on_air(BW_125KHZ, 13, 1, 8, false, false, 12).is_none());
    // bandwidth out of range (C: IS_LORA_BW fails)
    assert!(toa::lora_packet_time_on_air(BW_UNDEFINED, 7, 1, 8, false, false, 12).is_none());
    assert!(toa::lora_packet_time_on_air(0x01, 7, 1, 8, false, false, 12).is_none());
    // coding rate out of range (C: IS_LORA_CR fails)
    assert!(toa::lora_packet_time_on_air(BW_125KHZ, 7, 0, 8, false, false, 12).is_none());
    assert!(toa::lora_packet_time_on_air(BW_125KHZ, 7, 5, 8, false, false, 12).is_none());
}

/// `lgw_time_on_air` (C: `loragw_hal.c`) — milliseconds, including the
/// `+ 0.5` LoRa rounding and the `+ 1` FSK margin.
#[test]
fn hal_time_on_air_ms_matches_c() {
    let mut input = Vec::new();
    let mut expected = Vec::new();
    let mut gw = dummy_driver();

    for &bw in &[BW_125KHZ, BW_250KHZ, BW_500KHZ] {
        for sf in 5u8..=12 {
            for cr in 1u8..=4 {
                for &size in &[1u16, 12, 33, 251] {
                    for &no_crc in &[false, true] {
                        let mut p = TxPacket::default();
                        p.modulation = MOD_LORA;
                        p.bandwidth = bw;
                        p.datarate = sf as u32;
                        p.coderate = cr;
                        p.preamble = 8;
                        p.size = size;
                        p.no_crc = no_crc;
                        p.no_header = false;

                        input.push(format!(
                            "toa_hal_lora {bw} {sf} {cr} 8 0 {} {size}",
                            no_crc as u8
                        ));
                        expected.push(format!("toa_hal_lora {}", gw.time_on_air(&p)));
                    }
                }
            }
        }
    }

    // FSK: the sync word size comes from IF chain 9 (C: lgw_rxif_setconf)
    for &sync_word_size in &[2u8, 4] {
        for &datarate in &[50_000u32, 64_000, 100_000, 500_000] {
            for &size in &[1u16, 12, 64] {
                for &no_crc in &[false, true] {
                    gw.ctx.fsk.sync_word_size = sync_word_size;

                    let mut p = TxPacket::default();
                    p.modulation = MOD_FSK;
                    p.datarate = datarate;
                    p.preamble = 8;
                    p.size = size;
                    p.no_crc = no_crc;

                    input.push(format!(
                        "toa_fsk {datarate} 8 {sync_word_size} {size} {}",
                        no_crc as u8
                    ));
                    expected.push(format!("toa_fsk {}", gw.time_on_air(&p)));
                }
            }
        }
    }

    assert_matches_c(input, expected);
}

/// `sx1302_lora_payload_crc` (C: `loragw_sx1302.c`).
#[test]
fn lora_payload_crc_matches_c() {
    let mut input = Vec::new();
    let mut expected = Vec::new();
    let mut seed = 0x1234_5678u32;

    for &size in &[0usize, 1, 2, 7, 8, 15, 16, 32, 33, 64, 128, 255] {
        let payload = pseudo_bytes(&mut seed, size);
        input.push(format!("crc {size} {}", hex(&payload)));
        expected.push(format!("crc {}", rx::sx1302_lora_payload_crc(&payload)));

        // all-zero and all-ones payloads
        for fill in [0x00u8, 0xFF] {
            let payload = vec![fill; size];
            input.push(format!("crc {size} {}", hex(&payload)));
            expected.push(format!("crc {}", rx::sx1302_lora_payload_crc(&payload)));
        }
    }

    // "LoRa" and a counter pattern
    for payload in [
        b"LoRa".to_vec(),
        (0u8..=31).collect::<Vec<u8>>(),
        (0u8..=254).collect::<Vec<u8>>(),
    ] {
        input.push(format!("crc {} {}", payload.len(), hex(&payload)));
        expected.push(format!("crc {}", rx::sx1302_lora_payload_crc(&payload)));
    }

    assert_matches_c(input, expected);
}

/// `sx1302_rssi_get_temperature_offset` (C: `loragw_sx1302.c`) — the whole
/// polynomial is evaluated in `f64` on both sides and truncated to `f32`.
#[test]
fn rssi_temperature_offset_matches_c() {
    // (a, b, c, d, e) of the EU868 SX1250 board plus a few synthetic sets
    let coeff_sets: [[f32; 5]; 4] = [
        [0.0, 0.0, 20.41, 2162.56, 0.0],
        [0.0, 0.0, 0.0, 0.0, 0.0],
        [0.5, -0.25, 1.0, 100.0, -3.75],
        [-0.0001, 0.002, -0.05, 0.25, 1234.5],
    ];

    let mut input = Vec::new();
    let mut expected = Vec::new();

    for set in &coeff_sets {
        let tc = RssiTComp {
            coeff_a: set[0],
            coeff_b: set[1],
            coeff_c: set[2],
            coeff_d: set[3],
            coeff_e: set[4],
        };
        let mut temps = vec![-40.0f32, -20.5, -0.0, 0.0, 5.5, 12.25, 25.0, 37.25, 50.0, 85.0];
        for t in -40i32..=85 {
            temps.push(t as f32);
        }
        for temperature in temps {
            input.push(format!(
                "rssioff {} {} {} {} {} {}",
                f32_bits(set[0]),
                f32_bits(set[1]),
                f32_bits(set[2]),
                f32_bits(set[3]),
                f32_bits(set[4]),
                f32_bits(temperature),
            ));
            expected.push(format!(
                "rssioff {}",
                f32_bits(rx::sx1302_rssi_get_temperature_offset(&tc, temperature))
            ));
        }
    }

    assert_matches_c(input, expected);
}

/// `calculate_freq_to_time_drift` (C: `loragw_sx1302.c`).
#[test]
fn freq_to_time_drift_matches_c() {
    let mut input = Vec::new();
    let mut expected = Vec::new();

    for &freq in &[
        137_000_000u32, 169_000_000, 433_000_000, 470_000_000, 787_000_000, 863_000_000,
        868_000_000, 869_000_000, 915_000_000, 920_000_000, 923_000_000, 2_400_000_000,
    ] {
        for &bw in &[BW_125KHZ, BW_250KHZ, BW_500KHZ] {
            input.push(format!("drift {freq} {bw}"));
            expected.push(match sx1302::calculate_freq_to_time_drift(freq, bw) {
                Ok((mant, exp)) => format!("drift 0 {mant} {exp}"),
                Err(_) => "drift -1 0 0".to_string(),
            });
        }
    }

    // Unsupported bandwidths: the C function returns -1 with the outputs
    // untouched (the harness initialises them to 0).
    for &bw in &[0u8, 1, 2, 3, 7, 0xFF] {
        for &freq in &[868_000_000u32, 915_000_000] {
            input.push(format!("drift {freq} {bw}"));
            expected.push(match sx1302::calculate_freq_to_time_drift(freq, bw) {
                Ok((mant, exp)) => format!("drift 0 {mant} {exp}"),
                Err(_) => "drift -1 0 0".to_string(),
            });
        }
    }

    assert_matches_c(input, expected);
}

/// `sx1302_get_ifmod_config` (C: `loragw_sx1302.c`).
///
/// Only the in-range chains are compared: the C version indexes
/// `ifmod_config[if_chain]` without a bounds check, so an out-of-range chain
/// is undefined behaviour there, while the port returns `IF_UNDEFINED`.
#[test]
fn ifmod_config_matches_c() {
    const LGW_IF_CHAIN_NB: usize = crate::consts::LGW_IF_CHAIN_NB;

    let mut input = Vec::new();
    let mut expected = Vec::new();
    for chain in 0..LGW_IF_CHAIN_NB as u8 {
        input.push(format!("ifmod {chain}"));
        expected.push(format!("ifmod {}", sx1302::sx1302_get_ifmod_config(chain)));
    }

    // Out of range: documented divergence — the port rejects, C reads OOB.
    for chain in [LGW_IF_CHAIN_NB as u8, 14, 255] {
        assert_eq!(
            sx1302::sx1302_get_ifmod_config(chain),
            IF_UNDEFINED,
            "out-of-range IF chain must yield IF_UNDEFINED"
        );
    }

    assert_matches_c(input, expected);
}

/// `timestamp_counter_correction` (C: `loragw_sx1302_timestamp.c`), legacy
/// (fine timestamping disabled) path — the only one this port implements.
#[test]
fn timestamp_counter_correction_matches_c() {
    let mut input = Vec::new();
    let mut expected = Vec::new();
    let gw = dummy_driver();

    for &bw in &[BW_125KHZ, BW_250KHZ, BW_500KHZ] {
        for sf in 5u32..=12 {
            for cr in 1u8..=4 {
                for &crc_en in &[false, true] {
                    for &size in &[1u8, 12, 64, 255] {
                        for &dft in &[RX_DFT_PEAK_MODE_DISABLED, RX_DFT_PEAK_MODE_AUTO] {
                            input.push(format!(
                                "tscorr 0 {bw} {sf} {cr} {} {size} {dft}",
                                crc_en as u8
                            ));
                            let c = gw.timestamp_counter_correction(bw, sf, cr, crc_en, size, dft);
                            expected.push(format!("tscorr {c}"));
                        }
                    }
                }
            }
        }
    }

    // Out-of-range parameters: both sides return 0.
    for &(bw, sf, cr) in &[
        (BW_125KHZ, 4u32, 1u8),
        (BW_125KHZ, 13, 1),
        (0x01, 7, 1),
        (BW_125KHZ, 7, 0),
        (BW_125KHZ, 7, 5),
    ] {
        input.push(format!("tscorr 0 {bw} {sf} {cr} 1 12 0"));
        let c = gw.timestamp_counter_correction(bw, sf, cr, true, 12, RX_DFT_PEAK_MODE_DISABLED);
        expected.push(format!("tscorr {c}"));
    }

    assert_matches_c(input, expected);
}

/// `merge_packets` / `is_same_pkt` / `remove_pkt` (C: `loragw_hal.c`): the
/// fine-timestamp de-duplication and the ascending-`count_us` sort.
#[test]
fn merge_packets_matches_c() {
    // (count_us, if_chain, rf_chain, datarate, size, status, payload)
    type Pkt = (u32, u8, u8, u32, usize, u8, Vec<u8>);

    let dup = |count_us: u32, status: u8, payload: &[u8]| -> Pkt {
        (count_us, 5, 0, 7, payload.len(), status, payload.to_vec())
    };
    let payload_a = vec![0xAA];
    let payload_b = vec![0xBB];
    let long_a = vec![0xAA; 8];

    let cases: Vec<Vec<Pkt>> = vec![
        // empty
        vec![],
        // single packet
        vec![dup(1000, STAT_CRC_OK, &payload_a)],
        // already sorted, all distinct
        vec![
            dup(1000, STAT_CRC_OK, &payload_a),
            dup(2000, STAT_CRC_OK, &payload_b),
            dup(3000, STAT_CRC_OK, &long_a),
        ],
        // unsorted -> sorted by ascending count_us
        vec![
            dup(5000, STAT_CRC_OK, &payload_b),
            dup(1000, STAT_CRC_OK, &payload_a),
            dup(3000, STAT_CRC_OK, &long_a),
        ],
        // exact duplicates (same status) -> keep the first
        vec![
            dup(1000, STAT_CRC_OK, &payload_a),
            dup(1000, STAT_CRC_OK, &payload_a),
        ],
        // duplicate pair: CRC_OK wins over CRC_BAD regardless of order
        vec![
            dup(1000, STAT_CRC_BAD, &payload_a),
            dup(1000, STAT_CRC_OK, &payload_a),
        ],
        vec![
            dup(1000, STAT_CRC_OK, &payload_a),
            dup(1000, STAT_CRC_BAD, &payload_a),
        ],
        // near duplicates: within 24 us -> merged, 25 us apart -> distinct
        vec![
            dup(1000, STAT_CRC_OK, &payload_a),
            dup(1024, STAT_CRC_OK, &payload_a),
        ],
        vec![
            dup(1000, STAT_CRC_OK, &payload_a),
            dup(1025, STAT_CRC_OK, &payload_a),
        ],
        // three copies of the same packet -> forces the search restart
        vec![
            dup(1000, STAT_CRC_OK, &payload_a),
            dup(1001, STAT_CRC_OK, &payload_a),
            dup(1002, STAT_CRC_OK, &payload_a),
        ],
        // same payload but a different IF chain / datarate / size / payload
        vec![
            dup(1000, STAT_CRC_OK, &payload_a),
            (1001, 6, 0, 7, 1, STAT_CRC_OK, payload_a.clone()),
            dup(1001, STAT_CRC_OK, &payload_b),
            dup(1001, STAT_CRC_OK, &long_a),
        ],
        // datarate differs
        vec![
            dup(1000, STAT_CRC_OK, &payload_a),
            (1000, 5, 0, 8, 1, STAT_CRC_OK, payload_a.clone()),
        ],
        // 32-bit timestamp wrap: both sides must treat them as far apart
        vec![
            dup(0xFFFF_FFF0, STAT_CRC_OK, &payload_a),
            dup(0x0000_0010, STAT_CRC_OK, &payload_a),
        ],
        // mixed: duplicates removed, then sorted
        vec![
            dup(9000, STAT_CRC_BAD, &payload_b),
            dup(1000, STAT_CRC_OK, &payload_a),
            dup(9001, STAT_CRC_OK, &payload_b),
            dup(5000, STAT_CRC_OK, &long_a),
            dup(5000, STAT_CRC_OK, &long_a),
        ],
    ];

    let mut input = Vec::new();
    let mut expected = Vec::new();

    for (i, case) in cases.iter().enumerate() {
        input.push(format!("merge {}", case.len()));
        for p in case {
            input.push(format!(
                "{} {} {} {} {} {} {}",
                p.0,
                p.1,
                p.2,
                p.3,
                p.4,
                p.5,
                hex(&p.6)
            ));
        }

        let mut pkts = [RxPacket::default(); 256];
        for (slot, p) in pkts.iter_mut().zip(case.iter()) {
            slot.count_us = p.0;
            slot.if_chain = p.1;
            slot.rf_chain = p.2;
            slot.datarate = p.3;
            slot.size = p.4 as u16;
            slot.status = p.5;
            slot.payload[..p.4].copy_from_slice(&p.6);
            slot.ftime_received = false;
        }
        let mut nb = case.len();
        merge_packets(&mut pkts, &mut nb);

        let mut line = format!("merge {nb}");
        for p in pkts.iter().take(nb) {
            line.push_str(&format!(" {}", p.count_us));
        }
        expected.push(line);
        let _ = i;
    }

    assert_matches_c(input, expected);
}
