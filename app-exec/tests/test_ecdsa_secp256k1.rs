#![allow(clippy::ref_in_deref)]
#![cfg(feature = "crypto")]

use crate::utils::setup_exec;
use near_sdk_sim::{call, init_simulator};
use nearapps_near_ext::TERA;

use utils::_secp256k1;

mod utils;

#[test]
fn test_ecdsa_secp256k1() {
    use nearapps_exec::{crypto::ecdsa_secp256k1 as ec, hash};

    let root = init_simulator(None);
    let contract = setup_exec(&root);

    let seckey = [
        59, 148, 11, 85, 134, 130, 61, 253, 2, 174, 59, 70, 27, 180, 51, 107, 94, 203, 174, 253,
        102, 39, 170, 146, 46, 252, 4, 143, 236, 12, 136, 28,
    ];

    let seckey = ec::types::SecKey(seckey);

    // generates the pubkey on the contract
    let pubkey_compressed: ec::types::PubKeyCompressed = {
        let res = call!(&root, contract.secp256k1_pubkey_compressed(seckey.clone()));
        assert!(res.gas_burnt().0 < 33 * TERA);
        res.unwrap_json()
    };
    let pubkey_compressed = &pubkey_compressed;
    let expected_pubkey = [
        2, 29, 21, 35, 7, 198, 183, 43, 14, 208, 65, 139, 14, 112, 205, 128, 231, 245, 41, 91, 141,
        134, 245, 114, 45, 63, 82, 19, 251, 210, 57, 79, 54,
    ];
    assert_eq!(pubkey_compressed.0, expected_pubkey);

    let pubkey_uncompressed: ec::types::PubKeyUncompressed = {
        let res = call!(
            &root,
            contract.secp256k1_pubkey_uncompressed(seckey.clone())
        );
        assert!(res.gas_burnt().0 < 33 * TERA);
        res.unwrap_json()
    };
    let pubkey_uncompressed = &pubkey_uncompressed;

    let msg = "This is some message";

    // sign the message on the contract
    let sign: ec::types::SignCompact = {
        let res = call!(
            &root,
            contract.ecdsa_secp256k1_sign(seckey.clone(), msg.to_string())
        );
        assert!(res.gas_burnt().0 < 45 * TERA);
        res.unwrap_json()
    };
    let sign = &sign;

    // copy and change the resulting signature
    // (so we have one that is wrong)
    let bad_sign = {
        let mut bad_sign = sign.clone();
        bad_sign.0[0] += 1;
        bad_sign
    };
    let bad_sign = &bad_sign;
    assert!(sign != bad_sign);

    // ok: verifies the msg's signature with the pubkey
    let verify1: bool = {
        let res = call!(
            &root,
            contract.ecdsa_secp256k1_verify_compressed_msg(
                pubkey_compressed.clone(),
                sign.clone(),
                msg.to_string()
            )
        );
        assert!(res.gas_burnt().0 < 65 * TERA);
        res.unwrap_json()
    };
    assert!(verify1);

    // fail: pass in the wrong signature
    let bad_verify1: bool = {
        let res = call!(
            &root,
            contract.ecdsa_secp256k1_verify_compressed_msg(
                pubkey_compressed.clone(),
                bad_sign.clone(),
                msg.to_string()
            )
        );
        assert!(res.gas_burnt().0 < 65 * TERA);
        res.unwrap_json()
    };
    assert!(!bad_verify1);

    // ok: verifies without sending the msg (only it's hash)
    let verify2: bool = {
        // ok: get the hash.
        // could be generated locally, without using the contract
        let res = call!(&root, contract.hash_sha256(msg.as_bytes().to_vec()));
        assert!(res.gas_burnt().0 < 3 * TERA);
        let msg_hash: hash::Sha256 = res.unwrap_json();

        let res = call!(
            &root,
            contract.ecdsa_secp256k1_verify_prehashed_compressed(
                pubkey_compressed.clone(),
                sign.clone(),
                msg_hash
            )
        );
        assert!(res.gas_burnt().0 < 65 * TERA);
        res.unwrap_json()
    };
    assert!(verify2);

    // fail: same, but pass in the wrong signature
    let bad_verify2: bool = {
        // ok: get the hash.
        // could be generated locally, without using the contract
        let res = call!(&root, contract.hash_sha256(msg.as_bytes().to_vec()));
        assert!(res.gas_burnt().0 < 3 * TERA);
        let msg_hash: hash::Sha256 = res.unwrap_json();

        let res = call!(
            &root,
            contract.ecdsa_secp256k1_verify_prehashed_compressed(
                pubkey_compressed.clone(),
                bad_sign.clone(),
                msg_hash
            )
        );
        assert!(res.gas_burnt().0 < 65 * TERA);
        res.unwrap_json()
    };
    assert!(!bad_verify2);

    // ok: verifies the msg's signature with the uncompressed pubkey
    let verify3: bool = {
        let res = call!(
            &root,
            contract.ecdsa_secp256k1_verify_uncompressed_msg(
                pubkey_uncompressed.clone(),
                sign.clone(),
                msg.to_string()
            )
        );
        assert!(res.gas_burnt().0 < 65 * TERA);
        res.unwrap_json()
    };
    assert!(verify3);

    // fail: same, but pass in the wrong signature
    let bad_verify3: bool = {
        let res = call!(
            &root,
            contract.ecdsa_secp256k1_verify_uncompressed_msg(
                pubkey_uncompressed.clone(),
                bad_sign.clone(),
                msg.to_string()
            )
        );
        assert!(res.gas_burnt().0 < 65 * TERA);
        res.unwrap_json()
    };
    assert!(!bad_verify3);

    // ok: verifies the msg's signature with the uncompressed pubkey
    // verifies without sending the msg (only it's hash)
    let verify4: bool = {
        // ok: get the hash.
        // could be generated locally, without using the contract
        let res = call!(&root, contract.hash_sha256(msg.as_bytes().to_vec()));
        assert!(res.gas_burnt().0 < 3 * TERA);
        let msg_hash: hash::Sha256 = res.unwrap_json();

        let res = call!(
            &root,
            contract.ecdsa_secp256k1_verify_prehashed_uncompressed(
                pubkey_uncompressed.clone(),
                sign.clone(),
                msg_hash
            )
        );
        assert!(res.gas_burnt().0 < 65 * TERA);
        res.unwrap_json()
    };
    assert!(verify4);

    // fail: same, but pass in the wrong signature
    let bad_verify4: bool = {
        // ok: get the hash.
        // could be generated locally, without using the contract
        let res = call!(&root, contract.hash_sha256(msg.as_bytes().to_vec()));
        assert!(res.gas_burnt().0 < 3 * TERA);
        let msg_hash: hash::Sha256 = res.unwrap_json();

        let res = call!(
            &root,
            contract.ecdsa_secp256k1_verify_prehashed_uncompressed(
                pubkey_uncompressed.clone(),
                bad_sign.clone(),
                msg_hash
            )
        );
        assert!(res.gas_burnt().0 < 65 * TERA);
        res.unwrap_json()
    };
    assert!(!bad_verify4);

    // ok: sanity assertions with rust-secp256k1
    // (this is linked to the same library that bitcoin uses)
    {
        use _secp256k1 as s;
        use nearapps_exec::hash::Sha256;

        // ok: pubkeys match
        let pubkey2 = s::gen_pubkey(seckey.clone());
        assert_eq!(pubkey_compressed.0, pubkey2.serialize());

        let msg_hash: Sha256 =
            call!(&root, contract.hash_sha256(msg.as_bytes().to_vec())).unwrap_json();

        // ok: sha256 match (using sha2 library)
        {
            use digest::Digest;
            let mut sha2_hash = sha2::Sha256::new();
            sha2_hash.update(msg.as_bytes());
            let sha2_hash = sha2_hash.finalize();
            assert_eq!(sha2_hash[..], msg_hash.0);
        }

        let msg_hash = bitcoin_hashes::Hash::from_inner(msg_hash.0);

        // ok: it's signing match
        let sign2 = s::ecdsa_secp256k1_sign_hashed(seckey, msg_hash);
        assert_eq!(sign, &sign2);
    }
}
