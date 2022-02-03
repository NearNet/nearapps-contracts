#![allow(clippy::too_many_arguments)]

use crate::error::Error;
use crate::NftSeries;
use near_contract_standards::non_fungible_token as nft;
use near_sdk::{env, require, AccountId, Balance, Gas, PromiseOrValue};
use near_sdk::{ext_contract, near_bindgen, PromiseResult};
use near_units::parse_near;
use nearapps_log::{NearAppsAccount, NearAppsTags};
use nearapps_near_ext::ensure;
use std::collections::HashMap;

#[cfg(not(target_arch = "wasm32"))]
use crate::NftSeriesContract;

const GAS_FOR_RESOLVE_TRANSFER: Gas =
    Gas(8_000_000_000_000 + nearapps_log::GAS_FOR_BEST_EFFORT_LOG.0);
const GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(45_000_000_000_000 + GAS_FOR_RESOLVE_TRANSFER.0);

const NO_DEPOSIT: Balance = 0;

/// Based on [`nft::core::ext_self::nft_resolve_transfer`].
///
/// This needs to log the tags on success.
#[ext_contract(ext_self)]
trait NFTResolver {
    fn nft_resolve_transfer_logged(
        &mut self,
        previous_owner_id: AccountId,
        receiver_id: AccountId,
        token_id: nft::TokenId,
        approved_account_ids: Option<HashMap<AccountId, u64>>,
        nearapps_tags: NearAppsTags,
    ) -> bool;
}

/// Based on impl
/// [`nft::core::NonFungibleTokenCore::nft_transfer_call`]
/// of [`nft::NonFungibleToken`].
pub fn nft_transfer_call(
    token: &mut nft::NonFungibleToken,
    receiver_id: AccountId,
    token_id: nft::TokenId,
    approval_id: Option<u64>,
    memo: Option<String>,
    msg: String,
    nearapps_tags: NearAppsTags,
) -> PromiseOrValue<bool> {
    ensure(
        env::attached_deposit() == parse_near!("1 yN"),
        Error::OneYoctoNearRequired,
    );
    require!(
        env::prepaid_gas() > GAS_FOR_NFT_TRANSFER_CALL + GAS_FOR_RESOLVE_TRANSFER,
        "More gas is required"
    );

    let sender_id = env::predecessor_account_id();
    let (old_owner, old_approvals) =
        token.internal_transfer(&sender_id, &receiver_id, &token_id, approval_id, memo);
    // Initiating receiver's call and the callback
    nft::core::ext_receiver::nft_on_transfer(
        sender_id,
        old_owner.clone(),
        token_id.clone(),
        msg,
        receiver_id.clone(),
        0,
        env::prepaid_gas() - GAS_FOR_NFT_TRANSFER_CALL,
    )
    .then(ext_self::nft_resolve_transfer_logged(
        old_owner,
        receiver_id,
        token_id,
        old_approvals,
        nearapps_tags,
        env::current_account_id(),
        NO_DEPOSIT,
        GAS_FOR_RESOLVE_TRANSFER,
    ))
    .into()
}

#[near_bindgen]
// impl NonFungibleTokenResolver for NonFungibleToken{}
/// Copy of the std documentation:  
/// Used when an NFT is transferred using `nft_transfer_call`.
/// This is the method that's called after `nft_on_transfer`.
/// This trait is implemented on the NFT contract.
impl NftSeries {
    /// Based on impl
    /// [`nft::core::NonFungibleTokenResolver::nft_resolve_transfer`]
    /// of [`nft::NonFungibleToken`].
    ///
    ///
    ///
    /// Copy of the std documentation:  
    /// Finalize an `nft_transfer_call` chain of cross-contract calls.
    ///
    /// The `nft_transfer_call` process:
    ///
    /// 1. Sender calls `nft_transfer_call` on FT contract
    /// 2. NFT contract transfers token from sender to receiver
    /// 3. NFT contract calls `nft_on_transfer` on receiver contract
    /// 4+. [receiver contract may make other cross-contract calls]
    /// N. NFT contract resolves promise chain with `nft_resolve_transfer`, and may
    ///    transfer token back to sender
    ///
    /// Requirements:
    /// * Contract MUST forbid calls to this function by any account except self
    /// * If promise chain failed, contract MUST revert token transfer
    /// * If promise chain resolves with `true`, contract MUST return token to
    ///   `sender_id`
    ///
    /// Arguments:
    /// * `previous_owner_id`: the owner prior to the call to `nft_transfer_call`
    /// * `receiver_id`: the `receiver_id` argument given to `nft_transfer_call`
    /// * `token_id`: the `token_id` argument given to `ft_transfer_call`
    /// * `approvals`: if using Approval Management, contract MUST provide
    ///   set of original approved accounts in this argument, and restore these
    ///   approved accounts in case of revert.
    ///
    /// Returns true if token was successfully transferred to `receiver_id`.

    #[private]
    pub fn nft_resolve_transfer_logged(
        &mut self,
        // token: &mut nft::NonFungibleToken,
        previous_owner_id: AccountId,
        receiver_id: AccountId,
        token_id: nft::TokenId,
        approved_account_ids: Option<HashMap<AccountId, u64>>,
        nearapps_tags: NearAppsTags,
    ) -> bool {
        use near_sdk::log;

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
            // best-effort call for nearapps log
            let _ = self.log(nearapps_tags);

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
                nft::refund_approved_account_ids(previous_owner_id, &approved_account_ids);
            }
            return true;
        };

        log!(&format!(
            "Return token {} from @{} to @{}",
            token_id, receiver_id, previous_owner_id
        ));

        self.tokens
            .internal_transfer_unguarded(&token_id, &receiver_id, &previous_owner_id);

        // If using Approval Management extension,
        // 1. revert any approvals receiver already set, refunding storage costs
        // 2. reset approvals to what previous owner had set before call to nft_transfer_call
        if let Some(by_id) = &mut self.tokens.approvals_by_id {
            if let Some(receiver_approvals) = by_id.get(&token_id) {
                nft::refund_approved_account_ids(receiver_id, &receiver_approvals);
            }
            if let Some(previous_owner_approvals) = approved_account_ids {
                by_id.insert(&token_id, &previous_owner_approvals);
            }
        }

        false
    }
}
