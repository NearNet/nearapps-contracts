#![allow(clippy::let_and_return)]
#![allow(clippy::too_many_arguments)]

use crate::error::Error;
use crate::types::NftProtocol;
use crate::types::{NftContractId, NftUserAccountId};
use crate::SendNft;
use near_contract_standards::non_fungible_token as nft;
use near_sdk::PromiseOrValue;
use near_sdk::{env, near_bindgen, AccountId};
use nearapps_log::{NearAppsAccount, NearAppsTags, NearAppsTagsContained};
use nearapps_near_ext::OrPanicStr;

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
