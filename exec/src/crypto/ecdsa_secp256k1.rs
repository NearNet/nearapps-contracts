use crate::Contract;
use near_sdk::near_bindgen;

#[cfg(not(target_arch = "wasm32"))]
use crate::ContractContract;

pub mod sign;
pub mod types;
pub mod verify;

#[near_bindgen]
impl Contract {
    // TODO: hide behing a feature as this will not
    // be needed as a near app.
    //
    /// Creates a Public Key serialized in compressed form.
    ///
    /// Has a total size of 33 bytes.
    pub fn secp256k1_pubkey_compressed(seckey: types::SecKey) -> types::PubKeyCompressed {
        let seckey = k256::SecretKey::from_bytes(&seckey.0).unwrap();
        let pubkey = seckey.public_key();
        pubkey.into()
    }

    /// Creates a Public Key serialized in uncompressed form.
    ///
    /// Has a total size of 65 bytes.
    pub fn secp256k1_pubkey_uncompressed(seckey: types::SecKey) -> types::PubKeyUncompressed {
        let seckey = k256::SecretKey::from_bytes(&seckey.0).unwrap();
        let pubkey = seckey.public_key();
        pubkey.into()
    }
}
