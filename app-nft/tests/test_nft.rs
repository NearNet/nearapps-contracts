#![allow(clippy::ref_in_deref)]

use near_contract_standards::non_fungible_token as nft;
pub use near_sdk::json_types::{Base64VecU8, U64};
use near_sdk_sim::{call, init_simulator};
use nearapps_log::{print_vec, NearAppsTags};
use nearapps_nft::error::Error;
use nearapps_nft::series::{SeriesId, SeriesTokenIndex};

pub mod utils;

use crate::utils::{token_ids, user, AssertFailure, MEGA_TERA, YOTTA};

#[test]
fn test_nft() {
    const COMMON_ATTACHMENT: u128 = 6450 * MEGA_TERA;

    let root = init_simulator(None);
    let exec = utils::setup_exec(&root);
    let nft = utils::setup_nft(&root, exec.account_id());

    let users: Vec<_> = (0..10)
        .into_iter()
        .map(|i| root.create_user(user(i), 100 * YOTTA))
        .collect();

    // ok: root mints a token for user0
    let token_id_01 = &"token-01".to_string();
    let tags = NearAppsTags::new("nft", 0, "root");
    let res = call!(
        &root,
        nft.nft_mint_logged(
            token_id_01.clone(),
            user(0),
            utils::token_metadata(),
            tags.clone()
        ),
        deposit = 5630 * MEGA_TERA
    );
    print_vec(&res.all_logs());
    assert!(res.all_logs().contains(&tags.to_string()));
    res.assert_success();

    // fail: similar, but not enought storage deposit
    let token_id_02 = &"token-02".to_string();
    let tags = &NearAppsTags::new("nft", 1, "root");
    let res = call!(
        &root,
        nft.nft_mint_logged(
            token_id_02.clone(),
            user(0),
            utils::token_metadata(),
            tags.clone()
        ),
        // not enought deposit
        deposit = 4_290 * MEGA_TERA - 1
    );
    print_vec(&res.all_logs());
    assert!(res.all_logs().is_empty());
    res.assert_failure(
        0,
        "Must attach 4290000000000000000000 yoctoNEAR to cover storage",
    );

    // ok: user0 transfers it to user1
    let tags = NearAppsTags::new("nft", 0, "user0");
    let res = call!(
        users[0],
        nft.nft_transfer_logged(user(1), token_id_01.clone(), None, None, tags.clone()),
        deposit = 1
    );
    print_vec(&res.all_logs());
    assert!(res.all_logs().contains(&tags.to_string()));
    res.assert_success();

    // ok: root creates a series
    let series_01 = &"series-01".to_string();
    let tags = NearAppsTags::new("nft", 2, "root");
    let res = call!(
        &root,
        nft.nft_series_create_logged(
            series_01.clone(),
            SeriesTokenIndex(2),
            root.account_id(),
            tags.clone()
        )
    );
    print_vec(&res.all_logs());
    assert!(res.all_logs().contains(&tags.to_string()));
    let series_01_id: SeriesId = res.unwrap_json();

    // ok: root mints the series for user0
    let tags = NearAppsTags::new("nft", 3, "root");
    let res = call!(
        &root,
        nft.nft_series_mint_logged(series_01_id, user(0), None, tags.clone()),
        deposit = COMMON_ATTACHMENT
    );
    print_vec(&res.all_logs());
    assert!(res.all_logs().contains(&tags.to_string()));
    let series_01_token_0: nft::Token = res.unwrap_json();

    // ok: user0 transfers it to user1
    let tags = NearAppsTags::new("nft", 0, "user1");
    let res = call!(
        users[0],
        nft.nft_transfer_logged(
            user(1),
            series_01_token_0.token_id,
            None,
            None,
            tags.clone()
        ),
        deposit = 1
    );
    print_vec(&res.all_logs());
    assert!(res.all_logs().contains(&tags.to_string()));
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
    let tags = NearAppsTags::new("nft", 4, "root");
    let res = call!(
        &root,
        nft.nft_series_mint_logged(series_01_id, user(2), None, tags.clone()),
        deposit = COMMON_ATTACHMENT
    );
    print_vec(&res.all_logs());
    assert!(res.all_logs().contains(&tags.to_string()));
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
    let tags = &NearAppsTags::new("nft", 0, "user2");
    let res = call!(
        &root,
        nft.nft_series_mint_logged(series_01_id, user(2), None, tags.clone()),
        deposit = COMMON_ATTACHMENT
    );
    print_vec(&res.all_logs());
    assert!(res.all_logs().is_empty());
    res.assert_failure(0, Error::SeriesNotMintable);
}
