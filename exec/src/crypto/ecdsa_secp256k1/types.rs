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

impl From<k256::PublicKey> for PubKeyCompressed {
    fn from(pubkey: k256::PublicKey) -> Self {
        use k256::elliptic_curve::group::GroupEncoding;
        let pubkey = pubkey.as_affine().to_bytes();
        assert_eq!(pubkey.as_slice().len(), 33);
        let mut res = [0; 33];
        res.copy_from_slice(&pubkey.as_slice()[0..33]);
        PubKeyCompressed(res)
    }
}

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

impl From<k256::PublicKey> for PubKeyUncompressed {
    fn from(pubkey: k256::PublicKey) -> Self {
        use k256::elliptic_curve::sec1::ToEncodedPoint;
        let affine = pubkey.as_affine();
        let compress = false;
        let pubkey = affine.to_encoded_point(compress);
        let pubkey = pubkey.as_bytes();
        assert_eq!(pubkey.len(), 65);
        let mut res = [0; 65];
        res.copy_from_slice(&pubkey[0..65]);
        PubKeyUncompressed(res)
    }
}

// TODO: check if the order is x,y or y,x
//
/// Public Key serialized in extended form.  
/// Contains both `x` and `y` values.
///
/// Has a total size of 64 bytes, containing:
///
/// - `x` (32-bytes).
/// - `y` (32-bytes).
///
/// This is similar to [`PubKeyUncompressed`] except that
/// there is no `header`.  
/// When re-adding the header, it is always assumed to be `0x04`, as
/// demonstrated in [nearcore](https://github.com/near/nearcore/blob/22e997b559a75a07ed8cd2781e9acc0758d16aaf/core/crypto/src/signature.rs#L751).
#[derive(near_sdk::serde::Serialize, near_sdk::serde::Deserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
#[serde(transparent)]
pub struct PubKeyUncompressedNoHeader(
    #[serde(with = "serde_big_array::BigArray")]
    //
    pub [u8; 64],
);

impl From<k256::PublicKey> for PubKeyUncompressedNoHeader {
    fn from(pubkey: k256::PublicKey) -> Self {
        use k256::elliptic_curve::sec1::ToEncodedPoint;
        let affine = pubkey.as_affine();
        let compress = false;
        let pubkey = affine.to_encoded_point(compress);
        let pubkey = pubkey.as_bytes();
        assert_eq!(pubkey.len(), 65);

        let mut res = [0; 64];
        // skips the header (first byte)
        res.copy_from_slice(&pubkey[1..65]);
        PubKeyUncompressedNoHeader(res)
    }
}

impl From<near_sdk::PublicKey> for PubKeyUncompressedNoHeader {
    fn from(pubkey: near_sdk::PublicKey) -> Self {
        use near_sdk::CurveType;
        match pubkey.curve_type() {
            CurveType::ED25519 => panic!("wrong pubkey type"),
            CurveType::SECP256K1 => {
                let pubkey = pubkey.as_bytes();
                assert_eq!(pubkey.len(), 1 + 64);
                let mut res = [0; 64];
                res.copy_from_slice(&pubkey[1..]);
                Self(res)
            }
        }
    }
}

impl From<PubKeyUncompressedNoHeader> for PubKeyUncompressed {
    fn from(pubkey: PubKeyUncompressedNoHeader) -> Self {
        // re-insert the header (0x04) as the first byte
        let mut pubkey_raw = [0x04; 1 + 64];
        (&mut pubkey_raw[1..]).copy_from_slice(&pubkey.0);
        PubKeyUncompressed(pubkey_raw)
    }
}

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

impl From<k256::ecdsa::Signature> for SignCompact {
    fn from(sign: k256::ecdsa::Signature) -> Self {
        use k256::ecdsa::signature::Signature;
        let mut res = [0u8; 64];
        assert_eq!(sign.as_bytes().len(), 64);
        res.copy_from_slice(&sign.as_bytes()[0..64]);
        SignCompact(res)
    }
}

/// Recoverable signature in serialized form.
///
/// Has a total size of 65 bytes, containing:
///
/// - `r` (32-bytes big-endian);
/// - `s` (32-bytes big-endian).
/// - `recovery_id` (1-byte)
///
/// See also: [`k256::ecdsa::recoverable::Signature`].
#[derive(near_sdk::serde::Serialize, near_sdk::serde::Deserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
#[serde(transparent)]
pub struct SignRecoverable(
    #[serde(with = "serde_big_array::BigArray")]
    //
    pub [u8; 65],
);

impl From<k256::ecdsa::recoverable::Signature> for SignRecoverable {
    fn from(sign: k256::ecdsa::recoverable::Signature) -> Self {
        use k256::ecdsa::signature::Signature;
        let mut res = [0u8; 65];
        assert_eq!(sign.as_bytes().len(), 65);
        res.copy_from_slice(&sign.as_bytes()[0..65]);
        SignRecoverable(res)
    }
}

impl From<SignRecoverable> for SignCompact {
    fn from(sign: SignRecoverable) -> Self {
        use ecdsa::signature::Signature;
        let sign = k256::ecdsa::recoverable::Signature::from_bytes(&sign.0).unwrap();
        let sign: k256::ecdsa::Signature = sign.into();
        sign.into()
    }
}
