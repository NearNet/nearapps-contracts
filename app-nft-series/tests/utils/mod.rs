#![allow(dead_code)]

use near_contract_standards::non_fungible_token as nft;
pub use near_sdk::json_types::{Base64VecU8, U64};
use near_sdk::AccountId;
use near_sdk_sim::{deploy, ContractAccount, UserAccount};
use near_units::parse_near;
use nearapps_exec::ExecutorContract;
use nft::metadata::TokenMetadata;

use nearapps_nft_series::NftSeriesContract;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    EXEC_WASM_BYTES => "../res/nearapps_exec.wasm",
    NFT_SERIES_WASM_BYTES => "../res/nearapps_nft_series.wasm",
}

pub fn setup_exec(root: &UserAccount) -> ContractAccount<ExecutorContract> {
    let contract = deploy!(
        contract: ExecutorContract,
        contract_id: "executor".to_string(),
        bytes: &EXEC_WASM_BYTES,
        signer_account: root,
        deposit: parse_near!("200 N"),
        init_method: new(root.account_id())
    );
    contract
}

pub fn setup_nft(
    root: &UserAccount,
    nearapps_acc: AccountId,
) -> ContractAccount<NftSeriesContract> {
    deploy!(
        contract: NftSeriesContract,
        contract_id: "nft".to_string(),
        bytes: &NFT_SERIES_WASM_BYTES,
        signer_account: root,
        deposit: parse_near!("200 N"),
        init_method: new_default_meta(root.account_id(), nearapps_acc)
    )
}

pub fn user(id: u32) -> AccountId {
    format!("user{}", id).parse().unwrap()
}

pub fn token_metadata() -> TokenMetadata {
    TokenMetadata {
        title: Some("default-title".to_string()),
        description: None,
        media: None,
        media_hash: None,
        copies: None,
        issued_at: None,
        expires_at: None,
        starts_at: None,
        updated_at: None,
        extra: None,
        reference: None,
        reference_hash: None,
    }
}

pub fn token_ids(tokens: &[nft::Token]) -> Vec<nft::TokenId> {
    let mut tokens: Vec<_> = tokens.iter().map(|t| t.token_id.clone()).collect();
    tokens.sort();
    tokens
}
