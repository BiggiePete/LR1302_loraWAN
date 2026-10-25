//! # Mock-bus walkthrough of the whole `lr1302` API
//!
//! Runs on the host, no hardware needed:
//!
//! ```sh
//! cargo run -p lr1302 --example mock_bus
//! ```
//!
//! ## What is real and what is fake
//!
//! * **Real** — every `lr1302` call below, its signature, the argument
//!   validation, the error handling: exactly the code you write for a
//!   gateway. Configuration (`*_setconf`) is executed for real; those calls
//!   only write the driver's in-RAM context and are validated locally.
//! * **Fake** — [`MockSpiDevice`] pretends to be the concentrator. It accepts
//!   every SPI transaction and clocks back zeros, because *there is no chip
//!   behind it*. On real hardware those bytes are the SX1302's answers: the
//!   chip-version read in [`Lr1302::connect`] returns the chip's (non-zero)
//!   `COMMON_VERSION` byte instead of `0x00`, [`Lr1302::start`] uploads the
//!   MCU firmware and configures the modems, and [`Lr1302::receive`]
//!   returns actual packets.
//!
//! So against this mock `connect()` / `start()` report
//! [`lr1302::Error::ChipVersion`] and `send()` reports "concentrator not started".
//! That is the *expected* result here: the point of this example is the call
//! sequence and the signatures, not the answers. Every failure is printed,
//! not propagated, so you can watch the whole flow.
//!
//! On a real board the sequence is the one documented in the crate README:
//! reset/power the chip (see `embassy_firmware.rs.md`) -> configure ->
//! `start()` -> `receive()` / `send()` -> `stop()`.

use std::future::Future;
use std::task::{Context, Poll, Waker};
use std::time::Duration;

use embedded_hal::spi::{ErrorKind, ErrorType, Operation};
use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::spi::SpiDevice;

use lr1302::consts::*;
use lr1302::types::*;
use lr1302::Lr1302;

// ---------------------------------------------------------------------------
// mock SPI device
// ---------------------------------------------------------------------------

/// Error type of the mock SPI device.
///
/// The driver erases it into `lr1302::Error::Spi` anyway (see `error.rs`); a
/// real SPI driver would report framing / overrun errors here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MockSpiError;

impl embedded_hal::spi::Error for MockSpiError {
    fn kind(&self) -> ErrorKind {
        ErrorKind::Other
    }
}

/// A fake [`SpiDevice<u8>`] that never talks to a chip.
///
/// This is the `embedded-hal` surface the driver sees (the same trait an
/// embassy `SpiDevice` implements), so the driver cannot tell the difference:
///
/// * [`Operation::Write`] — recorded and dropped (the bytes would go to MOSI),
/// * [`Operation::Read`] / [`Operation::Transfer`] — the buffer is filled
///   with [`read_fill`](Self::read_fill) (`0x00` by default, i.e. an idle /
///   mis-wired bus), as if the chip had clocked those bytes in,
/// * [`Operation::TransferInPlace`] — left as-is: the buffer already carries
///   the outgoing bytes, and the "chip answer" for it is its current content,
/// * [`Operation::DelayNs`] — slept, like the driver's `wait_ms()` between
///   radio frames,
/// * the transaction always returns `Ok(())`.
///
/// Set `read_fill` to something else (e.g. `0x02`) to make `connect()` see a
/// plausible `COMMON_VERSION` and explore deeper into `start()` — the mock
/// will still make every later step fail, just later.
#[derive(Debug, Default)]
pub struct MockSpiDevice {
    /// Value the mock "clocks in" for every read byte (`0x00` by default).
    pub read_fill: u8,
    /// Number of transactions serviced (counter, printed at the end).
    pub transactions: u32,
    /// Total bytes the driver wrote (counter).
    pub bytes_written: usize,
    /// Total bytes the driver read (counter).
    pub bytes_read: usize,
}

impl ErrorType for MockSpiDevice {
    type Error = MockSpiError;
}

impl SpiDevice<u8> for MockSpiDevice {
    async fn transaction(
        &mut self,
        operations: &mut [Operation<'_, u8>],
    ) -> Result<(), Self::Error> {
        self.transactions += 1;
        for op in operations {
            match op {
                Operation::Write(buf) => self.bytes_written += buf.len(),
                Operation::Read(buf) => {
                    buf.fill(self.read_fill);
                    self.bytes_read += buf.len();
                }
                Operation::Transfer(r, w) => {
                    r.fill(self.read_fill);
                    self.bytes_read += r.len();
                    self.bytes_written += w.len();
                }
                Operation::TransferInPlace(buf) => self.bytes_written += buf.len(),
                Operation::DelayNs(ns) => {
                    std::thread::sleep(Duration::from_nanos(*ns as u64));
                }
            }
        }
        Ok(())
    }
}

/// Trivial [`DelayNs`]: sleeps the host thread.
///
/// In firmware you write `embassy_time::Delay` here — same trait, driven by
/// the executor's timer instead of the OS.
#[derive(Debug, Clone, Copy, Default)]
pub struct MockDelay;

impl DelayNs for MockDelay {
    async fn delay_ns(&mut self, ns: u32) {
        std::thread::sleep(Duration::from_nanos(ns as u64));
    }
}

/// Minimal single-future executor: poll until the future is ready.
///
/// The mock never returns [`Poll::Pending`] (every future completes inline),
/// so a plain poll loop is enough. In firmware, embassy's executor plays this
/// role — that is why the driver's methods are `async` and can be `.await`ed
/// inside a `#[embassy_executor::task]`.
pub fn block_on<F: Future>(fut: F) -> F::Output {
    let mut fut = core::pin::pin!(fut);
    let waker = Waker::noop();
    let mut cx = Context::from_waker(waker);
    loop {
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(v) => return v,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}

// ---------------------------------------------------------------------------
// the walkthrough
// ---------------------------------------------------------------------------

/// The full API flow: configure -> connect -> start -> receive -> send ->
/// stop. Hardware-dependent errors are printed instead of propagated so the
/// sequence keeps going; everything that *cannot* fail here uses `?`.
async fn walk_through(gw: &mut Lr1302<&mut MockSpiDevice, MockDelay>) -> lr1302::Result<()> {
    // ----- 1. configuration (no SPI: validation + in-RAM context only) -----

    // Board (C: `lgw_board_setconf`): public LoRaWAN network, RF chain 0
    // provides the clock, half duplex.
    gw.board_setconf(BoardConf {
        lorawan_public: true,
        clksrc: 0,
        full_duplex: false,
    })?;

    // RF chain 0 at 868.0 MHz with TX enabled (C: `lgw_rxrf_setconf`).
    // `type_` defaults to `RadioType::Sx1250` — the only radio this port
    // supports; `rssi_offset` / `rssi_tcomp` are board-specific (0 here).
    gw.rxrf_setconf(
        0,
        RfConf {
            enable: true,
            freq_hz: 868_000_000,
            tx_enable: true,
            ..Default::default()
        },
    )?;

    // The 8 LoRa multi-SF channels = IF chains 0..=7, 200 kHz spacing around
    // the RF chain centre: -700 .. +700 kHz, which fits the 1.6 MHz radio
    // bandwidth (checked by `rxif_setconf`). The reference EU868 config splits
    // the same 8 channels across *both* radios (-400 .. +400 kHz each, see
    // `config_eu868.rs`). `bandwidth`/`datarate` stay undefined: the call
    // fills the multi-SF defaults itself.
    for i in 0..8u8 {
        let mut conf = IfConf {
            enable: true,
            rf_chain: 0,
            freq_hz: (2 * i as i32 - 7) * 100_000,
            ..Default::default()
        };
        gw.rxif_setconf(i, &mut conf)?;
    }

    // LoRa "std" (single-SF) modem on IF chain 8: 125 kHz, SF7.
    let mut std = IfConf {
        enable: true,
        rf_chain: 0,
        freq_hz: 0,
        bandwidth: BW_125KHZ,
        datarate: DR_LORA_SF7,
        ..Default::default()
    };
    gw.rxif_setconf(8, &mut std)?;

    // FSK modem on IF chain 9. No sync word given: `rxif_setconf` keeps the
    // HAL default `0xC194C1` (3 bytes, LoRaWAN private network) in the FSK
    // context, exactly like the C `lgw_rxif_setconf`.
    let mut fsk = IfConf {
        enable: true,
        rf_chain: 0,
        freq_hz: 300_000,
        bandwidth: BW_125KHZ,
        datarate: 50_000,
        ..Default::default()
    };
    gw.rxif_setconf(9, &mut fsk)?;

    // Multi-SF correlator enable mask: bit n == SF(5+n); `LGW_MULTI_SF_EN`
    // (0xFF) = SF5..SF12, the default (C: `lgw_demod_setconf`).
    gw.demod_setconf(DemodConf {
        multisf_datarate: LGW_MULTI_SF_EN,
    })?;

    // Fine timestamping stays off — this port does not compute fine
    // timestamps, the flag is only carried in the config (C:
    // `lgw_ftime_setconf`).
    gw.ftime_setconf(FtimeConf::default())?;

    // TX gain LUT for RF chain 0: one entry, 14 dBm (the C default LUT).
    // A real board usually copies the `tx_gain_lut` array from its
    // `global_conf.json` — see `config_eu868.rs`.
    gw.txgain_setconf(
        0,
        TxGainLut {
            lut: [TxGain::default(); TX_GAIN_LUT_SIZE_MAX],
            size: 1,
        },
    )?;

    // Ambient temperature for the RSSI temperature compensation of
    // `receive()`. The C HAL reads an STTS751 over I2C; this port is told
    // instead (the I2C sensor is not ported). `None` disables the
    // compensation.
    gw.set_temperature(Some(22.5));

    // Show what was configured (all of it lives in the driver's context).
    let ctx = gw.context();
    println!(
        "configured: public={} clksrc={} rf0={}Hz tx={}",
        ctx.board.lorawan_public, ctx.board.clksrc, ctx.rf_chain[0].freq_hz, ctx.rf_chain[0].tx_enable
    );
    // Per-IF-chain slots: only rf_chain/freq_hz live there; the multi-SF
    // chains are always 125 kHz (SF mask comes from the demod config), and
    // the LoRa std / FSK parameters live in `lora_service` / `fsk` below.
    for (i, ifc) in ctx.if_chain.iter().enumerate() {
        if ifc.enable {
            println!("  IF chain {i}: rf_chain={} if={:+}Hz", ifc.rf_chain, ifc.freq_hz);
        }
    }
    println!(
        "  multi-SF: multisf_datarate={:#04x} (bit 0 = SF5 .. bit 7 = SF12), LoRa std: bw={:#04x} dr={}, FSK: dr={} sync_word={:#08x}",
        ctx.demod.multisf_datarate,
        ctx.lora_service.bandwidth,
        ctx.lora_service.datarate,
        ctx.fsk.datarate,
        ctx.fsk.sync_word
    );
    println!(
        "  txgain lut[0]: {} entry/entries, temperature compensation at {:?}C",
        ctx.tx_gain_lut[0].size, ctx.temperature
    );

    // ----- 2. connect / start (this is where the SPI traffic starts) -------

    // `connect()` only reads the COMMON_VERSION register (C: `lgw_connect`).
    println!("\nconnect(): reading COMMON_VERSION over SPI ...");
    match gw.connect().await {
        Ok(()) => println!("  SX1302 detected"),
        // Mock: the register reads back 0x00 -> `ChipVersion(0x00)`.
        // Real hardware: the chip answers with its (non-zero) version.
        Err(e) => println!("  {e:?} - expected against a mock bus (no chip drives MISO)"),
    }

    // `start()` = radio bring-up, image calibration, AGC/arbiter firmware
    // upload, channelizer + modem configuration (C: `lgw_start`).
    println!("start(): radio bring-up, MCU firmware upload, modem setup ...");
    match gw.start().await {
        Ok(()) => println!("  concentrator started"),
        Err(e) => {
            println!("  {e:?}");
            println!("  (expected here: `start()` re-reads the chip and gives up early.");
            println!("   On a real gateway - after the CoreCell power/reset sequence -");
            println!("   this returns `Ok(())` and `is_started()` becomes true)");
        }
    }
    println!("  is_started() = {}", gw.is_started());

    // ----- 3. receive loop -------------------------------------------------

    let mut rx = [RxPacket::default(); 8];
    for _ in 0..3 {
        // `receive()` fetches pending packets *and* refreshes the 27-bit
        // timestamp counter (which wraps every ~134 s), so it must be called
        // regularly even when the RX buffer is empty. On real hardware you
        // only get here after a successful `start()`.
        match gw.receive(&mut rx).await {
            Ok(0) => println!("\nreceive(): 0 packet(s)"),
            Ok(n) => {
                for p in &rx[..n] {
                    println!(
                        "\nreceive(): if_chain={} rf_chain={} freq={}Hz count_us={} dr={} snr={:.1}dB rssi={:.1}dB size={} status={:#04x}",
                        p.if_chain, p.rf_chain, p.freq_hz, p.count_us, p.datarate, p.snr, p.rssis, p.size, p.status
                    );
                }
            }
            Err(e) => println!("\nreceive(): {e:?}"),
        }
        // Idle wait between polls. The C packet forwarder uses
        // FETCH_SLEEP_MS (10 ms); in embassy this line is
        // `Timer::after_millis(10).await;`.
        MockDelay.delay_ms(10).await;
    }

    // ----- 4. transmit -----------------------------------------------------

    let mut tx = TxPacket::default();
    tx.freq_hz = 868_300_000; // downlink frequency (Hz)
    tx.tx_mode = IMMEDIATE; // IMMEDIATE / TIMESTAMPED / ON_GPS
    tx.rf_chain = 0; // must be an RF chain with `tx_enable`
    tx.rf_power = 14; // dBm: index into the TX gain LUT, not a raw register
    tx.modulation = MOD_LORA;
    tx.bandwidth = BW_125KHZ;
    tx.datarate = DR_LORA_SF7;
    tx.coderate = CR_LORA_4_5;
    tx.preamble = 8; // symbols; 0 = "use the standard preamble" on air,
    //                  but `time_on_air()` would then count 0 preamble symbols
    tx.payload[..5].copy_from_slice(b"hello");
    tx.size = 5;

    // Pure computation, no SPI: a bit-identical port of the C
    // `lgw_time_on_air` (see `time_on_air.rs`).
    println!("\ntime_on_air() = {} ms", gw.time_on_air(&tx));

    match gw.send(&tx).await {
        Ok(()) => println!("send(): packet scheduled"),
        // Mock: `start()` never succeeded, so the driver refuses with
        // `InvalidConfig("concentrator not started")`.
        Err(e) => println!("send(): {e:?} - expected, the mock never started"),
    }

    // TX status polling (`select` = TX_STATUS/RX_STATUS, `?` because both are
    // valid): while not started the driver answers `TX_OFF` without touching
    // the bus. On hardware you would poll after `send()` until `TX_FREE`
    // (or `TX_SCHEDULED` while waiting for a TIMESTAMPED trigger).
    let st = gw.status(0, TX_STATUS).await?;
    println!("status(0, TX_STATUS) = {st} (TX_OFF={TX_OFF}, TX_FREE={TX_FREE}, TX_SCHEDULED={TX_SCHEDULED})");

    // `abort_tx()` would cancel a scheduled/ongoing transmission; not called
    // here because it polls the modem for up to 1 s against a silent bus.

    // ----- 5. counters / misc ---------------------------------------------

    println!("\nget_instcnt() = {} us", gw.get_instcnt().await?); // 1 us counter
    println!("get_trigcnt() = {} us", gw.get_trigcnt().await?); // counter at last GPS pulse
    println!("get_eui()     = {:016x}", gw.get_eui().await?); // OTP, mock reads 0
    println!("chip_version() = {:>#04x}", gw.chip_version().await?); // COMMON_VERSION

    // ----- 6. stop ---------------------------------------------------------

    // No-op while never started; on hardware it aborts pending TX on both
    // chains and clears the started flag (C: `lgw_stop`).
    gw.stop().await?;
    println!("\nstop(): is_started() = {}", gw.is_started());

    Ok(())
}

fn main() -> lr1302::Result<()> {
    println!(
        "lr1302 {} - mock-bus walkthrough",
        Lr1302::<&mut MockSpiDevice, MockDelay>::version_info()
    );

    // The driver borrows the SPI device (`SpiDevice` is implemented for
    // `&mut T`), so we keep the mock and can inspect its counters afterwards.
    // On hardware you would move your `SpiDevice` in instead.
    let mut spi = MockSpiDevice::default();
    let mut gw = Lr1302::new(&mut spi, MockDelay);

    let result = block_on(walk_through(&mut gw));

    println!(
        "\nmock bus: {} transactions, {} bytes written, {} bytes read",
        spi.transactions, spi.bytes_written, spi.bytes_read
    );

    result
}
