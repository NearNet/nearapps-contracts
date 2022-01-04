use super::types;
use crate::hash;

impl types::SignCompact {
    /// Returns `true` if `pubkey` authenticates the
    /// `sign` of the `msg_hash`.  
    /// Returns `false` otherwise.
    ///
    /// The `msg` is hashed using `sha256` and that is used
    /// to verify the signature's authenticity.
    pub fn verify_compressed_msg(&self, pubkey: types::PubKeyCompressed, msg: String) -> bool {
        Self::verify(self, &pubkey.0, msg.as_bytes())
    }

    pub fn verify_uncompressed_msg(&self, pubkey: types::PubKeyUncompressed, msg: String) -> bool {
        Self::verify(self, &pubkey.0, msg.as_bytes())
    }

    /// Returns `true` if `pubkey` authenticates the
    /// `sign` of the `sha256` hash of the `msg`.  
    /// Returns `false` otherwise.
    ///
    /// The `msg_hash` must be the result of a `sha256` of the msg,
    /// and must have a total size of 32-bytes.
    pub fn verify_prehashed_compressed(
        &self,
        pubkey: types::PubKeyCompressed,
        hashed_msg: hash::Sha256,
    ) -> bool {
        Self::verify_prehashed(self, &pubkey.0, hashed_msg)
    }

    /// Returns `true` if `pubkey` authenticates the
    /// `sign` of the `sha256` hash of the `msg`.  
    /// Returns `false` otherwise.
    ///
    /// The `msg_hash` must be the result of a `sha256` of the msg,
    /// and must have a total size of 32-bytes.
    pub fn verify_prehashed_uncompressed(
        &self,
        pubkey: types::PubKeyUncompressed,
        hashed_msg: hash::Sha256,
    ) -> bool {
        Self::verify_prehashed(self, &pubkey.0, hashed_msg)
    }
}

impl types::SignCompact {
    /// Returns `true` if `pubkey` authenticates the
    /// `sign` of the `msg_hash`.  
    /// Returns `false` otherwise.
    ///
    /// The `msg` is hashed using `sha256` and that is used
    /// to verify the signature's authenticity.
    pub fn verify(&self, pubkey: &[u8], msg_bytes: &[u8]) -> bool {
        // this is able to read both compressed and uncompressed pubkeys
        let pubkey = k256::PublicKey::from_sec1_bytes(pubkey).unwrap();

        let hashed_msg = {
            use ecdsa::hazmat::FromDigest;
            let hashed_msg = hash::Sha256::hash_bytes(msg_bytes);
            k256::Scalar::from_digest(hashed_msg)
        };

        let sign = {
            use k256::ecdsa::signature::Signature;
            k256::ecdsa::Signature::from_bytes(&self.0).unwrap()
        };

        {
            use ecdsa::hazmat::VerifyPrimitive;
            pubkey
                .as_affine()
                .verify_prehashed(&hashed_msg, &sign)
                .is_ok()
        }
    }

    pub fn verify_compressed_msg_bytes(
        &self,
        pubkey: types::PubKeyCompressed,
        msg_bytes: &[u8],
    ) -> bool {
        Self::verify(self, &pubkey.0, msg_bytes)
    }

    pub fn verify_uncompressed_msg_bytes(
        &self,
        pubkey: types::PubKeyUncompressed,
        msg_bytes: &[u8],
    ) -> bool {
        Self::verify(self, &pubkey.0, msg_bytes)
    }

    /// Returns `true` if `pubkey` authenticates the
    /// `sign` of the `sha256` hash of the `msg`.  
    /// Returns `false` otherwise.
    ///
    /// The `msg_hash` must be the result of a `sha256` of the msg,
    /// and must have a total size of 32-bytes.
    pub fn verify_prehashed(&self, pubkey: &[u8], hashed_msg: hash::Sha256) -> bool {
        let pubkey = k256::PublicKey::from_sec1_bytes(pubkey).unwrap();

        let hashed_msg = {
            use ecdsa::hazmat::FromDigest;
            k256::Scalar::from_digest(hashed_msg)
        };

        let sign = {
            use k256::ecdsa::signature::Signature;
            k256::ecdsa::Signature::from_bytes(&self.0).unwrap()
        };

        {
            use ecdsa::hazmat::VerifyPrimitive;
            pubkey
                .as_affine()
                .verify_prehashed(&hashed_msg, &sign)
                .is_ok()
        }
    }
}
