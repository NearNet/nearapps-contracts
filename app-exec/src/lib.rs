use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};

pub mod crypto;
pub mod error;
pub mod exec;
pub mod hash;

use error::{ensure, Error};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Executor {
    owner_id: AccountId,
}

pub trait Owner {
    fn assert_owner(&self);
}

impl Owner for Executor {
    fn assert_owner(&self) {
        ensure(
            env::predecessor_account_id() == self.owner_id,
            Error::NotOwner,
        )
    }
}
