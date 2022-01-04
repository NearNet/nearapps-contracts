use super::types;
use crate::{hash, Crypto};
use near_sdk::near_bindgen;

#[cfg(not(target_arch = "wasm32"))]
use crate::CryptoContract;

#[near_bindgen]
impl Crypto {
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
        sign.verify_compressed_msg(pubkey, msg)
    }

    pub fn ecdsa_secp256k1_verify_uncompressed_msg(
        pubkey: types::PubKeyUncompressed,
        sign: types::SignCompact,
        msg: String,
    ) -> bool {
        sign.verify_uncompressed_msg(pubkey, msg)
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
        sign.verify_prehashed_compressed(pubkey, hashed_msg)
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
        sign.verify_prehashed_uncompressed(pubkey, hashed_msg)
    }
}
