use crate::Contract;
use near_sdk::{env, near_bindgen};

#[cfg(not(target_arch = "wasm32"))]
use crate::ContractContract;

pub mod types;

#[near_bindgen]
impl Contract {
    /// Generates a `sha256` hash of the given bytes.
    ///
    /// The returned hash has a total size of 32-bytes.
    ///
    /// See also: [`Self::hash_sha256_msg`]
    pub fn hash_sha256(msg_bytes: Vec<u8>) -> types::hash::Sha256 {
        let hash = env::sha256(&msg_bytes);
        let hash = hash.as_slice();
        assert_eq!(hash.len(), 32);
        let mut res = [0u8; 32];
        res.copy_from_slice(hash);
        types::hash::Sha256(res)
    }

    /// Generates a `sha256` hash of the byte-repesentation of the
    /// given `msg`.
    ///
    /// The returned hash has a total size of 32-bytes.
    ///
    /// See also: [`Self::hash_sha256`]
    pub fn hash_sha256_msg(msg: String) -> types::hash::Sha256 {
        let hash = env::sha256(msg.as_bytes());
        let hash = hash.as_slice();
        assert_eq!(hash.len(), 32);
        let mut res = [0u8; 32];
        res.copy_from_slice(hash);
        types::hash::Sha256(res)
    }

    /// Generates a `sha512` hash of the given bytes.
    ///
    /// The returned hash has a total size of 64-bytes.
    ///
    /// See also: [`Self::hash_sha512_msg`]
    pub fn hash_sha512(msg_bytes: Vec<u8>) -> types::hash::Sha512 {
        types::hash::Sha512::hash_bytes(&msg_bytes)
    }

    /// Generates a `sha512` hash of the byte-repesentation of the
    /// given `msg`.
    ///
    /// The returned hash has a total size of 64-bytes.
    ///
    /// See also: [`Self::hash_sha512`]
    pub fn hash_sha512_msg(msg: String) -> types::hash::Sha512 {
        types::hash::Sha512::hash_bytes(msg.as_bytes())
    }

    // TODO: hide behing a feature as this will not
    // be needed as a near app.
    //
    /// Creates a Public Key serialized in compressed form.
    ///
    /// Has a total size of 33 bytes.
    pub fn secp256k1_pubkey(seckey: types::secp256k1::SecKey) -> types::secp256k1::PubKeyCompact {
        let seckey = k256::SecretKey::from_bytes(&seckey.0).unwrap();
        let mut res = [0; 33];
        let pubkey = {
            use k256::elliptic_curve::group::GroupEncoding;
            seckey.public_key().as_affine().to_bytes()
        };
        assert_eq!(pubkey.as_slice().len(), 33);
        res.copy_from_slice(&pubkey.as_slice()[0..33]);
        types::secp256k1::PubKeyCompact(res)
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
    pub fn ecdsa_secp256k1_sign(
        seckey: types::secp256k1::SecKey,
        msg: String,
    ) -> types::secp256k1::SignCompact {
        let seckey = k256::SecretKey::from_bytes(&seckey.0).unwrap();
        let signing_key = k256::ecdsa::SigningKey::from(seckey);
        let mut sign: k256::ecdsa::Signature = {
            use k256::ecdsa::signature::DigestSigner;
            use sha2::Digest;
            let digest = digest::Digest::chain(sha2::Sha256::new(), msg);
            signing_key.try_sign_digest(digest).unwrap()
        };
        sign.normalize_s().unwrap();
        {
            use k256::ecdsa::signature::Signature;
            let mut res = [0u8; 64];
            assert_eq!(sign.as_bytes().len(), 64);
            res.copy_from_slice(&sign.as_bytes()[0..64]);
            types::secp256k1::SignCompact(res)
        }
    }

    /// Returns `true` if `pubkey` authenticates the
    /// `sign` of the `msg_hash`.  
    /// Returns `false` otherwise.
    ///
    /// The `msg` is hashed using `sha256` and that is used
    /// to verify the signature's authenticity.
    pub fn ecdsa_secp256k1_verify(
        pubkey: types::secp256k1::PubKeyCompact,
        sign: types::secp256k1::SignCompact,
        msg: String,
    ) -> bool {
        let pubkey = k256::PublicKey::from_sec1_bytes(&pubkey.0).unwrap();

        let hashed_msg = {
            use ecdsa::hazmat::FromDigest;
            let hashed_msg = types::hash::Sha256::hash_bytes(msg.as_bytes());
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
        pubkey: types::secp256k1::PubKeyCompact,
        sign: types::secp256k1::SignCompact,
        hashed_msg: types::hash::Sha256,
    ) -> bool {
        let pubkey = k256::PublicKey::from_sec1_bytes(&pubkey.0).unwrap();

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

    // TODO: hide behing a feature as this will not
    // be needed as a near app.
    //
    pub fn ed25519_pubkey(seckey: types::ed25519::SecKey) -> types::ed25519::PubKey {
        let seckey = ed25519_dalek::SecretKey::from_bytes(&seckey.0).unwrap();
        let pubkey: ed25519_dalek::PublicKey = (&seckey).into();
        types::ed25519::PubKey(pubkey.to_bytes())
    }

    // TODO: hide behing a feature as this will not
    // be needed as a near app.
    //
    pub fn eddsa_ed25519_sign(seckey: types::ed25519::SecKey, msg: String) -> types::ed25519::Sign {
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
        types::ed25519::Sign(sign.to_bytes())
    }

    // TODO: hide behing a feature as this will not
    // be needed as a near app.
    //
    // TODO: change the return into PrehashedSign
    pub fn eddsa_ed25519_sign_prehashed(
        seckey: types::ed25519::SecKey,
        msg_hash: types::hash::Sha512,
    ) -> types::ed25519::Sign {
        let seckey = ed25519_dalek::SecretKey::from_bytes(&seckey.0).unwrap();
        let pubkey: ed25519_dalek::PublicKey = (&seckey).into();
        let keypair = ed25519_dalek::Keypair {
            secret: seckey,
            public: pubkey,
        };
        let sign: ed25519_dalek::Signature = keypair.sign_prehashed(msg_hash, None).unwrap();
        types::ed25519::Sign(sign.to_bytes())
    }

    pub fn eddsa_ed25519_verify(
        pubkey: types::ed25519::PubKey,
        sign: types::ed25519::Sign,
        msg: String,
    ) -> bool {
        let pubkey = ed25519_dalek::PublicKey::from_bytes(&pubkey.0).unwrap();
        let sign = ed25519_dalek::Signature::from_bytes(&sign.0).unwrap();

        {
            use ed25519_dalek::Verifier;
            pubkey.verify(msg.as_bytes(), &sign).is_ok()
        }
    }

    pub fn eddsa_ed25519_verify_prehashed(
        pubkey: types::ed25519::PubKey,
        sign: types::ed25519::Sign,
        msg_hash: types::hash::Sha512,
    ) -> bool {
        let pubkey = ed25519_dalek::PublicKey::from_bytes(&pubkey.0).unwrap();
        let sign = ed25519_dalek::Signature::from_bytes(&sign.0).unwrap();
        pubkey.verify_prehashed(msg_hash, None, &sign).is_ok()
    }
}
