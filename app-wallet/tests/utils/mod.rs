#![allow(dead_code)]

pub use near_sdk::json_types::{Base64VecU8, U64};
use near_sdk::{AccountId, Gas};
use near_sdk_sim::transaction::ExecutionStatus;
use near_sdk_sim::{deploy, init_simulator, ContractAccount, ExecutionResult, UserAccount};

use nearapps_near_ext::YOTTA;
use nearapps_wallet::{AccountManagerContract, AllowedCalls, Defaults};

pub const DEFAULT_GAS: u64 = 300_000_000_000_000;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    WALLET_WASM_BYTES => "../res/nearapps_wallet.wasm",
    TESTNET_WASM_BYTES => "../res/testnet.wasm",
}

#[allow(clippy::identity_op)]
pub fn setup_testnet(root: &UserAccount) -> AccountId {
    let testnet: AccountId = "testnet".parse().unwrap();
    root.deploy(&TESTNET_WASM_BYTES, testnet.clone(), 100 * YOTTA);
    testnet
}

#[allow(clippy::identity_op)]
pub fn setup_wallet(root: &UserAccount) -> ContractAccount<AccountManagerContract> {
    deploy!(
        contract: AccountManagerContract,
        contract_id: "wallet".to_string(),
        bytes: &WALLET_WASM_BYTES,
        signer_account: root,
        deposit: 200 * YOTTA,
        init_method: new(root.account_id())
    )
}

// #[allow(clippy::identity_op)]
// pub fn setup_wallet(root: &UserAccount) -> ContractAccount<AccountManagerContract> {
//     let allowed_call = &AllowedCalls {
//         allowance: Some(300 * TERA as u128).map(Into::into),
//         receiver_id: "counter".parse().unwrap(),
//         method_names: vec!["increment".to_string()],
//     };
//     let defaults = Defaults {
//         initial_amount: (1 * YOTTA / 100).into(), // 0.01 N
//         allowance: (1 * MEGA_TERA).into(),
//         allowed_calls: vec![allowed_call.clone()],
//     };
//     deploy!(
//         contract: AccountManagerContract,
//         contract_id: "wallet".to_string(),
//         bytes: &WALLET_WASM_BYTES,
//         signer_account: root,
//         deposit: 200 * YOTTA,
//         init_method: new(root.account_id(), defaults)
//     )
// }

pub fn user(id: u32) -> AccountId {
    format!("user{}", id).parse().unwrap()
}
