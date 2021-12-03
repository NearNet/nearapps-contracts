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
