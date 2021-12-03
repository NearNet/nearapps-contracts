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
pub struct ContractCall {
    pub contract_id: AccountId,
    pub method_name: String,
    pub args: String,
}
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct CallContext {
    pub contract_call: ContractCall,
    pub public_key: types::ed25519::PubKey,
    pub signature: types::ed25519::Sign,
    pub app_id: Option<String>,
    pub caller: Option<CallerInformation>,
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
        Promise::new(context.contract_call.contract_id)
            .function_call(
                context.contract_call.method_name,
                context.contract_call.args.as_bytes().to_vec(),
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
