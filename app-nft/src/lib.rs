use near_contract_standards::non_fungible_token as nft;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, UnorderedMap, UnorderedSet};
use near_sdk::{env, near_bindgen, require, AccountId, BorshStorageKey, PanicOnDefault};
use nft::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};

pub mod error;
pub mod series;
pub mod utils;

use error::{ensure, Error};
pub use series::SERIES_DELIMETER;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Nft {
    tokens: nft::NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    series: UnorderedMap<series::SeriesId, series::Series>,
    next_series_id: series::SeriesId,
    series_minted_tokens: UnorderedMap<series::SeriesId, UnorderedSet<series::SeriesTokenIndex>>,
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
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
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
        }
    }

    /// Identical to the standard example.
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
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
        )
    }

    /// Adapted from the standard example.
    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id: nft::TokenId,
        token_owner_id: AccountId,
        token_metadata: TokenMetadata,
    ) -> nft::Token {
        self.assert_owner();

        // token_id must not contain the series delimiter
        ensure(
            !token_id.contains(SERIES_DELIMETER),
            Error::TokenIdWithSeriesDelimiter,
        );

        // standard minting
        self.tokens
            .internal_mint(token_id, token_owner_id, Some(token_metadata))
    }

    /// Adapted from the standard example.
    #[payable]
    pub fn nft_series_mint(
        &mut self,
        series_id: series::SeriesId,
        token_owner_id: AccountId,
        token_metadata: Option<TokenMetadata>,
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
        self.tokens
            .internal_mint(token.0, token_owner_id, Some(token_metadata))
    }
}

pub mod std_impls {
    use super::nft::{Token, TokenId};
    use super::Nft;
    use near_contract_standards as ncs;
    use near_sdk::{near_bindgen, AccountId, Promise, PromiseOrValue};

    #[cfg(not(target_arch = "wasm32"))]
    use crate::NftContract;

    ncs::impl_non_fungible_token_core!(Nft, tokens);
    ncs::impl_non_fungible_token_approval!(Nft, tokens);
    ncs::impl_non_fungible_token_enumeration!(Nft, tokens);
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

/// Identical to the standard example.
const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";
