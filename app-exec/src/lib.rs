use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedSet;
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault};
use nearapps_near_ext::ensure;

pub mod error;
pub mod exec;
pub mod owners;
pub mod version;

use error::Error;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Executor {
    owner_ids: UnorderedSet<AccountId>,
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Owners,
}

#[near_bindgen]
impl Executor {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        ensure(!env::state_exists(), Error::AlreadyInitialized);
        let mut owner_ids = UnorderedSet::new(StorageKey::Owners);
        owner_ids.insert(&owner_id);
        Self { owner_ids }
    }
}
