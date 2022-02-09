#![allow(clippy::let_and_return)]
#![allow(clippy::too_many_arguments)]

use crate::error::Error;
use crate::types::{NftContractId, NftProtocol, NftUserAccountId, TokenStatus};
use crate::SendNft;
use near_contract_standards::non_fungible_token as nft;
use near_sdk::PromiseOrValue;
use near_sdk::{env, near_bindgen, AccountId};
use nearapps_log::{NearAppsAccount, NearAppsTags, NearAppsTagsContained};
use nearapps_near_ext::{ensure, OrPanicStr};

use nft::core::NonFungibleTokenReceiver;

#[cfg(not(target_arch = "wasm32"))]
use crate::SendNftContract;

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
                Self::internal_receive_token(self, nft_contract, previous_owner_id, token_id);
                false
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
    /// Receive tokens.
    ///
    /// Returns "token_denied",  
    /// ie. `true` means the token was denied and
    /// should be returned back to the previous_owner.  
    /// Otherwise, `false` means the token receivement was accepted.
    pub(crate) fn internal_receive_token_logged(
        &mut self,
        nft_contract: NftContractId,
        previous_owner_id: NftUserAccountId,
        token_id: nft::TokenId,
        // msg: String,
        nearapps_tags: NearAppsTags,
    ) -> bool {
        Self::internal_receive_token(self, nft_contract.clone(), previous_owner_id, token_id);

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

        false
    }

    /// Receive tokens.
    ///
    /// Panics in case of an error or problem.
    pub fn internal_receive_token(
        &mut self,
        nft_contract: NftContractId,
        previous_owner_id: NftUserAccountId,
        token_id: nft::TokenId,
        // msg: String,
    ) {
        // unkown or not registered nft/protocol,
        // indicate that the transfer should be undone
        {
            let protocol = self
                .nft_protocols
                .get(&nft_contract)
                .or_panic_str(Error::NftProtocolNotIncluded);
            ensure(
                !matches!(protocol, NftProtocol::Unknown),
                Error::NftProtocolNotIncluded,
            );
        }

        let mut token_owners = self
            .nft_token_users
            .get(&nft_contract)
            .or_panic_str(Error::NftProtocolNotIncluded);

        let previous = token_owners.get(&token_id);
        ensure(previous.is_none(), Error::NftTokenAlreadyOwned);

        let _previous = token_owners.insert(
            &token_id,
            &(previous_owner_id.clone(), TokenStatus::OnStandby),
        );
        self.nft_token_users.insert(&nft_contract, &token_owners);

        let mut tokens_per_owner = self
            .nft_tokens_per_user
            .get(&previous_owner_id)
            .or_panic_str(Error::MissingUser);

        let mut tokens_per_owner_inner = tokens_per_owner
            .get(&nft_contract)
            // the nft contract for the user must already be registered
            .or_panic_str(Error::NftDisabledForUser);

        let had_previous = tokens_per_owner_inner.get(&token_id).is_some();
        ensure(!had_previous, Error::UserAlreadyOwnedTheNftToken);

        let _had_previous = tokens_per_owner_inner.insert(&token_id, &TokenStatus::OnStandby);
        let _previous = tokens_per_owner.insert(&nft_contract, &tokens_per_owner_inner);
        let _previous = self
            .nft_tokens_per_user
            .insert(&previous_owner_id, &tokens_per_owner);
    }
}
