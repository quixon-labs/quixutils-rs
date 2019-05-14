#![feature(async_await)]
#![deny(
    nonstandard_style,
    rust_2018_idioms,
    future_incompatible,
    missing_debug_implementations
)]

pub mod api;
pub mod iter;
pub mod logger;
pub mod prelude;
pub mod timer;
pub mod types;
pub mod utils;
