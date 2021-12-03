use crate::Contract;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, ext_contract, near_bindgen, AccountId, Promise};

#[cfg(not(target_arch = "wasm32"))]
use crate::ContractContract;

#[ext_contract(ext_self)]
pub trait ExtSelf {
    fn check_promise(caller: Option<CallerInformation>) -> Vec<u8>;
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct CallContext {
    pub contract_id: AccountId,
    pub action_id: String,
    pub args: String,
    pub app_id: Option<String>,
    pub caller: Option<CallerInformation>,
    pub api_hash: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct CallerInformation {
    company: String,
    contact: Option<String>,
    description: String,
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn execute(&mut self, context: CallContext) -> Promise {
        Promise::new(context.contract_id)
            .function_call(
                context.action_id,
                context.args.as_bytes().to_vec(),
                env::attached_deposit(),
                env::prepaid_gas(),
            )
            .then(ext_self::check_promise(
                context.caller,
                env::current_account_id(),
                0,
                env::prepaid_gas(),
            ))
    }
}
