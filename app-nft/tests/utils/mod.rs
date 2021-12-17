#![allow(dead_code)]

use near_contract_standards::non_fungible_token as nft;
pub use near_sdk::json_types::{Base64VecU8, U64};
use near_sdk::AccountId;
use near_sdk_sim::transaction::ExecutionStatus;
use near_sdk_sim::{deploy, init_simulator, ContractAccount, ExecutionResult, UserAccount};
use nft::metadata::TokenMetadata;

use nearapps_nft::NftContract;

pub const DEFAULT_GAS: u64 = 300_000_000_000_000;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    EXEC_WASM_BYTES => "../res/nearapps_exec.wasm",
    NFT_WASM_BYTES => "../res/nearapps_nft.wasm",
}

pub const KILO: u64 = 1000;
pub const MEGA: u64 = KILO * KILO;
pub const TERA: u64 = MEGA * MEGA;
pub const MEGA_TERA: u128 = MEGA as u128 * TERA as u128;
pub const YOTTA: u128 = (TERA as u128) * (TERA as u128);

pub trait AssertFailure {
    fn assert_failure<E: ToString>(&self, action: u32, err: E);
}

impl AssertFailure for ExecutionResult {
    fn assert_failure<E: ToString>(&self, action: u32, err: E) {
        let err = format!(
            "Action #{}: Smart contract panicked: {}",
            action,
            err.to_string()
        );
        match self.status() {
            ExecutionStatus::Failure(txerr_) => {
                assert_eq!(txerr_.to_string(), err)
            }
            ExecutionStatus::Unknown => panic!("Got Unknown. Should have failed with {}", err),
            ExecutionStatus::SuccessValue(_v) => {
                panic!("Got SuccessValue. Should have failed with {}", err)
            }
            ExecutionStatus::SuccessReceiptId(_id) => {
                panic!("Got SuccessReceiptId. Should have failed with {}", err)
            }
        }
    }
}

pub fn setup_nft(root: &UserAccount) -> ContractAccount<NftContract> {
    deploy!(
        contract: NftContract,
        contract_id: "nft".to_string(),
        bytes: &NFT_WASM_BYTES,
        signer_account: root,
        deposit: 200 * YOTTA,
        init_method: new_default_meta(root.account_id())
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
