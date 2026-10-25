# lr1302

`no_std` Rust driver for the **Semtech SX1302 / LR1302 LoRa gateway
concentrator**, built on the `embedded-hal` / `embedded-hal-async` traits so
it works with any embassy-compatible SPI driver.

It is a faithful port of the Semtech SX1302 HAL v2.1.0 (`libloragw`)
shipped in this repository:

| C HAL | Rust |
|---|---|
| `loragw_spi.c` / `loragw_reg.c` | `spi.rs`, generated `reg.rs` (1044 registers) |
| `loragw_sx1250.c` | `sx1250.rs` |
| `loragw_sx1302.c` | `sx1302.rs`, `fw.rs`, `rx.rs`, `tx.rs` |
| `loragw_sx1302_timestamp.c` | `timestamp.rs` |
| `loragw_sx1302_rx.c` | `rx.rs` |
| `loragw_aux.c` (time on air) | `toa.rs` |
| `loragw_hal.c` | `hal.rs`, `types.rs`, `context.rs` |

## What is ported

* Register map and SPI protocol (single / burst / read-modify-write, mux
  targets for the SX1302 and both SX1250 radios)
* SX1250 radio bring-up and image calibration
* AGC + arbiter MCU firmware upload with verification, mailbox protocol
* RX channelizer, LoRa multi-SF / single-SF and FSK modem configuration
* RX buffer fetch, packet parsing, payload CRC16, RSSI offsets and
  temperature compensation
* TX path: gain LUT, start delay, immediate / timestamped / on-GPS triggers
* Timestamp counters (27-bit → 32-bit expansion, wrap tracking)
* The complete `lgw_*` configuration / start / receive / send / status API
* Packet time-on-air computation (integer, bit-identical to the C code)

## What is *not* ported

Returns [`Error::Unsupported`](src/error.rs):

* SX1255 / SX1257 radios and their calibration firmware
* SX1261 Listen-Before-Talk and spectral scan
* USB transport, GPS UART, I2C temperature sensor (STTS751), AD5338R DAC
* Fine ("precision") timestamps
* Log *files* (diagnostics go through `defmt` instead, see [Features](#features))

## Usage

```rust,no_run
use lr1302::types::{BoardConf, RfConf, IfConf, DemodConf, TxPacket, RxPacket};
use lr1302::consts::{BW_125KHZ, DR_LORA_SF7, MOD_LORA, CR_LORA_4_5, IMMEDIATE};

# async fn demo(spi: impl embedded_hal_async::spi::SpiDevice<u8>,
#                delay: impl embedded_hal_async::delay::DelayNs) -> lr1302::Result<()> {
let mut gw = lr1302::Lr1302::new(spi, delay);

// board
gw.board_setconf(BoardConf {
    lorawan_public: true,
    clksrc: 0,
    full_duplex: false,
})?;

// RF chain 0 at 868 MHz
gw.rxrf_setconf(0, RfConf {
    enable: true,
    freq_hz: 868_000_000,
    tx_enable: true,
    ..Default::default()
})?;

// the 8 LoRa multi-SF channels (IF chains 0..8) + the single-SF channel
for if_chain in 0..8u8 {
    let mut conf = IfConf {
        enable: true,
        rf_chain: 0,
        freq_hz: (if_chain as i32 - 4) * 200_000, // -400 kHz .. +400 kHz
        ..Default::default()
    };
    gw.rxif_setconf(if_chain, &mut conf)?;
}
// LoRa "std" modem on IF chain 8
let mut std = IfConf {
    enable: true,
    rf_chain: 0,
    freq_hz: 0,
    bandwidth: BW_125KHZ,
    datarate: DR_LORA_SF7,
    ..Default::default()
};
gw.rxif_setconf(8, &mut std)?;

gw.demod_setconf(DemodConf::default())?;

// optional: temperature used for the RSSI compensation
// gw.set_temperature(Some(22.5));

gw.start().await?;

// receive
let mut rx = [RxPacket::default(); 8];
let n = gw.receive(&mut rx).await?;
for pkt in &rx[..n] {
    let _ = (pkt.count_us, pkt.snr, pkt.size);
}

// transmit
let mut tx = TxPacket::default();
tx.freq_hz = 868_300_000;
tx.rf_chain = 0;
tx.rf_power = 14;
tx.modulation = MOD_LORA;
tx.bandwidth = BW_125KHZ;
tx.datarate = DR_LORA_SF7;
tx.coderate = CR_LORA_4_5;
tx.tx_mode = IMMEDIATE;
tx.payload[..5].copy_from_slice(b"hello");
tx.size = 5;
gw.send(&tx).await?;

gw.stop().await?;
# Ok(())
# }
```

### Bus requirements

SPI **mode 0**, MSB first, **≤ 2 MHz** — configuring the bus is the caller's
responsibility (see `loragw_spi.h` in the C HAL). The driver only needs an
[`embedded_hal_async::spi::SpiDevice`] and an
[`embedded_hal_async::delay::DelayNs`].

## Examples

Runnable on the host against a mock SPI bus — no hardware needed
(`cargo run -p lr1302 --example <name>`, see [`examples/`](examples)):

| example | what it shows |
|---|---|
| `mock_bus` | the complete API flow (configuration → `connect()`/`start()` → `receive()` loop → `send()` → `stop()`) driven against a `MockSpiDevice` implementing `SpiDevice<u8>`, with every error handled and the ones expected without hardware explained |
| `config_eu868` | the reference packet-forwarder file `LR1302_HAL/sx1302_hal/packet_forwarder/global_conf.json.sx1250.EU868` translated block by block into the equivalent `board_setconf` / `rxrf_setconf` / `rxif_setconf` / `demod_setconf` / `txgain_setconf` calls |
| `time_on_air` | `time_on_air()` for several LoRa / FSK parameter combinations, and class-A downlink scheduling relative to an uplink's `count_us` (`TIMESTAMPED` vs `IMMEDIATE`, RX1/RX2 offsets, overlap check) |

`examples/embassy_firmware.rs.md` is a **reference listing that is not
compiled by this workspace** (a non-cargo file): how to wire the driver into
an embassy firmware — SPI bus at mode 0 / ≤ 2 MHz wrapped into a
`SpiDevice`, `embassy_time::Delay` as the delay, the CoreCell power/reset
GPIO sequence from `tools/reset_lgw.sh`, and a single task owning the
driver.

## Features

* `defmt` — derive `defmt::Format` for the public types **and** turn on the
  driver's diagnostics (off by default).

### Logging

With `--features defmt` the crate logs through `defmt`:

| level | where | what |
|---|---|---|
| `trace` | `spi.rs` | every SPI frame (mux target, address, length) and radio opcode frame |
| `trace` | `hal.rs` | `*_setconf` calls, packets fetched, TX scheduled |
| `debug` | `hal.rs` | chip detection, `start()`/`stop()` outcome, each received packet (IF/RF chain, `tmst`, datarate, RSSI, SNR), TX summary, failures |

Without the feature every macro expands to nothing (no `defmt` dependency, no
cost), so you can build the crate with or without logging from the same
source. Because `trace` prints a line per SPI transfer, expect a lot of
output: filter with your `defmt`/`defmt-rtt` log level (`DEFMT_LOG=trace`
with `defmt-log`, or `max_level` features of `defmt`).

## Using it with Embassy

The driver only needs an `embedded_hal_async::spi::SpiDevice<u8>` and an
`embedded_hal_async::delay::DelayNs`, both of which embassy provides:

* **SPI**: `embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice`
  wraps an `embassy_*::spi::Spi` (implements `SpiBus<u8>`, DMA-driven) and a
  CS `Output` pin into a `SpiDevice<u8>`. Bus config: **mode 0, MSB first,
  ≤ 2 MHz** (`spi::Config { frequency: 2.MHz(), mode: Mode { polarity: IdleLow, phase: CaptureOnFirstTransition }, .. }`).
* **Delay**: `embassy_time::Delay` implements
  `embedded_hal_async::delay::DelayNs` (and the blocking one).
* **Reset / power**: the crate does not touch GPIOs. Do the CoreCell
  sequence from `LR1302_HAL/sx1302_hal/tools/reset_lgw.sh` first — drive
  *power enable* high, wait ~100 ms, pulse *NRST* high→low, wait ~100 ms —
  then call `connect()` / `start()`.

```rust,ignore
use embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice;
use embassy_sync::mutex::Mutex;
use embassy_time::{Delay, Timer};

let mut gw = lr1302::Lr1302::new(dev, Delay);   // dev: SpiDevice<u8>

gw.board_setconf(BoardConf { lorawan_public: true, clksrc: 0, full_duplex: false })?;
gw.rxrf_setconf(0, RfConf { enable: true, freq_hz: 868_300_000, tx_enable: true, ..Default::default() })?;
// ... one rxif_setconf per IF chain, see the example above ...
gw.start().await?;

let mut rx = [RxPacket::default(); 8];
loop {
    let n = gw.receive(&mut rx).await?;   // also runs sx1302_update()
    for pkt in &rx[..n] {
        // forward pkt to your network server / LoRaWAN stack
    }
    if n == 0 {
        Timer::after_millis(10).await;    // C packet forwarder: FETCH_SLEEP_MS
    }
}
```

Notes:

* All driver methods take `&mut self` — give `Lr1302` to **one** task. If RX
  and TX must live in different tasks, wrap it in an
  `embassy_sync::mutex::Mutex` and lock it per operation (keeping
  `receive()` and `send()` in one task avoids TX latency behind a long RX
  transaction).
* `receive()` must be called regularly: it refreshes the timestamp counter
  (which wraps every 2^27 µs ≈ 134 s) and drains the RX buffer. 10–30 ms is
  what the reference packet forwarder uses.
* Downlinks: schedule `TxPacket { tx_mode: TIMESTAMPED, count_us }` from the
  uplink's `count_us` (Class A RX1/RX2), or `IMMEDIATE`. Absolute
  (beacon/class-B) timing would need the GPS PPS input, whose UART path is
  not ported.
* Temperature for the RSSI compensation is not read from a sensor: feed
  `set_temperature(Some(22.5))` from your board's own sensor if you want it.
* `defmt` needs a `defmt` *backend* in your firmware (`defmt-rtt` +
  `panic-probe` / `defmt-bbq`), and `lr1302` enabled with `features =
  ["defmt"]`. Keep the **major version aligned**: this crate pulls `defmt`
  1.x, so the rest of your firmware must use `defmt`/`defmt-rtt` 1.x too —
  mixing a 0.3 backend with 1.x logs means the backend never sees them (and
  `defmt` panics when no backend is registered).

## Regenerating the register map and firmware images

```sh
python tools/gen_reg.py    # loragw_reg.c/.h  -> lr1302/src/reg.rs
python tools/gen_fw.py     # *.var            -> lr1302/fw/*.bin
```

## License

Revised BSD License (same as the Semtech HAL this crate ports).
