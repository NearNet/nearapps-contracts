#![allow(clippy::ref_in_deref)]

use near_contract_standards::non_fungible_token as nft;
pub use near_sdk::json_types::{Base64VecU8, U64};
use near_sdk::serde_json::json;
use near_sdk_sim::{call, init_simulator};
use nearapps_near_ext::{workspaces, Call, ExecutionExt, MEGA, TERA, YOTTA};
use nearapps_nft::error::Error;
use nearapps_nft::series::{SeriesId, SeriesTokenIndex};
// use workspaces::prelude::DevAccountDeployer;

use nearapps_near_ext::workspaces::network::DevAccountDeployer;

// use crate::utils::Call;

const MEGA_TERA: u128 = (MEGA as u128) * (TERA as u128);

use near_primitives::views::FinalExecutionStatus;

pub trait FinalExecutionStatusExt {
    fn assert_success(&self) -> bool;
    fn unwrap_json<'de, T>(&'de self) -> Result<Option<T>, near_sdk::serde_json::Error>
    where
        T: near_sdk::serde::Deserialize<'de>;
}

impl FinalExecutionStatusExt for FinalExecutionStatus {
    fn assert_success(&self) -> bool {
        use FinalExecutionStatus as Fes;
        match self {
            Fes::SuccessValue(_v) => true,
            _other => false,
        }
    }

    fn unwrap_json<'de, T>(&'de self) -> Result<Option<T>, near_sdk::serde_json::Error>
    where
        T: near_sdk::serde::Deserialize<'de>,
    {
        use FinalExecutionStatus as Fes;
        let v = match self {
            Fes::SuccessValue(v) => v,
            other => {
                panic!(
                    "Expected FinalExecutionStatus::SuccessValue, got {:?}",
                    &other
                )
            }
        };

        // empty string "" is not valid json
        if v.is_empty() {
            return Ok(None);
        }

        Ok(Some(near_sdk::serde_json::from_str(v)?))
    }
}

type Empty = Option<()>;

pub mod utils;

use crate::utils::{token_ids, user};

const NFT_WASM: &str = "../res/nearapps_nft.wasm";

#[test]
fn test_nft() {
    let root = init_simulator(None);
    let nft = utils::setup_nft(&root);

    const DEPOSIT_MINT: u128 = 6410 * MEGA_TERA;

    let users: Vec<_> = (0..10)
        .into_iter()
        .map(|i| root.create_user(user(i), 100 * YOTTA))
        .collect();

    // ok: root mints a token for user0
    let token_id_01 = &"token-01".to_string();
    let res = call!(
        &root,
        nft.nft_mint(token_id_01.clone(), user(0), utils::token_metadata()),
        deposit = 5630 * MEGA_TERA
    );
    res.assert_success();

    // fail: similar, but not enought storage deposit
    let token_id_02 = &"token-02".to_string();
    let res = call!(
        &root,
        nft.nft_mint(token_id_02.clone(), user(0), utils::token_metadata()),
        // not enought deposit
        deposit = 4_290 * MEGA_TERA - 1
    );
    res.assert_failure(
        0,
        "Must attach 4290000000000000000000 yoctoNEAR to cover storage",
    );

    // ok: user0 transfers it to user1
    let res = call!(
        users[0],
        nft.nft_transfer(user(1), token_id_01.clone(), None, None),
        deposit = 1
    );
    res.assert_success();

    // ok: root creates a series
    let series_01 = &"series-01".to_string();
    let res = call!(
        &root,
        nft.nft_series_create(series_01.clone(), SeriesTokenIndex(2), root.account_id())
    );
    let series_01_id: SeriesId = res.unwrap_json();

    // ok: root mints the series for user0
    let res = call!(
        &root,
        nft.nft_series_mint(series_01_id, user(0), None),
        deposit = DEPOSIT_MINT
    );
    let series_01_token_0: nft::Token = res.unwrap_json();

    // ok: user0 transfers it to user1
    let res = call!(
        users[0],
        nft.nft_transfer(user(1), series_01_token_0.token_id, None, None),
        deposit = 1
    );
    res.assert_success();

    // ok: get user1 tokens
    let res = call!(
        //
        users[1],
        nft.nft_tokens_for_owner(user(1), None, None)
    );
    let tokens: Vec<nft::Token> = res.unwrap_json();
    let tokens = token_ids(&tokens);
    assert_eq!(tokens, vec!["series-01:0", "token-01"]);

    // ok: root mints the series for user2
    let res = call!(
        &root,
        nft.nft_series_mint(series_01_id, user(2), None),
        deposit = DEPOSIT_MINT
    );
    let _series_01_token_1: nft::Token = res.unwrap_json();

    // ok: get user2 tokens
    let res = call!(
        //
        users[2],
        nft.nft_tokens_for_owner(user(2), None, None)
    );
    let tokens: Vec<nft::Token> = res.unwrap_json();
    let tokens = token_ids(&tokens);
    assert_eq!(tokens, vec!["series-01:1"]);

    // fail: root tries to mint on the same series again
    // (no more capacity)
    let res = call!(
        &root,
        nft.nft_series_mint(series_01_id, user(2), None),
        deposit = DEPOSIT_MINT
    );
    res.assert_failure(0, Error::SeriesNotMintable);
}

#[tokio::test]
async fn test_nft2() -> anyhow::Result<()> {
    let worker = workspaces::sandbox();

    let owner = worker.dev_create().await?;
    let owner_id = &owner.id().to_string();

    let nft = std::fs::read(NFT_WASM)?;
    let nft = worker.dev_deploy(nft).await?;

    worker.client();
    owner.signer();

    // ok: owner initializes the nft contract
    let res = worker
        .call_with_json(
            owner.signer(),
            nft.id(),
            "new_default_meta",
            json!( { "owner_id": owner_id.clone() }),
            0,
            9 * TERA,
        )
        .await?;
    assert!(res.total_gas_burnt < 9 * TERA);
    let res: Empty = res.status.unwrap_json()?;
    assert!(res.is_none());

    let mut users = vec![];
    for _ in 0..2 {
        let user = worker.dev_create().await?;
        users.push(user);
    }

    // ok: owner mints a token for user0
    let token_id_01 = &"token-01".to_string();
    let res = worker.call_with_json(
        owner.signer(),
        nft.id(),
        "nft_mint",
        json!( {
            "token_id": token_id_01.clone(),
            "token_owner_id": users[0].id(),
            "token_metadata": near_sdk::serde_json::to_string(&utils::token_metadata()).unwrap(),
        }),
        5630 * MEGA_TERA,
        300 * TERA,
    ).await?;
    res.status.assert_success();

    Ok(())
}
