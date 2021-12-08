#![allow(clippy::let_and_return)]

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, UnorderedSet};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, BorshStorageKey, Gas, PanicOnDefault, Promise, PublicKey,
};

pub mod error;

pub use error::{ensure, Error};

pub const KILO: u64 = 1000;
pub const MEGA: u64 = KILO * KILO;
pub const TERA: u64 = MEGA * MEGA;
pub const YOTTA: u128 = (TERA as u128) * (TERA as u128);

#[near_sdk::ext_contract(ext_self)]
pub trait ExtSelf {
    fn on_account_created(config: AccountConfig) -> bool;
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Accounts,
    AccountsQueue,
}

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct AccountManager {
    /// Owner of this account.
    pub owner: AccountId,
    // Accounts that were successfuly created.
    pub accounts: UnorderedMap<AccountId, PublicKey>,
    /// Accounts that have been asked to be created.
    pub accounts_queue: UnorderedSet<AccountId>,
    pub defaults: Defaults,
}

#[derive(
    near_sdk::serde::Serialize, near_sdk::serde::Deserialize, BorshDeserialize, BorshSerialize,
)]
#[serde(crate = "near_sdk::serde")]

pub struct Defaults {
    /// The default initial amount to attach to created accounts.
    pub initial_amount: Balance,
    /// The default allowance to attach to allowed calls on created
    /// accounts.
    pub allowance: Balance,
    /// The default allowed calls that new accounts are able to make.
    pub allowed_calls: Vec<AllowedCalls>,
}

#[derive(near_sdk::serde::Serialize, near_sdk::serde::Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AccountConfig {
    /// The sub-account that is being created.
    /// Expected to be a sub-account on `AccountManager`.
    pub account_id: AccountId,
    /// The PublicKey of the sub-account being created.
    /// Expected to be generated by the user.
    pub user_public_key: PublicKey,
    /// The initial amount of deposit that the user should receive.
    ///
    /// If missing, the user will receive
    /// [`Defaults::initial_amount`]
    pub initial_amount: Option<Balance>,
}

#[derive(
    near_sdk::serde::Serialize,
    near_sdk::serde::Deserialize,
    BorshDeserialize,
    BorshSerialize,
    Clone,
)]
#[serde(crate = "near_sdk::serde")]
pub struct AllowedCalls {
    /// How much, in total, the user can spend when calling the contract.
    ///
    /// For each call for the given contract, the allowance will be
    /// decreasing as deposits and gas gets used by the call.
    ///
    /// For replenishment or increase of the allowance, the access_key
    /// must be removed and then added again, with the new `allowance`
    /// value.
    ///
    /// If missing, the value [`Defaults::allowance`]
    /// is used.
    pub allowance: Option<Balance>,
    /// The contract address that the user is allowed to call into.
    pub receiver_id: AccountId,
    /// List of method names (eg. `["method_a", "method_b"]`) that the user
    /// is allowed to call.  
    ///
    /// An empty list means all methods.
    pub method_names: Vec<String>,
}

#[near_bindgen]
impl AccountManager {
    #[init]
    pub fn new(owner: AccountId, defaults: Defaults) -> Self {
        Self {
            owner,
            accounts: UnorderedMap::new(StorageKey::Accounts),
            accounts_queue: UnorderedSet::new(StorageKey::AccountsQueue),
            defaults,
        }
    }

    /// Creates a new user account.
    ///
    /// For now, the [`Self::owner`] will be the full owner of the new
    /// account, where the actual public key is taken from the
    /// [`env::signer_account_pk()`] value.
    ///
    /// The [`AccountConfig::user_public_key`] will be allowed to call
    /// specific contracts and methods, as defined in `allowed_calls`.  
    /// If the value is missing, the [`Defaults::allowed_calls`] is used.  
    /// Otherwise if the value is present but the list is empty, then the
    /// user will not be allowed to make any calls into any contracts.
    ///
    /// The accounts, while being created, first enter a queue from which
    /// they are removed once the successfull creation has been
    /// confirmed.  
    /// The created account names are then tracked in [`Self::accounts`].
    pub fn create_account(
        &mut self,
        config: AccountConfig,
        allowed_calls: Option<Vec<AllowedCalls>>,
    ) -> Promise {
        // TODO: check when too many methods or allowances extrapolates the
        // reserved gas for the call
        // otherwise, a constant quantity could be reserved for the
        // callback only
        const GAS_CURRENT: Gas = Gas(5 * TERA);
        let gas = env::prepaid_gas() - env::used_gas() - GAS_CURRENT;

        ensure(self.owner == env::predecessor_account_id(), Error::NotOwner);

        let is_new_account = self.accounts_queue.insert(&config.account_id);
        ensure(is_new_account, Error::AccountAlreadyQueued);

        let owner_pk = env::signer_account_pk();

        let mut new_account = Promise::new(config.account_id.clone())
            .create_account()
            .add_full_access_key(owner_pk)
            .transfer(
                config
                    .initial_amount
                    .unwrap_or(self.defaults.initial_amount),
            );

        for allowed in allowed_calls.unwrap_or_else(|| self.defaults.allowed_calls.clone()) {
            new_account = new_account.add_access_key(
                config.user_public_key.clone(),
                allowed.allowance.unwrap_or(self.defaults.allowance),
                allowed.receiver_id,
                allowed.method_names.join(","),
            );
        }

        new_account = new_account.then(ext_self::on_account_created(
            config,
            env::current_account_id(),
            0u128,
            gas,
        ));

        new_account
    }

    #[private]
    pub fn on_account_created(&mut self, config: AccountConfig) -> bool {
        ensure(env::promise_results_count() == 1, Error::BadCallbackResults);

        let success: Option<bool> = match env::promise_result(0) {
            near_sdk::PromiseResult::Successful(v) => {
                // did not encounter any panicking failure,
                // but could still fail (false)
                Some(near_sdk::serde_json::from_slice(&v).unwrap())
            }
            // encountered some panicking failure
            _ => None,
        };

        match success {
            Some(true) => {
                let did_exist = self.accounts_queue.remove(&config.account_id);
                // sanity check
                assert!(did_exist);

                let previous_element = self
                    .accounts
                    .insert(&config.account_id, &config.user_public_key);
                // sanity check
                assert!(previous_element.is_none());

                env::log_str(&format!("account {} created.", config.account_id));
            }
            // The call has completed, but it resulted in the
            // account not being created. So it's removed from the queue,
            // as it can be retried later on
            //
            // TODO: test when the AccountManager doesn't have enough
            // funds, if it falls into this case
            Some(false) => {
                let did_exist = self.accounts_queue.remove(&config.account_id);
                // sanity check
                assert!(did_exist);
            }
            // TODO: could the call still be ongoing?
            // not sure if it can be removed form the queue here
            None => {}
        };

        // returns false on `success` = None
        success.unwrap_or_default()
    }
}
