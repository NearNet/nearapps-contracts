#![allow(dead_code)]
#![allow(unused_imports)]

pub use near_sdk::json_types::{Base64VecU8, U64};
use near_sdk::AccountId;
use near_sdk_sim::transaction::ExecutionStatus;
use near_sdk_sim::ExecutionResult;
use near_sdk_sim::{deploy, ContractAccount, UserAccount};
use near_units::parse_near;
use nearapps_counter::CounterContract;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    COUNTER_WASM_BYTES => "../res/nearapps_counter.wasm",
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

pub fn user(id: u32) -> AccountId {
    format!("user{}", id).parse().unwrap()
}
