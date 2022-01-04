use super::types;
use crate::hash;

impl types::SecKey {
    // TODO: hide behing a feature as this will not
    // be needed as a near app.
    //
    pub fn sign(&self, msg: String) -> types::Sign {
        let seckey = ed25519_dalek::SecretKey::from_bytes(&self.0).unwrap();
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
    pub fn sign_prehashed(
        &self,
        msg_hash: hash::Sha512,
        context: Option<String>,
    ) -> types::SignPrehashed {
        let seckey = ed25519_dalek::SecretKey::from_bytes(&self.0).unwrap();
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
