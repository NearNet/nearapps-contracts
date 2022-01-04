use crate::Crypto;
use near_sdk::near_bindgen;

#[cfg(not(target_arch = "wasm32"))]
use crate::CryptoContract;

pub mod ecdsa_secp256k1;
pub mod eddsa_ed25519;

pub use ecdsa_secp256k1::types::PubKeyUncompressedNoHeader as EcdsaSecp256k1PublicKey;
pub use ecdsa_secp256k1::types::SignRecoverable as EcdsaSecp256k1Signature;

pub use eddsa_ed25519::types::PubKey as EddsaEd25519PublicKey;
pub use eddsa_ed25519::types::Sign as EddsaEd25519Signature;

use nearapps_lib_crypto::crypto::{Bs58EncodedSignature, NearEncodedPubkey};

#[near_bindgen]
impl Crypto {
    // TODO: write comments
    //
    // signature verification that is compatible to Near
    pub fn verify_hashed_msg(
        sign: Bs58EncodedSignature,
        pubkey: NearEncodedPubkey,
        msg_hash: crate::hash::Sha256,
    ) -> bool {
        sign.verify_hashed_msg(pubkey, msg_hash)
    }

    /// Verifies if `pubkey` matches `sign` with the `sha256` hash of
    /// the `msg`.
    ///
    /// Note: Internally the hashed msg is hashed again by the
    /// signature verification algorithm. This is compatible with
    /// Near's behavior.
    pub fn verify_msg(sign: Bs58EncodedSignature, pubkey: NearEncodedPubkey, msg: String) -> bool {
        sign.verify_msg(pubkey, msg)
    }
}
