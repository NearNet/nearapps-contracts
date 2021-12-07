#![allow(clippy::ref_in_deref)]

pub use near_sdk::json_types::{Base64VecU8, U64};
use near_sdk_sim::{call, deploy, init_simulator, view, ContractAccount, UserAccount};

pub const DEFAULT_GAS: u64 = 300_000_000_000_000;
