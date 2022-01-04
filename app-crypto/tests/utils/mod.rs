#![allow(dead_code)]
#![allow(unused_imports)]

pub use near_sdk::json_types::{Base64VecU8, U64};
use near_sdk::AccountId;
use near_sdk_sim::transaction::ExecutionStatus;
use near_sdk_sim::{deploy, init_simulator, ContractAccount, ExecutionResult, UserAccount};
use near_units::parse_near;
use nearapps_crypto::CryptoContract;

pub mod _secp256k1;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    CRYPTO_WASM_BYTES => "../res/nearapps_crypto.wasm",
}

pub fn setup_crypto(root: &UserAccount) -> ContractAccount<CryptoContract> {
    let contract = deploy!(
        contract: CryptoContract,
        contract_id: "executor".to_string(),
        bytes: &CRYPTO_WASM_BYTES,
        signer_account: root,
        deposit: parse_near!("200 N"),
        // init_method: new(root.account_id())
    );
    contract
}

fn user(id: u32) -> AccountId {
    format!("user{}", id).parse().unwrap()
}
