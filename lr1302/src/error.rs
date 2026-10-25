//! Error type shared by every operation of this driver.

/// Result alias used throughout the crate.
pub type Result<T> = core::result::Result<T, Error>;

/// Errors reported by the LR1302 / SX1302 driver.
///
/// The C HAL reports errors as negative integers (`LGW_REG_ERROR`,
/// `LGW_HAL_ERROR`, ...); this enum carries a bit more context.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum Error {
    /// The SPI transaction itself failed (driver error).
    ///
    /// The `embedded_hal` SPI error is erased here; enable driver-level
    /// logging in your SPI driver if you need details.
    Spi,
    /// A polling loop (firmware mailbox, TX abort, ...) exceeded its timeout.
    Timeout,
    /// The `COMMON_VERSION` register did not hold an SX1302/2/3 version.
    ChipVersion(u8),
    /// A configuration value rejected by the validation routines.
    ///
    /// The payload is the (static) name of the offending field.
    InvalidConfig(&'static str),
    /// The requested register access is not supported (e.g. write to a
    /// read-only register, or a register spanning more than one byte with
    /// a non-zero offset).
    Unsupported,
    /// The SX1250 radio reported an error (image calibration failed, ...).
    Radio(u8),
    /// The firmware image read back from the concentrator did not match
    /// the image that was uploaded.
    FirmwareVerify,
    /// No packet available in the RX buffer / TX not configured.
    NothingToDo,
}
