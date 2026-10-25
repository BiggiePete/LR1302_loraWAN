//! # US915 regional configuration
//!
//! Translates the reference packet-forwarder configuration
//! `LR1302_HAL/sx1302_hal/packet_forwarder/global_conf.json.sx1250.US915`
//! into the equivalent `lr1302` `*_setconf` calls. Every block names the
//! JSON key it comes from, so you can diff it against your own
//! `global_conf.json`:
//!
//! ```sh
//! cargo run -p lr1302 --example config_us915
//! ```
//!
//! The example runs against the mock bus from `mock_bus.rs`, so no hardware
//! is needed: all `setconf` calls below execute for real (they only write the
//! driver's in-RAM context and validate the values), while `start()` — the
//! first call that touches the chip — is expected to fail here.
//!
//! See `config_eu868.rs` for the same mapping in the EU868 region; only the
//! numbers differ, not the structure.
//!
//! ## What this configuration actually is: US915 sub-band 2
//!
//! The eight 125 kHz uplink channels land on **903.9 .. 905.3 MHz**, which is
//! LoRaWAN sub-band 2 ("FSB2" / `chan_mask` 0x0200) — the band TTN in the US
//! uses:
//!
//! | channel | freq | radio + IF |
//! |---|---|---|
//! | `chan_multiSF_0` | 903.9 MHz | radio 0 (904.3) - 400 kHz |
//! | `chan_multiSF_1` | 904.1 MHz | radio 0 - 200 kHz |
//! | `chan_multiSF_2` | 904.3 MHz | radio 0 + 0 |
//! | `chan_multiSF_3` | 904.5 MHz | radio 0 + 200 kHz |
//! | `chan_multiSF_4` | 904.7 MHz | radio 1 (905.0) - 300 kHz |
//! | `chan_multiSF_5` | 904.9 MHz | radio 1 - 100 kHz |
//! | `chan_multiSF_6` | 905.1 MHz | radio 1 + 100 kHz |
//! | `chan_multiSF_7` | 905.3 MHz | radio 1 + 300 kHz |
//!
//! `chan_Lora_std` at radio 0 + 300 kHz = **904.6 MHz**, 500 kHz / SF8, is the
//! US915 uplink DR4 channel of this sub-band (uplink sub-bands place their
//! single 500 kHz channel at 903.0 + 1.6 MHz x (sub-band - 1)).
//!
//! `chan_FSK` is `"enable": false` in this region, so IF chain 9 is left
//! disabled — exactly as the C HAL would leave it.
//!
//! ## Things US915 does *differently* from EU868
//!
//! * **Two radios, both RX, only one TX.** `radio_1` has `"tx_enable": false`,
//!   so no TX gain LUT is configured for it (the C HAL keeps its default
//!   single entry there too).
//! * **Downlinks are on a different band than uplinks.** The JSON's
//!   `radio_0.tx_freq_min` / `tx_freq_max` are **923 .. 928 MHz**, and they are
//!   *not* HAL keys (the packet forwarder checks them). The HAL programs the
//!   transmit frequency per packet from `TxPacket::freq_hz`, so a US915
//!   downlink is just `tx.freq_hz = 923_300_000;` (whatever your network
//!   server picked) while `radio_0` stays centred on 904.3 MHz for RX. See
//!   `time_on_air.rs` for the Class-A scheduling part.
//! * **SF mask.** US915 uplinks only use SF9..SF12 at 125 kHz (DR0..DR3) and
//!   SF8 at 500 kHz (DR4), so `spreading_factor_enable` could be narrowed to
//!   `[9,10,11,12]` to save demodulator CPU. The reference JSON asks for all
//!   eight (`0xFF`) and that is what is applied below; the narrower mask is
//!   shown as a commented-out alternative.
//!
//! This region is *portable*; what is board-specific are the RSSI offsets and
//! the TX gain LUT values, which come from your board's calibration.

use lr1302::consts::*;
use lr1302::sx1302_defs::{IF_FSK_STD, IF_LORA_MULTI, IF_LORA_STD};
use lr1302::types::*;
use lr1302::Lr1302;

#[path = "common/mock.rs"]
mod mock;

use mock::{block_on, MockDelay, MockSpiDevice};

/// The two RF chains of this configuration (JSON `radio_0` / `radio_1`).
const RADIO_0_HZ: u32 = 904_300_000;
const RADIO_1_HZ: u32 = 905_000_000;

/// Apply `global_conf.json.sx1250.US915` to the driver.
fn configure(gw: &mut Lr1302<&mut MockSpiDevice, MockDelay>) -> lr1302::Result<()> {
    // "SX130x_conf": { "com_type": "SPI", ... } — the SPI device itself was
    // given to `Lr1302::new`, there is no com_path equivalent here.
    //
    // "SX130x_conf.lorawan_public": true,
    // "SX130x_conf.clksrc": 0,
    // "SX130x_conf.full_duplex": false,
    gw.board_setconf(BoardConf {
        lorawan_public: true,
        clksrc: 0,
        full_duplex: false,
    })?;

    // "radio_0": {
    //     "enable": true, "type": "SX1250", "freq": 904300000,
    //     "rssi_offset": -215.4,
    //     "rssi_tcomp": {"coeff_a": 0, "coeff_b": 0, "coeff_c": 20.41,
    //                    "coeff_d": 2162.56, "coeff_e": 0},
    //     "tx_enable": true,
    //     "tx_freq_min": 923000000, "tx_freq_max": 928000000, <- not HAL keys
    //     "tx_gain_lut": [ ... ]            <- see txgain_setconf() below
    // }
    // "type" defaults to `RadioType::Sx1250`, the only radio this port
    // supports; "single_input_mode" is absent from the JSON -> default false.
    gw.rxrf_setconf(
        0,
        RfConf {
            enable: true,
            freq_hz: RADIO_0_HZ,
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
    )?;

    // "radio_1": {
    //     "enable": true, "type": "SX1250", "freq": 905000000,
    //     "rssi_offset": -215.4, "rssi_tcomp": {...}, "tx_enable": false
    // }
    // TX is disabled on this chain, so there is no "tx_gain_lut" for it and
    // none is configured (the C HAL keeps its default single entry too).
    gw.rxrf_setconf(
        1,
        RfConf {
            enable: true,
            freq_hz: RADIO_1_HZ,
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
    )?;

    // "chan_multiSF_0" .. "chan_multiSF_7": { "enable", "radio", "if" }
    //
    // The IF chain index *is* the channel number (0..7), "radio" selects the
    // RF chain and "if" is the offset from that radio's centre frequency.
    // Unlike EU868 this region splits the channels 4/4 over the two radios.
    // "bandwidth" / "datarate" are not part of these keys: the multi-SF
    // chains are always 125 kHz, and which spreading factors are decoded is
    // decided by "chan_multiSF_All" (see demod_setconf below).
    const MULTI_SF: [(u8, i32); 8] = [
        (0, -400_000), // chan_multiSF_0: radio 0 (904.3 MHz) - 400 kHz = 903.9
        (0, -200_000), // chan_multiSF_1: 904.1 MHz
        (0, 0),        // chan_multiSF_2: 904.3 MHz
        (0, 200_000),  // chan_multiSF_3: 904.5 MHz
        (1, -300_000), // chan_multiSF_4: radio 1 (905.0 MHz) - 300 kHz = 904.7
        (1, -100_000), // chan_multiSF_5: 904.9 MHz
        (1, 100_000),  // chan_multiSF_6: 905.1 MHz
        (1, 300_000),  // chan_multiSF_7: 905.3 MHz
    ];
    for (if_chain, (rf_chain, freq_hz)) in MULTI_SF.into_iter().enumerate() {
        let mut conf = IfConf {
            enable: true,
            rf_chain,
            freq_hz,
            ..Default::default()
        };
        gw.rxif_setconf(if_chain as u8, &mut conf)?;
    }

    // "chan_multiSF_All": { "spreading_factor_enable": [5,6,7,8,9,10,11,12] }
    //
    // Bit n of `multisf_datarate` enables SF(5+n), so all eight SFs = 0xFF =
    // `LGW_MULTI_SF_EN` (the HAL default).
    //
    // US915 uplinks only ever use SF9..SF12 at 125 kHz (DR0..DR3); SF5..SF8
    // are not part of the region plan. If you want to decode exactly that and
    // nothing else, use bits 4..7:
    //     multisf_datarate: 0xF0, // SF9..SF12
    gw.demod_setconf(DemodConf {
        multisf_datarate: LGW_MULTI_SF_EN,
    })?;

    // "chan_Lora_std": {
    //     "enable": true, "radio": 0, "if": 300000,
    //     "bandwidth": 500000, "spread_factor": 8,
    //     "implicit_hdr": false, "implicit_payload_length": 17,
    //     "implicit_crc_en": false, "implicit_coderate": 1
    // }
    // -> IF chain 8, the single-SF LoRa modem: 904.3 MHz + 300 kHz =
    //    904.6 MHz, the 500 kHz uplink channel (DR4) of US915 sub-band 2.
    //    Bandwidth in Hz becomes a BW_* code, "spread_factor" becomes a
    //    DR_LORA_SF* datarate, and "implicit_*" is carried as-is (it only
    //    matters when implicit_hdr is true).
    let mut lora_std = IfConf {
        enable: true,
        rf_chain: 0,
        freq_hz: 300_000,
        bandwidth: BW_500KHZ,
        datarate: DR_LORA_SF8,
        implicit_hdr: false,
        implicit_payload_length: 17,
        implicit_crc_en: false,
        implicit_coderate: 1,
        ..Default::default()
    };
    gw.rxif_setconf(8, &mut lora_std)?;

    // "chan_FSK": {
    //     "enable": false, "radio": 1, "if": 300000,
    //     "bandwidth": 125000, "datarate": 50000
    // }
    // Disabled in this region. Passing `enable: false` does what the C
    // `lgw_rxif_setconf` does: it clears the chain (enable = false,
    // freq_hz = 0) and ignores every other field, so the rest of the key is
    // deliberately left at its default below.
    let mut fsk = IfConf {
        enable: false,
        ..Default::default()
    };
    gw.rxif_setconf(9, &mut fsk)?;

    // "radio_0.tx_gain_lut": 16 entries of { "rf_power", "pa_gain", "pwr_idx" }
    // (identical to the EU868 one in this reference config).
    //
    // The reference packet forwarder (lora_pkt_fwd.c) memsets each entry,
    // forces `mix_gain = 5` for an sx1250 LUT and reads only those three
    // keys, so `dig_gain`, `dac_gain` and the I/Q offsets stay 0. For the
    // SX1302 + SX1250 combination only `pa_gain > 0` (external-PA enable)
    // and `pwr_idx` reach the hardware (see `tx.rs`); `rf_power` is what
    // `TxPacket::rf_power` selects the entry by.
    const TX_GAIN_LUT: [(i8, u8, u8); 16] = [
        // (rf_power, pa_gain, pwr_idx)
        (12, 0, 15),
        (13, 0, 16),
        (14, 0, 17),
        (15, 0, 19),
        (16, 0, 20),
        (17, 0, 22),
        (18, 1, 1),
        (19, 1, 2),
        (20, 1, 3),
        (21, 1, 4),
        (22, 1, 5),
        (23, 1, 6),
        (24, 1, 7),
        (25, 1, 9),
        (26, 1, 11),
        (27, 1, 14),
    ];
    let mut lut = TxGainLut {
        lut: [TxGain::default(); TX_GAIN_LUT_SIZE_MAX],
        size: TX_GAIN_LUT.len() as u8,
    };
    for (i, (rf_power, pa_gain, pwr_idx)) in TX_GAIN_LUT.into_iter().enumerate() {
        lut.lut[i] = TxGain {
            rf_power,
            pa_gain,
            pwr_idx,
            mix_gain: 5, // forced by the packet forwarder for sx1250 LUTs
            dig_gain: 0,
            dac_gain: 0,
            offset_i: 0,
            offset_q: 0,
        };
    }
    gw.txgain_setconf(0, lut)?;

    // "SX130x_conf.fine_timestamp": { "enable": false, "mode": "all_sf" }
    gw.ftime_setconf(FtimeConf {
        enable: false,
        mode: FtimeMode::AllSf,
    })?;

    Ok(())
}

fn main() -> lr1302::Result<()> {
    let mut spi = MockSpiDevice::default();
    let mut gw = Lr1302::new(&mut spi, MockDelay);

    configure(&mut gw)?;

    // Everything above only wrote the driver's in-RAM context. Print the
    // result so it can be compared against the JSON, resolving each IF chain
    // to its absolute frequency.
    let ctx = gw.context();
    println!("US915 configuration (from global_conf.json.sx1250.US915):");
    println!(
        "  board: lorawan_public={} clksrc={} full_duplex={}",
        ctx.board.lorawan_public, ctx.board.clksrc, ctx.board.full_duplex
    );
    for (rf, name) in ["radio_0", "radio_1"].iter().enumerate() {
        let c = &ctx.rf_chain[rf];
        if c.enable {
            println!(
                "  {name}: {} Hz, rssi_offset={:.1} dB, tx_enable={}",
                c.freq_hz, c.rssi_offset, c.tx_enable
            );
        }
    }
    for (i, c) in ctx.if_chain.iter().enumerate() {
        if !c.enable {
            continue;
        }
        let radio_hz = ctx.rf_chain[c.rf_chain as usize].freq_hz;
        let abs_hz = radio_hz as i64 + c.freq_hz as i64;
        match lr1302::sx1302::sx1302_get_ifmod_config(i as u8) {
            IF_LORA_MULTI => println!(
                "  chan_multiSF_{i}: {abs_hz} Hz (rf_chain={} if={:+} Hz)",
                c.rf_chain, c.freq_hz
            ),
            IF_LORA_STD => println!(
                "  chan_Lora_std: {abs_hz} Hz bw={:#04x} sf={} (rf_chain={} if={:+} Hz)",
                ctx.lora_service.bandwidth,
                ctx.lora_service.datarate,
                c.rf_chain,
                c.freq_hz
            ),
            IF_FSK_STD => println!(
                "  chan_FSK: {abs_hz} Hz bw={:#04x} datarate={} sync_word={:#08x}",
                ctx.fsk.bandwidth, ctx.fsk.datarate, ctx.fsk.sync_word
            ),
            other => println!("  IF chain {i}: unsupported type {other}"),
        }
    }
    // IF chain 9 is disabled in this region; say so instead of staying silent.
    if !ctx.if_chain[9].enable {
        println!("  chan_FSK: disabled (chan_FSK.enable = false)");
    }
    println!(
        "  demod: multisf_datarate={:#04x} (SF5..SF12), fine_timestamp: enable={} mode={:?}",
        ctx.demod.multisf_datarate, ctx.ftime.enable, ctx.ftime.mode
    );
    println!(
        "  tx_gain_lut[0]: {} entries ({}..{} dBm), tx_gain_lut[1]: default (tx disabled)",
        ctx.tx_gain_lut[0].size,
        ctx.tx_gain_lut[0].lut[0].rf_power,
        ctx.tx_gain_lut[0].lut[15].rf_power
    );
    println!(
        "  downlinks: 923..928 MHz (tx_freq_min/max, not HAL keys) -> TxPacket::freq_hz"
    );

    // The setconf calls above are local; `start()` is the first call that
    // pushes the configuration to the chip (radio bring-up, MCU firmware
    // upload, modem programming).
    //
    // Against the mock bus this fails at the very first register read — on a
    // real gateway, after the CoreCell power/reset sequence (see
    // `embassy_firmware.rs.md`), it returns `Ok(())`.
    match block_on(gw.start()) {
        Ok(()) => println!("\nstart(): concentrator started"),
        Err(e) => println!("\nstart(): {e:?} - expected without hardware (the mock has no chip)"),
    }

    println!(
        "mock bus: {} transactions, {} bytes written, {} bytes read",
        spi.transactions, spi.bytes_written, spi.bytes_read
    );

    Ok(())
}
