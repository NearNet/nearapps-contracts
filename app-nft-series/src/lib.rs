use near_contract_standards::non_fungible_token as nft;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, UnorderedMap, UnorderedSet};
use near_sdk::{env, near_bindgen, require, AccountId, BorshStorageKey, PanicOnDefault};
use nearapps_near_ext::ensure;
use nft::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};

pub mod error;
pub mod migration;
pub mod owners;
pub mod series;
pub mod transfer_call;
pub mod utils;
pub mod version;

use error::Error;
pub use series::SERIES_DELIMETER;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct NftSeries {
    tokens: nft::NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    series: UnorderedMap<series::SeriesId, series::Series>,
    next_series_id: series::SeriesId,
    series_minted_tokens: UnorderedMap<series::SeriesId, UnorderedSet<series::SeriesTokenIndex>>,
    owner_ids: UnorderedSet<AccountId>,
    nearapps_logger: AccountId,
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    /// Note: must be a name-only variant, otherwise it could
    /// collide with [`nft::core::StorageKey::TokensPerOwner`].
    NonFungibleToken,
    /// Note: must be a name-only variant, otherwise it could
    /// collide with [`nft::core::StorageKey::TokenPerOwnerInner`].
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
    Series,
    TokensBySeries,
    TokensBySeriesInner {
        series_id: series::SeriesId,
    },
    Owners,
}

#[near_bindgen]
impl NftSeries {
    /// Initializes the NftSeries contract.
    ///
    /// Adapted from the standard example.
    #[init]
    pub fn new(
        owner_id: AccountId,
        metadata: NFTContractMetadata,
        nearapps_logger: AccountId,
    ) -> Self {
        require!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        let mut owner_ids = UnorderedSet::new(StorageKey::Owners);
        owner_ids.insert(&owner_id);
        Self {
            tokens: nft::NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            series: UnorderedMap::new(StorageKey::Series),
            next_series_id: series::SeriesId(0),
            series_minted_tokens: UnorderedMap::new(StorageKey::TokensBySeries),
            owner_ids,
            nearapps_logger,
        }
    }

    /// Initializes the NftSeries contract, using a dummy
    /// nft contract metadata.
    ///
    /// Identical to the standard example.
    #[init]
    pub fn new_default_meta(owner_id: AccountId, nearapps_logger: AccountId) -> Self {
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: NFT_METADATA_SPEC.to_string(),
                name: "Example NEAR non-fungible token".to_string(),
                symbol: "EXAMPLE".to_string(),
                icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
            nearapps_logger,
        )
    }

    /// Creates a new nft token.
    ///
    /// The `token_id` cannot contain the series delimiter
    /// character, which is `:`.
    ///
    /// Adapted from the standard example.
    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id: nft::TokenId,
        token_owner_id: AccountId,
        token_metadata: TokenMetadata,
    ) -> nft::Token {
        self.assert_owner();

        // token_id must not contain the series delimiter
        ensure(
            !token_id.contains(SERIES_DELIMETER),
            Error::TokenIdWithSeriesDelimiter,
        );

        // standard minting
        let token =
            self.tokens
                .internal_mint(token_id, token_owner_id.clone(), Some(token_metadata));

        // NEP171 (Non-Fungible Token Event) 1.0.0
        env::log_str(&format!("EVENT_JSON:{{\"standard\":\"nep171\",\"version\":\"1.0.0\",\"event\":\"nft_mint\",\"data\":[{{\"owner_id\":\"{}\",\"token_ids\":[\"{}\"]}}]}}", token_owner_id.as_str(), token.token_id.as_str()));

        token
    }

    /// Creates a new nft token from a created token series.
    ///
    /// Adapted from the standard example.
    #[payable]
    pub fn nft_series_mint(
        &mut self,
        series_id: series::SeriesId,
        token_owner_id: AccountId,
        token_metadata: Option<TokenMetadata>,
    ) -> nft::Token {
        self.assert_owner();

        let mut series = self.nft_series_get(series_id);
        let mut minted_tokens = self.nft_series_get_minted_tokens(series_id);

        let token = series.next_token();

        // updates series
        self.series.insert(&series_id, &series);

        // updates minted_tokens
        let token_index = series.last_token_index().unwrap();
        minted_tokens.insert(&token_index);
        self.series_minted_tokens.insert(&series_id, &minted_tokens);

        let token_metadata = token_metadata.unwrap_or_else(|| TokenMetadata {
            title: Some(token.0.clone()),
            description: Some(format!(
                "Token #{} from series {}",
                token_index.0, series.name
            )),
            media: None,
            media_hash: None,
            copies: None,
            issued_at: Some(env::block_timestamp().to_string()),
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
        });

        // standard minting
        let token =
            self.tokens
                .internal_mint(token.0, token_owner_id.clone(), Some(token_metadata));

        // NEP171 (Non-Fungible Token Event) 1.0.0
        env::log_str(&format!("EVENT_JSON:{{\"standard\":\"nep171\",\"version\":\"1.0.0\",\"event\":\"nft_mint\",\"data\":[{{\"owner_id\":\"{}\",\"token_ids\":[\"{}\"]}}]}}", token_owner_id.as_str(), token.token_id.as_str()));

        token
    }
}

pub mod std_impls {
    use super::nft::{self, Token, TokenId};
    use super::NftSeries;
    use near_sdk::{near_bindgen, AccountId, Promise, PromiseOrValue};
    use nearapps_log::{NearAppsAccount, NearAppsTags};

    #[allow(unused_imports)]
    use near_contract_standards as ncs;

    #[cfg(not(target_arch = "wasm32"))]
    use crate::NftSeriesContract;

    // ncs::impl_non_fungible_token_core!(Nft, tokens);
    //
    /// All functions are just the expanded macros from the standard,
    /// except that a logging procedure is inserted at the end
    /// of the state-changing  functions.
    ///
    /// Copy of the std documentation:  
    /// Used for all non-fungible tokens. The specification for the
    /// [core non-fungible token standard] lays out the reasoning for each method.
    /// It's important to check out [NonFungibleTokenReceiver](crate::non_fungible_token::core::NonFungibleTokenReceiver)
    /// and [NonFungibleTokenResolver](crate::non_fungible_token::core::NonFungibleTokenResolver) to
    /// understand how the cross-contract call work.
    ///
    /// [core non-fungible token standard]: https://nomicon.io/Standards/NonFungibleToken/Core.html
    use nft::core::NonFungibleTokenCore;
    #[near_bindgen]
    impl NftSeries {
        /// Copy of the std documentation:  
        /// Simple transfer. Transfer a given `token_id` from current owner to
        /// `receiver_id`.
        ///
        /// Requirements
        /// * Caller of the method must attach a deposit of 1 yoctoⓃ for security purposes
        /// * Contract MUST panic if called by someone other than token owner or,
        ///   if using Approval Management, one of the approved accounts
        /// * `approval_id` is for use with Approval Management,
        ///   see <https://nomicon.io/Standards/NonFungibleToken/ApprovalManagement.html>
        /// * If using Approval Management, contract MUST nullify approved accounts on
        ///   successful transfer.
        /// * TODO: needed? Both accounts must be registered with the contract for transfer to
        ///   succeed. See see <https://nomicon.io/Standards/StorageManagement.html>
        ///
        /// Arguments:
        /// * `receiver_id`: the valid NEAR account receiving the token
        /// * `token_id`: the token to transfer
        /// * `approval_id`: expected approval ID. A number smaller than
        ///    2^53, and therefore representable as JSON. See Approval Management
        ///    standard for full explanation.
        /// * `memo` (optional): for use cases that may benefit from indexing or
        ///    providing information for a transfer
        #[payable]
        pub fn nft_transfer_logged(
            &mut self,
            receiver_id: AccountId,
            token_id: TokenId,
            approval_id: Option<u64>,
            memo: Option<String>,
            nearapps_tags: NearAppsTags,
        ) {
            self.tokens
                .nft_transfer(receiver_id, token_id, approval_id, memo);

            // best-effort call for nearapps log
            let _ = self.log(nearapps_tags);
        }

        /// Copy of the std documentation:  
        /// Transfer token and call a method on a receiver contract. A successful
        /// workflow will end in a success execution outcome to the callback on the NFT
        /// contract at the method `nft_resolve_transfer`.
        ///
        /// You can think of this as being similar to attaching native NEAR tokens to a
        /// function call. It allows you to attach any Non-Fungible Token in a call to a
        /// receiver contract.
        ///
        /// Requirements:
        /// * Caller of the method must attach a deposit of 1 yoctoⓃ for security
        ///   purposes
        /// * Contract MUST panic if called by someone other than token owner or,
        ///   if using Approval Management, one of the approved accounts
        /// * The receiving contract must implement `ft_on_transfer` according to the
        ///   standard. If it does not, FT contract's `ft_resolve_transfer` MUST deal
        ///   with the resulting failed cross-contract call and roll back the transfer.
        /// * Contract MUST implement the behavior described in `ft_resolve_transfer`
        /// * `approval_id` is for use with Approval Management extension, see
        ///   that document for full explanation.
        /// * If using Approval Management, contract MUST nullify approved accounts on
        ///   successful transfer.
        ///
        /// Arguments:
        /// * `receiver_id`: the valid NEAR account receiving the token.
        /// * `token_id`: the token to send.
        /// * `approval_id`: expected approval ID. A number smaller than
        ///    2^53, and therefore representable as JSON. See Approval Management
        ///    standard for full explanation.
        /// * `memo` (optional): for use cases that may benefit from indexing or
        ///    providing information for a transfer.
        /// * `msg`: specifies information needed by the receiving contract in
        ///    order to properly handle the transfer. Can indicate both a function to
        ///    call and the parameters to pass to that function.
        #[payable]
        pub fn nft_transfer_call_logged(
            &mut self,
            receiver_id: AccountId,
            token_id: TokenId,
            approval_id: Option<u64>,
            memo: Option<String>,
            msg: String,
            nearapps_tags: NearAppsTags,
        ) -> PromiseOrValue<bool> {
            // avoids using the standard implementation on
            // `self.tokens.nft_transfer_call`
            // because the GAS requirement setup isn't sufficient
            crate::transfer_call::nft_transfer_call(
                &mut self.tokens,
                receiver_id,
                token_id,
                approval_id,
                memo,
                msg,
                nearapps_tags,
            )
        }

        /// Copy of the std documentation:  
        /// Returns the token with the given `token_id` or `null` if no such token.
        pub fn nft_token(
            //
            &self,
            token_id: TokenId,
        ) -> Option<Token> {
            self.tokens.nft_token(token_id)
        }
    }

    // ncs::impl_non_fungible_token_approval!(Nft, tokens);
    //
    /// All functions are just the expanded macros from the standard,
    /// except that a logging procedure is inserted at the end
    /// of the state-changing  functions.
    ///
    /// Copy of the std documentation:  
    /// Trait used when it's desired to have a non-fungible token that has a
    /// traditional escrow or approval system. This allows Alice to allow Bob
    /// to take only the token with the unique identifier "19" but not others.
    /// It should be noted that in the [core non-fungible token standard] there
    /// is a method to do "transfer and call" which may be preferred over using
    /// an approval management standard in certain use cases.
    ///
    /// [approval management standard]: https://nomicon.io/Standards/NonFungibleToken/ApprovalManagement.html
    /// [core non-fungible token standard]: https://nomicon.io/Standards/NonFungibleToken/Core.html
    use nft::approval::NonFungibleTokenApproval;
    #[near_bindgen]
    impl NftSeries {
        /// Copy of the std documentation:  
        /// Add an approved account for a specific token.
        ///
        /// Requirements
        /// * Caller of the method must attach a deposit of at least 1 yoctoⓃ for
        ///   security purposes
        /// * Contract MAY require caller to attach larger deposit, to cover cost of
        ///   storing approver data
        /// * Contract MUST panic if called by someone other than token owner
        /// * Contract MUST panic if addition would cause `nft_revoke_all` to exceed
        ///   single-block gas limit
        /// * Contract MUST increment approval ID even if re-approving an account
        /// * If successfully approved or if had already been approved, and if `msg` is
        ///   present, contract MUST call `nft_on_approve` on `account_id`. See
        ///   `nft_on_approve` description below for details.
        ///
        /// Arguments:
        /// * `token_id`: the token for which to add an approval
        /// * `account_id`: the account to add to `approvals`
        /// * `msg`: optional string to be passed to `nft_on_approve`
        ///
        /// Returns void, if no `msg` given. Otherwise, returns promise call to
        /// `nft_on_approve`, which can resolve with whatever it wants.
        #[payable]
        pub fn nft_approve_logged(
            &mut self,
            token_id: TokenId,
            account_id: AccountId,
            msg: Option<String>,
            nearapps_tags: NearAppsTags,
        ) -> Option<Promise> {
            let res = self.tokens.nft_approve(token_id, account_id, msg);

            // best-effort call for nearapps log
            let _ = self.log(nearapps_tags);

            res
        }

        /// Copy of the std documentation:  
        /// Revoke an approved account for a specific token.
        ///
        /// Requirements
        /// * Caller of the method must attach a deposit of 1 yoctoⓃ for security
        ///   purposes
        /// * If contract requires >1yN deposit on `nft_approve`, contract
        ///   MUST refund associated storage deposit when owner revokes approval
        /// * Contract MUST panic if called by someone other than token owner
        ///
        /// Arguments:
        /// * `token_id`: the token for which to revoke an approval
        /// * `account_id`: the account to remove from `approvals`
        #[payable]
        pub fn nft_revoke_logged(
            //
            &mut self,
            token_id: TokenId,
            account_id: AccountId,
            nearapps_tags: NearAppsTags,
        ) {
            self.tokens.nft_revoke(token_id, account_id);

            // best-effort call for nearapps log
            let _ = self.log(nearapps_tags);
        }

        /// Copy of the std documentation:  
        /// Revoke all approved accounts for a specific token.
        ///
        /// Requirements
        /// * Caller of the method must attach a deposit of 1 yoctoⓃ for security
        ///   purposes
        /// * If contract requires >1yN deposit on `nft_approve`, contract
        ///   MUST refund all associated storage deposit when owner revokes approvals
        /// * Contract MUST panic if called by someone other than token owner
        ///
        /// Arguments:
        /// * `token_id`: the token with approvals to revoke
        #[payable]
        pub fn nft_revoke_all_logged(
            //
            &mut self,
            token_id: TokenId,
            nearapps_tags: NearAppsTags,
        ) {
            self.tokens.nft_revoke_all(token_id);

            // best-effort call for nearapps log
            let _ = self.log(nearapps_tags);
        }

        /// Copy of the std documentation:  
        /// Check if a token is approved for transfer by a given account, optionally
        /// checking an approval_id
        ///
        /// Arguments:
        /// * `token_id`: the token for which to revoke an approval
        /// * `approved_account_id`: the account to check the existence of in `approvals`
        /// * `approval_id`: an optional approval ID to check against current approval ID for given account
        ///
        /// Returns:
        /// if `approval_id` given, `true` if `approved_account_id` is approved with given `approval_id`
        /// otherwise, `true` if `approved_account_id` is in list of approved accounts
        pub fn nft_is_approved(
            &self,
            token_id: TokenId,
            approved_account_id: AccountId,
            approval_id: Option<u64>,
        ) -> bool {
            self.tokens
                .nft_is_approved(token_id, approved_account_id, approval_id)
        }
    }

    // ncs::impl_non_fungible_token_enumeration!(Nft, tokens);
    //
    /// All functions are just the expanded macros from the standard,
    /// except that a logging procedure is inserted at the end
    /// of the state-changing  functions.
    ///
    /// Copy of the std documentation:  
    /// Offers methods helpful in determining account ownership of NFTs and provides a way to page through NFTs per owner, determine total supply, etc.
    use near_sdk::json_types::U128;
    use nft::enumeration::NonFungibleTokenEnumeration;
    #[near_bindgen]
    impl NonFungibleTokenEnumeration for NftSeries {
        /// Copy of the std documentation:  
        /// Returns the total supply of non-fungible tokens as a string representing an
        /// unsigned 128-bit integer to avoid JSON number limit of 2^53.
        fn nft_total_supply(&self) -> U128 {
            self.tokens.nft_total_supply()
        }

        /// Copy of the std documentation:  
        /// Get a list of all tokens
        ///
        /// Arguments:
        /// * `from_index`: a string representing an unsigned 128-bit integer,
        ///    representing the starting index of tokens to return
        /// * `limit`: the maximum number of tokens to return
        ///
        /// Returns an array of Token objects, as described in Core standard
        fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Token> {
            self.tokens.nft_tokens(from_index, limit)
        }

        /// Copy of the std documentation:  
        /// Get number of tokens owned by a given account
        ///
        /// Arguments:
        /// * `account_id`: a valid NEAR account
        ///
        /// Returns the number of non-fungible tokens owned by given `account_id` as
        /// a string representing the value as an unsigned 128-bit integer to avoid JSON
        /// number limit of 2^53.
        fn nft_supply_for_owner(&self, account_id: AccountId) -> U128 {
            self.tokens.nft_supply_for_owner(account_id)
        }

        /// Copy of the std documentation:  
        /// Get list of all tokens owned by a given account
        ///
        /// Arguments:
        /// * `account_id`: a valid NEAR account
        /// * `from_index`: a string representing an unsigned 128-bit integer,
        ///    representing the starting index of tokens to return
        /// * `limit`: the maximum number of tokens to return
        ///
        /// Returns a paginated list of all tokens owned by this account
        fn nft_tokens_for_owner(
            &self,
            account_id: AccountId,
            from_index: Option<U128>,
            limit: Option<u64>,
        ) -> Vec<Token> {
            self.tokens
                .nft_tokens_for_owner(account_id, from_index, limit)
        }
    }
}

/// standard implementation.
///
/// Copy of the std documentation:  
/// Offers details on the contract-level metadata.
#[near_bindgen]
impl NonFungibleTokenMetadataProvider for NftSeries {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}

impl nearapps_log::NearAppsAccount for NftSeries {
    fn nearapps_account(&self) -> near_sdk::AccountId {
        self.nearapps_logger.clone()
    }
}

/// Identical to the standard example.
const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";
