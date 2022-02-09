#![allow(clippy::let_and_return)]
#![allow(clippy::too_many_arguments)]

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault};
use nearapps_near_ext::ensure;

#[allow(unused_imports)]
use near_contract_standards::non_fungible_token as nft;
// note: import used for documentation

pub mod error;
pub mod ext_nft;
pub mod nft_receiver;
pub mod owner;
pub mod protocol;
pub mod send;
pub mod types;
pub mod user;
pub mod version;

use error::Error;
use types::{NftContractId, NftUserAccountId, Sha256From, TokenSetForNftContract, UserByTokenId};

#[allow(unused_imports)]
use types::TokenStatus;
// note: import used for documentation

pub use types::NftProtocol;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct SendNft {
    owner: AccountId,
    nearapps_logger: AccountId,

    /// [`NftContractId`]
    /// -> [`NftProtocol`].
    nft_protocols: UnorderedMap<
        //
        NftContractId,
        NftProtocol,
    >,

    /// [`NftContractId`]
    /// -> [`nft::TokenId`]
    /// -> ([`NftUserAccountId`], [`TokenStatus`] ).
    nft_token_users: UnorderedMap<
        //
        NftContractId,
        UserByTokenId,
    >,

    /// [`NftUserAccountId`]
    /// -> [`NftContractId`]
    /// -> [`nft::TokenId`]
    /// -> [`TokenStatus`].
    nft_tokens_per_user: LookupMap<
        //
        NftUserAccountId,
        TokenSetForNftContract,
    >,
}

#[derive(BorshSerialize, BorshStorageKey)]
#[allow(clippy::enum_variant_names)]
enum StorageKey {
    NftProtocols,
    NftTokenUsers,
    NftTokenUsersInner {
        /// As to why a Sha256 must be used in here, check
        /// [this SO](https://stackoverflow.com/questions/63277036) question.
        contract_id: Sha256From<NftContractId>,
    },
    NftTokensPerUser,
    NftTokensPerUserInner {
        user_id: NftUserAccountId,
    },
    NftTokensPerUserInnerInner {
        user_id: NftUserAccountId,
        contract_id: NftContractId,
    },
}

// TODO: have the attached deposit cover for the account storage, in case
// it's needed.
#[near_bindgen]
impl SendNft {
    /// Initializes the contract.
    #[init]
    pub fn new(owner: AccountId, nearapps_logger: AccountId) -> Self {
        ensure(!env::state_exists(), Error::AlreadyInitialized);
        Self {
            owner,
            nearapps_logger,
            nft_protocols: UnorderedMap::new(StorageKey::NftProtocols),
            nft_token_users: UnorderedMap::new(StorageKey::NftTokenUsers),
            nft_tokens_per_user: LookupMap::new(StorageKey::NftTokensPerUser),
        }
    }
}

impl nearapps_log::NearAppsAccount for SendNft {
    fn nearapps_account(&self) -> AccountId {
        self.nearapps_logger.clone()
    }
}
