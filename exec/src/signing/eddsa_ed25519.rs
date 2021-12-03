use crate::{hash, Contract};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, ext_contract, near_bindgen, AccountId, Promise};

#[cfg(not(target_arch = "wasm32"))]
use crate::ContractContract;

pub mod types;

#[near_bindgen]
impl Contract {
    // TODO: hide behing a feature as this will not
    // be needed as a near app.
    //
    pub fn ed25519_pubkey(seckey: types::SecKey) -> types::PubKey {
        let seckey = ed25519_dalek::SecretKey::from_bytes(&seckey.0).unwrap();
        let pubkey: ed25519_dalek::PublicKey = (&seckey).into();
        types::PubKey(pubkey.to_bytes())
    }

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
        types::Sign(sign.to_bytes())
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
        types::SignPrehashed(sign.to_bytes())
    }

    pub fn eddsa_ed25519_verify(pubkey: types::PubKey, sign: types::Sign, msg: String) -> bool {
        let pubkey = ed25519_dalek::PublicKey::from_bytes(&pubkey.0).unwrap();
        let sign = ed25519_dalek::Signature::from_bytes(&sign.0).unwrap();

        {
            use ed25519_dalek::Verifier;
            pubkey.verify(msg.as_bytes(), &sign).is_ok()
        }
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
