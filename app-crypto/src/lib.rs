use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;

pub mod crypto;
pub mod hash;
pub mod version;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct Crypto {
    _no_state: bool,
}
