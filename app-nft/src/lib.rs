use near_contract_standards::non_fungible_token as nft;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, UnorderedMap, UnorderedSet};
use near_sdk::{env, near_bindgen, require, AccountId, BorshStorageKey, PanicOnDefault};
use nft::metadata::{NonFungibleTokenMetadataProvider, NFT_METADATA_SPEC};

pub mod error;
pub mod series;
pub mod utils;

use error::{ensure, Error, OrPanicStr};
pub use series::SERIES_DELIMETER;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Nft {
    tokens: nft::NonFungibleToken,
    metadata: LazyOption<nft::metadata::NFTContractMetadata>,
    series: UnorderedMap<series::SeriesId, series::Series>,
    next_series_id: series::SeriesId,
    series_minted_tokens: UnorderedMap<series::SeriesId, UnorderedSet<series::SeriesTokenIndex>>,
    allowed_claims: UnorderedMap<
        series::SeriesId,
        UnorderedMap<AccountId, Option<nft::metadata::TokenMetadata>>,
    >,
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
    AllowedClaims,
    AllowedClaimsInner { series_id: series::SeriesId },
}

#[near_bindgen]
impl Nft {
    /// Initializes the nft contract with some `metadata`.
    ///
    /// Note: Adapted from the standard example.
    #[init]
    pub fn new(
        //
        owner_id: AccountId,
        metadata: nft::metadata::NFTContractMetadata,
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
            allowed_claims: UnorderedMap::new(StorageKey::AllowedClaims),
        }
    }

    /// Initializes the nft contract with some placeholder metadata.
    ///
    /// Note: Identical to the standard example.
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        Self::new(
            owner_id,
            nft::metadata::NFTContractMetadata {
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

    /// Mints a new non-series token `token_id` with the metadata
    /// `token_metadata` and makes it owned by `token_owner_id`.
    ///
    /// Can only be called by the contract owner.  
    /// `token_id` must not contain the [`SERIES_DELIMETER`].
    ///
    /// Note: Adapted from the standard example.
    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id: nft::TokenId,
        token_owner_id: AccountId,
        token_metadata: nft::metadata::TokenMetadata,
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

    /// Mints a new series-token with the optional metadata
    /// `token_metadata` and makes it owned by `token_owner_id`.
    ///
    /// Can only be called by the contract owner.  
    ///
    /// Note: Adapted from the standard example.
    #[payable]
    pub fn nft_series_mint(
        &mut self,
        series_id: series::SeriesId,
        token_owner_id: AccountId,
        token_metadata: Option<nft::metadata::TokenMetadata>,
    ) -> nft::Token {
        self.assert_owner();
        self.internal_nft_series_mint(series_id, token_owner_id, token_metadata)
    }

    /// Allows `user_id` to claim a new series-token from `series_id`.
    ///
    /// Can only be called by the contract owner.  
    #[payable]
    pub fn nft_series_allow_claim(
        &mut self,
        series_id: series::SeriesId,
        user_id: AccountId,
        token_metadata: Option<nft::metadata::TokenMetadata>,
    ) {
        self.assert_owner();

        let mut allowed = self
            .allowed_claims
            .get(&series_id)
            .or_panic_str(Error::MissingSeries);

        let previous_allow = allowed.insert(&user_id, &token_metadata);
        if previous_allow.is_some() {
            Error::AlreadyAllowedToClaim.panic()
        }

        self.allowed_claims.insert(&series_id, &allowed);
    }

    /// Claims a series-token from `series_id`.
    #[payable]
    pub fn nft_series_claim(&mut self, series_id: series::SeriesId) -> nft::Token {
        let user_id = env::predecessor_account_id();

        let mut allowed = self
            .allowed_claims
            .get(&series_id)
            .or_panic_str(Error::MissingSeries);

        let allowed_metadata = allowed.remove(&user_id);
        // the outer Option to be None indicates that the user
        // wasn't allowed
        if allowed_metadata.is_none() {
            Error::UserNotAllowedToClaim.panic()
        }
        self.allowed_claims.insert(&series_id, &allowed);

        let token_metadata = allowed_metadata.flatten();

        self.internal_nft_series_mint(series_id, user_id, token_metadata)
    }
}

impl Nft {
    /// Adapted from the standard example.
    ///
    /// Assumes `token_owner_id` to be [`env::predecessor_account_id()`],
    /// and that the account is allowed to mint.
    fn internal_nft_series_mint(
        &mut self,
        series_id: series::SeriesId,
        token_owner_id: AccountId,
        token_metadata: Option<nft::metadata::TokenMetadata>,
    ) -> nft::Token {
        let mut series = self.nft_series_get(series_id);
        let mut minted_tokens = self.nft_series_get_minted_tokens(series_id);

        let token = series.next_token();

        // updates series
        self.series.insert(&series_id, &series);

        // updates minted_tokens
        let token_index = series.last_token_index().unwrap();
        minted_tokens.insert(&token_index);
        self.series_minted_tokens.insert(&series_id, &minted_tokens);

        let token_metadata =
            new_token_metadata(&token, token_index, &series, token_metadata.as_ref());

        // standard minting
        self.tokens
            .internal_mint(token.0, token_owner_id, Some(token_metadata))
    }
}

/// Based on a `token_metadata` value, applies default values for some
/// fields.
pub fn new_token_metadata(
    token: &series::TokenSeriesId,
    token_index: series::SeriesTokenIndex,
    series: &series::Series,
    token_metadata: Option<&nft::metadata::TokenMetadata>,
) -> nft::metadata::TokenMetadata {
    let title = token_metadata
        .and_then(|m| m.title.as_ref())
        .unwrap_or(&token.0);
    let description = token_metadata
        .and_then(|m| m.description.clone())
        .unwrap_or_else(|| format!("Token #{} from series {}", token_index.0, series.name));
    let media = token_metadata.and_then(|m| m.media.as_ref());
    let media_hash = token_metadata.and_then(|m| m.media_hash.as_ref());
    let copies = token_metadata
        .and_then(|m| m.copies)
        .unwrap_or(series.capacity.0);
    let issued_at = token_metadata
        .and_then(|m| m.issued_at.clone())
        .unwrap_or_else(|| env::block_timestamp().to_string());
    let expires_at = token_metadata.and_then(|m| m.expires_at.as_ref());
    let starts_at = token_metadata.and_then(|m| m.starts_at.as_ref());
    let updated_at = token_metadata.and_then(|m| m.updated_at.as_ref());
    let extra = token_metadata.and_then(|m| m.extra.as_ref());
    let reference = token_metadata.and_then(|m| m.reference.as_ref());
    let reference_hash = token_metadata.and_then(|m| m.reference_hash.as_ref());

    nft::metadata::TokenMetadata {
        title: Some(title.clone()),
        description: Some(description),
        media: media.cloned(),
        media_hash: media_hash.cloned(),
        copies: Some(copies),
        issued_at: Some(issued_at),
        expires_at: expires_at.cloned(),
        starts_at: starts_at.cloned(),
        updated_at: updated_at.cloned(),
        extra: extra.cloned(),
        reference: reference.cloned(),
        reference_hash: reference_hash.cloned(),
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
    fn nft_metadata(&self) -> nft::metadata::NFTContractMetadata {
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
const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg%20xmlns%3D'http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg'%3E%3Crect%20width%3D'50'%20height%3D'50'%20%2F%3E%3C%2Fsvg%3E";
