use nearapps_contracts::signing::{SecKey, SignCompact};
use secp256k1 as s;

pub fn secp256k1_init_all() -> s::Secp256k1<s::All> {
    s::Secp256k1::new()
}

pub fn secp256k1_init_sign_only() -> s::Secp256k1<s::SignOnly> {
    s::Secp256k1::signing_only()
}

pub fn secp256k1_init_verify_only() -> s::Secp256k1<s::VerifyOnly> {
    s::Secp256k1::verification_only()
}

pub fn gen_pubkey(seckey: SecKey) -> secp256k1::key::PublicKey {
    let secp = secp256k1_init_sign_only();
    secp256k1::key::PublicKey::from_secret_key(&secp, &s::SecretKey::from_slice(&seckey.0).unwrap())
}

// TODO: make sure that all signatures created already are in lower-S
// form.
// According to the link below, they are.
// This means that the explicit call to normalize is not needed.
// https://github.com/bitcoin-core/secp256k1/blob/fecf436d5327717801da84beb3066f5a9b80ea8e/src/ecdsa_impl.h#L305
//
/// Signs a msg's hash using [`ecdsa` on `secp256k1`](s::Secp256k1::sign()).
///
/// Signing is deterministic and the "pseudo-random" value `k` depends
/// only on the hash of the combination of `seckey` and `msg_hash`.
/// See [rfc6979](https://datatracker.ietf.org/doc/html/rfc6979) for more info.
///
/// To avoid generating signatures that may have malleability issues,
/// they explicitly [normalized](s::Signature::normalize_s()) to
/// a lower-S form.
/// See [bitcoin-core/secp256k1/ecdsa_signature_normalize()](https://github.com/bitcoin-core/secp256k1/blob/2e5e4b67dfb67950563c5f0ab2a62e25eb1f35c5/include/secp256k1.h#L551) for more info.
///
/// Returns the signature in serialized compact form.
pub fn ecdsa_secp256k1_sign_hashed(
    seckey: SecKey,
    msg_hash: bitcoin_hashes::sha256::Hash,
) -> SignCompact {
    use bitcoin_hashes::Hash;
    let msg_hash = bitcoin_hashes::sha256::Hash::from_slice(&msg_hash)
        .unwrap_or_else(|_| panic!("ERR_HASH_BAD_LEN"));
    let secp = secp256k1_init_sign_only();

    let mut sig = secp.sign(
        //
        &s::Message::from_slice(&msg_hash).unwrap(),
        &s::SecretKey::from_slice(&seckey.0).unwrap(),
    );
    sig.normalize_s();
    SignCompact(sig.serialize_compact())
}
