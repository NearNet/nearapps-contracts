#![allow(clippy::let_and_return)]

#[macro_use]
pub mod macros;

pub mod error;
pub mod owners;
pub mod types;
pub mod version;

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

pub use owners::Owners;
pub use version::{IVersion, Version};
