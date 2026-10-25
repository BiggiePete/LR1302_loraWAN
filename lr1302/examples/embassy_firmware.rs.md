# Reference: wiring `lr1302` into an embassy firmware

> **This is a reference listing, not a buildable target.**
> `examples/*.rs.md` is not picked up by cargo (`cargo check --examples`
> ignores it), so nothing on this page is compiled, checked or tested by this
> workspace. It is meant to be copied into your own firmware crate and
> adapted. Do not treat it as verified code — check the API of the embassy
> crates against *your* versions, they change between releases.
>
> The parts that are **hardware-specific** are marked `[board]`: pin names,
> the SPI peripheral, and the reset / power-enable polarity. Everything from
> `Lr1302::new` downwards is portable.

## `Cargo.toml` of the firmware crate

```toml
[dependencies]
# executor + MCU HAL — versions drift, check with `cargo add`
embassy-executor = { version = "0.6", features = ["arch-cortex-m", "executor-thread"] }
embassy-stm32 = { version = "0.1", features = ["stm32l4r5", "time-driver-any", "exti"] }
embassy-time = "0.3"
embassy-sync = "0.6"
# embedded-hal bridges: SpiDevice wrapper + Delay
embassy-embedded-hal = "0.2"
embedded-hal = "1.0"
embedded-hal-async = "1.0"

# the driver (enable `defmt` to get the SPI/RX/TX diagnostics, see the README)
lr1302 = { path = "../../lr1302", features = ["defmt"] }

# logging backend — MUST be defmt 1.x-compatible: lr1302 pulls defmt 1.x, and
# a 0.3 backend never sees 1.x logs (defmt then panics with "no backend").
defmt = "1"
defmt-rtt = "0.4"
panic-probe = { version = "0.3", features = ["print-defmt"] }
```

## `src/main.rs`

```rust
//! SX1302 / LR1302 LoRa gateway firmware (EU868 sketch).
//!
//! Structure:
//!
//! 1. bring the CoreCell up (power enable + reset GPIOs)   [board]
//! 2. configure an SPI bus, mode 0, <= 2 MHz, and wrap it + CS into an
//!    `SpiDevice<u8>`                                     [board: pins]
//! 3. configure the concentrator (see examples/config_eu868.rs for the
//!    full `global_conf.json` translation) — portable
//! 4. one task owns the driver: receive loop + downlinks — portable

#![no_std]
#![no_main]

use defmt::{error, info, warn};
use embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::peripherals;
use embassy_stm32::spi::{Config as SpiConfig, Spi};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::{Delay, Timer};
use embedded_hal_async::delay::DelayNs;
use lr1302::consts::*;
use lr1302::types::*;
use lr1302::Lr1302;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut p = embassy_stm32::init(Default::default());

    // ------------------------------------------------------------------
    // 1. CoreCell power / reset                       [board-specific]
    //
    // The driver never touches GPIOs. Do what
    // `LR1302_HAL/sx1302_hal/tools/reset_lgw.sh start` does before
    // `start()`:
    //
    //   power enable -> 1, wait ~100 ms; NRST -> 1, wait ~100 ms;
    //   NRST -> 0, wait ~100 ms
    //
    // Pin numbers AND the reset polarity are board specific (the script
    // uses Linux GPIO17 = reset, GPIO18 = power enable on the CoreCell;
    // some carrier boards invert NRST — check your schematic).
    // ------------------------------------------------------------------
    let mut power_en = Output::new(p.PB_0, Level::Low, Speed::Low); // [board]
    let mut nrst = Output::new(p.PB_1, Level::Low, Speed::Low); // [board]

    nrst.set_high();
    power_en.set_high();
    Timer::after_millis(100).await;
    nrst.set_low(); // leave the chip out of reset
    Timer::after_millis(100).await;

    // ------------------------------------------------------------------
    // 2. SPI bus + chip select                        [board-specific]
    //
    // HARD REQUIREMENT (loragw_spi.h): SPI mode 0, MSB first, at most
    // 2 MHz. The driver inherits whatever the bus is configured with, so
    // this is your responsibility.
    // ------------------------------------------------------------------
    let mut spi_cfg = SpiConfig::default();
    spi_cfg.frequency = 2.MHz();
    spi_cfg.mode = embedded_hal::spi::MODE_0;

    // `Spi::new(sck, mosi, miso, peri, tx_dma, rx_dma, config)` — the
    // argument list differs between embassy-stm32 versions; see the
    // changelog for yours.
    let spi = Spi::new(
        p.SPI3,
        p.PB_3, // SCK    [board]
        p.PB_5, // MOSI   [board]
        p.PB_4, // MISO   [board]
        p.DMA1_CH1,
        p.DMA1_CH2,
        spi_cfg,
    );

    // The driver wants an `embedded_hal_async::spi::SpiDevice<u8>` — a
    // *bus plus one CS line*. If other chips share the bus, put the bus in
    // an `embassy_sync` Mutex and create one `SpiDevice` per CS.
    let spi_bus = Mutex::<CriticalSectionRawMutex, _>::new(spi);
    let cs = Output::new(p.PA_4, Level::High, Speed::High); // [board]
    let spi_dev = SpiDevice::new(&spi_bus, cs);

    // `embassy_time::Delay` implements `embedded_hal_async::delay::DelayNs`.
    let mut gw = Lr1302::new(spi_dev, Delay);

    // ------------------------------------------------------------------
    // 3. configuration — portable, same on every board
    // ------------------------------------------------------------------

    // Board: public LoRaWAN network, RF chain 0 is the clock source.
    gw.board_setconf(BoardConf {
        lorawan_public: true,
        clksrc: 0,
        full_duplex: false,
    })
    .unwrap();

    // Radio A at 867.5 MHz with TX enabled. `rssi_offset` / `rssi_tcomp`
    // and the TX gain LUT below are CALIBRATION DATA: take them from your
    // board's `global_conf.json`, the values here are the EU868 reference
    // ones (examples/config_eu868.rs shows the full translation).
    gw.rxrf_setconf(
        0,
        RfConf {
            enable: true,
            freq_hz: 867_500_000,
            rssi_offset: -215.4,
            rssi_tcomp: RssiTComp {
                coeff_a: 0.0,
                coeff_b: 0.0,
                coeff_c: 20.41,
                coeff_d: 2162.56,
                coeff_e: 0.0,
            },
            tx_enable: true,
            ..Default::default()
        },
    )
    .unwrap();

    // Radio B at 868.5 MHz, RX only.
    gw.rxrf_setconf(
        1,
        RfConf {
            enable: true,
            freq_hz: 868_500_000,
            rssi_offset: -215.4,
            rssi_tcomp: RssiTComp {
                coeff_a: 0.0,
                coeff_b: 0.0,
                coeff_c: 20.41,
                coeff_d: 2162.56,
                coeff_e: 0.0,
            },
            tx_enable: false,
            ..Default::default()
        },
    )
    .unwrap();

    // The 8 multi-SF channels (IF chains 0..8).
    let multi_sf_if: [(u8, i32); 8] = [
        (1, -400_000), // chan_multiSF_0..2: radio 1 (868.5 MHz)
        (1, -200_000),
        (1, 0),
        (0, -400_000), // chan_multiSF_3..7: radio 0 (867.5 MHz)
        (0, -200_000),
        (0, 0),
        (0, 200_000),
        (0, 400_000),
    ];
    for (i, (rf_chain, freq_hz)) in multi_sf_if.into_iter().enumerate() {
        let mut conf = IfConf {
            enable: true,
            rf_chain,
            freq_hz,
            ..Default::default()
        };
        gw.rxif_setconf(i as u8, &mut conf).unwrap();
    }

    // Single-SF LoRa channel (IF chain 8) and FSK channel (IF chain 9).
    let mut std = IfConf {
        enable: true,
        rf_chain: 1,
        freq_hz: -200_000,
        bandwidth: BW_250KHZ,
        datarate: DR_LORA_SF7,
        ..Default::default()
    };
    gw.rxif_setconf(8, &mut std).unwrap();
    let mut fsk = IfConf {
        enable: true,
        rf_chain: 1,
        freq_hz: 300_000,
        bandwidth: BW_125KHZ,
        datarate: 50_000,
        ..Default::default()
    };
    gw.rxif_setconf(9, &mut fsk).unwrap();

    // Decode all spreading factors SF5..SF12 on the multi-SF channels.
    gw.demod_setconf(DemodConf {
        multisf_datarate: LGW_MULTI_SF_EN,
    })
    .unwrap();

    // TX gain LUT, rf_chain 0 (14 dBm reference entry; replace with your
    // board's table). `rf_power` of a TxPacket selects the entry.
    let mut lut = TxGainLut {
        lut: [TxGain::default(); TX_GAIN_LUT_SIZE_MAX],
        size: 1,
    };
    lut.lut[0] = TxGain {
        rf_power: 14,
        pa_gain: 0,
        pwr_idx: 17,
        mix_gain: 5,
        ..Default::default()
    };
    gw.txgain_setconf(0, lut).unwrap();

    // Optional: ambient temperature for the RSSI temperature
    // compensation (replaces the C HAL's I2C STTS751 read, which is not
    // ported). Feed it from your board's own sensor, e.g. every minute.
    gw.set_temperature(Some(22.0));

    // ------------------------------------------------------------------
    // 4. start + run loop — one task owns the driver
    //
    // All driver methods take `&mut self`: give the `Lr1302` to ONE task.
    // If RX and TX must be in different tasks, wrap it in an
    // `embassy_sync::mutex::Mutex` and lock per operation (but keeping
    // `receive()` and `send()` in one task avoids TX latency behind a
    // long RX transaction).
    // ------------------------------------------------------------------
    if let Err(e) = gw.start().await {
        error!("start failed: {:?}", e);
        return;
    }
    info!("concentrator started");

    let mut rx = [RxPacket::default(); 8];
    loop {
        // `receive()` also refreshes the timestamp counter (wraps every
        // ~134 s), so it must be called regularly even when idle — the
        // reference packet forwarder polls every 10..30 ms.
        match gw.receive(&mut rx).await {
            Ok(0) => {
                Timer::after_millis(10).await;
            }
            Ok(n) => {
                for pkt in &rx[..n] {
                    // forward to your network server / LoRaWAN stack
                    info!(
                        "rx if={} rf={} tmst={} dr={} rssi={:.1} snr={:.1} size={}",
                        pkt.if_chain, pkt.rf_chain, pkt.count_us, pkt.datarate,
                        pkt.rssis, pkt.snr, pkt.size
                    );

                    // Example: class A downlink, EU868 RX1 = uplink + 1 s on
                    // the same channel (see examples/time_on_air.rs).
                    if pkt.size > 0 && pkt.payload[0] == 0x01 {
                        let mut dl = TxPacket::new(pkt.freq_hz, 0, [0u8; 256], 4);
                        dl.tx_mode = TIMESTAMPED;
                        // 32-bit us counter: wrapping arithmetic!
                        dl.count_us = pkt.count_us.wrapping_add(1_000_000);
                        dl.rf_power = 14;
                        if let Err(e) = gw.send(&dl).await {
                            warn!("downlink failed: {:?}", e);
                        }
                    }
                }
            }
            Err(e) => {
                error!("receive failed: {:?}", e);
                Timer::after_millis(100).await;
            }
        }
    }
}
```

## Checklist before it works on a board

| item | where | note |
|---|---|---|
| SPI mode 0, MSB first, ≤ 2 MHz | bus config | hard requirement, `loragw_spi.h` |
| CS pin | `SpiDevice::new(..., cs)` | one CS per device on a shared bus |
| Power enable + reset sequence | GPIOs before `start()` | from `tools/reset_lgw.sh`; polarity is board-specific |
| `clksrc` = the RF chain that is clocked and enabled | `board_setconf` | `start()` refuses otherwise |
| `rssi_offset`, `rssi_tcomp`, `tx_gain_lut` | `rxrf_setconf` / `txgain_setconf` | board calibration data, from your `global_conf.json` |
| `set_temperature(...)` | optional | feeds the RSSI temperature compensation |
| `defmt` 1.x everywhere | `Cargo.toml` | `lr1302/defmt` + a defmt **1.x** backend; a 0.3 backend never receives the logs |
