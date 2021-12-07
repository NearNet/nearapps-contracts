use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::{env, near_bindgen, require, AccountId, BorshStorageKey, PanicOnDefault};

pub mod error;
pub mod std_nft;

use error::Error;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
}

pub trait Owner {
    fn assert_owner(&self);
}

impl Owner for Contract {
    fn assert_owner(&self) {
        if env::predecessor_account_id() != self.tokens.owner_id {
            Error::NotOwner.panic()
        }
    }
}
