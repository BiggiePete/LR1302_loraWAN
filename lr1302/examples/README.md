# `lr1302` examples

Runnable on the **host** (`cargo run --example <name>`, no hardware needed).
They drive the driver against a mock SPI bus — every `*_setconf`, the
argument validation and the error handling are real, but the SPI answers are
zeros, so `connect()` / `start()` / `send()` report the errors documented in
each file. `mock_bus.rs` contains the mock implementation
(`MockSpiDevice` + a `DelayNs` + a minimal `block_on` executor);
`common/mock.rs` is the condensed copy the other examples include.

| file | what it shows |
|---|---|
| [`mock_bus.rs`](mock_bus.rs) | the complete API flow — board/RF/IF/demod/TX-gain configuration, `connect()`, `start()`, a `receive()` loop with idle sleeps, `send()`, `status()`, counters, `stop()` — with every error handled and explained |
| [`config_eu868.rs`](config_eu868.rs) | the reference packet-forwarder file `LR1302_HAL/sx1302_hal/packet_forwarder/global_conf.json.sx1250.EU868` translated block by block into `board_setconf` / `rxrf_setconf` / `rxif_setconf` / `demod_setconf` / `txgain_setconf` / `ftime_setconf` calls (each block names its JSON key) |
| [`config_us915.rs`](config_us915.rs) | the same translation for `global_conf.json.sx1250.US915` — both radios, the 8 channels split 4/4 over them (903.9..905.3 MHz = LoRaWAN sub-band 2), the 500 kHz DR4 channel, the disabled `chan_FSK`, and what the US915 923..928 MHz downlink band means for `TxPacket::freq_hz` |
| [`time_on_air.rs`](time_on_air.rs) | `time_on_air()` for several LoRa/FSK parameter combinations, and class-A downlink scheduling relative to an uplink's `count_us` (`TIMESTAMPED` vs `IMMEDIATE`, RX1/RX2 offsets, overlap check) |
| [`embassy_firmware.rs.md`](embassy_firmware.rs.md) | **not compiled by this workspace** — a reference listing for wiring the driver into an embassy firmware: CoreCell power/reset GPIO sequence, SPI bus at mode 0 / ≤ 2 MHz wrapped into a `SpiDevice`, `embassy_time::Delay`, one task owning the driver |

Run them from the repository root:

```sh
cargo run -p lr1302 --example mock_bus
cargo run -p lr1302 --example config_eu868
cargo run -p lr1302 --example config_us915
cargo run -p lr1302 --example time_on_air
```

(inside `lr1302/` the `-p lr1302` can be dropped).

Notes:

* The examples use `std` (`println!`, `thread::sleep`) and are built for the
  **host** only; they are not part of the `thumbv7em-none-eabi` library
  build and are never compiled for it.
* On real hardware the SPI bytes come from the chip: with a working bus,
  `connect()` / `start()` return `Ok(())` and `receive()` returns packets.
* Logging through `defmt` (the `defmt` cargo feature) is a firmware concern;
  the examples print with `println!` instead.
