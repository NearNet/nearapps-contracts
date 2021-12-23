use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedSet;
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault};

pub mod crypto;
pub mod error;
pub mod exec;
pub mod hash;

use error::{ensure, Error};

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

impl Executor {
    fn assert_owner(&self) {
        ensure(
            self.owner_ids.contains(&env::predecessor_account_id()),
            Error::NotOwner,
        )
    }
}

pub trait Owners {
    fn add_owner(&mut self, owner_id: AccountId) -> bool;

    /// Removes a owner.  
    ///
    /// Returns `true` if such owner was removed.  
    /// Returns `false` if the owner wasn't added in the first place.
    fn remove_owner(&mut self, owner_id: AccountId) -> bool;

    /// Checks if the given account is an owner.  
    ///
    /// Returns `true` if it is, and `false` otherwise.
    fn is_owner(&self, owner_id: AccountId) -> bool;

    /// Show owners.
    ///
    /// Returns a list of `AccountId`'s.
    fn get_owners(&self) -> Vec<AccountId>;
}

#[near_bindgen]
impl Owners for Executor {
    /// Adds a new owner.  
    ///
    /// Returns `true` if it's a newly added owner.  
    /// Returns `false` if the owner was already added.
    fn add_owner(&mut self, owner_id: AccountId) -> bool {
        self.assert_owner();
        self.owner_ids.insert(&owner_id)
    }

    /// Removes a owner.  
    ///
    /// Returns `true` if such owner was removed.  
    /// Returns `false` if the owner wasn't added in the first place.
    fn remove_owner(&mut self, owner_id: AccountId) -> bool {
        self.assert_owner();
        self.owner_ids.remove(&owner_id)
    }

    /// Checks if the given account is an owner.  
    ///
    /// Returns `true` if it is, and `false` otherwise.
    fn is_owner(&self, owner_id: AccountId) -> bool {
        self.owner_ids.contains(&owner_id)
    }

    /// Show owners.
    ///
    /// Returns a list of `AccountId`'s.
    fn get_owners(&self) -> Vec<AccountId> {
        self.owner_ids.iter().collect()
    }
}
