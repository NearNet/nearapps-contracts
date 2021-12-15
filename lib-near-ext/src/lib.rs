#![allow(clippy::let_and_return)]

pub mod error;

#[cfg(feature = "sim")]
pub mod sim;

pub use error::{ensure, OrPanicStr};

#[cfg(feature = "sim")]
pub use sim::ExecutionExt;

pub const KILO: u64 = 1000;
pub const MEGA: u64 = KILO * KILO;
pub const TERA: u64 = MEGA * MEGA;
pub const YOTTA: u128 = (TERA as u128) * (TERA as u128);
