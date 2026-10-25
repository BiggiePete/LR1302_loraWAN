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
* Debug logging / log files

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

## Features

* `defmt` — derive `defmt::Format` for the public types (off by default).

## Regenerating the register map and firmware images

```sh
python tools/gen_reg.py    # loragw_reg.c/.h  -> lr1302/src/reg.rs
python tools/gen_fw.py     # *.var            -> lr1302/fw/*.bin
```

## License

Revised BSD License (same as the Semtech HAL this crate ports).
