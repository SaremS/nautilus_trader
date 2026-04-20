//! [NautilusTrader](https://nautilustrader.io) adapter for [Slack](https://slack.com).
//!
//! The `nautilus-slack` crate provides a complete integration with the Slack API for
//! # NautilusTrader
//!
//! [NautilusTrader](https://nautilustrader.io) is an open-source, production-grade, Rust-native
//! engine for multi-asset, multi-venue trading systems.
//!
//! The system spans research, deterministic simulation, and live execution within a single
//! event-driven architecture, providing research-to-live semantic parity.
//!
//! # Feature Flags
//!
//! This crate provides feature flags to control source code inclusion during compilation,
//! depending on the intended use case:
//!
//! - `live` (default): Enables live data functionality including the `data`, `factories`, and `live` modules.
//! - `python`: Enables Python bindings from [PyO3](https://pyo3.rs).
//! - `extension-module`: Builds as a Python extension module.
//! - `high-precision`: Enables [high-precision mode](https://nautilustrader.io/docs/nightly/getting_started/installation#precision-mode) to use 128-bit value types.

#![warn(rustc::all)]
#![deny(unsafe_code)]
#![deny(nonstandard_style)]
#![deny(missing_debug_implementations)]
#![deny(clippy::missing_errors_doc)]
#![deny(clippy::missing_panics_doc)]
#![deny(rustdoc::broken_intra_doc_links)]

pub mod common;

#[cfg(feature = "python")]
pub mod python;

#[cfg(feature = "live")]
pub mod data;

#[cfg(feature = "live")]
pub mod factories;

#[cfg(feature = "live")]
pub mod live;
