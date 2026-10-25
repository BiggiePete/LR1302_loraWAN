//! # lr1302 — no_std driver for the Semtech SX1302 / LR1302 LoRa gateway
//! concentrator
//!
//! A faithful Rust port of the Semtech SX1302 HAL v2.1.0 (`libloragw`),
//! written for `#![no_std]` firmware and built on the `embedded-hal` /
//! `embedded-hal-async` traits so it works with any embassy-compatible SPI
//! driver.
//!
//! ## Scope
//!
//! Ported from the C HAL:
//!
//! * SX1302 register map and SPI protocol ([`reg`], [`spi`])
//! * SX1250 radio bring-up and calibration ([`sx1250`])
//! * firmware upload (AGC + arbiter MCU), mailbox protocol ([`fw`])
//! * RX channelizer / correlators / modem configuration ([`sx1302`])
//! * RX buffer fetch and packet parsing ([`rx`])
//! * TX path ([`tx`])
//! * timestamp counters ([`timestamp`])
//! * the full `lgw_*` configuration API ([`hal`])
//!
//! Not ported (returns [`Error::Unsupported`]): SX1255/1257 radios, SX1261
//! LBT / spectral scan, USB transport, GPS UART, the STTS751 temperature
//! sensor, the AD5338R DAC, and fine ("precision") timestamps.
//!
//! ## Logging
//!
//! Enable the `defmt` feature to get `defmt::trace!` / `defmt::debug!`
//! diagnostics (SPI traffic, chip detection, start/stop, every RX/TX packet)
//! in addition to `defmt::Format` derives on the public types. Without the
//! feature all logging compiles away.
//!
//! ## Example
//!
//! ```ignore
//! use lr1302::types::{BoardConf, RfConf, IfConf};
//!
//! let spi = /* embassy SPI transaction device on mode 0, <= 2 MHz */;
//! let delay = /* embassy timer */;
//!
//! let mut gw = lr1302::Lr1302::new(spi, delay);
//! gw.board_setconf(BoardConf { lorawan_public: true, clksrc: 0, full_duplex: false })?;
//! gw.rxrf_setconf(0, RfConf { enable: true, freq_hz: 868_000_000, ..Default::default() })?;
//! gw.start().await?;
//! ```

#![no_std]
#![deny(unsafe_code)]

/// AGC MCU firmware image (C: `agc_fw_sx1250.var`, `MCU_FW_SIZE` bytes).
pub const AGC_FW: &[u8; 8192] = include_bytes!("../fw/agc_fw_sx1250.bin");
/// Arbiter MCU firmware image (C: `arb_fw.var`, `MCU_FW_SIZE` bytes).
pub const ARB_FW: &[u8; 8192] = include_bytes!("../fw/arb_fw.bin");

pub mod consts;
pub mod context;
pub mod error;
pub mod fw;
pub mod hal;
mod log;
pub mod reg;
pub mod rx;
pub mod spi;
pub mod sx1250;
pub mod sx1302;
pub mod sx1302_defs;
pub mod timestamp;
pub mod toa;
pub mod tx;
pub mod types;
pub mod util;

use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::spi::SpiDevice;

use crate::context::Context;

pub use error::{Error, Result};
pub use spi::Com;

/// The LR1302 / SX1302 concentrator driver.
///
/// Owns the SPI transport and the configuration context, mirroring the C
/// globals `lgw_context` and the `lgw_*` API entry points (see [`hal`]).
pub struct Lr1302<SPI, D> {
    /// SPI transport (C: the `lgw_spi_*` / `sx1250_spi_*` layer).
    pub(crate) com: Com<SPI, D>,
    /// Configuration context (C: `lgw_context`).
    pub(crate) ctx: Context,
}

impl<SPI, D> Lr1302<SPI, D>
where
    SPI: SpiDevice<u8>,
    D: DelayNs,
{
    /// Create a driver for the given SPI device and delay provider.
    ///
    /// The bus must be configured for SPI mode 0, MSB first, at most 2 MHz.
    pub fn new(spi: SPI, delay: D) -> Self {
        Self {
            com: Com::new(spi, delay),
            ctx: Context::default(),
        }
    }

    /// Borrow the current configuration context (C: `lgw_context`).
    pub fn context(&self) -> &Context {
        &self.ctx
    }

    /// Whether `start()` has been called and `stop()` has not
    /// (C: `CONTEXT_STARTED`).
    pub fn is_started(&self) -> bool {
        self.ctx.is_started
    }
}
