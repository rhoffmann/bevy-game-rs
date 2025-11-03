#![warn(missing_docs)]
//! `my_library` is a Rust library that provides a suite of helpers to create games with bevy.
//!
//! ## What's inside?
//!
//! `my_library` includes:
//!
//! * Random number generation utilities with and without locking for multi-threaded contexts.
//!
//! ## Feature Flags
//!
//! The following feature flags are supported: `xorshift`, `pcg`, `locking`.
//!
//! ### Random Number Generation
//!
//! * The `locking` feature flag enables interior mutability for [`RandomNumberGenerator`], allowing it to be used as a Resource
//! in multi-threaded Bevy applications (`Res<RandomNumberGenerator>`) rather than `ResMut<RandomNumberGenerator>`.
//! * The `xorshift` and `pcg` feature flags allow you to choose different underlying RNG algorithms. If neither is specified, the default `StdRng` is used.
//!     * `xorshift` uses the `rand_xorshift` crate's XorShift algorithm.
//!     * `pcg` uses the `rand_pcg` crate's `PCG` algorithm.

#[cfg(not(feature = "locking"))]
mod random;
#[cfg(not(feature = "locking"))]
pub use random::*;

#[cfg(feature = "locking")]
mod random_locking;
#[cfg(feature = "locking")]
pub use random_locking::*;

/// [`RandomNumberGenerator`] wraps the `rand` crate and re-exports random number generation functionality.
pub use rand;
