//! Shared mock bus for the `lr1302` host examples.
//!
//! A condensed copy of the implementation documented at length in
//! `../mock_bus.rs` (which walks through the whole API) — read that file for
//! the explanations. Cargo only builds `examples/*.rs` and
//! `examples/*/main.rs` as targets, so this file is pulled in with
//! `#[path = "common/mock.rs"] mod mock;` and is never compiled on its own.
//!
//! None of the SPI answers come from a chip: reads are filled with
//! `read_fill` (`0x00`), so `connect()` / `start()` are expected to fail
//! with `Error::ChipVersion(0)` against it.

#![allow(dead_code)] // not every example uses every item below

use std::future::Future;
use std::task::{Context, Poll, Waker};
use std::time::Duration;

use embedded_hal::spi::{ErrorKind, ErrorType, Operation};
use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::spi::SpiDevice;

/// Error type of the mock SPI device (erased into `lr1302::Error::Spi`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MockSpiError;

impl embedded_hal::spi::Error for MockSpiError {
    fn kind(&self) -> ErrorKind {
        ErrorKind::Other
    }
}

/// A fake [`SpiDevice<u8>`]: records writes, fills reads with `read_fill`,
/// sleeps on `Operation::DelayNs`, always returns `Ok(())`.
#[derive(Debug, Default)]
pub struct MockSpiDevice {
    /// Value the mock "clocks in" for every read byte (`0x00` by default).
    pub read_fill: u8,
    /// Number of transactions serviced.
    pub transactions: u32,
    /// Total bytes the driver wrote.
    pub bytes_written: usize,
    /// Total bytes the driver read.
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

/// Trivial [`DelayNs`]: sleeps the host thread (firmware: `embassy_time::Delay`).
#[derive(Debug, Clone, Copy, Default)]
pub struct MockDelay;

impl DelayNs for MockDelay {
    async fn delay_ns(&mut self, ns: u32) {
        std::thread::sleep(Duration::from_nanos(ns as u64));
    }
}

/// Minimal executor: poll until the future is ready. The mock never returns
/// `Poll::Pending`, so a plain poll loop suffices (embassy's executor in
/// firmware).
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
