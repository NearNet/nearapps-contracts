//! # Manage Owner and Logger
//! - (as usual)
//!
//! # Manage Registered nft contracts
//! - add nft contract (account, protocol)
//!   - protocols may differ, such as: do they log themselves?
//!     or should "we" log?
//! - remove nft contract (account, protocol)
//! - change nft contract (different account)
//!
//!
//! # Receive funds
//! - implement nft_on_transfer (may need multiple implementations)
//!   - check if the predecessor (the nft contract) is registered
//!   - add users funds
//! - needs to store owner's funds information
//!   - TODO
//!
//! # Send funds
//! - may need multiple functions as there are multiple protocols
//!
//!

use near_contract_standards::non_fungible_token as nft;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, TreeMap, UnorderedMap, UnorderedSet};
use near_sdk::env::predecessor_account_id;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::PromiseOrValue;
use near_sdk::{
    env, ext_contract, near_bindgen, AccountId, Balance, BorshStorageKey, Gas, PanicOnDefault,
    Promise,
};
use near_units::parse_gas;
use nearapps_log::{NearAppsAccount, NearAppsTags, NearAppsTagsContained};
use nearapps_near_ext::{ensure, types::JBalance, OrPanicStr};

pub mod error;

use error::Error;

const GAS_ON_SEND: Gas = Gas(parse_gas!("30 Tgas") as u64);

/// The AccountId of a Nft contract.
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct NftContractId(pub AccountId);

/// The accountId of a Nft token owner.
pub struct NftUserAccountId(pub AccountId);

/// Maps a Nft token owner per [`nft::TokenId`] of a certain Nft contract.
pub type TokenOwnerByTokenId = TreeMap<nft::TokenId, AccountId>;

/// Maps a set of [`nft::TokenId`] per [`NftContractId`].
pub type TokenSetForNftContract = UnorderedMap<NftContractId, UnorderedSet<nft::TokenId>>;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct SendNft {
    owner: AccountId,
    nearapps_logger: AccountId,
    //
    nft_protocols: UnorderedMap<NftContractId, NftProtocol>,
    nft_token_owners: UnorderedMap<NftContractId, TokenOwnerByTokenId>,
    nft_tokens_per_owner: LookupMap<AccountId, TokenSetForNftContract>,
}

#[derive(BorshSerialize, BorshStorageKey)]
#[allow(clippy::enum_variant_names)]
enum StorageKey {
    NftProtocols,
    NftTokenOwners,
    NftTokenOwnersInner,
    NftTokensPerOwner,
    NftTokensPerOwnerInner,
    NftTokensPerOwnerInnerInner,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum NftProtocol {
    Unknown,
    Standard,
    NearApps,
}

#[ext_contract(ext_self)]
trait OnSend {
    fn on_send(sender: AccountId, amount: JBalance, nearapps_tags: NearAppsTags);
}

// TODO: have the attached deposit cover for the account storage, in case
// it's needed.
#[near_bindgen]
impl SendNft {
    #[init]
    pub fn new(owner: AccountId, nearapps_logger: AccountId) -> Self {
        Self {
            owner,
            nearapps_logger,
            nft_protocols: UnorderedMap::new(StorageKey::NftProtocols),
            nft_token_owners: UnorderedMap::new(StorageKey::NftTokenOwners),
            nft_tokens_per_owner: LookupMap::new(StorageKey::NftTokensPerOwner),
        }
    }

    pub fn get_owner(&mut self) -> AccountId {
        self.owner.clone()
    }

    pub fn change_owner(&mut self, new_owner: AccountId) {
        self.assert_owner();
        self.owner = new_owner;
    }

    // TODO: register SendNft contract as a user on the Nft contract
    // that is being added
    pub fn add_nft_protocol(&mut self, nft: AccountId, protocol: NftProtocol) {
        let previous = self.nft_protocols.insert(&NftContractId(nft), &protocol);
        ensure(previous.is_none(), Error::NftProtocolAlreadyIncluded);
    }

    pub fn change_nft_protocol(&mut self, nft: AccountId, new_protocol: NftProtocol) {
        let previous = self
            .nft_protocols
            .insert(&NftContractId(nft), &new_protocol);
        ensure(previous.is_some(), Error::NftProtocolNotIncluded);
    }

    // TODO: use state machine for "start removing a nft protocol" ?
    // - users may still have units of the nft that is being removed
    // - to send nft back to owners, a paginated operation is necessary
    // TODO: unregister SendNft contract as a user on the Nft contract
    // that is being removed
    pub fn remove_nft_protocol(&mut self, nft: AccountId) {
        // TODO: ensure that no user has that nft?

        unimplemented!()
    }

    pub fn get_nft_protocols(
        &self,
        from_index: Option<U128>,
        limit: Option<u16>,
    ) -> Vec<(NftContractId, NftProtocol)> {
        let from_index = from_index.unwrap_or_else(|| 0.into()).0 as usize;
        let limit = limit.unwrap_or(u16::MAX) as usize;
        self.nft_protocols
            .iter()
            .skip(from_index)
            .take(limit)
            .collect()
    }

    #[payable]
    pub fn send_logged(
        &mut self,
        nft_contract: AccountId,
        sender: AccountId,
        receiver: AccountId,
        nearapps_tags: NearAppsTags,
    ) -> Promise {
        /*
        let nft_contract = NftContractId(env::predecessor_account_id());


        match self.nft_protocols.get(&nft_contract) {
            // unkown or not registered nft/protocol,
            // indicate that the transfer should be undone
            None | Some(NftProtocol::Unknown) => {
                return PromiseOrValue::Value(true);
            }

            // the logging has presumably not been executed,
            // but the logging information must still be
            // present inside the `msg`
            Some(NftProtocol::Standard) => {
                use near_sdk::serde_json::from_str;

                let nearapps_tags = from_str::<NearAppsTagsContained>(&msg)
                    .or_panic_str(Error::NearAppsTagsMissing)
                    .nearapps_tags;

                // best-effort call for nearapps log
                let _ = self.log(nearapps_tags);
            }

            // the logging was presumably already executed
            Some(NftProtocol::NearApps) => {}
        };

        let mut token_owners = self
            .nft_token_owners
            .get(&nft_contract)
            // should not fail as this "is" already verified at the start
            // of this function
            .unwrap();
        let previous = token_owners.insert(&token_id, &previous_owner_id);
        ensure(previous.is_none(), Error::NftTokenAlreadyOwned);

        let tokens_per_owner = self
            .nft_tokens_per_owner
            .get(&previous_owner_id)
            // user must already be registered
            .unwrap();

        let mut tokens_per_owner_inner = tokens_per_owner
            .get(&nft_contract)
            // the nft contract for the user must already be registered
            .unwrap();

        let had_previous = tokens_per_owner_inner.insert(&token_id);
        ensure(!had_previous, Error::UserAlreadyOwnedTheNftToken);

        PromiseOrValue::Value(false)

        // let send = Promise::new(receiver).transfer(amount);
        // let on_send = ext_self::on_send(
        //     //
        //     sender,
        //     JBalance(amount),
        //     nearapps_tags,
        //     env::current_account_id(),
        //     0,
        //     GAS_ON_SEND,
        // );

        // send.then(on_send)

        */
        todo!()
    }
}

use nft::core::NonFungibleTokenReceiver;

#[near_bindgen]
impl NonFungibleTokenReceiver for SendNft {
    fn nft_on_transfer(
        &mut self,
        sender_id: AccountId,
        previous_owner_id: AccountId,
        token_id: nft::TokenId,
        msg: String,
    ) -> PromiseOrValue<bool> {
        let _sender_id = sender_id;
        let nft_contract = NftContractId(env::predecessor_account_id());

        match self.nft_protocols.get(&nft_contract) {
            // unkown or not registered nft/protocol,
            // indicate that the transfer should be undone
            None | Some(NftProtocol::Unknown) => {
                return PromiseOrValue::Value(true);
            }

            // the logging has presumably not been executed,
            // but the logging information must still be
            // present inside the `msg`
            Some(NftProtocol::Standard) => {
                use near_sdk::serde_json::from_str;

                let nearapps_tags = from_str::<NearAppsTagsContained>(&msg)
                    .or_panic_str(Error::NearAppsTagsMissing)
                    .nearapps_tags;

                // best-effort call for nearapps log
                let _ = self.log(nearapps_tags);
            }

            // the logging was presumably already executed
            Some(NftProtocol::NearApps) => {}
        };

        let mut token_owners = self
            .nft_token_owners
            .get(&nft_contract)
            // should not fail as this "is" already verified at the start
            // of this function
            .unwrap();
        let previous = token_owners.insert(&token_id, &previous_owner_id);
        ensure(previous.is_none(), Error::NftTokenAlreadyOwned);

        let tokens_per_owner = self
            .nft_tokens_per_owner
            .get(&previous_owner_id)
            // user must already be registered
            .unwrap();

        let mut tokens_per_owner_inner = tokens_per_owner
            .get(&nft_contract)
            // the nft contract for the user must already be registered
            .unwrap();

        let had_previous = tokens_per_owner_inner.insert(&token_id);
        ensure(!had_previous, Error::UserAlreadyOwnedTheNftToken);

        PromiseOrValue::Value(false)
    }
}

pub trait Owner {
    fn assert_owner(&self);
}

impl Owner for SendNft {
    fn assert_owner(&self) {
        ensure(env::predecessor_account_id() == self.owner, Error::NotOwner)
    }
}

impl nearapps_log::NearAppsAccount for SendNft {
    fn nearapps_account(&self) -> near_sdk::AccountId {
        self.nearapps_logger.clone()
    }
}
