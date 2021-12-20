use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, Gas, Promise, PromiseOrValue};
use near_units::parse_gas;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Counter {
    val: u8,
}

#[near_sdk::ext_contract(ext_self)]
pub trait ExtSelf {
    fn increment() -> u8;
    fn call_until(value: u8, target: u8) -> u8;
}

#[near_bindgen]
impl Counter {
    pub fn get(&self) -> u8 {
        self.val
    }

    pub fn increment(&mut self) -> u8 {
        self.val += 1;
        self.val
    }

    pub fn decrement(&mut self) -> u8 {
        self.val -= 1;
        self.val
    }

    pub fn reset(&mut self) {
        self.val = 0;
    }

    pub fn set(&mut self, val: u8) {
        self.val = val;
    }

    // return multiple values
    pub fn min_max() -> (u8, u8) {
        (u8::MIN, u8::MAX)
    }

    // returns promise
    /// Makes an `increment()` call into itself.
    #[allow(clippy::let_and_return)]
    pub fn call_increment() -> near_sdk::Promise {
        const GAS_CURRENT: Gas = Gas(parse_gas!("5 Tgas") as u64);
        let gas = env::prepaid_gas() - env::used_gas() - GAS_CURRENT;

        let call = ext_self::increment(
            // calling into itself
            env::current_account_id(),
            // deposit
            0,
            // gas
            gas,
        );
        call
    }

    pub fn log(&self) -> u8 {
        env::log_str(&self.val.to_string());
        self.val
    }

    #[payable]
    pub fn deposit(&mut self, increment: bool) -> u8 {
        if increment {
            let attached = env::attached_deposit();
            assert!(attached <= u8::MAX as u128);
            self.val += attached as u8;
        }
        self.val
    }

    #[payable]
    #[allow(clippy::let_and_return)]
    pub fn withdraw(&mut self, qty: u8, decrement: bool) -> Promise {
        if decrement {
            self.val -= qty as u8;
        }
        let transfer = Promise::new(env::predecessor_account_id())
            //
            .transfer(qty as u128);
        transfer
    }

    /// Calls repeteadly into itself until `value`
    /// reaches `target`.
    pub fn call_until(value: u8, target: u8) -> near_sdk::PromiseOrValue<u8> {
        const GAS_CURRENT: Gas = Gas(parse_gas!("5 Tgas") as u64);
        let gas = env::prepaid_gas() - env::used_gas() - GAS_CURRENT;

        if value >= target {
            PromiseOrValue::Value(value)
        } else {
            let call = ext_self::call_until(
                //
                value + 1,
                target,
                env::current_account_id(),
                0,
                gas,
            );
            PromiseOrValue::Promise(call)
        }
    }
}
