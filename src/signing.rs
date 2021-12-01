use crate::Contract;
use near_sdk::{env, near_bindgen};

#[cfg(not(target_arch = "wasm32"))]
use crate::ContractContract;

pub use types::{PubKey, SecKey, Sha256, SignCompact};

pub mod types {
    use digest::consts::{U32, U64};
    use digest::{BlockInput, FixedOutputDirty, Reset, Update};
    use near_sdk::env;

    /// Private Key value.
    ///
    /// Has a total size of 32 bytes.
    #[derive(near_sdk::serde::Serialize, near_sdk::serde::Deserialize, Clone, PartialEq, Debug)]
    #[serde(crate = "near_sdk::serde")]
    #[serde(transparent)]
    pub struct SecKey(
        #[serde(with = "serde_big_array::BigArray")]
        //
        pub [u8; 32],
    );

    /// Sha256 value.
    ///
    /// Has a total size of 32 bytes.
    #[derive(
        near_sdk::serde::Serialize, near_sdk::serde::Deserialize, Clone, PartialEq, Debug, Default,
    )]
    #[serde(crate = "near_sdk::serde")]
    #[serde(transparent)]
    pub struct Sha256(
        #[serde(with = "serde_big_array::BigArray")]
        //
        pub [u8; 32],
    );

    impl Sha256 {
        pub fn hash_bytes(msg_bytes: &[u8]) -> Self {
            let hash = env::sha256(msg_bytes);
            let hash = hash.as_slice();
            assert_eq!(hash.len(), 32);
            let mut res = [0u8; 32];
            res.copy_from_slice(hash);
            Sha256(res)
        }
    }

    // needed so this is a Digest
    impl BlockInput for Sha256 {
        type BlockSize = U64;
    }

    // needed so this is a Digest
    // but this is not needed for sign verification
    impl Update for Sha256 {
        fn update(&mut self, _input: impl AsRef<[u8]>) {
            unimplemented!();
        }
    }

    // needed so this is a Digest
    // but this is not needed for sign verification
    impl Reset for Sha256 {
        fn reset(&mut self) {
            unimplemented!();
        }
    }

    // needed so this is a Digest
    impl FixedOutputDirty for Sha256 {
        type OutputSize = U32;

        fn finalize_into_dirty(&mut self, out: &mut digest::Output<Self>) {
            out.copy_from_slice(&self.0);
        }
    }

    /// Public Key serialized in compressed form.  
    /// Instead of having both `x` and `y` values, only `x` is present,
    /// as `y` can be derived from that.
    ///
    /// Has a total size of 33 bytes, containing:
    ///
    /// - `header` (1-byte);
    ///   - If `y` was even, the `header` is `0x02`;
    ///   - If `y` was odd, the `header` is `0x03`.
    /// - `x` (32-bytes).
    #[derive(near_sdk::serde::Serialize, near_sdk::serde::Deserialize, Clone, PartialEq, Debug)]
    #[serde(crate = "near_sdk::serde")]
    #[serde(transparent)]
    pub struct PubKey(
        #[serde(with = "serde_big_array::BigArray")]
        //
        pub [u8; 33],
    );

    /// Signature in serialized compact form.
    ///
    /// Has a total size of 64 bytes, containing:
    ///
    /// - `r` (32-bytes big-endian);
    /// - `s` (32-bytes big-endian).
    ///
    /// See also: [`k256::ecdsa::Signature`].
    #[derive(near_sdk::serde::Serialize, near_sdk::serde::Deserialize, Clone, PartialEq, Debug)]
    #[serde(crate = "near_sdk::serde")]
    #[serde(transparent)]
    pub struct SignCompact(
        #[serde(with = "serde_big_array::BigArray")]
        //
        pub [u8; 64],
    );
}

#[near_bindgen]
impl Contract {
    /// Creates a Public Key serialized in compressed form.
    ///
    /// Has a total size of 33 bytes.
    pub fn secp256k1_pubkey(seckey: SecKey) -> PubKey {
        let seckey = k256::SecretKey::from_bytes(&seckey.0).unwrap();
        let mut res = [0; 33];
        let pubkey = {
            use k256::elliptic_curve::group::GroupEncoding;
            seckey.public_key().as_affine().to_bytes()
        };
        assert_eq!(pubkey.as_slice().len(), 33);
        res.copy_from_slice(&pubkey.as_slice()[0..33]);
        PubKey(res)
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
    pub fn ecdsa_secp256k1_sign(seckey: SecKey, msg: String) -> SignCompact {
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
            SignCompact(res)
        }
    }

    /// Returns `true` if `pubkey` authenticates the
    /// `sign` of the `msg_hash`.  
    /// Returns `false` otherwise.
    ///
    /// The `msg` is hashed using `sha256` and that is used
    /// to verify the signature's authenticity.
    pub fn ecdsa_secp256k1_verify(pubkey: PubKey, sign: SignCompact, msg: String) -> bool {
        let pubkey = k256::PublicKey::from_sec1_bytes(&pubkey.0).unwrap();

        let hashed_msg = {
            use ecdsa::hazmat::FromDigest;
            let hashed_msg = Sha256::hash_bytes(msg.as_bytes());
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
    pub fn ecdsa_secp256k1_verify_hashed(
        pubkey: PubKey,
        sign: SignCompact,
        hashed_msg: Sha256,
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

    /// Generates a `sha256` hash of the given bytes.
    ///
    /// The returned hash has a total size of 32-bytes.
    ///
    /// See also: [`Self::hash_sha256_msg`]
    pub fn hash_sha256(msg_bytes: Vec<u8>) -> Sha256 {
        let hash = env::sha256(&msg_bytes);
        let hash = hash.as_slice();
        assert_eq!(hash.len(), 32);
        let mut res = [0u8; 32];
        res.copy_from_slice(hash);
        Sha256(res)
    }

    /// Generates a `sha256` hash of the byte-repesentation of the
    /// given `msg`.
    ///
    /// The returned hash has a total size of 32-bytes.
    ///
    /// See also: [`Self::hash_sha256`]
    pub fn hash_sha256_msg(msg_bytes: Vec<u8>) -> Sha256 {
        let hash = env::sha256(&msg_bytes);
        let hash = hash.as_slice();
        assert_eq!(hash.len(), 32);
        let mut res = [0u8; 32];
        res.copy_from_slice(hash);
        Sha256(res)
    }
}
