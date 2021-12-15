use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};
use nearapps_near_ext::ensure;

#[cfg(feature = "crypto")]
pub mod crypto;
#[cfg(feature = "crypto")]
pub mod hash;

pub mod error;
pub mod exec;

use error::Error;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Executor {
    owner_id: AccountId,
}

#[near_bindgen]
impl Executor {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        ensure(!env::state_exists(), Error::AlreadyInitialized);
        Self { owner_id }
    }
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
