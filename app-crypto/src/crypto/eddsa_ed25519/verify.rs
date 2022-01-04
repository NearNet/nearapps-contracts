use super::types;
use crate::{hash, Crypto};
use near_sdk::near_bindgen;

#[cfg(not(target_arch = "wasm32"))]
use crate::CryptoContract;

#[near_bindgen]
impl Crypto {
    pub fn eddsa_ed25519_verify_bytes(
        pubkey: types::PubKey,
        sign: types::Sign,
        msg_bytes: Vec<u8>,
    ) -> bool {
        sign.verify_bytes(pubkey, msg_bytes)
    }

    pub fn eddsa_ed25519_verify_msg(pubkey: types::PubKey, sign: types::Sign, msg: String) -> bool {
        sign.verify_msg(pubkey, msg)
    }

    pub fn eddsa_ed25519_verify_prehashed(
        pubkey: types::PubKey,
        sign: types::SignPrehashed,
        msg_hash: hash::Sha512,
        context: Option<String>,
    ) -> bool {
        sign.verify_prehashed(pubkey, msg_hash, context)
    }
}
