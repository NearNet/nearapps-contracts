use crate::Contract;
use near_sdk::near_bindgen;

#[cfg(not(target_arch = "wasm32"))]
use crate::ContractContract;

#[near_bindgen]
impl Contract {
    /// test signatures.
    ///
    /// Based on two examples:
    /// - https://github.com/rust-bitcoin/rust-secp256k1/blob/master/examples/sign_verify.rs
    /// - https://github.com/rust-bitcoin/rust-secp256k1/blob/master/examples/sign_verify_recovery.rs
    pub fn test_sign(&mut self) {
        use bitcoin_hashes::{sha256, Hash};
        use secp256k1::{self as s};

        let secp = s::Secp256k1::new();

        let seckey = [
            59, 148, 11, 85, 134, 130, 61, 253, 2, 174, 59, 70, 27, 180, 51, 107, 94, 203, 174,
            253, 102, 39, 170, 146, 46, 252, 4, 143, 236, 12, 136, 28,
        ];
        let pubkey = [
            2, 29, 21, 35, 7, 198, 183, 43, 14, 208, 65, 139, 14, 112, 205, 128, 231, 245, 41, 91,
            141, 134, 245, 114, 45, 63, 82, 19, 251, 210, 57, 79, 54,
        ];
        let msg = b"This is some message";

        // normal signature
        {
            let serialized_sig = {
                let msg_hash = sha256::Hash::hash(msg);
                let sig = secp.sign(
                    //
                    &s::Message::from_slice(&msg_hash).unwrap(),
                    &s::SecretKey::from_slice(&seckey).unwrap(),
                );
                sig.serialize_compact()
            };

            let verify = {
                let msg_hash = sha256::Hash::hash(msg);
                let sig = s::Signature::from_compact(&serialized_sig).unwrap();
                let pubkey = s::PublicKey::from_slice(&pubkey).unwrap();
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
        {
            let (recovery_id, serialized_recoverable_sig) = {
                let msg_hash = sha256::Hash::hash(msg);
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
                let msg_hash = sha256::Hash::hash(msg);
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

            assert_eq!(recovered_pubkey, s::PublicKey::from_slice(&pubkey).unwrap());
        }
    }
}
