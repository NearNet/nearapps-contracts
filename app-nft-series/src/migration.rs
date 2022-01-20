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

/// This struct is from version `924a055`.
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct NftSeriesPrevious {
    tokens: nft::NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    series: UnorderedMap<series::SeriesId, series::Series>,
    next_series_id: series::SeriesId,
    series_minted_tokens: UnorderedMap<series::SeriesId, UnorderedSet<series::SeriesTokenIndex>>,
}

#[near_bindgen]
impl NftSeries {
    /// `nearapps_logger` should contain the `log` function.
    #[private]
    #[init(ignore_state)]
    pub fn migrate(nearapps_logger: AccountId) -> Self {
        let previous_state: NftSeriesPrevious = env::state_read().unwrap();
        let mut owner_ids = UnorderedSet::new(crate::StorageKey::Owners);
        owner_ids.insert(&previous_state.tokens.owner_id);
        Self {
            tokens: previous_state.tokens,
            metadata: previous_state.metadata,
            series: previous_state.series,
            next_series_id: previous_state.next_series_id,
            series_minted_tokens: previous_state.series_minted_tokens,
            owner_ids,
            nearapps_logger,
        }
    }
}
