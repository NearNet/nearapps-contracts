use crate::{hash, Contract};
use near_sdk::near_bindgen;

#[cfg(not(target_arch = "wasm32"))]
use crate::ContractContract;

pub mod types;

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

    // TODO: hide behing a feature as this will not
    // be needed as a near app.
    //
    /// Creates a `sha256` hash of the `msg` and signs it
    /// using `ecdsa` on `secp256k1`.
    ///
    /// Signing is deterministic and the "pseudo-random" value `k` depends
    /// only on the hash of the combination of `seckey` and the hash of
    /// `msg`.
    /// See [rfc6979](https://datatracker.ietf.org/doc/html/rfc6979) for more info.
    ///
    /// To avoid generating signatures that may have malleability issues,
    /// they are explicitly
    /// [normalized](k256::ecdsa::Signature::normalize_s()) to
    /// the lower-S form.
    ///
    /// Returns the signature in serialized compact form.
    /// Has a total size of 64-bytes.
    pub fn ecdsa_secp256k1_sign(seckey: types::SecKey, msg: String) -> types::SignCompact {
        let seckey = k256::SecretKey::from_bytes(&seckey.0).unwrap();
        let signing_key = k256::ecdsa::SigningKey::from(seckey);
        let mut sign: k256::ecdsa::Signature = {
            use k256::ecdsa::signature::DigestSigner;
            use sha2::Digest;
            let digest = digest::Digest::chain(sha2::Sha256::new(), msg);
            signing_key.try_sign_digest(digest).unwrap()
        };
        sign.normalize_s().unwrap();
        sign.into()
    }

    /// Returns `true` if `pubkey` authenticates the
    /// `sign` of the `msg_hash`.  
    /// Returns `false` otherwise.
    ///
    /// The `msg` is hashed using `sha256` and that is used
    /// to verify the signature's authenticity.
    pub fn ecdsa_secp256k1_verify_compressed(
        pubkey: types::PubKeyCompressed,
        sign: types::SignCompact,
        msg: String,
    ) -> bool {
        Self::ecdsa_secp256k1_verify(&pubkey.0, sign, msg)
    }

    pub fn ecdsa_secp256k1_verify_uncompressed(
        pubkey: types::PubKeyUncompressed,
        sign: types::SignCompact,
        msg: String,
    ) -> bool {
        Self::ecdsa_secp256k1_verify(&pubkey.0, sign, msg)
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
    pub fn ecdsa_secp256k1_verify(pubkey: &[u8], sign: types::SignCompact, msg: String) -> bool {
        let pubkey = k256::PublicKey::from_sec1_bytes(pubkey).unwrap();

        let hashed_msg = {
            use ecdsa::hazmat::FromDigest;
            let hashed_msg = hash::Sha256::hash_bytes(msg.as_bytes());
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
