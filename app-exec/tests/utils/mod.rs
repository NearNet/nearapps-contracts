#![allow(dead_code)]

pub use near_sdk::json_types::{Base64VecU8, U64};
use near_sdk::AccountId;
use near_sdk_sim::transaction::ExecutionStatus;
use near_sdk_sim::{deploy, init_simulator, ContractAccount, ExecutionResult, UserAccount};
use nearapps_counter::CounterContract;
use nearapps_exec::ExecutorContract;

#[cfg(feature = "crypto")]
pub mod _secp256k1;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    EXEC_WASM_BYTES => "../res/nearapps_exec.wasm",
    COUNTER_WASM_BYTES => "../res/nearapps_counter.wasm",
}

pub type Contract = ContractAccount<ExecutorContract>;

pub const KILO: u64 = 1000;
pub const MEGA: u64 = KILO * KILO;
pub const TERA: u64 = MEGA * MEGA;
pub const YOTTA: u128 = (TERA as u128) * (TERA as u128);

pub trait AssertFailure {
    fn assert_failure<E: ToString>(&self, action: u32, err: E);
}

impl AssertFailure for ExecutionResult {
    fn assert_failure<E: ToString>(&self, action: u32, err: E) {
        let err = format!(
            "Action #{}: Smart contract panicked: {}",
            action,
            err.to_string()
        );
        match self.status() {
            ExecutionStatus::Failure(txerr_) => {
                assert_eq!(txerr_.to_string(), err)
            }
            ExecutionStatus::Unknown => panic!("Got Unknown. Should have failed with {}", err),
            ExecutionStatus::SuccessValue(_v) => {
                panic!("Got SuccessValue. Should have failed with {}", err)
            }
            ExecutionStatus::SuccessReceiptId(_id) => {
                panic!("Got SuccessReceiptId. Should have failed with {}", err)
            }
        }
    }
}

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
