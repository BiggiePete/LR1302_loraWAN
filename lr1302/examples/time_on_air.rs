//! # Time on air and Class-A downlink scheduling
//!
//! ```sh
//! cargo run -p lr1302 --example time_on_air
//! ```
//!
//! Two things a gateway needs almost immediately:
//!
//! 1. **`time_on_air(&TxPacket)`** — how long a packet occupies the channel,
//!    in milliseconds. This is pure arithmetic (a bit-identical port of the C
//!    `lgw_time_on_air`): no SPI, no chip, no `start()` needed. It only needs
//!    a driver instance because the FSK variant reads the configured sync
//!    word length from the driver's context.
//! 2. **Class-A downlink scheduling** — a downlink is transmitted at an
//!    absolute concentrator timestamp derived from the uplink's
//!    `RxPacket::count_us`, using `TxPacket::tx_mode = TIMESTAMPED`
//!    (`IMMEDIATE` fires as soon as the TX modem is free).
//!
//! Everything runs against the mock bus from `mock_bus.rs`, so it works
//! without hardware; `send()` is shown too, and its "concentrator not
//! started" error is the expected answer here.

use lr1302::consts::*;
use lr1302::sx1302_defs::{STD_FSK_PREAMBLE, STD_LORA_PREAMBLE};
use lr1302::types::*;
use lr1302::Lr1302;

#[path = "common/mock.rs"]
mod mock;

use mock::{block_on, MockDelay, MockSpiDevice};

/// Build a LoRa packet for `time_on_air()`: only the fields that influence
/// the air time matter, the rest stays at the `TxPacket::default()` value.
///
/// `preamble` is set explicitly: `send()` substitutes the standard preamble
/// (8 symbols) when `TxPacket::preamble == 0`, but `time_on_air()` counts
/// exactly the symbols in `preamble` — set it and the two agree.
fn lora_packet(freq_hz: u32, bw: u8, datarate: u32, coderate: u8, size: u16) -> TxPacket {
    let mut p = TxPacket::new(freq_hz, 0, [0u8; 256], size);
    p.bandwidth = bw;
    p.datarate = datarate;
    p.coderate = coderate;
    p.preamble = STD_LORA_PREAMBLE;
    p
}

fn main() -> lr1302::Result<()> {
    let mut spi = MockSpiDevice::default();
    let mut gw = Lr1302::new(&mut spi, MockDelay);

    // -----------------------------------------------------------------------
    // 1. time on air
    // -----------------------------------------------------------------------

    let uplink_20b = lora_packet(868_100_000, BW_125KHZ, DR_LORA_SF7, CR_LORA_4_5, 20);
    let mut downlink_50b = lora_packet(868_100_000, BW_125KHZ, DR_LORA_SF9, CR_LORA_4_5, 50);
    downlink_50b.invert_pol = true; // downlinks are sent with inverted polarity

    // A variant with CRC omitted and an implicit header (e.g. a fixed-size
    // proprietary payload): both shorten the air time.
    let mut short_pkt = lora_packet(868_100_000, BW_125KHZ, DR_LORA_SF12, CR_LORA_4_5, 10);
    short_pkt.no_crc = true;
    short_pkt.no_header = true;

    // 500 kHz halves the symbol duration for the same SF.
    let wide_pkt = lora_packet(869_400_000, BW_500KHZ, DR_LORA_SF7, CR_LORA_4_5, 20);

    // FSK packet: air time = 8 * (preamble + sync_word + 1 + payload + CRC)
    // / baudrate, using the sync word size configured on IF chain 9
    // (3 bytes = 0xC194C1 by default) plus 1 ms of rounding margin.
    let mut fsk_pkt = lora_packet(867_100_000, BW_125KHZ, DR_LORA_SF7, CR_LORA_4_5, 20);
    fsk_pkt.modulation = MOD_FSK;
    fsk_pkt.datarate = 50_000; // 50 kbps
    fsk_pkt.f_dev = 25; // 25 kHz frequency deviation
    fsk_pkt.preamble = STD_FSK_PREAMBLE; // 5 symbols

    println!("time_on_air() [ms] (no SPI, works without hardware):");
    let cases: [(&str, &TxPacket); 6] = [
        ("uplink  20 B  SF7  BW125 CR4/5", &uplink_20b),
        ("downlink 50 B SF9  BW125 CR4/5", &downlink_50b),
        ("short   10 B  SF12 BW125 CR4/5, no CRC, implicit hdr", &short_pkt),
        ("wide    20 B  SF7  BW500 CR4/5", &wide_pkt),
        ("FSK     20 B  50 kbps", &fsk_pkt),
        ("RX2-like 50 B SF12 BW125 CR4/5", &lora_packet(869_525_000, BW_125KHZ, DR_LORA_SF12, CR_LORA_4_5, 50)),
    ];
    for (name, pkt) in cases {
        println!("  {:<52} {:>4} ms", name, gw.time_on_air(pkt));
    }
    println!(
        "  (a `TxPacket` with preamble 0 counts 0 preamble symbols here,\n   but `send()` would transmit {} on air - keep the field set)",
        STD_LORA_PREAMBLE
    );

    // -----------------------------------------------------------------------
    // 2. Class-A downlink scheduling
    // -----------------------------------------------------------------------

    // What `receive()` reported for the uplink (EU868, 868.1 MHz, SF7):
    // `count_us` is the concentrator's 1 us timestamp of the packet and is
    // the anchor for both receive windows.
    let uplink = RxPacket {
        freq_hz: 868_100_000,
        count_us: 1_234_567_890,
        if_chain: 0,
        rf_chain: 1,
        modulation: MOD_LORA,
        bandwidth: BW_125KHZ,
        datarate: DR_LORA_SF7,
        coderate: CR_LORA_4_5,
        size: 20,
        ..Default::default()
    };
    println!("\nclass A downlink, uplink count_us = {} us:", uplink.count_us);

    // RX1: EU868 receive delay 1 s -> same frequency / datarate as the
    // uplink, transmitted at count_us + 1_000_000 us.
    let mut rx1 = lora_packet(uplink.freq_hz, BW_125KHZ, uplink.datarate, CR_LORA_4_5, 50);
    rx1.tx_mode = TIMESTAMPED; // trigger: internal counter == count_us
    rx1.count_us = uplink.count_us.wrapping_add(1_000_000);

    // RX2: receive delay 2 s -> 869.525 MHz, SF12, BW125 kHz (EU868 RX2).
    let mut rx2 = lora_packet(869_525_000, BW_125KHZ, DR_LORA_SF12, CR_LORA_4_5, 50);
    rx2.tx_mode = TIMESTAMPED;
    rx2.count_us = uplink.count_us.wrapping_add(2_000_000);

    // IMMEDIATE: `count_us` is ignored, the packet goes on air as soon as
    // the TX modem is free. Use it for non-time-critical downlinks (class C
    // pushes, tests) - never for the class A windows.
    let mut immediate = lora_packet(868_300_000, BW_125KHZ, DR_LORA_SF7, CR_LORA_4_5, 10);
    immediate.tx_mode = IMMEDIATE;
    immediate.count_us = 42; // ignored in IMMEDIATE mode

    let toa_rx1 = gw.time_on_air(&rx1);
    let toa_rx2 = gw.time_on_air(&rx2);
    for (name, pkt, toa, at_ms) in [
        ("RX1", &rx1, toa_rx1, 1_000u64),
        ("RX2", &rx2, toa_rx2, 2_000u64),
        ("IMMEDIATE", &immediate, gw.time_on_air(&immediate), 0),
    ] {
        if pkt.tx_mode == TIMESTAMPED {
            println!(
                "  {name}: {} Hz, SF{}, {} ms on air, trigger at count_us + {} us (tmst + {} ms)",
                pkt.freq_hz,
                pkt.datarate,
                toa,
                at_ms * 1000,
                at_ms
            );
        } else {
            println!("  {name}: {} Hz, SF{}, {} ms on air, as soon as the TX modem is free", pkt.freq_hz, pkt.datarate, toa);
        }
    }

    // Sanity checks worth doing before sending:
    //
    // * `count_us` is a 32-bit us counter: use `wrapping_add` (it rolls over
    //   every ~71.6 min; the driver expands the chip's 27-bit counter for
    //   you, see `timestamp.rs`).
    // * Call `send()` early enough — the driver computes the remaining delay
    //   to the trigger from the counter *now*, so the further ahead the
    //   timestamp, the earlier the packet must be handed over.
    // * The two windows must not overlap: RX1 opens at tmst+1 s and RX2 at
    //   tmst+2 s.
    println!(
        "  RX1 occupies tmst+{}..{} ms, RX2 opens at tmst+2000 ms -> {}",
        1_000,
        1_000 + toa_rx1 as u64,
        if 1_000 + toa_rx1 as u64 > 2_000 {
            "OVERLAP, shorten RX1 or move it"
        } else {
            "no overlap"
        }
    );

    // On hardware (after `start()`) both packets are simply sent:
    //
    //     gw.send(&rx1).await?;
    //     gw.send(&rx2).await?;
    //
    // then the TX modem is polled until it is free again:
    //
    //     loop {
    //         if gw.status(0, TX_STATUS).await? == TX_FREE { break; }
    //         Timer::after_millis(10).await;
    //     }
    //
    // `abort_tx(0)` cancels a TIMESTAMPED packet before it triggers.
    match block_on(gw.send(&rx1)) {
        Ok(()) => println!("\nsend(): RX1 scheduled at tmst + 1000 ms"),
        // mock: the driver never reached `start()` -> `InvalidConfig`
        Err(e) => println!("\nsend(): {e:?} - expected without hardware (start() did not run)"),
    }

    println!(
        "mock bus: {} transactions, {} bytes written, {} bytes read",
        spi.transactions, spi.bytes_written, spi.bytes_read
    );

    Ok(())
}
