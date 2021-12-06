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
    pub fn ed25519_pubkey(seckey: types::SecKey) -> types::PubKey {
        let seckey = ed25519_dalek::SecretKey::from_bytes(&seckey.0).unwrap();
        let pubkey: ed25519_dalek::PublicKey = (&seckey).into();
        pubkey.into()
    }
}
