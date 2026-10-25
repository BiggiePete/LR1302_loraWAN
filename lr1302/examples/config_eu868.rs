//! # EU868 regional configuration
//!
//! Translates the reference packet-forwarder configuration
//! `LR1302_HAL/sx1302_hal/packet_forwarder/global_conf.json.sx1250.EU868`
//! into the equivalent `lr1302` `*_setconf` calls. Every block names the
//! JSON key it comes from, so you can diff it against your own
//! `global_conf.json`:
//!
//! ```sh
//! cargo run -p lr1302 --example config_eu868
//! ```
//!
//! The example runs against the mock bus from `mock_bus.rs`, so no hardware
//! is needed: all `setconf` calls below execute for real (they only write the
//! driver's in-RAM context and validate the values), while `start()` — the
//! first call that touches the chip — is expected to fail here.
//!
//! ## What maps where
//!
//! | `global_conf.json` | `lr1302` |
//! |---|---|
//! | `SX130x_conf.lorawan_public` / `clksrc` / `full_duplex` | [`BoardConf`] |
//! | `radio_0` / `radio_1` | [`Lr1302::rxrf_setconf`] ([`RfConf`]) |
//! | `chan_multiSF_0..7`, `chan_Lora_std`, `chan_FSK` | [`Lr1302::rxif_setconf`] ([`IfConf`], IF chain = channel index) |
//! | `chan_multiSF_All.spreading_factor_enable` | [`Lr1302::demod_setconf`] ([`DemodConf`]) |
//! | `radio_0.tx_gain_lut` | [`Lr1302::txgain_setconf`] ([`TxGainLut`]) |
//! | `SX130x_conf.fine_timestamp` | [`Lr1302::ftime_setconf`] ([`FtimeConf`]) |
//!
//! Not represented in the HAL (they belong to other layers): `com_type` /
//! `com_path` (the SPI device is passed to `Lr1302::new` instead),
//! `antenna_gain`, `tx_freq_min`/`tx_freq_max` (checked by the packet
//! forwarder, not by the HAL), the whole `sx1261_conf` block (SX1261 LBT /
//! spectral scan are not ported), `gateway_conf` (your network-server
//! settings) and `debug_conf`.
//!
//! This region is *portable*; what is board-specific are the RSSI offsets
//! and the TX gain LUT values, which come from your board's calibration.

use lr1302::consts::*;
use lr1302::sx1302_defs::{IF_FSK_STD, IF_LORA_MULTI, IF_LORA_STD};
use lr1302::types::*;
use lr1302::Lr1302;

#[path = "common/mock.rs"]
mod mock;

use mock::{block_on, MockDelay, MockSpiDevice};

/// Apply `global_conf.json.sx1250.EU868` to the driver.
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
    //     "enable": true, "type": "SX1250", "freq": 867500000,
    //     "rssi_offset": -215.4,
    //     "rssi_tcomp": {"coeff_a": 0, "coeff_b": 0, "coeff_c": 20.41,
    //                    "coeff_d": 2162.56, "coeff_e": 0},
    //     "tx_enable": true, "tx_freq_min": 863000000, "tx_freq_max": 870000000,
    //     "tx_gain_lut": [ ... ]            <- see txgain_setconf() below
    // }
    // "type" defaults to `RadioType::Sx1250`, the only radio this port
    // supports; "single_input_mode" is absent from the JSON -> default false.
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
    )?;

    // "radio_1": {
    //     "enable": true, "type": "SX1250", "freq": 868500000,
    //     "rssi_offset": -215.4, "rssi_tcomp": {...}, "tx_enable": false
    // }
    // No "tx_gain_lut" (TX is disabled): the HAL keeps its default single
    // entry, exactly like the C HAL.
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
    )?;

    // "chan_multiSF_0" .. "chan_multiSF_7": { "enable", "radio", "if" }
    //
    // The IF chain index *is* the channel number (0..7), "radio" selects the
    // RF chain and "if" is the offset from that radio's centre frequency.
    // "bandwidth" / "datarate" are not part of these keys: the multi-SF
    // chains are always 125 kHz, and which spreading factors are decoded is
    // decided by "chan_multiSF_All" (see demod_setconf below).
    const MULTI_SF: [(u8, i32); 8] = [
        (1, -400_000), // chan_multiSF_0: radio 1 (868.5 MHz) - 400 kHz
        (1, -200_000), // chan_multiSF_1
        (1, 0),        // chan_multiSF_2
        (0, -400_000), // chan_multiSF_3: radio 0 (867.5 MHz) - 400 kHz
        (0, -200_000), // chan_multiSF_4
        (0, 0),        // chan_multiSF_5
        (0, 200_000),  // chan_multiSF_6
        (0, 400_000),  // chan_multiSF_7
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
    // `LGW_MULTI_SF_EN` (the HAL default). Only SF7..SF12 would be
    // `(1 << 2) | ... | (1 << 7)` = 0xFC.
    gw.demod_setconf(DemodConf {
        multisf_datarate: LGW_MULTI_SF_EN,
    })?;

    // "chan_Lora_std": {
    //     "enable": true, "radio": 1, "if": -200000,
    //     "bandwidth": 250000, "spread_factor": 7,
    //     "implicit_hdr": false, "implicit_payload_length": 17,
    //     "implicit_crc_en": false, "implicit_coderate": 1
    // }
    // -> IF chain 8, the single-SF LoRa modem. Bandwidth in Hz becomes a
    //    BW_* code, "spread_factor" becomes a DR_LORA_SF* datarate, and
    //    "implicit_*" is carried as-is (it only matters when implicit_hdr
    //    is true).
    let mut lora_std = IfConf {
        enable: true,
        rf_chain: 1,
        freq_hz: -200_000,
        bandwidth: BW_250KHZ,
        datarate: DR_LORA_SF7,
        implicit_hdr: false,
        implicit_payload_length: 17,
        implicit_crc_en: false,
        implicit_coderate: 1,
        ..Default::default()
    };
    gw.rxif_setconf(8, &mut lora_std)?;

    // "chan_FSK": {
    //     "enable": true, "radio": 1, "if": 300000,
    //     "bandwidth": 125000, "datarate": 50000
    // }
    // -> IF chain 9, the FSK modem. There is no "sync_word" key: like the C
    //    `lgw_rxif_setconf`, the call only overwrites the sync word when one
    //    is given, so the HAL default 0xC194C1 (3 bytes, LoRaWAN private
    //    network) is kept.
    let mut fsk = IfConf {
        enable: true,
        rf_chain: 1,
        freq_hz: 300_000,
        bandwidth: BW_125KHZ,
        datarate: 50_000,
        ..Default::default()
    };
    gw.rxif_setconf(9, &mut fsk)?;

    // "radio_0.tx_gain_lut": 16 entries of { "rf_power", "pa_gain", "pwr_idx" }.
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
    // result so it can be compared against the JSON.
    let ctx = gw.context();
    println!("EU868 configuration (from global_conf.json.sx1250.EU868):");
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
        if c.enable {
            match lr1302::sx1302::sx1302_get_ifmod_config(i as u8) {
                IF_LORA_MULTI => println!("  chan_multiSF_{i}: rf_chain={} if={:+} Hz", c.rf_chain, c.freq_hz),
                IF_LORA_STD => println!(
                    "  chan_Lora_std: rf_chain={} if={:+} Hz bw={:#04x} sf={}",
                    c.rf_chain, c.freq_hz, ctx.lora_service.bandwidth, ctx.lora_service.datarate
                ),
                IF_FSK_STD => println!(
                    "  chan_FSK: rf_chain={} if={:+} Hz bw={:#04x} datarate={} sync_word={:#08x}",
                    c.rf_chain, c.freq_hz, ctx.fsk.bandwidth, ctx.fsk.datarate, ctx.fsk.sync_word
                ),
                other => println!("  IF chain {i}: unsupported type {other}"),
            }
        }
    }
    println!(
        "  demod: multisf_datarate={:#04x} (all SF5..SF12), fine_timestamp: enable={} mode={:?}",
        ctx.demod.multisf_datarate, ctx.ftime.enable, ctx.ftime.mode
    );
    println!("  tx_gain_lut[0]: {} entries ({}..{} dBm)", ctx.tx_gain_lut[0].size, ctx.tx_gain_lut[0].lut[0].rf_power, ctx.tx_gain_lut[0].lut[15].rf_power);

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
