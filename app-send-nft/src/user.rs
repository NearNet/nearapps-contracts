use crate::error::Error;
use crate::owner::Owner;
use crate::types::{NftContractId, NftUserAccountId};
use crate::SendNft;
use crate::StorageKey;
use near_contract_standards::non_fungible_token as nft;
use near_sdk::collections::{UnorderedMap, UnorderedSet};
use near_sdk::json_types::U128;
use near_sdk::near_bindgen;
use nearapps_near_ext::{ensure, OrPanicStr};

#[cfg(not(target_arch = "wasm32"))]
use crate::SendNftContract;

#[near_bindgen]

impl SendNft {
    /// Register a user on this contract.
    ///
    /// The user must also have specific nft contracts enabled
    /// for them. See [`Self::enable_nft_for_user`].
    #[payable]
    pub fn add_user(&mut self, user: NftUserAccountId) {
        self.assert_owner();

        ensure(
            self.nft_tokens_per_user.get(&user).is_none(),
            Error::UserAlreadyRegistered,
        );

        let token_set_per_nft = UnorderedMap::new(StorageKey::NftTokensPerUserInner {
            user_id: user.clone(),
        });
        let _previous = self.nft_tokens_per_user.insert(&user, &token_set_per_nft);
    }

    /// Un-register a user from this contract.
    ///
    /// The user must not be owning any token.
    pub fn remove_user(&mut self, user: NftUserAccountId) {
        self.assert_owner();

        let tokens_per_contract = self
            .nft_tokens_per_user
            .get(&user)
            .or_panic_str(Error::MissingUser);

        // check if user doesn't have any registered
        // nft contract that they can interact with
        assert_eq!(tokens_per_contract.len(), 0);

        // actual removal is not yet implemented
        unimplemented!()
    }

    /// Enables a user to be able to interact with a given nft
    /// contract. Otherwise, the user cannot register a token
    /// from that nft contract.
    ///
    /// The user must also be registered themselves,
    /// see [`Self::add_user`].
    pub fn enable_nft_for_user(&mut self, user: NftUserAccountId, nft: NftContractId) {
        self.assert_owner();

        let previous = self.nft_protocols.get(&nft);
        ensure(previous.is_some(), Error::NftProtocolNotIncluded);

        let mut token_set_per_nft = self
            .nft_tokens_per_user
            .get(&user)
            .or_panic_str(Error::MissingUser);

        ensure(
            token_set_per_nft.get(&nft).is_none(),
            Error::NftContractAlreadyEnabled,
        );

        let token_set = UnorderedSet::new(StorageKey::NftTokensPerUserInnerInner {
            user_id: user.clone(),
            contract_id: nft.clone(),
        });
        let _previous = token_set_per_nft.insert(&nft, &token_set);

        self.nft_tokens_per_user.insert(&user, &token_set_per_nft);
    }

    /// Makes a user to be unable to interact with a given nft
    /// contract.
    ///
    /// The user must not be owning any token on a given
    /// nft contract.
    pub fn disable_nft_for_user(&mut self, user: NftUserAccountId, nft: NftContractId) {
        self.assert_owner();

        let tokens_per_contract = self
            .nft_tokens_per_user
            .get(&user)
            .or_panic_str(Error::MissingUser);

        let tokens = tokens_per_contract
            .get(&nft)
            .or_panic_str(Error::NftDisabledForUser);

        // check if user doesn't have any registered
        // nft tokens for this nft contract
        ensure(tokens.is_empty(), Error::NftTokensStillOwnedByUser);

        // actual disabling is not yet implemented
        unimplemented!()
    }

    ///
    /// Get nft contracts that a user has enabled for their
    /// usage.
    ///
    /// The nft account_id elements are ordered by their
    /// name, so enabling/disabling nfts can change the elements
    /// position.
    pub fn get_enabled_nfts_for_user(
        &self,
        user: NftUserAccountId,
        from_index: Option<U128>,
        limit: Option<u16>,
    ) -> Vec<NftContractId> {
        let from_index = from_index.unwrap_or_else(|| 0.into()).0 as usize;
        let limit = limit.unwrap_or(u16::MAX) as usize;

        let tokens_by_contracts = self
            .nft_tokens_per_user
            .get(&user)
            .or_panic_str(Error::MissingUser);
        tokens_by_contracts
            .keys()
            .skip(from_index)
            .take(limit)
            .collect()
    }

    /// Get tokens owned by users for a given nft contract.
    ///
    /// Note: The token_id elements are ordered by their name,
    /// so additions and removals of tokens can change the
    /// elements position.
    pub fn get_tokens_owned_by_users(
        &self,
        nft: NftContractId,
        from_index: Option<U128>,
        limit: Option<u16>,
    ) -> Vec<(nft::TokenId, NftUserAccountId)> {
        let tokens = self
            .nft_token_users
            .get(&nft)
            .or_panic_str(Error::NftProtocolNotIncluded);

        let from_index = from_index.map(u128::from).unwrap_or_default() as usize;

        let mut cursor = tokens.into_iter();

        let (limit, mut v) = {
            let (limit, v) = match limit.map(usize::from) {
                Some(l) => (l, Vec::with_capacity(l)),
                None => (usize::MAX, Vec::new()),
            };

            // reduces one from limit because the first element
            // is extracted separately
            ensure(limit > 0, Error::LimitAtZero);
            (limit - 1, v)
        };

        // skip elements (if needed) and get the first one
        let first = cursor.nth(from_index);
        match first {
            None => {
                return v;
            }
            Some((token_id, user)) => v.push((token_id, user)),
        };

        // add remaining elements
        for (token_id, user) in cursor.take(limit) {
            v.push((token_id, user));
        }

        v
    }

    /// Get token_id's of tokens that are owned by a user in a
    /// given nft contract.  
    ///
    /// The token_id elements are ordered by their name, so
    /// additions and removals of tokens can change the elements
    /// position.
    pub fn get_tokens_for_user(
        &self,
        nft: NftContractId,
        user: NftUserAccountId,
        from_index: Option<U128>,
        limit: Option<u16>,
    ) -> Vec<nft::TokenId> {
        let tokens_by_contracts = self
            .nft_tokens_per_user
            .get(&user)
            .or_panic_str(Error::MissingUser);
        let tokens = tokens_by_contracts
            .get(&nft)
            .or_panic_str(Error::NftProtocolNotIncluded);

        let from_index = from_index.unwrap_or_else(|| 0.into()).0 as usize;
        let limit = limit.unwrap_or(u16::MAX) as usize;
        tokens.iter().skip(from_index).take(limit).collect()
    }
}
