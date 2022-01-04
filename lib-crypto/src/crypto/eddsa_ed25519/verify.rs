use super::types;
use crate::hash;

impl types::Sign {
    pub fn verify_bytes(&self, pubkey: types::PubKey, msg_bytes: Vec<u8>) -> bool {
        Self::verify(self, pubkey, &msg_bytes)
    }

    pub fn verify_msg(&self, pubkey: types::PubKey, msg: String) -> bool {
        Self::verify(self, pubkey, msg.as_bytes())
    }
}
impl types::SignPrehashed {
    pub fn verify_prehashed(
        &self,
        pubkey: types::PubKey,
        msg_hash: hash::Sha512,
        context: Option<String>,
    ) -> bool {
        let pubkey = ed25519_dalek::PublicKey::from_bytes(&pubkey.0).unwrap();
        let sign = ed25519_dalek::Signature::from_bytes(&self.0).unwrap();
        let context = context.as_ref().map(|s| s.as_bytes());
        pubkey.verify_prehashed(msg_hash, context, &sign).is_ok()
    }
}

impl types::Sign {
    pub fn verify(&self, pubkey: types::PubKey, msg_bytes: &[u8]) -> bool {
        let pubkey = ed25519_dalek::PublicKey::from_bytes(&pubkey.0).unwrap();
        let sign = ed25519_dalek::Signature::from_bytes(&self.0).unwrap();

        {
            use ed25519_dalek::Verifier;
            pubkey.verify(msg_bytes, &sign).is_ok()
        }
    }
}
