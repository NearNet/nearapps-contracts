#![allow(clippy::ref_in_deref)]
#![allow(clippy::identity_op)]

pub use near_sdk::json_types::{Base64VecU8, U64};
use near_sdk_sim::{call, deploy, init_simulator, view, ContractAccount, UserAccount};

pub const DEFAULT_GAS: u64 = 300_000_000_000_000;

pub mod utils;

use crate::utils::{user, AssertFailure, MEGA_TERA, TERA, YOTTA};
use nearapps_wallet::AccountConfig;

#[ignore]
#[test]
fn test_wallet() {
    let root = init_simulator(None);
    let wallet = utils::setup_wallet(&root);

    let users: Vec<_> = (0..10)
        .into_iter()
        .map(|i| root.create_user(user(i), 100 * YOTTA))
        .collect();

    let created_01: &near_sdk::AccountId = &"created-01".parse().unwrap();

    let config = AccountConfig {
        account_id: created_01.clone(),
        user_public_key: pubkey(),
        initial_amount: Some(1.into()),
    };

    // errors: not within a catch_unsafe_unwind scope
    let res = root.function_call(
        //
        wallet.contract.create_subaccount(config, None),
        15 * TERA,
        1 * YOTTA / 100, // 0.01 N
    );
    res.assert_success();
    assert!(res.gas_burnt().0 < 4 * TERA);
}

fn pubkey() -> near_sdk::PublicKey {
    use std::convert::TryInto;

    use ed25519_dalek::{PublicKey, SecretKey};
    let seckey_bytes: [u8; 32] = [
        62, 70, 27, 163, 92, 182, 11, 3, 77, 234, 98, 4, 11, 127, 79, 228, 243, 187, 150, 73, 201,
        137, 76, 22, 85, 251, 152, 2, 241, 42, 72, 54,
    ];

    let secret: SecretKey = SecretKey::from_bytes(&seckey_bytes).unwrap();
    let public: PublicKey = PublicKey::from(&secret);

    use nearapps_exec::crypto::eddsa_ed25519::types::PubKey;
    let public: PubKey = public.into();

    let public = public.try_into().unwrap();

    let pk = near_sdk::serde_json::to_string(&pubkey()).unwrap();
    assert_eq!(&pk, "ed25519:9m52dqbkTFJWDxb3oSZ5EuHav1YaR8PbCTux59q4xRwM");

    public
}
