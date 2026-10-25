# Porting conventions — C HAL → Rust (`lr1302` crate)

This document is the single source of truth for porting chunks of the Semtech
SX1302 C HAL into the Rust crate. Follow it exactly so independently ported
modules fit together.

* C sources: `LR1302_HAL/sx1302_hal/libloragw/src/*.c`, `.../inc/*.h`
* Rust crate: `lr1302/` (`#![no_std]`, `#![deny(unsafe_code)]`)

## 1. Scope of work

* Only create/edit the file you were assigned. Every other file in
  `lr1302/src` is fixed infrastructure:
  * `lib.rs` — `pub struct Lr1302<SPI, D> { pub(crate) com: Com<SPI,D>, pub(crate) ctx: Context }`, module list.
  * `spi.rs` — `Com<SPI,D>` transport (see §3).
  * `reg.rs` — **generated**, register ids `crate::reg::SX1302_REG_<NAME>`.
  * `sx1302_defs.rs` — shared constants/macros (see §4).
  * `consts.rs`, `types.rs`, `context.rs`, `error.rs`, `util.rs`, `sx1250.rs`.
* Drop everything listed as out of scope in the crate docs (USB, GPS UART,
  I2C temperature/DAC, sx1255/1257, sx1261 LBT, spectral scan, fine
  timestamps, log files, `printf`/`DEBUG_*`).
* If a non-ported feature is reachable from your code, return
  `Err(Error::Unsupported)`.

## 2. File skeleton

```rust
//! <module doc naming the C file(s)>

use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::spi::SpiDevice;

use crate::error::{Error, Result};
use crate::{Lr1302};
// ... other crate imports

impl<SPI, D> Lr1302<SPI, D>
where
    SPI: SpiDevice<u8>,
    D: DelayNs,
{
    /// Doc comment naming the C function (C: `foo`).
    pub(crate) async fn foo(&mut self, ...) -> Result<()> { ... }
}
```

Free functions/structs that are not tied to driver state stay at module level
(e.g. `pub fn sx1302_get_ifmod_config(...)`, `pub fn sx1302_lora_payload_crc(...)`).

## 3. Naming and signatures

* Method name = exact C function name (`sx1302_radio_calibrate`,
  `sx1302_agc_load_firmware`, `rx_buffer_fetch`, ...). No `lgw_` prefix for
  the top-level API (that lives in `hal.rs`, ported separately).
* C functions receiving a **pointer to a context struct**
  (`lgw_context_t*`, `lgw_conf_rxrf_s*`, `lgw_conf_rxif_s*`,
  `lgw_conf_demod_s*`, `lgw_conf_ftime_s*`, `lgw_tx_gain_lut_s*`,
  `lgw_conf_board_s*`) get **no corresponding Rust parameter** — they read or
  write `self.ctx.<field>` instead. Scalar parameters are kept.
  Context fields (`context.rs`): `is_started`, `board: BoardConf`,
  `rf_chain: [RfConf; 2]`, `if_chain: [IfConf; 10]`, `demod: DemodConf`,
  `lora_service: IfConf`, `fsk: IfConf`, `tx_gain_lut: [TxGainLut; 2]`,
  `ftime: FtimeConf`, `rx_buffer: RxBuffer`, `counter_us: TimestampCounter`.
* C output pointers (`uint8_t *status`) become return values (tuples if there
  are several). C out-struct writes become filling a `&mut` argument or
  returning the struct.
* Visibility: `pub(crate)` unless the item is part of the public API.
* All structs/arrays are `Copy` or indexed by value where possible; remember
  `RfConf`, `IfConf`, `TxGainLut` are `Copy` — copying a config array into a
  local before an `.await` avoids borrow conflicts.

## 4. Register and SPI access

```rust
self.com.reg_w(crate::reg::SX1302_REG_..., value).await?;   // lgw_reg_w
let v: i32 = self.com.reg_r(crate::reg::SX1302_REG_...).await?;  // lgw_reg_r
self.com.reg_wb(id, &buf).await?;      // lgw_reg_wb (burst write)
self.com.reg_rb(id, &mut buf).await?;  // lgw_reg_rb (burst read)
self.com.mem_wb(addr, &buf).await?;    // lgw_mem_wb
self.com.mem_rb(addr, &mut buf, fifo).await?; // lgw_mem_rb
self.com.wait_ms(n).await;             // wait_ms
self.com.wait_us(n).await;             // wait_us
```

* Register id: C `SX1302_REG_FOO` → Rust `crate::reg::SX1302_REG_FOO`
  (identical name). Do **not** edit `reg.rs`.
* C `REG_SELECT(rf_chain, A, B)` →
  `crate::sx1302_defs::reg_select(rf_chain, A, B)` (ids are `u16`).
* C `TAKE_N_BITS_FROM(b, p, n)` → `crate::util::take_bits(b, p, n)`.
* C `lgw_bw_getval(bw)` → `crate::util::bw_getval(bw) -> Option<u32>`
  (`None` is the C `-1`). C `lgw_sf_getval(dr)` →
  `crate::util::sf_getval(dr) -> Option<u8>`.
* Radio (C: `sx1250_reg_w/r`, see `sx1250.rs`):

```rust
self.com.sx1250_w(rf_chain, crate::sx1250::Op::WriteRegister, &[hi, lo, val]).await?;
let mut buff = [0u8; 3];
self.com.sx1250_r(rf_chain, crate::sx1250::Op::GetStatus, &mut buff[..3]).await?;
```

* C read-modify-write of a bit field via `lgw_reg_w` on a sub-field is
  already handled by `reg_w` (it uses the register descriptor's `offs/leng`).

## 5. Error handling

* C `err = f(); if (err != LGW_REG_SUCCESS) return LGW_REG_ERROR;` → `f().await?;`
* `Error::InvalidConfig("field name")` — value rejected by validation
  (use the C message's subject, a `&'static str`).
* `Error::Timeout` — polling loop exceeded the C retry/time limit.
* `Error::Radio(v)` — SX1250 reported an error status.
* `Error::FirmwareVerify` — firmware read-back mismatch.
* `Error::NothingToDo` — no packet / nothing to do (C `LGW_REG_WARNING`
  "no packet" cases where the caller should just stop).
* `Error::Unsupported` — feature not ported.
* `Error::Spi` — transport failure (produced by `Com`).

`LGW_REG_WARNING` in a *packet parsing* context maps to `Ok(None)` (see §6).

## 6. Cross-module contract (exact signatures — do not change)

Callers and callees in different files rely on these:

**`timestamp.rs` (module `timestamp`)**
```rust
pub(crate) fn timestamp_counter_new(&mut self);                       // reset state
pub(crate) async fn timestamp_counter_mode(&mut self, ftime_enable: bool) -> Result<()>;
pub(crate) async fn timestamp_counter_get(&mut self) -> Result<(u32, u32)>;   // (inst, pps)
pub(crate) fn timestamp_counter_expand(&self, pps: bool, cnt_us: u32) -> u32;
pub(crate) fn timestamp_pkt_expand(&self, pkt_cnt_us: u32) -> u32;
pub(crate) fn timestamp_counter_correction(&self, bandwidth: u8, datarate: u32,
    coderate: u8, crc_en: bool, payload_length: u8, dft_peak_mode: u8) -> i32;
```
(`dft_peak_mode` is the C `sx1302_rx_dft_peak_mode_t` value, e.g.
`RX_DFT_PEAK_MODE_AUTO = 0x03`; define it as a `pub const` in `timestamp.rs`.)

**`fw.rs`**
```rust
pub(crate) async fn sx1302_agc_load_firmware(&mut self, firmware: &[u8]) -> Result<()>;
pub(crate) async fn sx1302_agc_status(&mut self) -> Result<u8>;
pub(crate) async fn sx1302_agc_wait_status(&mut self, status: u8) -> Result<()>;
pub(crate) async fn sx1302_agc_mailbox_read(&mut self, mailbox: u8) -> Result<u8>;
pub(crate) async fn sx1302_agc_mailbox_write(&mut self, mailbox: u8, value: u8) -> Result<()>;
pub(crate) async fn sx1302_agc_start(&mut self, version: u8, radio_type: RadioType,
    ana_gain: u8, dec_gain: u8, full_duplex: bool, lbt_enable: bool) -> Result<()>;
pub(crate) async fn sx1302_arb_load_firmware(&mut self, firmware: &[u8]) -> Result<()>;
pub(crate) async fn sx1302_arb_status(&mut self) -> Result<u8>;
pub(crate) async fn sx1302_arb_wait_status(&mut self, status: u8) -> Result<()>;
pub(crate) async fn sx1302_arb_start(&mut self, version: u8) -> Result<()>;   // reads self.ctx.ftime
```

**`sx1302.rs`**
```rust
pub fn sx1302_get_ifmod_config(if_chain: u8) -> u8;                  // free fn, index-checks
pub(crate) async fn sx1302_set_gpio(&mut self, gpio_reg_val: u8) -> Result<()>;
pub(crate) async fn sx1302_gps_enable(&mut self, enable: bool) -> Result<()>;
pub(crate) async fn sx1302_timestamp_counter(&mut self, pps: bool) -> Result<u32>;
pub(crate) async fn sx1302_init(&mut self) -> Result<()>;
pub(crate) async fn sx1302_update(&mut self) -> Result<()>;
pub(crate) async fn sx1302_get_eui(&mut self) -> Result<u64>;
pub(crate) async fn sx1302_get_model_id(&mut self) -> Result<u8>;
pub(crate) async fn sx1302_radio_clock_select(&mut self, rf_chain: u8) -> Result<()>;
pub(crate) async fn sx1302_radio_reset(&mut self, rf_chain: u8, type_: RadioType) -> Result<()>;
pub(crate) async fn sx1302_radio_set_mode(&mut self, rf_chain: u8, type_: RadioType) -> Result<()>;
pub(crate) async fn sx1302_radio_host_ctrl(&mut self, host_ctrl: bool) -> Result<()>;
pub(crate) async fn sx1302_radio_calibrate(&mut self) -> Result<()>;
pub(crate) async fn sx1302_pa_lna_lut_configure(&mut self) -> Result<()>;
pub(crate) async fn sx1302_radio_fe_configure(&mut self) -> Result<()>;
pub(crate) async fn sx1302_channelizer_configure(&mut self, fix_gain: bool) -> Result<()>;
pub(crate) async fn sx1302_lora_correlator_configure(&mut self) -> Result<()>;
pub(crate) async fn sx1302_lora_service_correlator_configure(&mut self) -> Result<()>;
pub(crate) async fn sx1302_lora_modem_configure(&mut self, radio_freq_hz: u32) -> Result<()>;
pub(crate) async fn sx1302_lora_service_modem_configure(&mut self, radio_freq_hz: u32) -> Result<()>;
pub(crate) async fn sx1302_fsk_configure(&mut self) -> Result<()>;
pub(crate) async fn sx1302_modem_enable(&mut self) -> Result<()>;
pub(crate) async fn sx1302_lora_syncword(&mut self, public: bool, lora_service_sf: u8) -> Result<()>;
```
`sx1302_init` must call `self.rx_buffer_new()` and
`self.timestamp_counter_new()` where the C calls `rx_buffer_new()` /
`timestamp_counter_new()`, and `self.timestamp_counter_mode(self.ctx.ftime.enable).await?`
where the C calls `timestamp_counter_mode(...)`.

**`rx.rs`**
```rust
pub(crate) fn rx_buffer_new(&mut self);                                // no I/O
pub(crate) fn rx_buffer_del(&mut self);                                // no I/O
pub(crate) async fn rx_buffer_fetch(&mut self) -> Result<()>;          // SPI
pub(crate) fn rx_buffer_pop(&mut self) -> Result<Option<RawRxPacket>>; // Ok(None) == C LGW_REG_WARNING
pub(crate) async fn sx1302_fetch(&mut self) -> Result<u8>;             // number of packets
pub(crate) async fn sx1302_parse(&mut self) -> Result<Option<RxPacket>>;
    // Ok(Some) = C LGW_REG_SUCCESS (packet parsed)
    // Ok(None) = C LGW_REG_WARNING (discard/stop)
    // Err(_)   = C LGW_REG_ERROR (fatal)
pub fn sx1302_lora_payload_crc(data: &[u8]) -> u16;                    // free fn
pub fn sx1302_rssi_get_temperature_offset(context: &RssiTComp, temperature: f32) -> f32;
```

**`tx.rs`**
```rust
pub(crate) async fn sx1302_tx_configure(&mut self, radio_type: RadioType) -> Result<()>;
pub(crate) async fn sx1302_send(&mut self, pkt: &TxPacket) -> Result<()>;
    // radio type: self.ctx.rf_chain[pkt.rf_chain].type_
    // LUT:       self.ctx.tx_gain_lut[pkt.rf_chain]
    // public:    self.ctx.board.lorawan_public
    // FSK conf:  self.ctx.fsk
pub(crate) async fn sx1302_tx_status(&mut self, rf_chain: u8) -> Result<u8>;
pub(crate) async fn sx1302_rx_status(&mut self, rf_chain: u8) -> Result<u8>;
pub(crate) async fn sx1302_tx_abort(&mut self, rf_chain: u8) -> Result<()>;
pub(crate) async fn sx1302_tx_set_start_delay(&mut self, rf_chain: u8, radio_type: RadioType,
    modulation: u8, bandwidth: u8, chirp_lowpass: u8) -> Result<u16>;
```

**`hal.rs`** provides the public API (`board_setconf`, `rxrf_setconf`,
`rxif_setconf`, `demod_setconf`, `txgain_setconf`, `ftime_setconf`, `start`,
`stop`, `receive`, `send`, `status`, `abort_tx`, `get_trigcnt`, `get_instcnt`,
`get_eui`, `get_temperature`, `set_temperature`, `time_on_air`) — ported by
the integrator, not by module authors.

## 7. Types recap

* Public structs: `crate::types::{BoardConf, RssiTComp, RfConf, IfConf,
  DemodConf, FtimeConf, FtimeMode, TxGain, TxGainLut, RxPacket, TxPacket}`.
* `RadioType` lives in `crate::consts::RadioType` (`RadioType::Sx1250`, ...).
* Internal RX packet struct parsed from the buffer: `RawRxPacket` in `rx.rs`.
* `RxBuffer` / `TimestampCounter` state: `crate::context`.

## 8. Style

* `f32`/`f64` **arithmetic** (`+ - * /`, casts, comparisons) is available in
  `no_std`, but the std-only methods are **not**: `ceil`, `floor`, `round`,
  `sqrt`, `powi`/`powf`, `exp`, `log*`. Use integer arithmetic (see
  `util::ceil_div`) or an explicit manual equivalent instead.
* Keep C computation order and casts (`as u16`, `as i8`, ...) so results stay
  bit-identical. Prefer explicit typed literals (`0x1234u32`).
* Integer division truncates in both languages — no change needed.
* No `unsafe`, no `alloc`, no `unwrap()` on fallible SPI results.
* Doc-comment every `pub`/`pub(crate)` item: `/// ... (C: `foo`)`.
* When finished, run `cargo check --target thumbv7em-none-eabi` from the
  repository root and fix errors **in your file only**; errors reported
  against other files (written concurrently) are expected — list them in your
  summary instead of fixing them.

## 9. Logging

* `src/log.rs` defines `crate::log::{trace, debug}`; with the `defmt` cargo
  feature they forward to `defmt::trace!` / `defmt::debug!`, without it they
  expand to nothing. Import with `use crate::log::debug;` (only what you use,
  or you get an unused-import warning with the feature off).
* Format specs must be defmt's, not Rust's: `{=u8}`, `{=u32}`, `{=i32}`,
  `{=bool}` — `{}` only for types deriving `defmt::Format`. Never log `f32`/
  `f64` values directly: scale them (`(snr * 100.0) as i32`, centi-dB).
* With the feature off the arguments are **not evaluated**, so a log must
  never be the only use of a variable/parameter (it would warn when the
  feature is off). Log values that are already used elsewhere, or log
  argument-free messages.
