#![allow(clippy::ref_in_deref)]

pub use near_sdk::json_types::{Base64VecU8, U64};
use near_sdk_sim::{call, deploy, init_simulator, view, ContractAccount, UserAccount};
use nearapps_counter::CounterContract;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    COUNTER_WASM_BYTES => "../res/nearapps_counter.wasm",
}

pub const DEFAULT_GAS: u64 = 300_000_000_000_000;

fn init() -> (UserAccount, ContractAccount<CounterContract>) {
    let root = init_simulator(None);

    let counter: ContractAccount<CounterContract> = deploy!(
        contract: CounterContract,
        contract_id: "counter".to_string(),
        bytes: &COUNTER_WASM_BYTES,
        signer_account: root
    );

    (root, counter)
}

#[test]
fn simulate_increment() {
    let (root, counter) = init();

    let mut current_num: i8 = view!(counter.get()).unwrap_json();
    assert_eq!(&current_num, &0);

    call!(root, counter.increment()).assert_success();

    current_num = view!(counter.get()).unwrap_json();
    assert_eq!(&current_num, &1);
}
