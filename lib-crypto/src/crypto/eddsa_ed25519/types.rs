use std::convert::TryFrom;

// todo: replace this by an extended key, as this
// is what nearcore uses
//
/// Private Key value.
///
/// Has a total size of 32 bytes.
#[derive(near_sdk::serde::Serialize, near_sdk::serde::Deserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
#[serde(transparent)]
pub struct SecKey(
    #[serde(with = "serde_big_array::BigArray")]
    //
    pub [u8; ed25519_dalek::SECRET_KEY_LENGTH],
);

/// Public Key value.  
///
/// Has a total size of 32 bytes.
#[derive(near_sdk::serde::Serialize, near_sdk::serde::Deserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
#[serde(transparent)]
pub struct PubKey(
    #[serde(with = "serde_big_array::BigArray")]
    //
    pub [u8; ed25519_dalek::PUBLIC_KEY_LENGTH],
);

impl From<ed25519_dalek::PublicKey> for PubKey {
    fn from(pubkey: ed25519_dalek::PublicKey) -> Self {
        PubKey(pubkey.to_bytes())
    }
}

impl From<PubKey> for ed25519_dalek::PublicKey {
    fn from(val: PubKey) -> Self {
        ed25519_dalek::PublicKey::from_bytes(&val.0).unwrap()
    }
}

impl From<near_sdk::PublicKey> for PubKey {
    fn from(pubkey: near_sdk::PublicKey) -> Self {
        use near_sdk::CurveType;
        match pubkey.curve_type() {
            CurveType::ED25519 => {
                let pubkey = pubkey.as_bytes();
                assert_eq!(pubkey.len(), ed25519_dalek::PUBLIC_KEY_LENGTH + 1);
                let mut res = [0; ed25519_dalek::PUBLIC_KEY_LENGTH];
                res.copy_from_slice(&pubkey[1..]);
                PubKey(res)
            }
            CurveType::SECP256K1 => panic!("wrong pubkey type"),
        }
    }
}

impl std::convert::TryFrom<PubKey> for near_sdk::PublicKey {
    type Error = <near_sdk::PublicKey as TryFrom<Vec<u8>>>::Error;
    fn try_from(pubkey: PubKey) -> Result<Self, Self::Error> {
        let mut res = Vec::with_capacity(ed25519_dalek::PUBLIC_KEY_LENGTH + 1);
        res.push(near_sdk::CurveType::ED25519 as u8);
        res.extend_from_slice(&pubkey.0);
        near_sdk::PublicKey::try_from(res)
    }
}

/// Signature in serialized form.
///
/// Has a total size of 64 bytes.
#[derive(near_sdk::serde::Serialize, near_sdk::serde::Deserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
#[serde(transparent)]
pub struct Sign(
    #[serde(with = "serde_big_array::BigArray")]
    //
    pub [u8; 64],
);

impl From<ed25519_dalek::Signature> for Sign {
    fn from(sign: ed25519_dalek::Signature) -> Self {
        Sign(sign.to_bytes())
    }
}

/// Signature in serialized form, formed from a prehashed message.  
/// Note that this Signature itself is not "prehashed".
///
/// A [`Sign`] that is formed from a non-prehashed message _m_ will
/// use the `Ed25519` algorithm, while a [`SignPrehashed`] that is
/// formed from a prehashed _m_ will use the `Ed25519ph` algorithm.  
/// This results in different and incompatible signatures. The
/// verification also uses different algorithms, so `Ed25519` cannot
/// be used to verify a [`SignPrehashed`] and `Ed25519ph` cannot be
/// used to verify a [`Sign`].
///
/// Note that in case of `ecdsa-secp256k1`, the same algorithm is used
/// (ie. the hashing is "transparent") and the resulting signatures
/// _are_ the same.
///
/// Has a total size of 64 bytes.
#[derive(near_sdk::serde::Serialize, near_sdk::serde::Deserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
#[serde(transparent)]
pub struct SignPrehashed(
    #[serde(with = "serde_big_array::BigArray")]
    //
    pub [u8; 64],
);

impl From<ed25519_dalek::Signature> for SignPrehashed {
    fn from(sign: ed25519_dalek::Signature) -> Self {
        SignPrehashed(sign.to_bytes())
    }
}
