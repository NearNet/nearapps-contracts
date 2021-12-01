#![allow(clippy::ref_in_deref)]

use near_sdk_sim::call;

mod utils;

use crate::utils::*;

#[test]
fn test_sign() {
    use nearapps_contracts::signing::{PubKey, SecKey, SignCompact};

    let (root, contract) = setup_contract();

    let seckey = [
        59, 148, 11, 85, 134, 130, 61, 253, 2, 174, 59, 70, 27, 180, 51, 107, 94, 203, 174, 253,
        102, 39, 170, 146, 46, 252, 4, 143, 236, 12, 136, 28,
    ];

    let seckey = SecKey(seckey);

    let pubkey: PubKey = {
        let res = call!(&root, contract.secp256k1_pubkey(seckey.clone()));
        assert!(res.gas_burnt().0 < 33 * MEGA * MEGA);
        res.unwrap_json()
    };
    let expected_pubkey = [
        2, 29, 21, 35, 7, 198, 183, 43, 14, 208, 65, 139, 14, 112, 205, 128, 231, 245, 41, 91, 141,
        134, 245, 114, 45, 63, 82, 19, 251, 210, 57, 79, 54,
    ];
    assert_eq!(pubkey.0, expected_pubkey);

    let msg = "This is some message";

    let sign: SignCompact = {
        let res = call!(
            &root,
            contract.ecdsa_secp256k1_sign(seckey.clone(), msg.to_string())
        );
        assert!(res.gas_burnt().0 < 45 * MEGA * MEGA);
        res.unwrap_json()
    };

    let verify: bool = {
        let res = call!(
            &root,
            contract.ecdsa_secp256k1_verify(pubkey.clone(), sign.clone(), msg.to_string())
        );
        assert!(res.gas_burnt().0 < 65 * MEGA * MEGA);
        res.unwrap_json()
    };
    assert!(verify);

    // sanity assertions with rust-secp256k1
    {
        use _secp256k1 as s;
        use nearapps_contracts::signing::Sha256;

        let pubkey2 = s::gen_pubkey(seckey.clone());
        assert_eq!(pubkey.0, pubkey2.serialize());

        let msg_hash: Sha256 =
            call!(&root, contract.hash_sha256(msg.as_bytes().to_vec())).unwrap_json();
        let msg_hash = bitcoin_hashes::Hash::from_inner(msg_hash.0);

        let sign2 = s::ecdsa_secp256k1_sign_hashed(seckey, msg_hash);
        assert_eq!(&sign, &sign2);
    }
}
