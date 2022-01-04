pub mod sign;
pub mod types;
pub mod verify;

impl types::SecKey {
    // TODO: hide behing a feature as this will not
    // be needed as a near app.
    //
    pub fn pubkey(&self) -> types::PubKey {
        let seckey = ed25519_dalek::SecretKey::from_bytes(&self.0).unwrap();
        let pubkey: ed25519_dalek::PublicKey = (&seckey).into();
        pubkey.into()
    }
}
