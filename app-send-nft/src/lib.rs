#![allow(clippy::let_and_return)]
#![allow(clippy::too_many_arguments)]

use near_contract_standards::non_fungible_token as nft;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, TreeMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::U128;
use near_sdk::PromiseOrValue;
use near_sdk::{
    env, ext_contract, near_bindgen, AccountId, Balance, BorshStorageKey, Gas, PanicOnDefault,
    Promise,
};
use near_units::{parse_gas, parse_near};
use nearapps_log::{NearAppsAccount, NearAppsTags, NearAppsTagsContained};
use nearapps_near_ext::{ensure, OrPanicStr};

pub mod error;
pub mod types;

use error::Error;
use types::{NftContractId, NftUserAccountId, Sha256From, TokenSetForNftContract, UserByTokenId};

pub use types::NftProtocol;

const GAS_ON_SEND: Gas = Gas(parse_gas!("30 Tgas") as u64);

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct SendNft {
    owner: AccountId,
    nearapps_logger: AccountId,

    /// [`NftContractId`] -> [`NftProtocol`].
    nft_protocols: UnorderedMap<
        //
        NftContractId,
        NftProtocol,
    >,

    /// [`NftContractId`] -> [`nft::TokenId`] -> [`NftUserAccountId`].
    nft_token_users: UnorderedMap<
        //
        NftContractId,
        UserByTokenId,
    >,

    /// [`NftUserAccountId`] -> [`NftContractId`] -> [`nft::TokenId`].
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

#[ext_contract(ext_self)]
trait OnSend {
    /// Confirms that the transfer hasn't panicked, re-registering
    /// the token information in case of failure.
    ///
    /// This won't log the tags because it is assumed
    /// that this has already happened by the nft contract.
    ///
    /// This should be a callback to the nearapps-like nft transfer
    /// (which should log on transfer).
    fn on_send(
        nft_contract: NftContractId,
        sender: NftUserAccountId,
        // receiver: NftUserAccountId,
        token_id: nft::TokenId,
        // approval_id: Option<u64>,
        // memo: Option<String>,
        nearapps_tags: NearAppsTags,
    );

    /// Confirms that the transfer hasn't panicked, re-registering
    /// the token information in case of failure.
    ///
    /// This will log the tags because it is assumed
    /// that this hasn't happened yet.
    ///
    /// This should be a callback to a standard nft transfer.
    fn on_send_logged(
        nft_contract: NftContractId,
        sender: NftUserAccountId,
        receiver: NftUserAccountId,
        token_id: nft::TokenId,
        // approval_id: Option<u64>,
        // memo: Option<String>,
        nearapps_tags: NearAppsTags,
    );

    fn on_send_call(
        nft_contract: NftContractId,
        sender: NftUserAccountId,
        token_id: nft::TokenId,
        nearapps_tags: NearAppsTags,
    ) -> bool;

    fn on_send_call_logged(
        nft_contract: NftContractId,
        sender: NftUserAccountId,
        token_id: nft::TokenId,
        nearapps_tags: NearAppsTags,
    ) -> bool;
}

#[allow(clippy::too_many_arguments)]
pub mod ext_nft {
    pub mod nearapps {
        use crate::NftUserAccountId;
        use near_contract_standards::non_fungible_token as nft;
        use near_sdk::ext_contract;
        use nearapps_log::NearAppsTags;

        #[ext_contract]
        trait Transfer {
            fn nft_transfer_logged(
                receiver_id: NftUserAccountId,
                token_id: nft::TokenId,
                approval_id: Option<u64>,
                memo: Option<String>,
                nearapps_tags: NearAppsTags,
            );
            fn nft_transfer_call_logged(
                receiver_id: NftUserAccountId,
                token_id: nft::TokenId,
                approval_id: Option<u64>,
                memo: Option<String>,
                msg: String,
                nearapps_tags: NearAppsTags,
            ) -> PromiseOrValue<bool>;
        }
    }

    pub mod standard {
        use crate::NftUserAccountId;
        use near_contract_standards::non_fungible_token as nft;
        use near_sdk::ext_contract;

        #[ext_contract]
        trait Transfer {
            fn nft_transfer(
                receiver_id: NftUserAccountId,
                token_id: nft::TokenId,
                approval_id: Option<u64>,
                memo: Option<String>,
            );
            fn nft_transfer_call(
                receiver_id: NftUserAccountId,
                token_id: nft::TokenId,
                approval_id: Option<u64>,
                memo: Option<String>,
                msg: String,
            ) -> PromiseOrValue<bool>;
        }
    }
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

    pub fn get_owner(&self) -> AccountId {
        self.owner.clone()
    }

    pub fn change_owner(&mut self, new_owner: AccountId) {
        self.assert_owner();
        self.owner = new_owner;
    }


    /// Adds a new Nft contract and registers it's protocol.
    pub fn add_nft_protocol(&mut self, nft: NftContractId, protocol: NftProtocol) {
        self.assert_owner();

        let previous = self.nft_protocols.insert(&nft, &protocol);
        ensure(previous.is_none(), Error::NftProtocolAlreadyIncluded);

        let new_token_tree = TreeMap::new(StorageKey::NftTokenUsersInner {
            contract_id: Sha256From::new(&nft),
        });
        ensure(
            self.nft_token_users.get(&nft).is_none(),
            Error::NftProtocolAlreadyIncluded,
        );
        let _previous = self.nft_token_users.insert(&nft, &new_token_tree);
    }

    /// Edits a Nft contract's protocol.
    pub fn change_nft_protocol(&mut self, nft: NftContractId, new_protocol: NftProtocol) {
        self.assert_owner();

        let previous = self.nft_protocols.insert(&nft, &new_protocol);
        ensure(previous.is_some(), Error::NftProtocolNotIncluded);
    }

    /// Removes a nft protocol.
    ///     
    /// Note: No user can be owning any token on this nft
    /// contract.
    #[allow(unused_variables)]
    pub fn remove_nft_protocol(&mut self, nft: NftContractId) {
        self.assert_owner();

        let tokens_per_user = self
            .nft_token_users
            .get(&nft)
            .or_panic_str(&Error::NftProtocolNotIncluded);

        ensure(
            tokens_per_user.is_empty(),
            Error::NftTokensStillOwnedByUsers,
        );

        // actual removal is not yet implemented
        unimplemented!()
    }

    /// Shows registered nft contracts and their protocols.
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

    /// Register a user on this contract.
    ///
    /// The user must also have specific nft contracts enabled
    /// for them. See [`Self::enable_nft_for_user`].
    #[payable]
    pub fn add_user(&mut self, user: NftUserAccountId) {
        self.assert_owner();

        ensure(
            self.nft_tokens_per_user.get(&user).is_none(),
            Error::UserAlreadyRegistered,
        );

        let token_set_per_nft = UnorderedMap::new(StorageKey::NftTokensPerUserInner {
            user_id: user.clone(),
        });
        let _previous = self.nft_tokens_per_user.insert(&user, &token_set_per_nft);
    }

    /// Un-register a user from this contract.
    ///
    /// First the user must not be owning any token, then they
    /// can be removed from the contract.
    pub fn remove_user(&mut self, user: NftUserAccountId) {
        self.assert_owner();

        let tokens_per_contract = self
            .nft_tokens_per_user
            .get(&user)
            .or_panic_str(Error::MissingUser);

        // check if user doesn't have any registered
        // nft contract that they can interact with
        assert_eq!(tokens_per_contract.len(), 0);

        // actual removal is not yet implemented
        unimplemented!()
    }

    /// Enables a user to be able to interact with a given nft
    /// contract. Otherwise, the user cannot register a token
    /// from that nft contract.
    ///
    /// The user must also be registered themselves,
    /// See [`Self::add_user`].
    pub fn enable_nft_for_user(&mut self, user: NftUserAccountId, nft: NftContractId) {
        self.assert_owner();

        let previous = self.nft_protocols.get(&nft);
        ensure(previous.is_some(), Error::NftProtocolNotIncluded);

        let mut token_set_per_nft = self
            .nft_tokens_per_user
            .get(&user)
            .or_panic_str(Error::MissingUser);

        ensure(
            token_set_per_nft.get(&nft).is_none(),
            Error::NftContractAlreadyEnabled,
        );

        let token_set = UnorderedSet::new(StorageKey::NftTokensPerUserInnerInner {
            user_id: user.clone(),
            contract_id: nft.clone(),
        });
        let _previous = token_set_per_nft.insert(&nft, &token_set);

        self.nft_tokens_per_user.insert(&user, &token_set_per_nft);
    }

    /// Makes a user to be unable to interact with a given nft
    /// contract.
    ///
    /// First the user must not be owning any token on a given
    /// nft contract, then they can be stopped from interacting
    /// with it.
    pub fn disable_nft_for_user(&mut self, user: NftUserAccountId, nft: NftContractId) {
        self.assert_owner();

        let tokens_per_contract = self
            .nft_tokens_per_user
            .get(&user)
            .or_panic_str(Error::MissingUser);

        let tokens = tokens_per_contract
            .get(&nft)
            .or_panic_str(Error::NftDisabledForUser);

        // check if user doesn't have any registered
        // nft tokens for this nft contract
        ensure(tokens.is_empty(), Error::NftTokensStillOwnedByUser);

        // actual disabling is not yet implemented
        unimplemented!()
    }

    /// Get tokens owned by users for a given nft contract.
    ///
    /// Note: The token_id elements are ordered by their name,
    /// so additions and removals of tokens can change the
    /// elements position.
    pub fn get_tokens_owned_by_users(
        &self,
        nft: NftContractId,
        from_index: Option<U128>,
        limit: Option<u16>,
    ) -> Vec<(nft::TokenId, NftUserAccountId)> {
        let tokens = self
            .nft_token_users
            .get(&nft)
            .or_panic_str(Error::NftProtocolNotIncluded);

        let from_index = from_index.map(u128::from).unwrap_or_default() as usize;

        if tokens.len() != 1 {
            env::log_str(&format!("line: {}. amount: {}", std::line!(), tokens.len()));
        } else {
            env::log_str(&format!("line: {}", std::line!()));
        }
        let mut cursor = tokens.into_iter();

        let (limit, mut v) = {
            let (limit, v) = match limit.map(usize::from) {
                Some(l) => (l, Vec::with_capacity(l)),
                None => (usize::MAX, Vec::new()),
            };

            // reduces one from limit because the first element
            // is extracted separately
            ensure(limit > 0, Error::LimitAtZero);
            (limit - 1, v)
        };

        // skip elements (if needed) and get the first one
        let first = cursor.nth(from_index);
        match first {
            None => {
                env::log_str(&format!("line: {}", std::line!()));
                return v;
            }
            Some((token_id, user)) => {
                env::log_str(&format!("line: {}", std::line!()));
                v.push((token_id, user))
            }
        };
        env::log_str(&format!("line: {}", std::line!()));

        // add remaining elements
        for (token_id, user) in cursor.take(limit) {
            v.push((token_id, user));
        }

        v
    }

    pub fn get_tokens_for_user(
        &self,
        nft: NftContractId,
        user: NftUserAccountId,
        from_index: Option<U128>,
        limit: Option<u16>,
    ) -> Vec<nft::TokenId> {
        let tokens_by_contracts = self
            .nft_tokens_per_user
            .get(&user)
            .or_panic_str(Error::MissingUser);
        let tokens = tokens_by_contracts
            .get(&nft)
            .or_panic_str(Error::NftProtocolNotIncluded);

        let from_index = from_index.unwrap_or_else(|| 0.into()).0 as usize;
        let limit = limit.unwrap_or(u16::MAX) as usize;
        tokens.iter().skip(from_index).take(limit).collect()
    }

    pub fn get_registered_nfts_for_user(
        &self,
        user: NftUserAccountId,
        from_index: Option<U128>,
        limit: Option<u16>,
    ) -> Vec<NftContractId> {
        let from_index = from_index.unwrap_or_else(|| 0.into()).0 as usize;
        let limit = limit.unwrap_or(u16::MAX) as usize;

        let tokens_by_contracts = self
            .nft_tokens_per_user
            .get(&user)
            .or_panic_str(Error::MissingUser);
        tokens_by_contracts
            .keys()
            .skip(from_index)
            .take(limit)
            .collect()
    }

    /// Sends the `token_id` to `receiver`.
    ///
    /// This function is intended to be called by users, not
    /// by [`Self::owner`]. The `sender` is implicitly the
    /// predecessor. The send-nft owner should use
    /// [`Self::send_logged()`] instead.
    ///
    /// This will de-register the token from the current user.
    /// In case of an (external contract call) transfer failure,
    /// an internall callback will re-register the token for the
    /// previous user.
    #[payable]
    pub fn user_send_logged(
        &mut self,
        nft_contract: NftContractId,
        receiver: NftUserAccountId,
        token_id: nft::TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
        nearapps_tags: NearAppsTags,
    ) -> Promise {
        let predecessor = env::predecessor_account_id();
        ensure(predecessor != self.owner, Error::MustNotBeOwner);

        Self::send_logged(
            self,
            nft_contract,
            NftUserAccountId(predecessor),
            receiver,
            token_id,
            approval_id,
            memo,
            nearapps_tags,
        )
    }

    /// Sends the `token_id` to `receiver`.
    ///
    /// This will de-register the token from the current user.
    /// In case of an (external contract call) transfer failure,
    /// an internall callback will re-register the token for the
    /// previous user.
    #[payable]
    pub fn send_logged(
        &mut self,
        nft_contract: NftContractId,
        sender: NftUserAccountId,
        receiver: NftUserAccountId,
        token_id: nft::TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
        nearapps_tags: NearAppsTags,
    ) -> Promise {
        const GAS_CURRENT: Gas = Gas(parse_gas!("30 Tgas") as u64);
        const ONE_YOCTO: Balance = parse_near!("1 yN");
        const NO_DEPOSIT: Balance = 0;

        let gas_current = env::prepaid_gas() - env::used_gas();
        let attached_current = env::attached_deposit();
        let current_account = env::current_account_id();

        ensure(attached_current == ONE_YOCTO, Error::OneYoctoNearRequired);
        ensure(receiver.0 != current_account, Error::SelfReceiver);

        Self::internal_unregister_token(self, nft_contract.clone(), sender.clone(), token_id.clone());

        let (send, on_send) = match self.nft_protocols.get(&nft_contract) {
            // unkown or not registered nft/protocol,
            None | Some(NftProtocol::Unknown) => Error::UnkownProtocol.panic(),

            Some(NftProtocol::Standard) => {
                // the logging has presumably not been executed

                // memo not allowed because it will contain the nearapps
                // tags
                ensure(memo.is_none(), Error::MemoNotAllowed);

                let memo = near_sdk::serde_json::to_string(&nearapps_tags).unwrap();

                let send = ext_nft::standard::transfer::nft_transfer(
                    receiver.clone(),
                    token_id.clone(),
                    approval_id,
                    Some(memo),
                    nft_contract.0.clone(),
                    ONE_YOCTO,
                    gas_current - GAS_CURRENT - GAS_ON_SEND,
                );

                // the logging will happen on the callback
                let on_send = ext_self::on_send_logged(
                    nft_contract,
                    sender,
                    receiver,
                    token_id,
                    nearapps_tags,
                    current_account,
                    NO_DEPOSIT,
                    GAS_ON_SEND,
                );

                (send, on_send)
            }

            Some(NftProtocol::NearApps) => {
                // the logging was presumably already executed
                let send = ext_nft::nearapps::transfer::nft_transfer_logged(
                    receiver,
                    token_id.clone(),
                    approval_id,
                    memo,
                    nearapps_tags.clone(),
                    nft_contract.0.clone(),
                    ONE_YOCTO,
                    gas_current - GAS_CURRENT - GAS_ON_SEND,
                );

                let on_send = ext_self::on_send(
                    nft_contract,
                    sender,
                    // receiver,
                    token_id,
                    nearapps_tags,
                    current_account,
                    NO_DEPOSIT,
                    GAS_ON_SEND,
                );

                (send, on_send)
            }
        };

        send.then(on_send)
    }

    /// Sends the `token_id` to `receiver`, making the nft 
    /// call a function on the receiver.
    ///
    /// This will de-register the token from the current user.
    /// In case of an (external contract call) transfer failure,
    /// an internall callback will re-register the token for the
    /// previous user.
    #[payable]
    pub fn send_call_logged(
        &mut self,
        nft_contract: NftContractId,
        sender: NftUserAccountId,
        receiver: NftUserAccountId,
        token_id: nft::TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
        msg: String,
        nearapps_tags: NearAppsTags,
    ) -> Promise {
        const GAS_CURRENT: Gas = Gas(parse_gas!("30 Tgas") as u64);
        const ONE_YOCTO: Balance = parse_near!("1 yN");
        const NO_DEPOSIT: Balance = 0;

        let gas_current = env::prepaid_gas() - env::used_gas();
        let attached_current = env::attached_deposit();
        let current_account = env::current_account_id();

        ensure(attached_current == ONE_YOCTO, Error::OneYoctoNearRequired);
        ensure(receiver.0 != current_account, Error::SelfReceiver);

        Self::internal_unregister_token(self, nft_contract.clone(), sender.clone(), token_id.clone());

        let (send, on_send_call) = match self.nft_protocols.get(&nft_contract) {
            // unkown or not registered nft/protocol,
            None | Some(NftProtocol::Unknown) => Error::UnkownProtocol.panic(),

            Some(NftProtocol::Standard) => {
                // the logging has presumably not been executed

                // memo not allowed because it will contain the nearapps
                // tags
                ensure(memo.is_none(), Error::MemoNotAllowed);

                let memo = near_sdk::serde_json::to_string(&nearapps_tags).unwrap();

                let send = ext_nft::standard::transfer::nft_transfer_call(
                    receiver.clone(),
                    token_id.clone(),
                    approval_id,
                    Some(memo),
                    msg,
                    nft_contract.0.clone(),
                    ONE_YOCTO,
                    gas_current - GAS_CURRENT - GAS_ON_SEND,
                );

                // the logging will happen on the callback
                let on_send_call = ext_self::on_send_call_logged(
                    nft_contract,
                    sender,
                    token_id,
                    nearapps_tags,
                    current_account,
                    NO_DEPOSIT,
                    GAS_ON_SEND,
                );

                (send, on_send_call)
            }

            Some(NftProtocol::NearApps) => {
                // the logging was presumably already executed
                let send = ext_nft::nearapps::transfer::nft_transfer_call_logged(
                    receiver,
                    token_id.clone(),
                    approval_id,
                    memo,
                    msg,
                    nearapps_tags.clone(),
                    nft_contract.0.clone(),
                    ONE_YOCTO,
                    gas_current - GAS_CURRENT - GAS_ON_SEND,
                );

                let on_send_call = ext_self::on_send_call(
                    nft_contract,
                    sender,
                    // receiver,
                    token_id,
                    nearapps_tags,
                    current_account,
                    NO_DEPOSIT,
                    GAS_ON_SEND,
                );

                (send, on_send_call)
            }
        };

        send.then(on_send_call)
    }

    /// Callback after sending the Nft Token to some other user.
    ///
    /// Must not panic.
    ///
    /// Checks for failure when sending the token. In case of
    /// failure, the token is internally re-registered for the
    /// user.
    ///
    /// Returns "transfer_success",  
    /// ie. `true` means the token transfer was accepted.  
    /// `false` means the token transfer was denied and
    /// had been internally re-registered for the user that was
    /// trying to send the token.  
    #[private]
    pub fn on_send(
        &mut self,
        nft_contract: NftContractId,
        sender: NftUserAccountId,
        token_id: nft::TokenId,
        nearapps_tags: NearAppsTags,
    ) -> bool {
        let undo_send = || {
            SendNft::internal_receive_token_logged(
                self,
                nft_contract,
                sender,
                token_id,
                nearapps_tags.clone(),
            )
        };

        if env::promise_results_count() != 1 {
            return !undo_send();
        }

        use near_sdk::PromiseResult;
        match env::promise_result(0) {
            PromiseResult::NotReady => unimplemented!(),
            PromiseResult::Failed => {
                return !undo_send();
            }
            PromiseResult::Successful(_) => (),
        }

        true
    }

    /// Callback after sending the Nft Token to some other user.
    ///
    /// Must not panic.
    ///
    /// Checks for failure when sending the token. In case of
    /// failure, the token is internally re-registered for the
    /// user.
    ///
    /// Returns "transfer_success",  
    /// ie. `true` means the token transfer was accepted.  
    /// `false` means the token transfer was denied and
    /// had been internally re-registered for the user that was
    /// trying to send the token.  
    pub fn on_send_logged(
        &mut self,
        nft_contract: NftContractId,
        sender: NftUserAccountId,
        token_id: nft::TokenId,
        nearapps_tags: NearAppsTags,
    ) -> bool {
        let transfer_successful =
            Self::on_send(self, nft_contract, sender, token_id, nearapps_tags.clone());

        if transfer_successful {
            // on success, makes a best-effort call for nearapps log
            let _ = self.log(nearapps_tags);
        }

        transfer_successful
    }

    /// Callback after sending the Nft Token to some other user.
    ///
    /// Must not panic.
    ///
    /// Checks for failure when sending the token. In case of
    /// failure, the token is internally re-registered for the
    /// user.
    ///
    /// Returns "transfer_success",  
    /// ie. `true` means the token transfer was accepted.  
    /// `false` means the token transfer was denied and
    /// had been internally re-registered for the user that was
    /// trying to send the token.  
    #[private]
    pub fn on_send_call(
        &mut self,
        nft_contract: NftContractId,
        sender: NftUserAccountId,
        token_id: nft::TokenId,
        nearapps_tags: NearAppsTags,
    ) -> bool {
        let undo_send = || {
            SendNft::internal_receive_token_logged(
                self,
                nft_contract,
                sender,
                token_id,
                nearapps_tags.clone(),
            )
        };

        if env::promise_results_count() != 1 {
            return !undo_send();
        }

        use near_sdk::PromiseResult;
        match env::promise_result(0) {
            PromiseResult::NotReady => unimplemented!(),
            PromiseResult::Failed => {
                !undo_send()
            }
            PromiseResult::Successful(success) => {
                let success = near_sdk::serde_json::from_slice::<bool>(&success)
                // the nft contract misbehaved.
                // it is safer to not re-register the token
                .or_panic_str(Error::NftContractUnknownSuccess);

                if success {
                    true
                } else {
                    !undo_send()
                }
            },
        }
    }

    /// Callback after sending the Nft Token to some other user.
    ///
    /// Must not panic.
    ///
    /// Checks for failure when sending the token. In case of
    /// failure, the token is internally re-registered for the
    /// user.
    ///
    /// Returns "transfer_success",  
    /// ie. `true` means the token transfer was accepted.  
    /// `false` means the token transfer was denied and
    /// had been internally re-registered for the user that was
    /// trying to send the token.  
    #[private]
    pub fn on_send_call_logged(
        &mut self,
        nft_contract: NftContractId,
        sender: NftUserAccountId,
        token_id: nft::TokenId,
        nearapps_tags: NearAppsTags,
    ) -> bool {

        let transfer_successful =
            Self::on_send_call(self, nft_contract, sender, token_id, nearapps_tags.clone());

        if transfer_successful {
            // on success, makes a best-effort call for nearapps log
            let _ = self.log(nearapps_tags);
        }

        transfer_successful
    }
}

use nft::core::NonFungibleTokenReceiver;

#[near_bindgen]
impl NonFungibleTokenReceiver for SendNft {
    /// Receive tokens.
    ///
    /// Returns "token_denied",  
    /// ie. `true` means the token was denied and
    /// should be returned back to the previous_owner.  
    /// Otherwise, `false` means the token receivement was accepted.
    fn nft_on_transfer(
        &mut self,
        sender_id: AccountId,
        previous_owner_id: AccountId,
        token_id: nft::TokenId,
        msg: String,
    ) -> PromiseOrValue<bool> {
        let _sender_id = sender_id;

        let nft_contract = NftContractId(env::predecessor_account_id());
        let previous_owner_id = NftUserAccountId(previous_owner_id);

        let must_cancel = match self.nft_protocols.get(&nft_contract) {
            // unkown or not registered nft/protocol,
            // indicate that the transfer should be undone
            None | Some(NftProtocol::Unknown) => true,

            // the logging has presumably not been executed,
            // but the logging information must still be
            // present inside the `msg`
            Some(NftProtocol::Standard) => {
                use near_sdk::serde_json::from_str;

                let nearapps_tags = from_str::<NearAppsTagsContained>(&msg)
                    .or_panic_str(Error::NearAppsTagsMissing)
                    .nearapps_tags;

                let res = Self::internal_receive_token_logged(
                    self,
                    nft_contract,
                    previous_owner_id,
                    token_id,
                    nearapps_tags,
                );
                res
            }

            // the logging was presumably already executed
            Some(NftProtocol::NearApps) => {
                let res =
                    Self::internal_receive_token(self, nft_contract, previous_owner_id, token_id);
                res
            }
        };

        if must_cancel {
            env::panic_str("");
        } else {
            PromiseOrValue::Value(false)
        }
    }
}

impl SendNft {


    pub fn internal_unregister_token(
        &mut self,
        nft_contract: NftContractId,
        sender: NftUserAccountId,
        token_id: nft::TokenId
    ) {
        // unregisters the token-id
        {
            let mut token_owners = self.nft_token_users.get(&nft_contract).unwrap();
            let token_owner = token_owners
                .remove(&token_id)
                .or_panic_str(Error::MissingTokenId);
            ensure(sender == token_owner, Error::NotTokenOwner);
            self.nft_token_users.insert(&nft_contract, &token_owners);

            let predecessor = env::predecessor_account_id();
            ensure(
                // ensure it was invoked by the user who's the 
                // token owner
                predecessor == sender.0 
                // or by the send-nft contract owner
                || predecessor == self.owner,
                Error::NotTokenOwner,
            );
        }

        // unregister that token-id (mapped per user)
        #[allow(clippy::bool_comparison)]
        {
            let mut sender_tokens = self
                .nft_tokens_per_user
                .get(&sender)
                .or_panic_str(Error::MissingUser);
            let mut sender_tokens_for_contract = sender_tokens
                .get(&nft_contract)
                .or_panic_str(Error::NftDisabledForUser);

            let token_removed = sender_tokens_for_contract.remove(&token_id);
            ensure(token_removed == true, Error::MissingTokenId);
            sender_tokens.insert(&nft_contract, &sender_tokens_for_contract);
            self.nft_tokens_per_user.insert(&sender, &sender_tokens);
        }
    }

    /// Receive tokens.
    ///
    /// Must not panic.
    ///
    /// Returns "token_denied",  
    /// ie. `true` means the token was denied and
    /// should be returned back to the previous_owner.  
    /// Otherwise, `false` means the token receivement was accepted.
    pub fn internal_receive_token_logged(
        &mut self,
        nft_contract: NftContractId,
        previous_owner_id: NftUserAccountId,
        token_id: nft::TokenId,
        // msg: String,
        nearapps_tags: NearAppsTags,
    ) -> bool {
        if let Some(NftProtocol::Standard) = self.nft_protocols.get(&nft_contract) {
            // let nearapps_tags = match from_str::<NearAppsTagsContained>(&msg) {
            //     Err(_json_error) => {
            //         env::log_str(Error::JsonErrorForNearAppsTags.to_string().as_str());
            //         return PromiseOrValue::Value(true);
            //     }
            //     Ok(tags) => tags,
            // }
            // .nearapps_tags;

            // best-effort call for nearapps log
            let _ = self.log(nearapps_tags);
        }

        Self::internal_receive_token(self, nft_contract, previous_owner_id, token_id)
    }

    pub fn internal_receive_token(
        &mut self,
        nft_contract: NftContractId,
        previous_owner_id: NftUserAccountId,
        token_id: nft::TokenId,
        // msg: String,
    ) -> bool {
        if let None | Some(NftProtocol::Unknown) = self.nft_protocols.get(&nft_contract) {
            // unkown or not registered nft/protocol,
            // indicate that the transfer should be undone
            env::log_str(Error::NftProtocolNotIncluded.to_string().as_str());
            return true;
        };

        let mut token_owners = match self.nft_token_users.get(&nft_contract) {
            None => {
                env::log_str(Error::NftProtocolNotIncluded.to_string().as_str());
                return true;
            }
            Some(v) => v,
        };

        let previous = token_owners.get(&token_id);
        if previous.is_some() {
            env::log_str(&format!("previous owner: {}", previous.unwrap().0));
            env::log_str(Error::NftTokenAlreadyOwned.to_string().as_str());
            return true;
        }
        let _previous = token_owners.insert(&token_id, &previous_owner_id);
        self.nft_token_users.insert(&nft_contract, &token_owners);

        let mut tokens_per_owner = match self.nft_tokens_per_user.get(&previous_owner_id) {
            None => {
                env::log_str(Error::MissingUser.to_string().as_str());
                return true;
            }
            Some(v) => v,
        };

        let mut tokens_per_owner_inner = match tokens_per_owner.get(&nft_contract) {
            None => {
                // the nft contract for the user must already be registered
                env::log_str(Error::NftDisabledForUser.to_string().as_str());
                return true;
            }
            Some(v) => v,
        };

        let had_previous = tokens_per_owner_inner.contains(&token_id);
        if had_previous {
            env::log_str(Error::UserAlreadyOwnedTheNftToken.to_string().as_str());
            return true;
        }

        let _had_previous = tokens_per_owner_inner.insert(&token_id);
        let _previous = tokens_per_owner.insert(&nft_contract, &tokens_per_owner_inner);
        let _previous = self
            .nft_tokens_per_user
            .insert(&previous_owner_id, &tokens_per_owner);

        env::log_str(&format!("line: {}", std::line!()));
        false
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
    fn nearapps_account(&self) -> AccountId {
        self.nearapps_logger.clone()
    }
}
