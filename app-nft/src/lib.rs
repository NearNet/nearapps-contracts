use near_contract_standards::non_fungible_token as nft;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, UnorderedMap, UnorderedSet};
use near_sdk::{env, near_bindgen, require, AccountId, BorshStorageKey, PanicOnDefault};
use nearapps_log::{NearAppsAccount, NearAppsTags};
use nearapps_near_ext::ensure;
use nft::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};

pub mod error;
pub mod series;
pub mod utils;

use error::Error;
pub use series::SERIES_DELIMETER;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Nft {
    tokens: nft::NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    series: UnorderedMap<series::SeriesId, series::Series>,
    next_series_id: series::SeriesId,
    series_minted_tokens: UnorderedMap<series::SeriesId, UnorderedSet<series::SeriesTokenIndex>>,
    nearapps_logger: AccountId,
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
    Series,
    TokensBySeries,
    TokensBySeriesInner { series_id: series::SeriesId },
}

#[near_bindgen]
impl Nft {
    /// Adapted from the standard example.
    #[init]
    pub fn new(
        owner_id: AccountId,
        metadata: NFTContractMetadata,
        nearapps_logger: AccountId,
    ) -> Self {
        require!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        Self {
            tokens: nft::NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            series: UnorderedMap::new(StorageKey::Series),
            next_series_id: series::SeriesId(0),
            series_minted_tokens: UnorderedMap::new(StorageKey::TokensBySeries),
            nearapps_logger,
        }
    }

    /// Identical to the standard example.
    #[init]
    pub fn new_default_meta(owner_id: AccountId, nearapps_logger: AccountId) -> Self {
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: NFT_METADATA_SPEC.to_string(),
                name: "Example NEAR non-fungible token".to_string(),
                symbol: "EXAMPLE".to_string(),
                icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
            nearapps_logger,
        )
    }

    pub fn get_owner(&mut self) -> AccountId {
        self.tokens.owner_id.clone()
    }

    pub fn change_owner(&mut self, new_owner: AccountId) {
        self.assert_owner();
        self.tokens.owner_id = new_owner;
    }

    /// Adapted from the standard example.
    #[payable]
    pub fn nft_mint_logged(
        &mut self,
        token_id: nft::TokenId,
        token_owner_id: AccountId,
        token_metadata: TokenMetadata,
        nearapps_tags: NearAppsTags,
    ) -> nft::Token {
        self.assert_owner();

        // token_id must not contain the series delimiter
        ensure(
            !token_id.contains(SERIES_DELIMETER),
            Error::TokenIdWithSeriesDelimiter,
        );

        // standard minting
        let token = self
            .tokens
            .internal_mint(token_id, token_owner_id, Some(token_metadata));

        // best-effort call for nearapps log
        let _ = self.log(nearapps_tags);

        token
    }

    /// Adapted from the standard example.
    #[payable]
    pub fn nft_series_mint_logged(
        &mut self,
        series_id: series::SeriesId,
        token_owner_id: AccountId,
        token_metadata: Option<TokenMetadata>,
        nearapps_tags: NearAppsTags,
    ) -> nft::Token {
        self.assert_owner();

        let mut series = self.nft_series_get(series_id);
        let mut minted_tokens = self.nft_series_get_minted_tokens(series_id);

        let token = series.next_token();

        // updates series
        self.series.insert(&series_id, &series);

        // updates minted_tokens
        let token_index = series.last_token_index().unwrap();
        minted_tokens.insert(&token_index);
        self.series_minted_tokens.insert(&series_id, &minted_tokens);

        let token_metadata = token_metadata.unwrap_or_else(|| TokenMetadata {
            title: Some(token.0.clone()),
            description: Some(format!(
                "Token #{} from series {}",
                token_index.0, series.name
            )),
            media: None,
            media_hash: None,
            copies: None,
            issued_at: Some(env::block_timestamp().to_string()),
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
        });

        // standard minting
        let token = self
            .tokens
            .internal_mint(token.0, token_owner_id, Some(token_metadata));

        // best-effort call for nearapps log
        let _ = self.log(nearapps_tags);

        token
    }
}

pub mod std_impls {
    use super::nft::{self, Token, TokenId};
    use super::Nft;
    use near_sdk::{near_bindgen, AccountId, Promise, PromiseOrValue};
    use nearapps_log::{NearAppsAccount, NearAppsTags};

    #[allow(unused_imports)]
    use near_contract_standards as ncs;

    #[cfg(not(target_arch = "wasm32"))]
    use crate::NftContract;

    // ncs::impl_non_fungible_token_core!(Nft, tokens);
    //
    /// All functions are just the expanded macros from the standard,
    /// except that a logging procedure is inserted at the end
    /// of the state-changing  functions.
    use nft::core::NonFungibleTokenCore;
    use nft::core::NonFungibleTokenResolver;
    use std::collections::HashMap;
    #[near_bindgen]
    impl Nft {
        #[payable]
        pub fn nft_transfer_logged(
            &mut self,
            receiver_id: AccountId,
            token_id: TokenId,
            approval_id: Option<u64>,
            memo: Option<String>,
            nearapps_tags: NearAppsTags,
        ) {
            self.tokens
                .nft_transfer(receiver_id, token_id, approval_id, memo);

            // best-effort call for nearapps log
            let _ = self.log(nearapps_tags);
        }

        #[payable]
        pub fn nft_transfer_call_logged(
            &mut self,
            receiver_id: AccountId,
            token_id: TokenId,
            approval_id: Option<u64>,
            memo: Option<String>,
            msg: String,
            nearapps_tags: NearAppsTags,
        ) -> PromiseOrValue<bool> {
            let res = self
                .tokens
                .nft_transfer_call(receiver_id, token_id, approval_id, memo, msg);

            // best-effort call for nearapps log
            let _ = self.log(nearapps_tags);

            res
        }

        pub fn nft_token(
            //
            &self,
            token_id: TokenId,
        ) -> Option<Token> {
            self.tokens.nft_token(token_id)
        }
    }

    /// All functions are just the expanded macros from the standard,
    /// except that a logging procedure is inserted at the end
    /// of the state-changing  functions.
    #[near_bindgen]
    impl NonFungibleTokenResolver for Nft {
        #[private]
        fn nft_resolve_transfer(
            &mut self,
            previous_owner_id: AccountId,
            receiver_id: AccountId,
            token_id: TokenId,
            approved_account_ids: Option<HashMap<AccountId, u64>>,
        ) -> bool {
            self.tokens.nft_resolve_transfer(
                previous_owner_id,
                receiver_id,
                token_id,
                approved_account_ids,
            )
        }
    }

    // ncs::impl_non_fungible_token_approval!(Nft, tokens);
    //
    /// All functions are just the expanded macros from the standard,
    /// except that a logging procedure is inserted at the end
    /// of the state-changing  functions.
    use nft::approval::NonFungibleTokenApproval;
    #[near_bindgen]
    impl Nft {
        #[payable]
        pub fn nft_approve_logged(
            &mut self,
            token_id: TokenId,
            account_id: AccountId,
            msg: Option<String>,
            nearapps_tags: NearAppsTags,
        ) -> Option<Promise> {
            let res = self.tokens.nft_approve(token_id, account_id, msg);

            // best-effort call for nearapps log
            let _ = self.log(nearapps_tags);

            res
        }

        #[payable]
        pub fn nft_revoke_logged(
            //
            &mut self,
            token_id: TokenId,
            account_id: AccountId,
            nearapps_tags: NearAppsTags,
        ) {
            self.tokens.nft_revoke(token_id, account_id);

            // best-effort call for nearapps log
            let _ = self.log(nearapps_tags);
        }

        #[payable]
        pub fn nft_revoke_all_logged(
            //
            &mut self,
            token_id: TokenId,
            nearapps_tags: NearAppsTags,
        ) {
            self.tokens.nft_revoke_all(token_id);

            // best-effort call for nearapps log
            let _ = self.log(nearapps_tags);
        }

        pub fn nft_is_approved(
            &self,
            token_id: TokenId,
            approved_account_id: AccountId,
            approval_id: Option<u64>,
        ) -> bool {
            self.tokens
                .nft_is_approved(token_id, approved_account_id, approval_id)
        }
    }

    // ncs::impl_non_fungible_token_enumeration!(Nft, tokens);
    //
    /// All functions are just the expanded macros from the standard,
    /// except that a logging procedure is inserted at the end
    /// of the state-changing  functions.
    use near_sdk::json_types::U128;
    use nft::enumeration::NonFungibleTokenEnumeration;
    #[near_bindgen]
    impl NonFungibleTokenEnumeration for Nft {
        fn nft_total_supply(&self) -> U128 {
            self.tokens.nft_total_supply()
        }
        fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Token> {
            self.tokens.nft_tokens(from_index, limit)
        }
        fn nft_supply_for_owner(&self, account_id: AccountId) -> U128 {
            self.tokens.nft_supply_for_owner(account_id)
        }
        fn nft_tokens_for_owner(
            &self,
            account_id: AccountId,
            from_index: Option<U128>,
            limit: Option<u64>,
        ) -> Vec<Token> {
            self.tokens
                .nft_tokens_for_owner(account_id, from_index, limit)
        }
    }
}

/// standard implementation.
#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Nft {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}

pub trait Owner {
    fn assert_owner(&self);
}

impl Owner for Nft {
    fn assert_owner(&self) {
        ensure(
            env::predecessor_account_id() == self.tokens.owner_id,
            Error::NotOwner,
        )
    }
}

impl nearapps_log::NearAppsAccount for Nft {
    fn nearapps_account(&self) -> near_sdk::AccountId {
        self.nearapps_logger.clone()
    }
}

/// Identical to the standard example.
const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";
