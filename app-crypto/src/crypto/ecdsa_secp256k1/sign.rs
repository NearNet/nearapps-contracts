use super::types;
use crate::Crypto;
use near_sdk::near_bindgen;

#[cfg(not(target_arch = "wasm32"))]
use crate::CryptoContract;

#[near_bindgen]
impl Crypto {
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
        seckey.sign(msg)
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
        seckey.sign_recoverable(msg)
    }
}
