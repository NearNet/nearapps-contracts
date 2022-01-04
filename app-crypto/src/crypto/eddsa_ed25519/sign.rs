use super::types;
use crate::{hash, Crypto};
use near_sdk::near_bindgen;

#[cfg(not(target_arch = "wasm32"))]
use crate::CryptoContract;

#[near_bindgen]
impl Crypto {
    // TODO: hide behing a feature as this will not
    // be needed as a near app.
    //
    pub fn eddsa_ed25519_sign(seckey: types::SecKey, msg: String) -> types::Sign {
        seckey.sign(msg)
    }

    // TODO: hide behing a feature as this will not
    // be needed as a near app.
    pub fn eddsa_ed25519_sign_prehashed(
        seckey: types::SecKey,
        msg_hash: hash::Sha512,
        context: Option<String>,
    ) -> types::SignPrehashed {
        seckey.sign_prehashed(msg_hash, context)
    }
}
