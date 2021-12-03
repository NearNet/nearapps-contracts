use crate::Contract;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, ext_contract, near_bindgen, AccountId, Promise};

#[cfg(not(target_arch = "wasm32"))]
use crate::ContractContract;

pub mod ecdsa_secp256k1;
pub mod eddsa_ed25519;
