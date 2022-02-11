use crate::error::Error;
use crate::UserFactory;
use near_sdk::{env, near_bindgen, AccountId};
use nearapps_near_ext::{ensure, Owners};

#[cfg(not(target_arch = "wasm32"))]
use crate::UserFactoryContract;

#[near_bindgen]
impl UserFactory {
    pub fn assert_owner(&self) {
        ensure(
            self.owner_ids.contains(&env::predecessor_account_id()),
            Error::NotOwner,
        )
    }
}

#[near_bindgen]
impl Owners for UserFactory {
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
    fn get_owners(
        &self,
        from_index: Option<near_sdk::json_types::U128>,
        limit: Option<u16>,
    ) -> Vec<AccountId> {
        let from_index = from_index.unwrap_or_else(|| 0.into()).0 as usize;
        let limit = limit.unwrap_or(u16::MAX) as usize;
        self.owner_ids.iter().skip(from_index).take(limit).collect()
    }
}
