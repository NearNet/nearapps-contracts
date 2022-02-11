use crate::error::Error;
use crate::types::{NftContractId, NftProtocol, Sha256From};
use crate::SendNft;
use crate::StorageKey;
use near_sdk::collections::TreeMap;
use near_sdk::json_types::U128;
use near_sdk::near_bindgen;
use nearapps_near_ext::{ensure, OrPanicStr};

#[cfg(not(target_arch = "wasm32"))]
use crate::SendNftContract;

#[near_bindgen]
impl SendNft {
    /// Adds a new Nft contract and registers it's protocol.
    pub fn add_nft_protocol(&mut self, nft: NftContractId, protocol: NftProtocol) {
        self.assert_owner();

        let previous = self.nft_protocols.insert(&nft, &protocol);
        ensure(previous.is_none(), Error::NftProtocolAlreadyIncluded);

        let new_token_tree = TreeMap::new(StorageKey::NftTokenUsersInner {
            contract_id: Sha256From::new(&nft),
        });
        ensure(
            self.nft_token_users.get(&nft).is_none(),
            Error::NftProtocolAlreadyIncluded,
        );
        let _previous = self.nft_token_users.insert(&nft, &new_token_tree);
    }

    /// Edits a Nft contract's protocol.
    pub fn change_nft_protocol(&mut self, nft: NftContractId, new_protocol: NftProtocol) {
        self.assert_owner();

        let previous = self.nft_protocols.insert(&nft, &new_protocol);
        ensure(previous.is_some(), Error::NftProtocolNotIncluded);
    }

    /// Removes a nft protocol.
    ///     
    /// Note: No user can be owning any token on this nft
    /// contract.
    #[allow(unused_variables)]
    pub fn remove_nft_protocol(&mut self, nft: NftContractId) {
        self.assert_owner();

        let tokens_per_user = self
            .nft_token_users
            .get(&nft)
            .or_panic_str(&Error::NftProtocolNotIncluded);

        ensure(
            tokens_per_user.is_empty(),
            Error::NftTokensStillOwnedByUsers,
        );

        // actual removal is not yet implemented
        unimplemented!()
    }

    /// Shows registered nft contracts and their protocols.
    pub fn get_nft_protocols(
        &self,
        from_index: Option<U128>,
        limit: Option<u16>,
    ) -> Vec<(NftContractId, NftProtocol)> {
        let from_index = from_index.unwrap_or_else(|| 0.into()).0 as usize;
        let limit = limit.unwrap_or(u16::MAX) as usize;
        self.nft_protocols
            .iter()
            .skip(from_index)
            .take(limit)
            .collect()
    }
}
