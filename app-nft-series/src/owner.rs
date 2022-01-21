use crate::error::Error;
use crate::{series, NftSeries};
use near_contract_standards::non_fungible_token as nft;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, UnorderedMap, UnorderedSet};
use near_sdk::{env, near_bindgen, require, AccountId, BorshStorageKey, PanicOnDefault};
use nearapps_log::{NearAppsAccount, NearAppsTags};
use nearapps_near_ext::ensure;
use nft::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};

#[cfg(not(target_arch = "wasm32"))]
use crate::NftSeriesContract;

#[near_bindgen]
impl NftSeries {
    pub fn assert_owner(&self) {
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
    fn get_owners(
        &self,
        from_index: Option<near_sdk::json_types::U128>,
        limit: Option<u16>,
    ) -> Vec<AccountId>;
}

#[near_bindgen]
impl Owners for NftSeries {
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
