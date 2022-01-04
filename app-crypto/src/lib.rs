use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;

pub mod crypto;
pub mod hash;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Crypto {
    _no_state: bool,
}
