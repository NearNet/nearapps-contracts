use crate::error::{ensure, Error, OrPanicStr};
use crate::{NearAppsTags, Nft, Owner, StorageKey};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedSet;
use near_sdk::json_types::U64;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{near_bindgen, AccountId};
use serde_with::{serde_as, FromInto};

pub const SERIES_DELIMETER: char = ':';

#[cfg(not(target_arch = "wasm32"))]
use crate::NftContract;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Series {
    pub id: SeriesId,
    pub name: String,
    pub creator: AccountId,
    pub len: SeriesTokenIndex,
    /// The maximum number of token units that this series can have minted.
    ///
    /// eg. `0` means that it will never mint any token.
    pub capacity: SeriesTokenIndex,
    pub is_mintable: bool,
}

#[serde_as]
#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, Copy)]
#[serde(transparent)]
#[serde(crate = "near_sdk::serde")]
pub struct SeriesId(
    #[serde_as(as = "FromInto<U64>")]
    //
    pub u64,
);

pub type SeriesName = String;

/// The index of a token inside a series.
/// eg. `0`, `1`, `2`.  
/// The maximum value for a token's index will be
/// `capacity - 1`.
///
/// Can also represent `len`, the quantity of tokens minted.  
/// eg. `0` means no tokens have been minted.  
/// eg. `1` means that a single token has been minted, which
/// will have the index of `0`.  
/// The maximum value for a serie's token `len` will be
/// `capacity`.
///
/// Can also represent the maximum `capacity` of a series.  
/// eg. `0` won't be able to have any tokens.  
/// eg. `1` will be able to have a single token,
/// which will have the index of `0`.
#[serde_as]
#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, PartialEq, PartialOrd, Clone, Copy,
)]
#[serde(transparent)]
#[serde(crate = "near_sdk::serde")]
pub struct SeriesTokenIndex(
    #[serde_as(as = "FromInto<U64>")]
    //
    pub u64,
);

/// A token name produced from a series.
///
/// See [`TokenSeriesId::new()`] for more info.
pub struct TokenSeriesId(pub String);

impl TokenSeriesId {
    /// Creates a new [`nft::TokenId`] based on the series' name, the
    /// series' id, [`SERIES_DELIMETER`], and some `index`.
    pub fn new(name: SeriesName, series_id: SeriesId, token_index: SeriesTokenIndex) -> Self {
        Self(format!(
            "{}{}{}{}{}",
            //
            name,
            SERIES_DELIMETER,
            series_id.0,
            SERIES_DELIMETER,
            token_index.0
        ))
    }
}

impl Series {
    pub fn next_token(&mut self) -> TokenSeriesId {
        ensure(self.is_mintable, Error::SeriesNotMintable);
        ensure(self.len < self.capacity, Error::SeriesMaxCapacity);

        let token = TokenSeriesId::new(self.name.clone(), self.id, self.len);
        self.len.0 += 1;

        if self.len == self.capacity {
            self.is_mintable = false;
        }

        token
    }

    pub fn last_token_index(&self) -> Option<SeriesTokenIndex> {
        if self.len.0 > 0 {
            Some(SeriesTokenIndex(self.len.0 - 1))
        } else {
            None
        }
    }
}

#[near_bindgen]
impl Nft {
    pub fn nft_series_supply(&self) -> String {
        self.series.len().to_string()
    }

    pub fn nft_series_get(&self, series_id: SeriesId) -> Series {
        self.series
            .get(&series_id)
            .or_panic_str(Error::MissingSeries)
    }

    pub(crate) fn nft_series_get_minted_tokens(
        &self,
        series_id: SeriesId,
    ) -> UnorderedSet<SeriesTokenIndex> {
        self.series_minted_tokens
            .get(&series_id)
            .or_panic_str(Error::MissingSeries)
    }

    pub fn nft_series_get_minted_tokens_vec(&self, series_id: SeriesId) -> Vec<SeriesTokenIndex> {
        self.nft_series_get_minted_tokens(series_id)
            .iter()
            .collect()
    }

    pub fn nft_series_create_logged(
        &mut self,
        name: SeriesName,
        capacity: SeriesTokenIndex,
        creator: AccountId,
        nearapps_tags: NearAppsTags,
    ) -> SeriesId {
        self.assert_owner();

        let id = self.next_series_id;
        self.next_series_id.0 += 1;

        let series = Series {
            id,
            name,
            creator,
            len: SeriesTokenIndex(0),
            capacity,
            is_mintable: true,
        };

        self.series_minted_tokens.insert(
            &id,
            &UnorderedSet::new(StorageKey::TokensBySeriesInner { series_id: id }),
        );

        self.series.insert(&id, &series);

        nearapps_tags.log_str();
        id
    }

    pub fn nft_series_set_mintable(
        //
        &mut self,
        series_id: SeriesId,
        is_mintable: bool,
        nearapps_tags: NearAppsTags,
    ) {
        self.assert_owner();
        let mut series = self.nft_series_get(series_id);
        if series.is_mintable != is_mintable {
            series.is_mintable = is_mintable;
            self.series.insert(&series_id, &series);
        }
        nearapps_tags.log_str();
    }

    pub fn nft_series_set_capacity(
        //
        &mut self,
        series_id: SeriesId,
        capacity: SeriesTokenIndex,
        nearapps_tags: NearAppsTags,
    ) {
        self.assert_owner();

        let mut series = self.nft_series_get(series_id);
        ensure(capacity >= series.len, Error::SeriesNotEnoughtCapacity);

        series.capacity = capacity;

        if capacity == series.len {
            series.is_mintable = false;
        }

        self.series.insert(&series_id, &series);
        nearapps_tags.log_str();
    }
}
