#![allow(clippy::ref_in_deref)]

use crate::utils::{setup_exec, TERA};
use near_sdk_sim::{call, init_simulator};

mod utils;

#[cfg(feature = "crypto")]
#[allow(clippy::zero_prefixed_literal)]
#[test]
fn test_eddsa_ed25519() {
    use nearapps_exec::{crypto::eddsa_ed25519 as ed, hash};

    let root = init_simulator(None);
    let contract = setup_exec(&root);

    // the msg is an empty string
    let msg = "";
    let msg_bytes: &[u8] = msg.as_bytes();

    let seckey_bytes: [u8; 32] = [
        062, 070, 027, 163, 092, 182, 011, 003, 077, 234, 098, 004, 011, 127, 079, 228, 243, 187,
        150, 073, 201, 137, 076, 022, 085, 251, 152, 002, 241, 042, 072, 054,
    ];

    let expected_pubkey_bytes: [u8; 32] = [
        130, 039, 155, 015, 062, 076, 188, 063, 124, 122, 026, 251, 233, 253, 225, 220, 014, 041,
        166, 120, 108, 035, 254, 077, 160, 083, 172, 058, 219, 042, 086, 120,
    ];

    // Signature with the above keypair of a blank message.
    let expected_sign_bytes: [u8; 64] = [
        010, 126, 151, 143, 157, 064, 047, 001, 196, 140, 179, 058, 226, 152, 018, 102, 160, 123,
        080, 016, 210, 086, 196, 028, 053, 231, 012, 157, 169, 019, 158, 063, 045, 154, 238, 007,
        053, 185, 227, 229, 079, 108, 213, 080, 124, 252, 084, 167, 216, 085, 134, 144, 129, 149,
        041, 081, 063, 120, 126, 100, 092, 059, 050, 011,
    ];

    // ok: get the pub key using the contract
    let pubkey: ed::types::PubKey = {
        let seckey = ed::types::SecKey(seckey_bytes);
        let res = call!(&root, contract.ed25519_pubkey(seckey));
        assert!(res.gas_burnt().0 < 13 * TERA);
        res.unwrap_json()
    };
    assert_eq!(pubkey.0, expected_pubkey_bytes);

    // ok: sign the msg using the contract
    let sign: ed::types::Sign = {
        let seckey = ed::types::SecKey(seckey_bytes);
        let res = call!(&root, contract.eddsa_ed25519_sign(seckey, msg.to_string()));
        assert!(res.gas_burnt().0 < 23 * TERA);
        res.unwrap_json()
    };
    assert_eq!(sign.0, expected_sign_bytes);

    // ok: get the msg hash (to be used by prehashed calls)
    let msg_hash: hash::Sha512 = {
        let res = call!(&root, contract.hash_sha512(msg.as_bytes().to_vec()));
        assert!(res.gas_burnt().0 < 3 * TERA);
        res.unwrap_json()
    };
    // ok: confirms that it matches with directly using sha2 library
    {
        use digest::Digest;
        let mut sha2_hash = sha2::Sha512::new();
        sha2_hash.update(msg.as_bytes());
        let sha2_hash = sha2_hash.finalize();
        assert_eq!(&sha2_hash[..], &msg_hash.0);
    }

    // ok: sign the prehashed version
    // note: this results in a different signature from the normal sign!
    let prehashed_sign: ed::types::SignPrehashed = {
        let seckey = ed::types::SecKey(seckey_bytes);

        let res = call!(
            &root,
            contract.eddsa_ed25519_sign_prehashed(seckey, msg_hash.clone(), None)
        );
        assert!(res.gas_burnt().0 < 24 * TERA);
        res.unwrap_json()
    };

    // ok: creates a bad sign, to test failure cases
    let bad_sign = {
        let mut bad_sign = sign.clone();
        bad_sign.0[0] += 1;
        bad_sign
    };
    assert!(sign != bad_sign);

    // ok: checks signature
    let verify1: bool = {
        let res = call!(
            &root,
            contract.eddsa_ed25519_verify_msg(pubkey.clone(), sign.clone(), msg.to_string())
        );
        assert!(res.gas_burnt().0 < 35 * TERA);
        res.unwrap_json()
    };
    assert!(verify1);

    // fail: pass in wrong signature
    let bad_verify1: bool = {
        let res = call!(
            &root,
            contract.eddsa_ed25519_verify_msg(pubkey.clone(), bad_sign, msg.to_string())
        );
        assert!(res.gas_burnt().0 < 35 * TERA);
        res.unwrap_json()
    };
    assert!(!bad_verify1);

    // ok: checks prehashed signature
    let verify2: bool = {
        // ok: checks without using the contract
        let sha2_verify: bool = {
            use sha2::Digest;
            let pubkey = ed25519_dalek::PublicKey::from_bytes(&pubkey.0).unwrap();
            let prehashed_sign = ed25519_dalek::Signature::from_bytes(&prehashed_sign.0).unwrap();
            let mut digest = sha2::Sha512::new();
            digest.update(msg);
            pubkey
                .verify_prehashed(digest, None, &prehashed_sign)
                .is_ok()
        };
        assert!(sha2_verify);

        // ok: checks by using the contract
        let res = call!(
            &root,
            contract.eddsa_ed25519_verify_prehashed(pubkey, prehashed_sign, msg_hash, None)
        );
        assert!(res.gas_burnt().0 < 35 * TERA);
        res.unwrap_json()
    };
    assert!(verify2);

    // sanity assertions with ed25519-dalek (without using the contract)
    {
        use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature};

        let seckey: SecretKey = SecretKey::from_bytes(&seckey_bytes).unwrap();
        let pubkey: PublicKey = PublicKey::from_bytes(&expected_pubkey_bytes).unwrap();

        let keypair: Keypair = Keypair {
            secret: seckey,
            public: pubkey,
        };

        let sig1: Signature = Signature::from_bytes(&expected_sign_bytes).unwrap();
        let sig2: Signature = {
            use ed25519_dalek::Signer;
            keypair.sign(msg_bytes)
        };

        assert_eq!(sig1, sig2);
        assert_eq!(sig1.to_bytes(), expected_sign_bytes);
        assert_eq!(sig1.to_bytes(), sign.0);
        assert!(keypair.verify(msg_bytes, &sig2).is_ok());
    }
}

// test based on data from:
// https://github.com/near/near-api-js/blob/7ea21f330af1a00543b1ae655761a98820f6a368/test/key_pair.test.js#L5
#[cfg(feature = "crypto")]
#[allow(unused_variables)]
#[test]
fn test_near_verification() {
    let msg = "message";
    let pubkey = "ed25519:AYWv9RAN1hpSQA4p1DLhCNnpnNXwxhfH9qeHN8B4nJ59";
    let sign =
        "26gFr4xth7W9K7HPWAxq3BLsua8oTy378mC1MYFiEXHBBpeBjP8WmJEJo8XTBowetvqbRshcQEtBUdwQcAqDyP8T";

    // test using the contract / call!()
    {
        let root = init_simulator(None);
        let contract = setup_exec(&root);

        // ok: signature verified
        let res = call!(
            &root,
            contract.verify_msg(
                sign.to_string().into(),
                pubkey.to_string().into(),
                msg.to_string()
            )
        );
        res.assert_success();
        let verify: bool = res.unwrap_json();
        assert!(verify);

        // fail: different message
        let verify2: bool = call!(
            &root,
            contract.verify_msg(
                sign.to_string().into(),
                pubkey.to_string().into(),
                msg.to_string() + "0"
            )
        )
        .unwrap_json();
        assert!(!verify2);

        // fail: different signature
        let sign3 = "3".to_string() + &sign.chars().skip(1).collect::<String>();
        let verify3: bool = call!(
            &root,
            contract.verify_msg(sign3.into(), pubkey.to_string().into(), msg.to_string())
        )
        .unwrap_json();
        assert!(!verify3);

        // fail: different pubkey
        let pubkey4 = "ed25519:9m52dqbkTFJWDxb3oSZ5EuHav1YaR8PbCTux59q4xRwM";
        let verify4: bool = call!(
            &root,
            contract.verify_msg(
                sign.to_string().into(),
                pubkey4.to_string().into(),
                msg.to_string()
            )
        )
        .unwrap_json();
        assert!(!verify4);
    }

    // direct test (no contract / call!() involved)
    {
        // for the Ft.. part,
        // must skip the first byte as it's used to indicate the curve type
        let pubkey: near_sdk::PublicKey = pubkey.parse().unwrap();
        let pubkey = &pubkey.as_bytes()[1..];

        let mut sign_res = [0u8; 64];
        let _sign = near_sdk::bs58::decode(sign.as_bytes())
            .into(&mut sign_res)
            .unwrap();
        let sign = sign_res;

        let msg_hash = {
            use digest::Digest;
            let mut sha2_hash = sha2::Sha256::new();
            sha2_hash.update(msg.as_bytes());
            let sha2_hash = sha2_hash.finalize();
            sha2_hash.to_vec()
        };

        {
            use ed25519_dalek::{PublicKey, Signature};
            let pubkey = PublicKey::from_bytes(pubkey).unwrap();
            let sign = Signature::from_bytes(&sign).unwrap();

            use ecdsa::signature::Verifier;
            assert!(pubkey.verify(&msg_hash, &sign).is_ok());
        }
    }
}

// let msg = r#"{"contract_id":"testnet","method_name":"create_account","args":{"new_account_id":"dasasdasd.testnet","new_public_key":"ed25519:FtB84LCX12AmjovMjXxq86sA8pdbMnixk7aixudPNNuN"}}"#;
// let expected_hash = "7146bfc46487e635c5dbdc2976dc0cda1eac1c36fbdfe7481ff41c70196b9081";
// let pubkey = "ed25519:FtB84LCX12AmjovMjXxq86sA8pdbMnixk7aixudPNNuN";
// let sign = "679dbb85416ee137cdbb8327b630368dbd89fae811ccae47b39ca2082fd62a966ac72c9dabbd55f92d5c1e6a696a7e640e5d60ff10a6428741f04a92746a7e0a";
