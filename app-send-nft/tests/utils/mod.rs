#![allow(dead_code)]

pub use near_sdk::json_types::{Base64VecU8, U64};
use near_sdk::AccountId;
use near_sdk_sim::{deploy, ContractAccount, UserAccount};
use near_units::parse_near;
use nearapps_exec::ExecutorContract;

use nearapps_send_nft::SendNftContract;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    EXEC_WASM_BYTES => "../res/nearapps_exec.wasm",
    SEND_NFT_WASM_BYTES => "../res/nearapps_send_near.wasm",
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

pub fn setup_send_near(
    root: &UserAccount,
    nearapps_acc: AccountId,
) -> ContractAccount<SendNftContract> {
    deploy!(
        contract: SendNftContract,
        contract_id: "send-nft".to_string(),
        bytes: &SEND_NFT_WASM_BYTES,
        signer_account: root,
        deposit: parse_near!("200 N"),
        init_method: new(root.account_id(), nearapps_acc)
    )
}

pub fn user(id: u32) -> AccountId {
    format!("user{}", id).parse().unwrap()
}
