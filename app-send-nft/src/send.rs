use crate::error::Error;
use crate::ext_nft;
use crate::types::{NftContractId, NftProtocol, NftUserAccountId, TokenStatus};
use crate::SendNft;
use near_contract_standards::non_fungible_token as nft;
use near_sdk::{env, ext_contract, near_bindgen, Balance, Gas, Promise};
use near_units::{parse_gas, parse_near};
use nearapps_log::{NearAppsAccount, NearAppsTags};
use nearapps_near_ext::{ensure, OrPanicStr};

#[cfg(not(target_arch = "wasm32"))]
use crate::SendNftContract;

const GAS_ON_SEND: Gas = Gas(parse_gas!("30 Tgas") as u64);

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

#[near_bindgen]
impl SendNft {
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

        Self::internal_token_to_send(self, nft_contract.clone(), sender.clone(), token_id.clone());

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

        Self::internal_token_to_send(self, nft_contract.clone(), sender.clone(), token_id.clone());

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
                    receiver,
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
    /// Checks for failure when sending the token. In case of
    /// failure, the token is internally re-registered for the
    /// user.
    ///
    /// Returns "transfer_success",  
    /// ie. `true` means the token transfer was accepted.  
    /// `false` means the token transfer was denied and
    /// had been internally re-registered for the user that was
    /// trying to send the token.  
    ///
    /// In case of panic, the original state is safe against
    /// re-entrancy attacks; but the token status won't be
    /// changed back to "standby".
    #[private]
    pub fn on_send(
        &mut self,
        nft_contract: NftContractId,
        sender: NftUserAccountId,
        token_id: nft::TokenId,
    ) -> bool {
        let mut undo_send = || {
            SendNft::internal_token_to_standby(
                self,
                nft_contract.clone(),
                sender.clone(),
                token_id.clone(),
            );
            false
        };

        if env::promise_results_count() != 1 {
            return undo_send();
        }
        use near_sdk::PromiseResult;
        match env::promise_result(0) {
            PromiseResult::NotReady => unimplemented!(),
            PromiseResult::Failed => undo_send(),
            PromiseResult::Successful(_) => {
                Self::internal_token_unregister(
                    self,
                    nft_contract.clone(),
                    sender.clone(),
                    token_id.clone(),
                );

                true
            }
        }
    }

    /// Callback after sending the Nft Token to some other user.
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
    ///
    /// In case of panic, the original state is safe against
    /// re-entrancy attacks; but the token status won't be
    /// changed back to "standby".
    pub fn on_send_logged(
        &mut self,
        nft_contract: NftContractId,
        sender: NftUserAccountId,
        token_id: nft::TokenId,
        nearapps_tags: NearAppsTags,
    ) -> bool {
        let transfer_successful = Self::on_send(self, nft_contract, sender, token_id);

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
    ) -> bool {
        let mut undo_send = || {
            SendNft::internal_token_to_standby(
                self,
                nft_contract.clone(),
                sender.clone(),
                token_id.clone(),
            );
            false
        };

        if env::promise_results_count() != 1 {
            return undo_send();
        }

        use near_sdk::PromiseResult;
        match env::promise_result(0) {
            PromiseResult::NotReady => unimplemented!(),
            PromiseResult::Failed => undo_send(),
            PromiseResult::Successful(success) => {
                let success = near_sdk::serde_json::from_slice::<bool>(&success)
                    // the nft contract misbehaved.
                    // it is safer to not re-register the token
                    .or_panic_str(Error::NftContractUnknownSuccess);

                if success {
                    Self::internal_token_unregister(
                        self,
                        nft_contract.clone(),
                        sender.clone(),
                        token_id.clone(),
                    );

                    true
                } else {
                    undo_send()
                }
            }
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
        let transfer_successful = Self::on_send_call(self, nft_contract, sender, token_id);

        if transfer_successful {
            // on success, makes a best-effort call for nearapps log
            let _ = self.log(nearapps_tags);
        }

        transfer_successful
    }
}

impl SendNft {
    pub fn internal_token_to_send(
        &mut self,
        nft_contract: NftContractId,
        sender: NftUserAccountId,
        token_id: nft::TokenId,
    ) {
        // sets the token-id's status to "sending"
        {
            let mut token_owners = self.nft_token_users.get(&nft_contract).unwrap();
            let (previous_token_owner, previous_token_status) = token_owners
                .insert(&token_id, &(sender.clone(), TokenStatus::OnSending))
                .or_panic_str(Error::MissingTokenId);
            ensure(sender == previous_token_owner, Error::NotTokenOwner);
            ensure(
                previous_token_status.is_on_standby(),
                Error::TokenNotOnStandby,
            );
            // propagate data changes
            self.nft_token_users.insert(&nft_contract, &token_owners);

            let predecessor = env::predecessor_account_id();
            ensure(
                // ensure it was invoked by the user who's the
                // token owner
                predecessor == sender.0
                // or by the send-nft contract owner
                || self.owner_ids.contains(&predecessor),
                //
                Error::NotTokenOwner,
            );
        }

        // sets the token-id's status to "sending" (mapped per user)
        #[allow(clippy::bool_comparison)]
        {
            let mut sender_tokens = self
                .nft_tokens_per_user
                .get(&sender)
                .or_panic_str(Error::MissingUser);
            let mut sender_tokens_for_contract = sender_tokens
                .get(&nft_contract)
                .or_panic_str(Error::NftDisabledForUser);

            let previous_token_status = sender_tokens_for_contract
                .insert(&token_id, &TokenStatus::OnSending)
                .or_panic_str(Error::MissingTokenId);
            ensure(
                previous_token_status.is_on_standby(),
                Error::TokenNotOnStandby,
            );
            // propagate data changes
            sender_tokens.insert(&nft_contract, &sender_tokens_for_contract);
            self.nft_tokens_per_user.insert(&sender, &sender_tokens);
        }
    }

    /// When a token is being sent-out but fails, it needs to
    /// be reset to standby
    pub fn internal_token_to_standby(
        &mut self,
        nft_contract: NftContractId,
        sender: NftUserAccountId,
        token_id: nft::TokenId,
    ) {
        // sets the token-id's status to "standby"
        {
            let mut token_owners = self.nft_token_users.get(&nft_contract).unwrap();
            let (previous_token_owner, previous_token_status) = token_owners
                .insert(&token_id, &(sender.clone(), TokenStatus::OnStandby))
                .or_panic_str(Error::MissingTokenId);
            ensure(sender == previous_token_owner, Error::NotTokenOwner);
            ensure(
                previous_token_status.is_on_sending(),
                Error::TokenNotOnSending,
            );
            // propagate data changes
            self.nft_token_users.insert(&nft_contract, &token_owners);
        }

        // sets the token-id's status to "estandby" (mapped per user)
        #[allow(clippy::bool_comparison)]
        {
            let mut sender_tokens = self
                .nft_tokens_per_user
                .get(&sender)
                .or_panic_str(Error::MissingUser);
            let mut sender_tokens_for_contract = sender_tokens
                .get(&nft_contract)
                .or_panic_str(Error::NftDisabledForUser);

            let previous_token_status = sender_tokens_for_contract
                .insert(&token_id, &TokenStatus::OnStandby)
                .or_panic_str(Error::MissingTokenId);
            ensure(
                previous_token_status.is_on_sending(),
                Error::TokenNotOnSending,
            );
            // propagate data changes
            sender_tokens.insert(&nft_contract, &sender_tokens_for_contract);
            self.nft_tokens_per_user.insert(&sender, &sender_tokens);
        }
    }

    pub(crate) fn internal_token_unregister(
        &mut self,
        nft_contract: NftContractId,
        sender: NftUserAccountId,
        token_id: nft::TokenId,
    ) {
        // unregisters the token-id
        {
            let mut token_owners = self.nft_token_users.get(&nft_contract).unwrap();
            let (previous_token_owner, previous_token_status) = token_owners
                .remove(&token_id)
                .or_panic_str(Error::MissingTokenId);
            ensure(sender == previous_token_owner, Error::NotTokenOwner);
            ensure(
                previous_token_status.is_on_sending(),
                Error::TokenNotOnStandby,
            );
            // propagate data changes
            self.nft_token_users.insert(&nft_contract, &token_owners);
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

            let previous_token_status = sender_tokens_for_contract
                .remove(&token_id)
                .or_panic_str(Error::MissingTokenId);
            ensure(
                previous_token_status.is_on_sending(),
                Error::TokenNotOnStandby,
            );
            sender_tokens.insert(&nft_contract, &sender_tokens_for_contract);
            self.nft_tokens_per_user.insert(&sender, &sender_tokens);
        }
    }
}
