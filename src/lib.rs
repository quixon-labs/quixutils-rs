#![cfg_attr(nightly, feature(backtrace))]

#![deny(
    nonstandard_style,
    rust_2018_idioms,
    future_incompatible,
    missing_debug_implementations
)]

// pub mod api;
pub mod iter;
pub mod logger;
pub mod prelude;
pub mod types;
pub mod utils;
pub mod errors;

#[cfg(feature = "tokio_utils")]
pub mod timer;
