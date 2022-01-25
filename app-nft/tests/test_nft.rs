#![allow(clippy::ref_in_deref)]

use near_contract_standards::non_fungible_token as nft;
pub use near_sdk::json_types::{Base64VecU8, U64};
use near_sdk::serde_json;
use near_sdk_sim::{call, init_simulator};
use nearapps_nft::error::Error;
use nearapps_nft::series::{SeriesId, SeriesTokenIndex};

pub mod utils;

use crate::utils::{token_ids, user, AssertFailure, MEGA_TERA, YOTTA};

#[test]
fn test_nft() {
    const COMMON_ATTACHMENT: u128 = 6450 * MEGA_TERA;

    let root = init_simulator(None);
    let nft = utils::setup_nft(&root);

    let users: Vec<_> = (0..10)
        .into_iter()
        .map(|i| root.create_user(user(i), 100 * YOTTA))
        .collect();

    // ok: root mints a token for user0
    let metadata = utils::token_metadata();
    let token_id_01 = &"token-01".to_string();
    println!(
        "metadata: {}",
        serde_json::to_string_pretty(&metadata).unwrap()
    );
    let res = call!(
        &root,
        nft.nft_mint(token_id_01.clone(), user(0), metadata),
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
        deposit = COMMON_ATTACHMENT
    );
    let series_01_token_0: nft::Token = res.unwrap_json();
    let log = r#"EVENT_JSON:{"standard":"nep171","version":"1.0.0","event":"nft_mint","data":[{"owner_id":"user0","token_ids":["series-01:0:0"]}]}"#;
    assert!(res.logs().contains(&log.to_string()));

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
    let log = r#"EVENT_JSON:{"standard":"nep171","version":"1.0.0","event":"nft_mint","data":[{"owner_id":"user2","token_ids":["series-01:0:1"]}]}"#;
    assert!(res.logs().contains(&log.to_string()));

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
    let non_log = r#"EVENT_JSON:{"standard":"nep171","version":"1.0.0","event":"nft_mint","data":[{"owner_id":"user2","token_ids":["series-01:0:2"]}]}"#;
    assert!(!res.logs().contains(&non_log.to_string()));
}
