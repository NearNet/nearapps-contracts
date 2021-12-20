#![allow(clippy::ref_in_deref)]

use anyhow::anyhow;
use near_contract_standards::non_fungible_token as nft;
pub use near_sdk::json_types::{Base64VecU8, U64};
use near_sdk::serde_json::json;
use near_sdk_sim::{call, init_simulator};
use near_units::{parse_gas, parse_near};
use nearapps_near_ext::{workspaces, Call, ExecutionExt};
use nearapps_nft::error::Error;
use nearapps_nft::series::{SeriesId, SeriesTokenIndex};

use nearapps_near_ext::workspaces::network::DevAccountDeployer;

// use crate::utils::Call;

use near_primitives::views::FinalExecutionStatus;

pub trait FinalExecutionStatusExt {
    fn success(&self) -> anyhow::Result<()>;
    fn failure<E: ToString>(&self, action: u32, err: E) -> anyhow::Result<()>;
    fn unwrap_json<'de, T>(&'de self) -> Result<Option<T>, near_sdk::serde_json::Error>
    where
        T: near_sdk::serde::Deserialize<'de>;
}

impl FinalExecutionStatusExt for FinalExecutionStatus {
    fn success(&self) -> anyhow::Result<()> {
        use FinalExecutionStatus as Fes;
        match self {
            Fes::SuccessValue(_v) => Ok(()),
            other => Err(anyhow!("Expected success. Received: {:?}", other)),
        }
    }
    fn failure<E: ToString>(&self, action: u32, err: E) -> anyhow::Result<()> {
        use FinalExecutionStatus as Fes;

        let err = format!(
            "Action #{}: ExecutionError(\"Smart contract panicked: {}\")",
            action,
            err.to_string()
        );
        match self {
            Fes::Failure(txerr) => {
                if err == txerr.to_string() {
                    Ok(())
                } else {
                    Err(anyhow!("Expected {}, received {}", err, txerr.to_string()))
                }
            }
            other => Err(anyhow!("Expected failure. Received: {:?}", other)),
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
    const COMMON_ATTACHMENT: u128 = parse_near!("6530 microN");

    let root = init_simulator(None);
    let nft = utils::setup_nft(&root);

    let users: Vec<_> = (0..10)
        .into_iter()
        .map(|i| root.create_user(user(i), parse_near!("100 N")))
        .collect();

    // ok: root mints a token for user0
    let token_id_01 = &"token-01".to_string();
    let res = call!(
        &root,
        nft.nft_mint(token_id_01.clone(), user(0), utils::token_metadata()),
        deposit = parse_near!("5630 microN")
    );
    res.assert_success();

    // fail: similar, but not enought storage deposit
    let token_id_02 = &"token-02".to_string();
    let res = call!(
        &root,
        nft.nft_mint(token_id_02.clone(), user(0), utils::token_metadata()),
        // not enought deposit
        deposit = parse_near!("4290 microN") - parse_near!("1 yN")
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
        deposit = COMMON_ATTACHMENT
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
    assert_eq!(tokens, vec!["series-01:0:0", "token-01"]);

    // ok: root mints the series for user2
    let res = call!(
        &root,
        nft.nft_series_mint(series_01_id, user(2), None),
        deposit = COMMON_ATTACHMENT
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
    assert_eq!(tokens, vec!["series-01:0:1"]);

    // fail: root tries to mint on the same series again
    // (no more capacity)
    let res = call!(
        &root,
        nft.nft_series_mint(series_01_id, user(2), None),
        deposit = COMMON_ATTACHMENT
    );
    res.assert_failure(0, Error::SeriesNotMintable);
}

#[tokio::test]
async fn test_nft2() -> anyhow::Result<()> {
    let worker = workspaces::sandbox();

    // let user = owner.dev_create_account().await?;
    let owner = worker.dev_create_account().await?;
    let owner_id = &owner.id().to_string();

    let nft = std::fs::read(NFT_WASM)?;
    let nft = worker.dev_deploy(nft).await?;

    worker.client();
    owner.signer();

    println!();
    println!("ok: owner initializes the nft contract");
    let res = worker
        .call_with_json(
            owner.signer(),
            nft.id(),
            "new_default_meta",
            json!( { "owner_id": owner_id.clone() }),
            parse_gas!("9 Tgas") as u64,
            0,
        )
        .await?;
    assert!(res.total_gas_burnt < parse_gas!("9 Tgas") as u64);
    let res: Empty = res.status.unwrap_json()?;
    assert!(res.is_none());
    println!();

    let mut users = vec![];
    for i in 0..2 {
        let user = user(i).to_string();
        let user = owner
            .create_subaccount(&worker, &user)
            .transact()
            .await?
            .unwrap();
        users.push(user);
    }

    println!();
    println!("ok: owner mints a token for user0");
    let token_id_01 = &"token-01".to_string();
    let res = worker
        .call_with_json(
            owner.signer(),
            nft.id(),
            "nft_mint",
            json!( {
                "token_id": token_id_01.clone(),
                "token_owner_id": users[0].id(),
                "token_metadata": utils::token_metadata(),
            }),
            parse_gas!("300 Tgas") as u64,
            parse_near!("6310 microN"),
        )
        .await?;
    res.status.success()?;

    println!();
    println!("fail: similar, but not enought storage deposit");
    let token_id_02 = &"token-02".to_string();
    let res = worker
        .call_with_json(
            owner.signer(),
            nft.id(),
            "nft_mint",
            json!( {
                "token_id": token_id_02.clone(),
                "token_owner_id": users[0].id(),
                "token_metadata": utils::token_metadata(),
            }),
            parse_gas!("300 Tgas") as u64,
            0,
        )
        .await?;
    res.status.failure(
        0,
        "Must attach 4630000000000000000000 yoctoNEAR to cover storage",
    )?;

    Ok(())
}
