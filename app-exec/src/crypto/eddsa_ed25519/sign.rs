use super::types;
use crate::{hash, Contract};
use near_sdk::near_bindgen;

#[cfg(not(target_arch = "wasm32"))]
use crate::ContractContract;

#[near_bindgen]
impl Contract {
    // TODO: hide behing a feature as this will not
    // be needed as a near app.
    //
    pub fn eddsa_ed25519_sign(seckey: types::SecKey, msg: String) -> types::Sign {
        let seckey = ed25519_dalek::SecretKey::from_bytes(&seckey.0).unwrap();
        let pubkey: ed25519_dalek::PublicKey = (&seckey).into();
        let keypair = ed25519_dalek::Keypair {
            secret: seckey,
            public: pubkey,
        };
        let sign: ed25519_dalek::Signature = {
            use ed25519_dalek::Signer;
            keypair.sign(msg.as_bytes())
        };
        sign.into()
    }

    // TODO: hide behing a feature as this will not
    // be needed as a near app.
    pub fn eddsa_ed25519_sign_prehashed(
        seckey: types::SecKey,
        msg_hash: hash::Sha512,
        context: Option<String>,
    ) -> types::SignPrehashed {
        let seckey = ed25519_dalek::SecretKey::from_bytes(&seckey.0).unwrap();
        let pubkey: ed25519_dalek::PublicKey = (&seckey).into();
        let keypair = ed25519_dalek::Keypair {
            secret: seckey,
            public: pubkey,
        };
        let context = context.as_ref().map(|s| s.as_bytes());
        let sign: ed25519_dalek::Signature = keypair.sign_prehashed(msg_hash, context).unwrap();
        sign.into()
    }
}
