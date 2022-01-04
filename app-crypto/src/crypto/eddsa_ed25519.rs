use crate::Crypto;
use near_sdk::near_bindgen;
pub use nearapps_lib_crypto::crypto::eddsa_ed25519::types;

#[cfg(not(target_arch = "wasm32"))]
use crate::CryptoContract;

pub mod sign;
pub mod verify;

#[near_bindgen]
impl Crypto {
    // TODO: hide behing a feature as this will not
    // be needed as a near app.
    //
    pub fn ed25519_pubkey(seckey: types::SecKey) -> types::PubKey {
        seckey.pubkey()
    }
}
