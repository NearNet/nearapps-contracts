use crate::error::Error;
use crate::{NftSeries, Owner, StorageKey};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedSet;
use near_sdk::json_types::U64;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId};
use nearapps_near_ext::{ensure, OrPanicStr};
use serde_with::{serde_as, FromInto};

pub const SERIES_DELIMETER: char = ':';

#[cfg(not(target_arch = "wasm32"))]
use crate::NftSeriesContract;

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
impl NftSeries {
    /// Shows how many series were created.
    pub fn nft_series_supply(&self) -> U64 {
        self.series.len().into()
    }

    /// Gets information on a series.
    pub fn nft_series_get(&self, series_id: SeriesId) -> Series {
        self.series
            .get(&series_id)
            .or_panic_str(Error::MissingSeries)
    }

    /// Get minted tokens from a series.
    pub(crate) fn nft_series_get_minted_tokens(
        &self,
        series_id: SeriesId,
    ) -> UnorderedSet<SeriesTokenIndex> {
        self.series_minted_tokens
            .get(&series_id)
            .or_panic_str(Error::MissingSeries)
    }

    /// Get minted tokens from a series.
    pub fn nft_series_get_minted_tokens_vec(
        &self,
        series_id: SeriesId,
        from_index: Option<near_sdk::json_types::U128>,
        limit: Option<u16>,
    ) -> Vec<SeriesTokenIndex> {
        let from_index = from_index.unwrap_or_else(|| 0.into()).0 as usize;
        let limit = limit.unwrap_or(u16::MAX) as usize;
        self.nft_series_get_minted_tokens(series_id)
            .iter()
            .skip(from_index)
            .take(limit)
            .collect()
    }

    /// Creates a new NFT series.
    pub fn nft_series_create(
        &mut self,
        name: SeriesName,
        capacity: SeriesTokenIndex,
        creator: AccountId,
    ) -> SeriesId {
        self.assert_owner();

        let id = self.next_series_id;
        self.next_series_id.0 += 1;

        let series = Series {
            id,
            name: name.clone(),
            creator: creator.clone(),
            len: SeriesTokenIndex(0),
            capacity,
            is_mintable: true,
        };

        ensure(
            self.series_minted_tokens.get(&id).is_none(),
            Error::SeriesAlreadyCreated,
        );

        self.series_minted_tokens.insert(
            &id,
            &UnorderedSet::new(StorageKey::TokensBySeriesInner { series_id: id }),
        );

        self.series.insert(&id, &series);

        // ~NEP171 (Non-Fungible Token Event) 1.0.0(?)
        // note: event:nft_series_create is not on the std 1.0.0
        env::log_str(&format!("EVENT_JSON:{{\"standard\":\"nep171\",\"version\":\"1.0.0\",\"event\":\"nft_series_create\",\"data\":[{{\"owner_id\":\"{}\",\"series\":[\"{}:{}\"]}}]}}", creator.as_str(), name, id.0));

        id
    }

    /// Sets whether a series is mintable or not.
    pub fn nft_series_set_mintable(
        //
        &mut self,
        series_id: SeriesId,
        is_mintable: bool,
    ) {
        self.assert_owner();
        let mut series = self.nft_series_get(series_id);
        if series.is_mintable != is_mintable {
            series.is_mintable = is_mintable;
            self.series.insert(&series_id, &series);
        }
    }

    /// Sets the token capacity (the token max length) of a
    /// series.
    pub fn nft_series_set_capacity(
        //
        &mut self,
        series_id: SeriesId,
        capacity: SeriesTokenIndex,
    ) {
        self.assert_owner();

        let mut series = self.nft_series_get(series_id);
        ensure(capacity >= series.len, Error::SeriesNotEnoughtCapacity);

        series.capacity = capacity;

        if capacity == series.len {
            series.is_mintable = false;
        }

        self.series.insert(&series_id, &series);
    }
}
