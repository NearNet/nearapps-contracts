/// Private Key value.
///
/// Has a total size of 32 bytes.
#[derive(near_sdk::serde::Serialize, near_sdk::serde::Deserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
#[serde(transparent)]
pub struct SecKey(
    #[serde(with = "serde_big_array::BigArray")]
    //
    pub [u8; 32],
);

/// Public Key serialized in compressed form.  
/// Instead of having both `x` and `y` values, only `x` is present,
/// as `y` can be derived from that.
///
/// Has a total size of 33 bytes, containing:
///
/// - `header` (1-byte);
///   - If `y` was even, the `header` is `0x02`;
///   - If `y` was odd, the `header` is `0x03`.
/// - `x` (32-bytes).
#[derive(near_sdk::serde::Serialize, near_sdk::serde::Deserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
#[serde(transparent)]
pub struct PubKeyCompressed(
    #[serde(with = "serde_big_array::BigArray")]
    //
    pub [u8; 33],
);

// TODO: check if the order is x,y or y,x
//
/// Public Key serialized in extended form.  
/// Contains both `x` and `y` values.
///
/// Has a total size of 65 bytes, containing:
///
/// - `header` (1-byte, with value `0x04`);
/// - `x` (32-bytes).
/// - `y` (32-bytes).
#[derive(near_sdk::serde::Serialize, near_sdk::serde::Deserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
#[serde(transparent)]
pub struct PubKeyUncompressed(
    #[serde(with = "serde_big_array::BigArray")]
    //
    pub [u8; 65],
);

/// Signature in serialized compact form.
///
/// Has a total size of 64 bytes, containing:
///
/// - `r` (32-bytes big-endian);
/// - `s` (32-bytes big-endian).
///
/// See also: [`k256::ecdsa::Signature`].
#[derive(near_sdk::serde::Serialize, near_sdk::serde::Deserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
#[serde(transparent)]
pub struct SignCompact(
    #[serde(with = "serde_big_array::BigArray")]
    //
    pub [u8; 64],
);
