use crate::error::{ensure, Error, OrPanicStr};
use crate::series;
use crate::SERIES_DELIMETER;
use crate::{Contract, Owner, StorageKey};
use near_contract_standards::non_fungible_token as nft;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::{
    env, near_bindgen, require, AccountId, BorshStorageKey, PanicOnDefault, Promise, PromiseOrValue,
};
use nft::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};

#[cfg(not(target_arch = "wasm32"))]
use crate::ContractContract;

#[near_bindgen]
impl Contract {
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

    #[payable]
    pub fn nft_series_mint(
        &mut self,
        series_id: series::SeriesId,
        token_owner_id: AccountId,
        token_metadata: Option<TokenMetadata>,
    ) -> nft::Token {
        self.assert_owner();

        let mut series = self
            .series
            .get(&series_id)
            .or_panic_str(Error::MissingSeries);

        let token = series.next_token();
        self.series.insert(&series_id, &series);

        let token_metadata = token_metadata.unwrap_or_else(|| TokenMetadata {
            title: Some(token.0.clone()),
            description: Some(format!(
                "Token #{} from series {}",
                series.len - 1,
                series.name
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
