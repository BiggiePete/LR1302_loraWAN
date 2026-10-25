//! Feature-gated logging macros.
//!
//! With the `defmt` cargo feature enabled, [`trace!`] and [`debug!`] forward
//! to `defmt::trace!` / `defmt::debug!` so every diagnostic the driver emits
//! shows up on the RTT/defmt wire. With the feature disabled they expand to
//! nothing, keeping the binary free of logging overhead and the crate usable
//! without a `defmt` backend.
//!
//! The macros are `pub(crate)`: logging is an internal implementation
//! detail, not part of the public API.

/// `defmt::trace`-level log, only emitted with the `defmt` feature.
#[cfg(feature = "defmt")]
macro_rules! trace {
    ($($arg:tt)*) => {
        ::defmt::trace!($($arg)*)
    };
}

/// `defmt::trace`-level log, only emitted with the `defmt` feature.
#[cfg(not(feature = "defmt"))]
macro_rules! trace {
    ($($arg:tt)*) => {};
}

/// `defmt::debug`-level log, only emitted with the `defmt` feature.
#[cfg(feature = "defmt")]
macro_rules! debug {
    ($($arg:tt)*) => {
        ::defmt::debug!($($arg)*)
    };
}

/// `defmt::debug`-level log, only emitted with the `defmt` feature.
#[cfg(not(feature = "defmt"))]
macro_rules! debug {
    ($($arg:tt)*) => {};
}

pub(crate) use debug;
pub(crate) use trace;
