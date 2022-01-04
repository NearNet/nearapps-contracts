use crate::Crypto;
use near_sdk::near_bindgen;
pub use nearapps_lib_crypto::crypto::ecdsa_secp256k1::types;

#[cfg(not(target_arch = "wasm32"))]
use crate::CryptoContract;

pub mod sign;
pub mod verify;

#[near_bindgen]
impl Crypto {
    // TODO: hide behing a feature as this will not
    // be needed as a near app.
    //
    /// Creates a Public Key serialized in compressed form.
    ///
    /// Has a total size of 33 bytes.
    pub fn secp256k1_pubkey_compressed(seckey: types::SecKey) -> types::PubKeyCompressed {
        seckey.pubkey_compressed()
    }

    /// Creates a Public Key serialized in uncompressed form.
    ///
    /// Has a total size of 65 bytes.
    pub fn secp256k1_pubkey_uncompressed(seckey: types::SecKey) -> types::PubKeyUncompressed {
        seckey.pubkey_uncompressed()
    }
}
