#![allow(dead_code)]

pub use near_sdk::json_types::{Base64VecU8, U64};
use near_sdk::AccountId;
use near_sdk_sim::transaction::ExecutionStatus;
use near_sdk_sim::{deploy, init_simulator, ContractAccount, ExecutionResult, UserAccount};
use near_units::parse_near;
use nearapps_counter::CounterContract;
use nearapps_exec::ExecutorContract;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    EXEC_WASM_BYTES => "../res/nearapps_exec.wasm",
    COUNTER_WASM_BYTES => "../res/nearapps_counter.wasm",
}

pub fn setup_exec(root: &UserAccount) -> ContractAccount<ExecutorContract> {
    let contract = deploy!(
        contract: ExecutorContract,
        contract_id: "executor".to_string(),
        bytes: &EXEC_WASM_BYTES,
        signer_account: root,
        deposit: parse_near!("200 N"),
        init_method: new(root.account_id())
    );
    contract
}

pub fn setup_counter(
    root: &UserAccount,
    nearapps_acc: AccountId,
) -> ContractAccount<CounterContract> {
    let counter: ContractAccount<CounterContract> = deploy!(
        contract: CounterContract,
        contract_id: "counter".to_string(),
        bytes: &COUNTER_WASM_BYTES,
        signer_account: root,
        init_method: new(nearapps_acc.clone(), nearapps_acc)
    );

    counter
}

fn user(id: u32) -> AccountId {
    format!("user{}", id).parse().unwrap()
}
