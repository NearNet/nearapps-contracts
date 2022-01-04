#![allow(clippy::let_and_return)]

pub mod error;

#[cfg(feature = "sim")]
pub mod sim;

#[cfg(feature = "sim")]
pub mod workspace;

#[cfg(feature = "sim")]
pub use workspaces;

pub use error::{ensure, OrPanicStr};

#[cfg(feature = "sim")]
pub use sim::ExecutionExt;

#[cfg(feature = "sim")]
pub use workspace::Call;
