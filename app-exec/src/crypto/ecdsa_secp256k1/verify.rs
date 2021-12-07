use super::types;
use crate::{hash, Contract};
use near_sdk::near_bindgen;

#[cfg(not(target_arch = "wasm32"))]
use crate::ContractContract;

#[near_bindgen]
impl Contract {
    /// Returns `true` if `pubkey` authenticates the
    /// `sign` of the `msg_hash`.  
    /// Returns `false` otherwise.
    ///
    /// The `msg` is hashed using `sha256` and that is used
    /// to verify the signature's authenticity.
    pub fn ecdsa_secp256k1_verify_compressed_msg(
        pubkey: types::PubKeyCompressed,
        sign: types::SignCompact,
        msg: String,
    ) -> bool {
        Self::ecdsa_secp256k1_verify(&pubkey.0, sign, msg.as_bytes())
    }

    pub fn ecdsa_secp256k1_verify_uncompressed_msg(
        pubkey: types::PubKeyUncompressed,
        sign: types::SignCompact,
        msg: String,
    ) -> bool {
        Self::ecdsa_secp256k1_verify(&pubkey.0, sign, msg.as_bytes())
    }

    /// Returns `true` if `pubkey` authenticates the
    /// `sign` of the `sha256` hash of the `msg`.  
    /// Returns `false` otherwise.
    ///
    /// The `msg_hash` must be the result of a `sha256` of the msg,
    /// and must have a total size of 32-bytes.
    pub fn ecdsa_secp256k1_verify_prehashed_compressed(
        pubkey: types::PubKeyCompressed,
        sign: types::SignCompact,
        hashed_msg: hash::Sha256,
    ) -> bool {
        Self::ecdsa_secp256k1_verify_prehashed(&pubkey.0, sign, hashed_msg)
    }

    /// Returns `true` if `pubkey` authenticates the
    /// `sign` of the `sha256` hash of the `msg`.  
    /// Returns `false` otherwise.
    ///
    /// The `msg_hash` must be the result of a `sha256` of the msg,
    /// and must have a total size of 32-bytes.
    pub fn ecdsa_secp256k1_verify_prehashed_uncompressed(
        pubkey: types::PubKeyUncompressed,
        sign: types::SignCompact,
        hashed_msg: hash::Sha256,
    ) -> bool {
        Self::ecdsa_secp256k1_verify_prehashed(&pubkey.0, sign, hashed_msg)
    }
}

impl Contract {
    /// Returns `true` if `pubkey` authenticates the
    /// `sign` of the `msg_hash`.  
    /// Returns `false` otherwise.
    ///
    /// The `msg` is hashed using `sha256` and that is used
    /// to verify the signature's authenticity.
    pub fn ecdsa_secp256k1_verify(
        pubkey: &[u8],
        sign: types::SignCompact,
        msg_bytes: &[u8],
    ) -> bool {
        // this is able to read both compressed and uncompressed pubkeys
        let pubkey = k256::PublicKey::from_sec1_bytes(pubkey).unwrap();

        let hashed_msg = {
            use ecdsa::hazmat::FromDigest;
            let hashed_msg = hash::Sha256::hash_bytes(msg_bytes);
            k256::Scalar::from_digest(hashed_msg)
        };

        let sign = {
            use k256::ecdsa::signature::Signature;
            k256::ecdsa::Signature::from_bytes(&sign.0).unwrap()
        };

        {
            use ecdsa::hazmat::VerifyPrimitive;
            pubkey
                .as_affine()
                .verify_prehashed(&hashed_msg, &sign)
                .is_ok()
        }
    }

    pub fn ecdsa_secp256k1_verify_compressed_msg_bytes(
        pubkey: types::PubKeyCompressed,
        sign: types::SignCompact,
        msg_bytes: &[u8],
    ) -> bool {
        Self::ecdsa_secp256k1_verify(&pubkey.0, sign, msg_bytes)
    }

    pub fn ecdsa_secp256k1_verify_uncompressed_msg_bytes(
        pubkey: types::PubKeyUncompressed,
        sign: types::SignCompact,
        msg_bytes: &[u8],
    ) -> bool {
        Self::ecdsa_secp256k1_verify(&pubkey.0, sign, msg_bytes)
    }

    /// Returns `true` if `pubkey` authenticates the
    /// `sign` of the `sha256` hash of the `msg`.  
    /// Returns `false` otherwise.
    ///
    /// The `msg_hash` must be the result of a `sha256` of the msg,
    /// and must have a total size of 32-bytes.
    pub fn ecdsa_secp256k1_verify_prehashed(
        pubkey: &[u8],
        sign: types::SignCompact,
        hashed_msg: hash::Sha256,
    ) -> bool {
        let pubkey = k256::PublicKey::from_sec1_bytes(pubkey).unwrap();

        let hashed_msg = {
            use ecdsa::hazmat::FromDigest;
            k256::Scalar::from_digest(hashed_msg)
        };

        let sign = {
            use k256::ecdsa::signature::Signature;
            k256::ecdsa::Signature::from_bytes(&sign.0).unwrap()
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
