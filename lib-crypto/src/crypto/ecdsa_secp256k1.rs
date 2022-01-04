pub mod sign;
pub mod types;
pub mod verify;

impl types::SecKey {
    // TODO: hide behing a feature as this will not
    // be needed as a near app.
    //
    /// Creates a Public Key serialized in compressed form.
    ///
    /// Has a total size of 33 bytes.
    pub fn pubkey_compressed(&self) -> types::PubKeyCompressed {
        let seckey = k256::SecretKey::from_bytes(&self.0).unwrap();
        let pubkey = seckey.public_key();
        pubkey.into()
    }

    /// Creates a Public Key serialized in uncompressed form.
    ///
    /// Has a total size of 65 bytes.
    pub fn pubkey_uncompressed(&self) -> types::PubKeyUncompressed {
        let seckey = k256::SecretKey::from_bytes(&self.0).unwrap();
        let pubkey = seckey.public_key();
        pubkey.into()
    }
}
