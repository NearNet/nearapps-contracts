use super::utils;
use crate::Contract;
use crate::Owner;
use near_contract_standards::non_fungible_token as nft;
use near_sdk::{assert_one_yocto, env, near_bindgen, require, AccountId, Balance, Gas, Promise};

#[cfg(not(target_arch = "wasm32"))]
use crate::ContractContract;

const GAS_FOR_NFT_APPROVE: Gas = Gas(10_000_000_000_000);
const NO_DEPOSIT: Balance = 0;

/// Adaption from [`nft::approval::NonFungibleTokenApproval`] so that the
/// expected owner_id information, instead of being the
/// [`env::predecessor_account_id()`], is explicit.
///
/// The predecessor must always be the [`Contract::tokens::owner_id`].
pub trait NonFungibleTokenApprovalFrom {
    /// Similar to [`nft::core::NonFungibleTokenApproval::nft_approve()`],
    /// except that the expected owner_id is explicit.
    fn nft_approve_from(
        &mut self,
        token_id: nft::TokenId,
        expected_owner_id: AccountId,
        account_id: AccountId,
        msg: Option<String>,
    ) -> Option<Promise>;

    /// Similar to
    /// [`nft::core::NonFungibleTokenApproval::nft_revoke_from()`],
    /// except that the expected owner_id is explicit.
    fn nft_revoke_from(
        &mut self,
        token_id: nft::TokenId,
        expected_owner_id: AccountId,
        account_id: AccountId,
    );

    /// Similar to [
    /// `nft::core::NonFungibleTokenApproval::nft_revoke_all_from()`],
    /// except that the expected owner_id is explicit.
    fn nft_revoke_all_from(&mut self, token_id: nft::TokenId, expected_owner_id: AccountId);

    /// Identical to
    /// [`nft::core::NonFungibleTokenCore::nft_is_approved()`].
    fn nft_is_approved(
        &self,
        token_id: nft::TokenId,
        approved_account_id: AccountId,
        approval_id: Option<u64>,
    ) -> bool;
}

#[near_bindgen]
impl NonFungibleTokenApprovalFrom for Contract {
    /// Similar to [`nft::core::NonFungibleTokenApproval::nft_approve()`],
    /// except that the expected owner_id is explicit.
    #[payable]
    fn nft_approve_from(
        &mut self,
        token_id: nft::TokenId,
        expected_owner_id: AccountId,
        account_id: AccountId,
        msg: Option<String>,
    ) -> Option<Promise> {
        self.assert_owner();
        utils::assert_at_least_one_yocto();

        let approvals_by_id = self
            .tokens
            .approvals_by_id
            .as_mut()
            .unwrap_or_else(|| env::panic_str("NFT does not support Approval Management"));

        let owner_id = expect_token_found(self.tokens.owner_by_id.get(&token_id));

        require!(expected_owner_id == owner_id, "Wrong expected owner.");

        let next_approval_id_by_id = expect_approval(self.tokens.next_approval_id_by_id.as_mut());
        // update HashMap of approvals for this token
        let approved_account_ids = &mut approvals_by_id.get(&token_id).unwrap_or_default();
        let approval_id: u64 = next_approval_id_by_id.get(&token_id).unwrap_or(1u64);
        let old_approval_id = approved_account_ids.insert(account_id.clone(), approval_id);

        // save updated approvals HashMap to contract's LookupMap
        approvals_by_id.insert(&token_id, approved_account_ids);

        // increment next_approval_id for this token
        next_approval_id_by_id.insert(&token_id, &(approval_id + 1));

        // If this approval replaced existing for same account, no storage was used.
        // Otherwise, require that enough deposit was attached to pay for storage, and refund
        // excess.
        let storage_used = if old_approval_id.is_none() {
            utils::bytes_for_approved_account_id(&account_id)
        } else {
            0
        };
        utils::refund_deposit(storage_used);

        // if given `msg`, schedule call to `nft_on_approve` and return it. Else, return None.
        msg.map(|msg| {
            nft::approval::ext_approval_receiver::nft_on_approve(
                token_id,
                owner_id,
                approval_id,
                msg,
                account_id,
                NO_DEPOSIT,
                env::prepaid_gas() - GAS_FOR_NFT_APPROVE,
            )
        })
    }

    /// Similar to
    /// [`nft::core::NonFungibleTokenApproval::nft_revoke_from()`],
    /// except that the expected owner_id is explicit.
    #[payable]
    fn nft_revoke_from(
        &mut self,
        token_id: nft::TokenId,
        expected_owner_id: AccountId,
        account_id: AccountId,
    ) {
        self.assert_owner();
        assert_one_yocto();

        let approvals_by_id = self.tokens.approvals_by_id.as_mut().unwrap_or_else(|| {
            env::panic_str("NFT does not support Approval Management");
        });

        let owner_id = expect_token_found(self.tokens.owner_by_id.get(&token_id));

        require!(expected_owner_id == owner_id, "Wrong expected owner.");

        // if token has no approvals, do nothing
        if let Some(approved_account_ids) = &mut approvals_by_id.get(&token_id) {
            // if account_id was already not approved, do nothing
            if approved_account_ids.remove(&account_id).is_some() {
                utils::refund_approved_account_ids_iter(
                    self.tokens.owner_id.clone(),
                    core::iter::once(&account_id),
                );
                // if this was the last approval, remove the whole HashMap to save space.
                if approved_account_ids.is_empty() {
                    approvals_by_id.remove(&token_id);
                } else {
                    // otherwise, update approvals_by_id with updated HashMap
                    approvals_by_id.insert(&token_id, approved_account_ids);
                }
            }
        }
    }

    /// Similar to
    /// [`nft::core::NonFungibleTokenApproval::nft_revoke_all_from()`],
    /// except that the expected owner_id is explicit.
    #[payable]
    fn nft_revoke_all_from(&mut self, token_id: nft::TokenId, expected_owner_id: AccountId) {
        self.assert_owner();
        assert_one_yocto();

        let approvals_by_id = self.tokens.approvals_by_id.as_mut().unwrap_or_else(|| {
            env::panic_str("NFT does not support Approval Management");
        });

        let owner_id = expect_token_found(self.tokens.owner_by_id.get(&token_id));

        require!(expected_owner_id == owner_id, "Wrong expected owner.");

        // if token has no approvals, do nothing
        if let Some(approved_account_ids) = &mut approvals_by_id.get(&token_id) {
            // otherwise, refund owner for storage costs of all approvals...
            utils::refund_approved_account_ids(self.tokens.owner_id.clone(), approved_account_ids);
            // ...and remove whole HashMap of approvals
            approvals_by_id.remove(&token_id);
        }
    }

    /// Identical to
    /// [`nft::core::NonFungibleTokenCore::nft_is_approved()`].
    fn nft_is_approved(
        &self,
        token_id: nft::TokenId,
        approved_account_id: AccountId,
        approval_id: Option<u64>,
    ) -> bool {
        use nft::approval::NonFungibleTokenApproval;
        self.tokens
            .nft_is_approved(token_id, approved_account_id, approval_id)
    }
}

/// Identical to [`nft::approval::approval_impl::expect_token_found()`].
fn expect_token_found<T>(option: Option<T>) -> T {
    option.unwrap_or_else(|| env::panic_str("Token not found"))
}

/// Identical to [`nft::approval::approval_impl::expect_approval()`].
fn expect_approval<T>(option: Option<T>) -> T {
    option.unwrap_or_else(|| env::panic_str("next_approval_by_id must be set for approval ext"))
}
