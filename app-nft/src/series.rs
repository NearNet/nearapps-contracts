use crate::error::{ensure, Error, OrPanicStr};
use crate::{Contract, Owner, StorageKey};
use near_contract_standards::non_fungible_token as nft;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, UnorderedSet};
use near_sdk::{
    env, near_bindgen, require, AccountId, BorshStorageKey, PanicOnDefault, Promise, PromiseOrValue,
};
use nft::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};

pub const SERIES_DELIMETER: char = ':';

#[cfg(not(target_arch = "wasm32"))]
use crate::ContractContract;

pub type SeriesId = u64;
pub type SeriesName = String;

/// The index of a token inside a series.
/// eg. `0`, `1`, `2`.  
/// The maximum value for a token's index will be
/// `capacity - 1`.
///
/// Can also represent the quantity of tokens minted.  
/// eg. `0` means no tokens have been minted.  
/// eg. `1` means that a single token has been minted, which
/// will have the index of `0`.  
/// The maximum value for a serie's token `len` will be
/// `capacity`.
///
/// Can also represent the maximum capacity of a series.  
/// eg. `0` won't be able to have any tokens.  
/// eg. `1` will be able to have a single token,
/// which will have the index of `0`.
pub type SeriesTokenIndex = usize;

/// A token name produced from a series.
///
/// See [`TokenSeriesId::new()`] for more info.
pub struct TokenSeriesId(pub String);

impl TokenSeriesId {
    /// Creates a new [`nft::TokenId`] based on the series names,
    /// [`SERIES_DELIMETER`], and some `index`.
    pub fn new(name: SeriesName, index: SeriesTokenIndex) -> Self {
        Self(format!("{}{}{}", name, SERIES_DELIMETER, index))
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Series {
    pub id: SeriesId,
    pub name: String,
    pub creator: AccountId,
    pub len: SeriesTokenIndex,
    /// The maximum number of token units that this series can mint.
    ///
    /// eg. `0` means that it will never mint any token.
    pub capacity: SeriesTokenIndex,
    pub is_mintable: bool,
    pub minted_tokens: UnorderedSet<SeriesTokenIndex>,
}

impl Series {
    pub fn next_token(&mut self) -> TokenSeriesId {
        ensure(self.is_mintable, Error::SeriesNotMintable);
        ensure(self.len < self.capacity, Error::SeriesMaxCapacity);

        let token = TokenSeriesId::new(self.name.clone(), self.len);
        self.len += 1;

        if self.len == self.capacity {
            self.is_mintable = false;
        }

        token
    }
}

#[near_bindgen]
impl Contract {
    pub fn nft_series_create(
        &mut self,
        name: SeriesName,
        capacity: SeriesTokenIndex,
        creator: AccountId,
    ) -> SeriesId {
        self.assert_owner();

        let id = self.next_series_id;
        self.next_series_id += 1;

        let series = Series {
            id,
            name,
            creator,
            len: 0,
            capacity,
            is_mintable: true,
            minted_tokens: UnorderedSet::new(StorageKey::TokensBySeries { series_id: id }),
        };

        self.series.insert(&id, &series);
        id
    }
}
