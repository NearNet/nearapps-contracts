#![allow(clippy::let_and_return)]

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, Promise};
use nearapps_near_ext::ensure;

pub mod error;
pub mod version;

pub use error::Error;

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct UserFactory {
    /// Owner of this account.
    pub owner_id: AccountId,
}

#[near_bindgen]
impl UserFactory {
    #[init]
    pub fn new() -> Self {
        ensure(!env::state_exists(), Error::AlreadyInitialized);
        let predecessor = env::predecessor_account_id();
        Self {
            owner_id: predecessor,
        }
    }

    /// Creates a new user sub-account on the current contract account.  
    /// The account name will be automatically postfixed with the current
    /// contract account name.
    ///
    #[payable]
    pub fn create_subaccount(&mut self, prefix: AccountId, yocto: Option<U128>) -> Promise {
        ensure(
            self.owner_id == env::predecessor_account_id(),
            Error::NotOwner,
        );

        let amount = yocto.unwrap_or(U128(1000000000000000000000000)).0;

        ensure(env::attached_deposit() >= amount, Error::NotEnoughtDeposit);

        let owner_pk = env::signer_account_pk();
        let new_account = format!("{}.{}", &prefix, env::current_account_id());

        Promise::new(new_account.parse().unwrap())
            .create_account()
            .add_full_access_key(owner_pk)
            .transfer(amount)
    }
}
