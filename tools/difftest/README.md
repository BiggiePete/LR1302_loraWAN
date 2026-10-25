# C ↔ Rust conformance difftest

Proves that the Rust port in [`lr1302/`](../../lr1302) computes the same
results as the unmodified Semtech C HAL it was ported from.

```
lr1302/src/conformance.rs   Rust side: computes the expected answer with the
                            ported code and diffs it against the C output
        │  stdin: one command per line
        ▼
tools/difftest/harness.c    C side: runs the same command through the vendor
tools/difftest/stubs.c      sources and prints one result line per command
        │  stdout
        ▼
      diff, line by line
```

## What is compared

| command | C function | Rust counterpart |
|---|---|---|
| `toa_lora` | `lora_packet_time_on_air` (`loragw_aux.c`) | `toa::lora_packet_time_on_air` |
| `toa_hal_lora` | `lgw_time_on_air` (`loragw_hal.c`, ms + rounding) | `Lr1302::time_on_air` |
| `toa_fsk` | `lgw_time_on_air`, FSK branch (+1 margin) | `Lr1302::time_on_air` |
| `crc` | `sx1302_lora_payload_crc` | `rx::sx1302_lora_payload_crc` |
| `rssioff` | `sx1302_rssi_get_temperature_offset` | `rx::sx1302_rssi_get_temperature_offset` |
| `drift` | `calculate_freq_to_time_drift` | `sx1302::calculate_freq_to_time_drift` |
| `ifmod` | `sx1302_get_ifmod_config` | `sx1302::sx1302_get_ifmod_config` |
| `tscorr` | `timestamp_counter_correction` (legacy path) | `timestamp::timestamp_counter_correction` |
| `merge` | `merge_packets` / `is_same_pkt` / `remove_pkt` | `hal::merge_packets` … |

Every sweep is exhaustive over the legal parameter space (all
BW × SF × CR × preamble × header × CRC × size combinations for time on air,
every IF chain, several thousand `tscorr` combinations, …), plus deliberately
chosen edge cases (0- and 255-byte payloads, all-zero / all-ones payloads,
the 32-bit timestamp rollover, duplicates 24 µs vs 25 µs apart).

## Protocol

One command per line on stdin, one response line on stdout. `f32` values
travelling as 8-digit lowercase hex **IEEE-754 bit patterns** so no precision
is lost between the languages.

```
toa_lora  <bw> <sf> <cr> <preamble> <no_header> <no_crc> <size>
          -> toa_lora <toa_us> <t_symbol_us> <nb_payload> <nb_symbols_quarters>
toa_hal_lora <bw> <sf> <cr> <preamble> <no_header> <no_crc> <size>
          -> toa_hal_lora <ms>
toa_fsk   <datarate> <preamble> <sync_word_size> <size> <no_crc>
          -> toa_fsk <ms>
crc       <size> [<hex-payload>]      -> crc <u16>
rssioff   <a> <b> <c> <d> <e> <temperature>   (f32 bit patterns)
          -> rssioff <f32 bit pattern of the offset>
drift     <freq_hz> <bw>              -> drift <0|-1> <mantissa> <exponent>
ifmod     <if_chain>                   -> ifmod <value>
tscorr    <ftime_enable> <bw> <dr> <cr> <crc_en> <payload_len> <dft_peak_mode>
          -> tscorr <correction>
merge     <n>
          <n lines: count_us if_chain rf_chain datarate size status hex-payload>
          -> merge <k> <count_us of the k surviving packets, ascending>
quit                                     -> (exit)
```

The C HAL prints its own diagnostics (`ERROR: ...`) on the same stdout
stream; the Rust side only compares lines whose first token matches the
command it is waiting for and treats everything else as noise.

## How the C side is built

`build.ps1` compiles the **unmodified** vendor sources:

* `loragw_hal.c` is `#include`d by `harness.c` rather than compiled on its own,
  which makes its file-static helpers (`merge_packets`, `is_same_pkt`,
  `remove_pkt`) and the global `lgw_context` reachable;
* `loragw_sx1302.c`, `loragw_sx1302_timestamp.c`, `loragw_sx1302_rx.c`,
  `loragw_aux.c` are compiled normally;
* `stubs.c` supplies the register / SPI / I2C / GPS / LBT / calibration layers,
  which none of the tested functions reaches — every stub `abort()`s if it is
  ever called, so a silently-wrong comparison is impossible. It also provides
  `qsort_r` (missing from MinGW's runtime) implemented as a merge sort using
  the comparator the C HAL passes in;
* `cinclude/config.h` stands in for the `inc/config.h` the vendor Makefile
  normally generates (this repository ships the HAL tree without it).

Requires a C compiler: MinGW `gcc` at `C:\MinGW\bin\gcc.exe` (used
automatically on Windows) or `gcc`/`clang` on `PATH`.

## Running

```sh
cargo test -p lr1302
```

The tests build `difftest_c.exe` on first use and compare against it. Set
`LR1302_DIFTEST_SKIP=1` to skip the comparison on a machine with no C
compiler (the tests then pass without checking anything).

## Known, deliberate differences

| case | C | Rust port |
|---|---|---|
| out-of-range IF chain to `sx1302_get_ifmod_config` | reads past `ifmod_config[]` (UB) | returns `IF_UNDEFINED` |
| out-of-range params to `lora_packet_time_on_air` | prints an error, returns `0` with uninitialised out-parameters | returns `None` (covered by `invalid_time_on_air_inputs_are_rejected`, no C comparison) |
| `timestamp_counter_correction` with fine timestamps enabled | precision-timestamp branch | not ported — always the legacy value, so only `ftime_enable = 0` is compared |
| `lgw_connect` | only logs the chip version | additionally rejects `0x00` / `0xFF` as [`Error::ChipVersion`] |

Sort order in `merge_packets` matches the C comparator exactly: it copies
`count_us` into a signed `int`, so the sort is *signed* 32-bit — which is what
orders a timestamp just before the rollover before one just after it.
