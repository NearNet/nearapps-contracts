use super::types;
use crate::{hash, Executor};
use near_sdk::near_bindgen;

#[cfg(not(target_arch = "wasm32"))]
use crate::ExecutorContract;

#[near_bindgen]
impl Executor {
    pub fn eddsa_ed25519_verify_bytes(
        pubkey: types::PubKey,
        sign: types::Sign,
        msg_bytes: Vec<u8>,
    ) -> bool {
        Self::eddsa_ed25519_verify(pubkey, sign, &msg_bytes)
    }

    pub fn eddsa_ed25519_verify_msg(pubkey: types::PubKey, sign: types::Sign, msg: String) -> bool {
        Self::eddsa_ed25519_verify(pubkey, sign, &msg.as_bytes())
    }

    pub fn eddsa_ed25519_verify_prehashed(
        pubkey: types::PubKey,
        sign: types::SignPrehashed,
        msg_hash: hash::Sha512,
        context: Option<String>,
    ) -> bool {
        let pubkey = ed25519_dalek::PublicKey::from_bytes(&pubkey.0).unwrap();
        let sign = ed25519_dalek::Signature::from_bytes(&sign.0).unwrap();
        let context = context.as_ref().map(|s| s.as_bytes());
        pubkey.verify_prehashed(msg_hash, context, &sign).is_ok()
    }
}

impl Executor {
    pub fn eddsa_ed25519_verify(
        pubkey: types::PubKey,
        sign: types::Sign,
        msg_bytes: &[u8],
    ) -> bool {
        let pubkey = ed25519_dalek::PublicKey::from_bytes(&pubkey.0).unwrap();
        let sign = ed25519_dalek::Signature::from_bytes(&sign.0).unwrap();

        {
            use ed25519_dalek::Verifier;
            pubkey.verify(msg_bytes, &sign).is_ok()
        }
    }
}
