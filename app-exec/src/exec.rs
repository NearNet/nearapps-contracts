#![allow(clippy::let_and_return)]

use crate::error::{ensure, Error};
use crate::Executor;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, ext_contract, near_bindgen, serde_json, AccountId, Promise, PromiseResult};
use nearapps_log::{NearAppsTags, NearAppsTagsContained};

#[cfg(not(target_arch = "wasm32"))]
use crate::ExecutorContract;

#[ext_contract(ext_self)]
pub trait ExtSelf {
    /// Executes an external contract's function, logging on the callback
    /// and forwarding the calls result back.
    ///
    /// Only forwards the first result.
    fn on_execute_then_log(nearapps_tags: NearAppsTags) -> Vec<u8>;
}

// #[derive(Serialize, Deserialize)]
// #[serde(crate = "near_sdk::serde")]
// pub struct ContractCall {
//     pub contract_id: AccountId,
//     pub method_name: String,
//     pub args: String,
// }

#[near_bindgen]
impl Executor {
    /// Executes an external contract's function, where all of the
    /// logging should be proactively made by the contract that is to be
    /// called.
    ///
    /// The logging information must be contained in `args` under the
    /// field `nearapps_tags`. See [`NearAppsTagsContained`].
    #[payable]
    pub fn execute(
        &mut self,
        contract_id: AccountId,
        method_name: String,
        args: String,
    ) -> Promise {
        use crate::Owner;
        self.assert_owner();

        ensure(
            near_sdk::serde_json::from_str::<NearAppsTagsContained>(&args).is_ok(),
            Error::CallBadNearAppsTags,
        );

        // makes sure it won't call an internal private function
        ensure(
            contract_id != env::current_account_id(),
            Error::CallCurrentAccount,
        );

        Promise::new(contract_id).function_call(
            method_name,
            args.as_bytes().to_vec(),
            env::attached_deposit(),
            env::prepaid_gas() / 3,
        )
    }

    /// Executes an external contract's function, logging on the callback
    /// and forwarding the calls result back.
    ///
    /// Only forwards the first result.
    #[payable]
    pub fn execute_then_log(
        &mut self,
        contract_id: AccountId,
        method_name: String,
        args: String,
        nearapps_tags: NearAppsTags,
    ) -> Promise {
        use crate::Owner;
        self.assert_owner();

        // makes sure it won't call an internal private function
        ensure(
            contract_id != env::current_account_id(),
            Error::CallCurrentAccount,
        );

        Promise::new(contract_id)
            .function_call(
                method_name,
                args.as_bytes().to_vec(),
                env::attached_deposit(),
                env::prepaid_gas() / 3,
            )
            .then(ext_self::on_execute_then_log(
                nearapps_tags,
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
    pub fn on_execute_then_log(nearapps_tags: NearAppsTags) {
        let ret = match env::promise_result(0) {
            PromiseResult::Successful(val) => val,
            _ => env::panic_str("Promise with index 0 failed"),
        };
        env::log_str(&serde_json::to_string(&nearapps_tags).unwrap());
        env::value_return(&ret);
    }
}
