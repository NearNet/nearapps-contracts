use super::types;
use crate::Contract;
use near_sdk::near_bindgen;

#[cfg(not(target_arch = "wasm32"))]
use crate::ContractContract;

#[near_bindgen]
impl Contract {
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

    // TODO: hide behing a feature as this will not
    // be needed as a near app.
    //
    /// Creates a `sha256` hash of the `msg` and signs it
    /// using `ecdsa` on `secp256k1`.  
    /// This creates a recoverable signature, ie. the PublicKey
    /// can be recovered from the signature and the original message.
    ///
    /// Signing is deterministic and the "pseudo-random" value `k` depends
    /// only on the hash of the combination of `seckey` and the hash of
    /// `msg`.
    /// See [rfc6979](https://datatracker.ietf.org/doc/html/rfc6979) for more info.
    ///
    /// TODO: re-check this:
    /// To avoid generating signatures that may have malleability issues,
    /// they are explicitly
    /// [normalized](k256::ecdsa::Signature::normalize_s()) to
    /// the lower-S form.
    ///
    /// TODO: re-check this:
    /// Returns the signature in serialized compact form.
    /// Has a total size of 64-bytes.
    pub fn ecdsa_secp256k1_sign_recoverable(
        seckey: types::SecKey,
        msg: String,
    ) -> types::SignRecoverable {
        let seckey = k256::SecretKey::from_bytes(&seckey.0).unwrap();
        let signing_key = k256::ecdsa::SigningKey::from(seckey);
        let sign: k256::ecdsa::recoverable::Signature = {
            use k256::ecdsa::signature::DigestSigner;
            use sha2::Digest;
            let digest = digest::Digest::chain(sha2::Sha256::new(), msg);
            signing_key.try_sign_digest(digest).unwrap()
        };

        // asserts it's in the lower-s form
        let mut without_recovery_id: k256::ecdsa::Signature = sign.into();
        let without_recovery_id_original = without_recovery_id;
        without_recovery_id.normalize_s().unwrap();
        if without_recovery_id != without_recovery_id_original {
            near_sdk::env::panic_str("ERR_ECDSA_SECP256K1_SIGN_NON_LOWER_S");
        }

        // TODO: check if nearcore/sdk only allows the recovery id
        // to be zero

        sign.into()
    }
}
