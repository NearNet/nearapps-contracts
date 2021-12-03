#![allow(dead_code)]

pub mod _secp256k1;

pub use near_sdk::json_types::{Base64VecU8, U64};
use near_sdk::AccountId;
use near_sdk_sim::transaction::ExecutionStatus;
use near_sdk_sim::{
    deploy, init_simulator, to_yocto, ContractAccount, ExecutionResult, UserAccount,
};

use nearapps_exec::ContractContract;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    EXEC_WASM_BYTES => "../res/nearapps_exec.wasm"
}

pub type Contract = ContractAccount<ContractContract>;

pub const KILO: u64 = 1000;
pub const MEGA: u64 = KILO * KILO;
pub const YOTTA: u128 = (MEGA as u128) * (MEGA as u128) * (MEGA as u128) * (MEGA as u128);

pub fn setup_contract() -> (UserAccount, Contract) {
    let root = init_simulator(None);
    let contract = deploy!(
        contract: ContractContract,
        contract_id: "contract".to_string(),
        bytes: &EXEC_WASM_BYTES,
        signer_account: root,
        deposit: to_yocto("200"),
        // init_method: new()
    );
    (root, contract)
}

fn user(id: u32) -> AccountId {
    format!("user{}", id).parse().unwrap()
}

pub fn should_fail_with(r: ExecutionResult, action: u32, err: &str) {
    let err = format!("Action #{}: Smart contract panicked: {}", action, err);
    match r.status() {
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
