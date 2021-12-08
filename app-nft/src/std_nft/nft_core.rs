use super::utils;
use crate::Contract;
use crate::Owner;
use near_contract_standards::non_fungible_token as nft;
use near_sdk::{
    assert_one_yocto, env, ext_contract, near_bindgen, AccountId, Balance, Gas, PromiseOrValue,
};
use std::collections::HashMap;

#[cfg(not(target_arch = "wasm32"))]
use crate::ContractContract;

const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(5_000_000_000_000);
const GAS_FOR_FT_TRANSFER_CALL: Gas = Gas(25_000_000_000_000 + GAS_FOR_RESOLVE_TRANSFER.0);
const NO_DEPOSIT: Balance = 0;

/// Adaption from [`nft::core::NonFungibleTokenCore`] so that the
/// `sender_id` information, instead of being the
/// [`env::predecessor_account_id()`], is explicit.
///
/// The predecessor must always be the [`Contract::tokens::owner_id`].  
/// All storage refunding is always sent to [`Self::tokens::owner_id`].
pub trait NonFungibleTokenCoreFrom {
    /// Similar to [`nft::core::NonFungibleTokenCore::nft_transfer()`],
    /// except that the `sender_id` is explicit.
    fn nft_transfer_from(
        &mut self,
        sender_id: AccountId,
        receiver_id: AccountId,
        token_id: nft::TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
    );

    /// Similar to
    /// [`nft::core::NonFungibleTokenCore::nft_transfer_call()`],
    /// except that the `sender_id` is explicit.
    fn nft_transfer_call_from(
        &mut self,
        sender_id: AccountId,
        receiver_id: AccountId,
        token_id: nft::TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<bool>;

    /// Identical to [`nft::core::NonFungibleTokenCore::nft_token()`].
    fn nft_token(&self, token_id: nft::TokenId) -> Option<nft::Token>;
}

#[near_bindgen]
impl NonFungibleTokenCoreFrom for Contract {
    /// Similar to [`nft::core::NonFungibleTokenCore::nft_transfer()`],
    /// except that the `sender_id` is explicit.
    #[payable]
    fn nft_transfer_from(
        &mut self,
        sender_id: AccountId,
        receiver_id: AccountId,
        token_id: nft::TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
    ) {
        assert_one_yocto();
        self.assert_owner();

        self.tokens
            .internal_transfer(&sender_id, &receiver_id, &token_id, approval_id, memo);
    }

    /// Similar to
    /// [`nft::core::NonFungibleTokenCore::nft_transfer_call()`],
    /// except that the `sender_id` is explicit.
    #[payable]
    fn nft_transfer_call_from(
        &mut self,
        sender_id: AccountId,
        receiver_id: AccountId,
        token_id: nft::TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<bool> {
        assert_one_yocto();
        self.assert_owner();

        let (old_owner, old_approvals) =
            self.tokens
                .internal_transfer(&sender_id, &receiver_id, &token_id, approval_id, memo);
        // Initiating receiver's call and the callback
        nft::core::ext_receiver::nft_on_transfer(
            sender_id,
            old_owner.clone(),
            token_id.clone(),
            msg,
            receiver_id.clone(),
            NO_DEPOSIT,
            env::prepaid_gas() - GAS_FOR_FT_TRANSFER_CALL,
        )
        .then(ext_self::nft_resolve_transfer(
            old_owner,
            receiver_id,
            token_id,
            old_approvals,
            env::current_account_id(),
            NO_DEPOSIT,
            GAS_FOR_RESOLVE_TRANSFER,
        ))
        .into()
    }

    /// Identical to [`nft::core::NonFungibleTokenCore::nft_token()`].
    fn nft_token(&self, token_id: nft::TokenId) -> Option<nft::Token> {
        use nft::core::NonFungibleTokenCore;
        self.tokens.nft_token(token_id)
    }
}

#[cfg(target_arch = "wasm32")]
use nft::core::NonFungibleTokenResolver;

#[ext_contract(ext_self)]
trait NFTResolver {
    fn nft_resolve_transfer(
        &mut self,
        previous_owner_id: AccountId,
        receiver_id: AccountId,
        token_id: nft::TokenId,
        approved_account_ids: Option<HashMap<AccountId, u64>>,
    ) -> bool;
}

#[near_bindgen]
impl nft::core::NonFungibleTokenResolver for Contract {
    /// Similar to
    /// [`nft::core::NonFungibleTokenResolver::nft_resolve_transfer()`],
    /// except that the refunding is always sent to
    /// [`Self::tokens::owner_id`].
    #[private]
    fn nft_resolve_transfer(
        &mut self,
        previous_owner_id: AccountId,
        receiver_id: AccountId,
        token_id: nft::TokenId,
        approved_account_ids: Option<HashMap<AccountId, u64>>,
    ) -> bool {
        use near_sdk::PromiseResult;

        // Get whether token should be returned
        let must_revert = match env::promise_result(0) {
            PromiseResult::NotReady => env::abort(),
            PromiseResult::Successful(value) => {
                if let Ok(yes_or_no) = near_sdk::serde_json::from_slice::<bool>(&value) {
                    yes_or_no
                } else {
                    true
                }
            }
            PromiseResult::Failed => true,
        };

        // if call succeeded, return early
        if !must_revert {
            return true;
        }

        // OTHERWISE, try to set owner back to previous_owner_id and restore approved_account_ids

        // Check that receiver didn't already transfer it away or burn it.
        if let Some(current_owner) = self.tokens.owner_by_id.get(&token_id) {
            if current_owner != receiver_id {
                // The token is not owned by the receiver anymore. Can't return it.
                return true;
            }
        } else {
            // The token was burned and doesn't exist anymore.
            // Refund storage cost for storing approvals to original owner and return early.
            if let Some(approved_account_ids) = approved_account_ids {
                utils::refund_approved_account_ids(
                    self.tokens.owner_id.clone(),
                    &approved_account_ids,
                );
            }
            return true;
        };

        near_sdk::log!(
            "Return token {} from @{} to @{}",
            token_id,
            receiver_id,
            previous_owner_id
        );

        self.tokens
            .internal_transfer_unguarded(&token_id, &receiver_id, &previous_owner_id);

        // If using Approval Management extension,
        // 1. revert any approvals receiver already set, refunding storage costs
        // 2. reset approvals to what previous owner had set before call to nft_transfer_call
        if let Some(by_id) = &mut self.tokens.approvals_by_id {
            if let Some(receiver_approvals) = by_id.get(&token_id) {
                utils::refund_approved_account_ids(
                    self.tokens.owner_id.clone(),
                    &receiver_approvals,
                );
            }
            if let Some(previous_owner_approvals) = approved_account_ids {
                by_id.insert(&token_id, &previous_owner_approvals);
            }
        }

        false
    }
}
