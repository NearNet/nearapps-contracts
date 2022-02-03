use crate::Crypto;
use near_sdk::near_bindgen;

#[cfg(not(target_arch = "wasm32"))]
use crate::CryptoContract;

pub use nearapps_lib_crypto::hash::{Sha256, Sha512};

#[near_bindgen]
impl Crypto {
    /// Generates a `sha256` hash of the given bytes.
    ///
    /// The returned hash has a total size of 32-bytes.
    ///
    /// See also: [`Crypto::hash_sha256_msg`]
    pub fn hash_sha256(msg_bytes: Vec<u8>) -> Sha256 {
        Sha256::hash(msg_bytes)
    }

    /// Generates a `sha256` hash of the byte-repesentation of the
    /// given `msg`.
    ///
    /// The returned hash has a total size of 32-bytes.
    ///
    /// See also: [`Crypto::hash_sha256`]
    pub fn hash_sha256_msg(msg: String) -> Sha256 {
        Sha256::hash_msg(msg)
    }

    /// Generates a `sha512` hash of the given bytes.
    ///
    /// The returned hash has a total size of 64-bytes.
    ///
    /// See also: [`Crypto::hash_sha512_msg`]
    pub fn hash_sha512(msg_bytes: Vec<u8>) -> Sha512 {
        Sha512::hash(msg_bytes)
    }

    /// Generates a `sha512` hash of the byte-repesentation of the
    /// given `msg`.
    ///
    /// The returned hash has a total size of 64-bytes.
    ///
    /// See also: [`Crypto::hash_sha512`]
    pub fn hash_sha512_msg(msg: String) -> Sha512 {
        Sha512::hash_msg(msg)
    }
}
