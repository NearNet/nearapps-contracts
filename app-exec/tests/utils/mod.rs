#![allow(dead_code)]

pub use near_sdk::json_types::{Base64VecU8, U64};
use near_sdk::AccountId;
use near_sdk_sim::{deploy, ContractAccount, UserAccount};
use nearapps_counter::CounterContract;
use nearapps_exec::ExecutorContract;
use nearapps_near_ext::YOTTA;

#[cfg(feature = "crypto")]
pub mod _secp256k1;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    EXEC_WASM_BYTES => "../res/nearapps_exec.wasm",
    COUNTER_WASM_BYTES => "../res/nearapps_counter.wasm",
}

pub type Contract = ContractAccount<ExecutorContract>;

pub fn setup_exec(root: &UserAccount) -> Contract {
    let contract = deploy!(
        contract: ExecutorContract,
        contract_id: "executor".to_string(),
        bytes: &EXEC_WASM_BYTES,
        signer_account: root,
        deposit: 200 * YOTTA,
        init_method: new(root.account_id())
    );
    contract
}

pub fn setup_counter(root: &UserAccount) -> ContractAccount<CounterContract> {
    deploy!(
        contract: CounterContract,
        contract_id: "counter".to_string(),
        bytes: &COUNTER_WASM_BYTES,
        signer_account: root,
        deposit: 200 * YOTTA,
        // init_method: new()
    )
}

fn user(id: u32) -> AccountId {
    format!("user{}", id).parse().unwrap()
}
