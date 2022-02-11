#![allow(unused_imports)]

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedSet;
use near_sdk::{
    env, near_bindgen, AccountId, BorshStorageKey, Gas, PanicOnDefault, Promise, PromiseOrValue,
};
use near_units::parse_gas;
use nearapps_log::{LoggerAccount, NearAppsTags};
use nearapps_near_ext::{ensure, OrPanicStr};

pub mod error;
pub mod logging;
pub mod owners;
pub mod version;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Counter {
    val: u32,
    owner_ids: UnorderedSet<AccountId>,
    nearapps_logger: AccountId,
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Owners,
}

#[near_sdk::ext_contract(ext_self)]
pub trait ExtSelf {
    fn increment() -> u32;
    fn call_until(value: u32, target: u32) -> u32;
}

#[near_bindgen]
impl Counter {
    #[init]
    pub fn new(owner_id: AccountId, nearapps_logger: AccountId) -> Self {
        near_sdk::require!(!env::state_exists(), "Already initialized");
        let mut owner_ids = UnorderedSet::new(StorageKey::Owners);
        owner_ids.insert(&owner_id);
        Self {
            val: 0,
            owner_ids,
            nearapps_logger,
        }
    }

    pub fn get(&self) -> u32 {
        self.val
    }

    pub fn increment(&mut self, nearapps_tags: NearAppsTags) -> u32 {
        self.val += 1;

        // best-effort call for nearapps log
        let _ = self.log(nearapps_tags);

        self.val
    }

    pub fn increment_non_logging(&mut self) -> u32 {
        self.val += 1;

        self.val
    }

    pub fn decrement(&mut self, nearapps_tags: NearAppsTags) -> u32 {
        self.val -= 1;

        // best-effort call for nearapps log
        let _ = self.log(nearapps_tags);

        self.val
    }

    pub fn reset(&mut self, nearapps_tags: NearAppsTags) {
        self.val = 0;

        // best-effort call for nearapps log
        let _ = self.log(nearapps_tags);
    }

    pub fn set(&mut self, val: u32, nearapps_tags: NearAppsTags) {
        self.val = val;

        // best-effort call for nearapps log
        let _ = self.log(nearapps_tags);
    }

    // return multiple values
    pub fn min_max() -> (u32, u32) {
        (u32::MIN, u32::MAX)
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

    #[payable]
    pub fn deposit(&mut self, increment: bool, nearapps_tags: NearAppsTags) -> u32 {
        if increment {
            let attached = env::attached_deposit();
            assert!(attached <= u32::MAX as u128);
            self.val += attached as u32;
        }

        // best-effort call for nearapps log
        let _ = self.log(nearapps_tags);

        self.val
    }

    #[payable]
    #[allow(clippy::let_and_return)]
    pub fn withdraw(&mut self, qty: u32, decrement: bool, nearapps_tags: NearAppsTags) -> Promise {
        if decrement {
            self.val -= qty as u32;
        }
        let transfer = Promise::new(env::predecessor_account_id())
            //
            .transfer(qty as u128);

        // best-effort call for nearapps log
        let _ = self.log(nearapps_tags);

        transfer
    }

    /// Calls repeteadly into itself until `value`
    /// reaches `target`.
    pub fn call_until(value: u32, target: u32) -> near_sdk::PromiseOrValue<u32> {
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
