use crate::signing::eddsa_ed25519 as ed;
use crate::Contract;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, ext_contract, near_bindgen, serde_json, AccountId, Promise, PromiseResult};

#[cfg(not(target_arch = "wasm32"))]
use crate::ContractContract;

#[ext_contract(ext_self)]
pub trait ExtSelf {
    fn check_promise(caller: Option<CallerInformation>) -> Vec<u8>;
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
    pub app_id: Option<String>,
    pub caller: Option<CallerInformation>,
    pub public_key: ed::types::PubKey,
    pub signature: ed::types::Sign,
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
    pub fn execute(context: CallContext) -> Promise {
        Promise::new(context.contract_call.contract_id)
            .function_call(
                context.contract_call.method_name,
                context.contract_call.args.as_bytes().to_vec(),
                env::attached_deposit(),
                env::prepaid_gas() / 3,
            )
            .then(ext_self::check_promise(
                context.caller,
                env::current_account_id(),
                0,
                env::prepaid_gas() / 3,
            ))
    }

    ///Can only be called by predecessor_account_id().
    #[private]
    pub fn check_promise(caller: Option<CallerInformation>) {
        match env::promise_result(0) {
            PromiseResult::Successful(val) => {
                if let Some(inf) = caller {
                    env::log_str(&serde_json::to_string(&inf).unwrap());
                }
                env::value_return(&val);
            }
            _ => env::panic_str("Promise with index 0 failed"),
        }
    }
}
