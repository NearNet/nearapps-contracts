#![allow(dead_code)]

pub use near_sdk::json_types::{Base64VecU8, U64};
use near_sdk::{AccountId, Gas};
use near_sdk_sim::transaction::ExecutionStatus;
use near_sdk_sim::{deploy, init_simulator, ContractAccount, ExecutionResult, UserAccount};

use nearapps_wallet::{AccountManagerContract, AllowedCalls, Defaults};

pub const DEFAULT_GAS: u64 = 300_000_000_000_000;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    WALLET_WASM_BYTES => "../res/nearapps_wallet.wasm",
}

pub const KILO: u64 = 1000;
pub const MEGA: u64 = KILO * KILO;
pub const TERA: u64 = MEGA * MEGA;
pub const MEGA_TERA: u128 = MEGA as u128 * TERA as u128;
pub const YOTTA: u128 = (TERA as u128) * (TERA as u128);

pub trait ExecutionExt {
    fn assert_failure<E: ToString>(&self, action: u32, err: E);
    fn total_gas_burnt(&self) -> Gas;
}

impl ExecutionExt for ExecutionResult {
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
    fn total_gas_burnt(&self) -> Gas {
        self.get_receipt_results()
            .into_iter()
            .chain(self.promise_results())
            .flatten()
            .map(|o| o.gas_burnt().0)
            .sum::<u64>()
            .into()
    }
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
