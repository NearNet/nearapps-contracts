#![allow(clippy::let_and_return)]

use crate::error::{ensure, Error};
use crate::Executor;
use near_sdk::json_types::U64;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, ext_contract, near_bindgen, serde_json, AccountId, Promise, PromiseResult};

#[cfg(not(target_arch = "wasm32"))]
use crate::ExecutorContract;

#[ext_contract(ext_self)]
pub trait ExtSelf {
    /// Executes an external contract's function, logging on the callback
    /// and forwarding the calls result back.
    ///
    /// Only forwards the first result.
    fn check_promise(tag_info: TagInfo) -> Vec<u8>;
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ContractCall {
    pub contract_id: AccountId,
    pub method_name: String,
    pub args: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct CallContext {
    pub contract_call: ContractCall,
    //
    pub tag_info: TagInfo,
    //
    // pub public_key: near_sdk::PublicKey,
    // pub signature: crate::crypto::Bs58EncodedSignature,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TagInfo {
    pub app_id: String,
    pub action_id: U64,
    pub user_id: AccountId,
}

#[near_bindgen]
impl Executor {
    /// Executes an external contract's function, logging on the callback
    /// and forwarding the calls result back.
    ///
    /// Only forwards the first result.
    #[payable]
    pub fn execute(&mut self, context: CallContext) -> Promise {
        self.assert_owner();

        // makes sure it won't call an internal private function
        ensure(
            context.contract_call.contract_id != env::current_account_id(),
            Error::CallCurrentAccount,
        );

        Promise::new(context.contract_call.contract_id)
            .function_call(
                context.contract_call.method_name,
                context.contract_call.args.as_bytes().to_vec(),
                env::attached_deposit(),
                env::prepaid_gas() / 3,
            )
            .then(ext_self::check_promise(
                context.tag_info,
                env::current_account_id(),
                0,
                env::prepaid_gas() / 3,
            ))
    }

    /// Checks the first result of an external call that was made,
    /// forwarding the first promise result as the value result.
    ///
    /// Logs on successful promise.
    #[private]
    pub fn check_promise(tag_info: TagInfo) {
        let ret = match env::promise_result(0) {
            PromiseResult::Successful(val) => val,
            _ => env::panic_str("Promise with index 0 failed"),
        };
        env::log_str(&serde_json::to_string(&tag_info).unwrap());
        env::value_return(&ret);
    }
}
