use crate::Contract;
use near_sdk::{env, near_bindgen};
use secp256k1 as s;

#[cfg(not(target_arch = "wasm32"))]
use crate::ContractContract;

pub type SecKey = [u8; 32];

#[derive(near_sdk::serde::Serialize, near_sdk::serde::Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct SignCompact(
    #[serde(with = "serde_big_array::BigArray")]
    //
    pub [u8; 64],
);

impl From<[u8; 64]> for SignCompact {
    fn from(sign: [u8; 64]) -> Self {
        Self(sign)
    }
}

impl Contract {
    pub fn secp256k1_init_all() -> s::Secp256k1<s::All> {
        s::Secp256k1::new()
    }

    pub fn secp256k1_init_sign_only() -> s::Secp256k1<s::SignOnly> {
        s::Secp256k1::signing_only()
    }

    pub fn secp256k1_init_verify_only() -> s::Secp256k1<s::VerifyOnly> {
        s::Secp256k1::verification_only()
    }

    /// Generates a `sha256 hash` ([`bitcoin_hashes::Hash::hash()`] from [`bitcoin_hashes::sha256::Hash`]) of some bytes message.
    pub fn internal_sha256(msg: &[u8]) -> bitcoin_hashes::sha256::Hash {
        use bitcoin_hashes::{sha256, Hash};
        sha256::Hash::hash(msg)
    }
}

#[near_bindgen]
impl Contract {
    /// Generates a `sha256 hash` ([`bitcoin_hashes::Hash::hash()`] from [`bitcoin_hashes::sha256::Hash`]) of some bytes message.
    pub fn hash_sha256(msg: Vec<u8>) -> bitcoin_hashes::sha256::Hash {
        Self::internal_sha256(&msg)
    }

    pub fn gen_pubkey(seckey: SecKey) -> secp256k1::key::PublicKey {
        let secp = Self::secp256k1_init_sign_only();
        secp256k1::key::PublicKey::from_secret_key(
            &secp,
            &s::SecretKey::from_slice(&seckey).unwrap(),
        )
    }

    // key::PublicKey::from_secret_key

    // TODO: make sure that all signatures created already are in lower-S
    // form.
    // According to the link below, they are.
    // This means that the explicit call to normalize is not needed.
    // https://github.com/bitcoin-core/secp256k1/blob/fecf436d5327717801da84beb3066f5a9b80ea8e/src/ecdsa_impl.h#L305
    //
    /// Signs a msg's hash using [`ecdsa` on `secp256k1`](s::Secp256k1::sign()).
    ///
    /// Signing is deterministic and the "pseudo-random" value `k` depends
    /// only on the hash of the combination of `seckey` and `msg_hash`.  
    /// See [rfc6979](https://datatracker.ietf.org/doc/html/rfc6979) for more info.
    ///
    /// To avoid generating signatures that may have malleability issues,
    /// they explicitly [normalized](s::Signature::normalize_s()) to
    /// a lower-S form.  
    /// See [bitcoin-core/secp256k1/ecdsa_signature_normalize()](https://github.com/bitcoin-core/secp256k1/blob/2e5e4b67dfb67950563c5f0ab2a62e25eb1f35c5/include/secp256k1.h#L551) for more info.
    ///
    /// Returns the signature in serialized compact form.
    pub fn ecdsa_secp256k1_sign_hashed(
        seckey: SecKey,
        msg_hash: bitcoin_hashes::sha256::Hash,
    ) -> SignCompact {
        use bitcoin_hashes::Hash;
        let msg_hash = bitcoin_hashes::sha256::Hash::from_slice(&msg_hash)
            .unwrap_or_else(|_| env::panic_str("ERR_HASH_BAD_LEN"));
        let secp = Self::secp256k1_init_sign_only();

        let mut sig = secp.sign(
            //
            &s::Message::from_slice(&msg_hash).unwrap(),
            &s::SecretKey::from_slice(&seckey).unwrap(),
        );
        sig.normalize_s();
        SignCompact::from(sig.serialize_compact())
    }

    /// Hashes a msg and then signs it using [`ecdsa` on `secp256k1`](s::Secp256k1::sign()).
    ///
    /// Signing is deterministic and the "pseudo-random" value `k` depends
    /// only on the hash of the combination of `seckey` and `msg_hash`.
    ///
    /// Returns the signature in serialized compact form.
    pub fn ecdsa_secp256k1_sign_msg(seckey: SecKey, msg: String) -> SignCompact {
        let msg_hash = Self::internal_sha256(msg.as_bytes());
        Self::ecdsa_secp256k1_sign_hashed(seckey, msg_hash)
    }

    /// test signatures.
    ///
    /// Based on two examples:
    /// - https://github.com/rust-bitcoin/rust-secp256k1/blob/master/examples/sign_verify.rs
    /// - https://github.com/rust-bitcoin/rust-secp256k1/blob/master/examples/sign_verify_recovery.rs
    pub fn test_sign(&self) {
        use secp256k1::{self as s};

        let secp = Self::secp256k1_init_verify_only();

        // let secp = s::Secp256k1::new();

        let seckey = [
            59, 148, 11, 85, 134, 130, 61, 253, 2, 174, 59, 70, 27, 180, 51, 107, 94, 203, 174,
            253, 102, 39, 170, 146, 46, 252, 4, 143, 236, 12, 136, 28,
        ];
        let pubkey = Self::gen_pubkey(seckey);
        panic!("PANIC 4");
        assert_eq!(
            pubkey.serialize(),
            [
                2, 29, 21, 35, 7, 198, 183, 43, 14, 208, 65, 139, 14, 112, 205, 128, 231, 245, 41,
                91, 141, 134, 245, 114, 45, 63, 82, 19, 251, 210, 57, 79, 54,
            ]
        );
        panic!("PANIC 5");
        assert_eq!(
            pubkey.serialize(),
            [
                2, 29, 21, 35, 7, 198, 183, 43, 14, 208, 65, 139, 14, 112, 205, 128, 231, 245, 41,
                91, 141, 134, 245, 114, 45, 63, 82, 19, 251, 210, 57, 79, 53,
            ]
        );
        panic!("PANIC 6");
        let msg = "This is some message";

        // normal signature
        {
            let serialized_sig = Self::ecdsa_secp256k1_sign_msg(seckey, msg.to_string());

            let verify = {
                let msg_hash = Self::hash_sha256(msg.as_bytes().to_vec());
                let sig = s::Signature::from_compact(&serialized_sig.0).unwrap();
                secp.verify(
                    //
                    &s::Message::from_slice(&msg_hash).unwrap(),
                    &sig,
                    &pubkey,
                )
            };
            verify.unwrap();
        }

        // recoverable signature
        /*
        {
            let (recovery_id, serialized_recoverable_sig) = {
                let msg_hash = Self::hash_sha256(msg.as_bytes().to_vec());
                let recoverable_sig = secp.sign_recoverable(
                    //
                    &s::Message::from_slice(&msg_hash).unwrap(),
                    &s::SecretKey::from_slice(&seckey).unwrap(),
                );

                // recover_id appears to be one of (0, 1, 2, 3) u8 value.
                //
                // https://docs.rs/secp256k1/0.20.3/secp256k1/recovery/struct.RecoveryId.html
                //
                // https://docs.rs/secp256k1/0.20.3/secp256k1/recovery/struct.RecoverableSignature.html#method.from_compact
                //
                // https://github.com/bitcoin-core/secp256k1/blob/793ad9016a09c3bf5a5f280c812c46526250d839/include/secp256k1_recovery.h#L34
                //
                recoverable_sig.serialize_compact()
            };
            let serialized_recovery_id = recovery_id.to_i32() as u8;

            let recovered_pubkey = {
                let msg_hash = Self::hash_sha256(msg.as_bytes().to_vec());
                let recovery_id =
                    s::recovery::RecoveryId::from_i32(serialized_recovery_id as i32).unwrap();
                let recoverable_sig = s::recovery::RecoverableSignature::from_compact(
                    //
                    &serialized_recoverable_sig,
                    recovery_id,
                )
                .unwrap();

                secp.recover(
                    //
                    &s::Message::from_slice(&msg_hash).unwrap(),
                    &recoverable_sig,
                )
                .unwrap()
            };

            assert_eq!(recovered_pubkey, pubkey);
        }
        */
    }
}
