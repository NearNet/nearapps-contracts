use crate::Contract;
use near_sdk::near_bindgen;

#[cfg(not(target_arch = "wasm32"))]
use crate::ContractContract;

pub mod ecdsa_secp256k1;
pub mod eddsa_ed25519;

pub use ecdsa_secp256k1::types::PubKeyUncompressedNoHeader as EcdsaSecp256k1PublicKey;
pub use ecdsa_secp256k1::types::SignRecoverable as EcdsaSecp256k1Signature;

pub use eddsa_ed25519::types::PubKey as EddsaEd25519PublicKey;
pub use eddsa_ed25519::types::Sign as EddsaEd25519Signature;

pub type Bs58EncodedString = String;
pub type PubKey = String;

#[near_bindgen]
impl Contract {
    // TODO: write comments
    //
    // signature verification that is compatible to Near
    pub fn verify_hashed_msg(
        sign: Bs58EncodedString,
        pubkey: PubKey,
        msg_hash: crate::hash::Sha256,
    ) -> bool {
        let pubkey: near_sdk::PublicKey = pubkey.parse().unwrap();
        Self::verify_inner(sign, pubkey, msg_hash)
    }

    pub fn verify_msg(sign: Bs58EncodedString, pubkey: PubKey, msg: String) -> bool {
        let msg_hash = {
            use digest::Digest;
            let mut sha2_hash = sha2::Sha256::new();
            sha2_hash.update(msg.as_bytes());
            let sha2_hash = sha2_hash.finalize();

            let mut arr = [0; 32];
            arr.copy_from_slice(sha2_hash.as_slice());
            crate::hash::Sha256(arr)
        };
        let pubkey: near_sdk::PublicKey = pubkey.parse().unwrap();
        Self::verify_inner(sign, pubkey, msg_hash)
    }
}

impl Contract {
    // TODO: write comments
    //
    // signature verification that is compatible to Near
    pub fn verify_inner(
        sign: Bs58EncodedString,
        pubkey: near_sdk::PublicKey,
        msg_hash: crate::hash::Sha256,
    ) -> bool {
        use near_sdk::CurveType;
        let sign = near_sdk::bs58::decode(sign).into_vec().unwrap();
        match (pubkey.curve_type(), sign.len()) {
            (CurveType::ED25519, 64) => {
                let mut sign_raw = [0; 64];
                sign_raw.copy_from_slice(&sign[0..64]);
                let sign = EddsaEd25519Signature(sign_raw);

                let pubkey: EddsaEd25519PublicKey = pubkey.into();

                Contract::eddsa_ed25519_verify(pubkey, sign, &msg_hash.0)
            }
            (CurveType::SECP256K1, 65) => {
                let mut sign_raw = [0; 65];
                sign_raw.copy_from_slice(&sign[0..65]);
                let sign = EcdsaSecp256k1Signature(sign_raw);
                let sign: ecdsa_secp256k1::types::SignCompact = sign.into();

                let pubkey: EcdsaSecp256k1PublicKey = pubkey.into();
                let pubkey: ecdsa_secp256k1::types::PubKeyUncompressed = pubkey.into();

                Contract::ecdsa_secp256k1_verify_uncompressed_msg_bytes(pubkey, sign, &msg_hash.0)
            }
            (curve_type, sign_len) => panic!(
                "Wrong sign length of {} for the curve type {:?}",
                sign_len, curve_type
            ),
        }
    }
}
