#![allow(clippy::ref_in_deref)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use near_contract_standards::non_fungible_token as nft;
pub use near_sdk::json_types::{Base64VecU8, U128, U64};
use near_sdk::AccountId;
use near_sdk_sim::{call, init_simulator, view};
use near_units::{parse_gas, parse_near};
use nearapps_log::{print_vec, NearAppsTags};
use nearapps_near_ext::ExecutionExt;
use nearapps_send_nft::error::Error;
use nearapps_send_nft::NftProtocol;

pub mod utils;

use crate::utils::user;

#[allow(clippy::bool_assert_comparison)]
#[allow(non_snake_case)]
#[test]
fn test_nft() {
    let root = init_simulator(None);
    let exec = utils::setup_exec(&root);
    let (nftA, nftB) = (
        utils::setup_nft(&root, exec.account_id(), "nft-a"),
        utils::setup_nft(&root, exec.account_id(), "nft-b"),
    );
    let send_nftA = utils::setup_send_nft(&root, exec.account_id(), "send-nft-a");
    let send_nftB = utils::setup_send_nft(&root, exec.account_id(), "send-nft-b");

    let users: Vec<_> = (0..10)
        .into_iter()
        .map(|i| root.create_user(user(i), parse_near!("100 N")))
        .collect();

    // token names
    let tokens = vec![
        //
        "token-0", "token-1", "token-2",
    ];

    // ok: root mints token0 for user0 on nftA
    let tags = NearAppsTags::new("nftA", 0, "root");
    let res = call!(
        &root,
        nftA.nft_mint_logged(
            tokens[0].into(),
            user(0),
            utils::token_metadata(),
            tags.clone()
        ),
        deposit = parse_near!("5630 microN")
    );
    print_vec(&res.all_logs());
    assert!(res.all_logs().contains(&tags.to_string()));
    res.assert_success();

    // fail: user0 sends token0 to send-nft,
    // but send-nft doesn't allow that nftA contract yet
    {
        // fail: user0 sends token0 to send-nft,
        // but send-nft doesn't allow that nftA contract yet
        let tags = NearAppsTags::new("nftA", 0, "user0");
        let res = call!(
            &users[0],
            nftA.nft_transfer_call_logged(
                send_nftA.account_id(),
                tokens[0].into(),
                None,
                None,
                "".to_string(),
                tags.clone()
            ),
            deposit = parse_near!("1 yN")
        );
        print_vec(&res.all_logs());
        assert!(res.all_logs().contains(&tags.to_string()));
        // transfer failed because the receiver (send-nft) denied
        // receiving it
        let transfer_success = res.unwrap_json::<bool>();
        assert!(!transfer_success);

        // ok: confirm that user0 owns token0 on nftA
        let res = view!(nftA.nft_token(tokens[0].into()));
        assert_eq!(
            res.unwrap_json::<nft::Token>().owner_id,
            users[0].account_id()
        );

        // ok: confirm that no nft contract is registered on
        // send-nft
        let res = view!(send_nftA.get_nft_protocols(None, None));
        assert_eq!(res.unwrap_json::<Vec<(AccountId, NftProtocol)>>(), vec![]);

        // ok: registers nftA on send-nft
        let res = call!(
            &root,
            send_nftA.add_nft_protocol(
                //
                nftA.account_id().into(),
                nearapps_send_nft::NftProtocol::NearApps
            )
        );
        print_vec(&res.all_logs());
        res.assert_success();

        // ok: confirm that nftA is registered on send-nft
        let res = view!(send_nftA.get_nft_protocols(None, None));
        assert_eq!(
            res.unwrap_json::<Vec<(AccountId, NftProtocol)>>(),
            vec![(nftA.account_id(), NftProtocol::NearApps)]
        );

        // fail: tries to register nftA on send-nft again
        let res = call!(
            &root,
            send_nftA.add_nft_protocol(
                //
                nftA.account_id().into(),
                nearapps_send_nft::NftProtocol::NearApps
            )
        );
        print_vec(&res.all_logs());
        res.assert_failure(0, Error::NftProtocolAlreadyIncluded);
    }

    // fail: user0 sends the token0 to send-nft,
    // but user0 isn't registered on send-nft contract yet
    {
        // fail: user0 sends the token0 to send-nft,
        // but user0 isn't registered on send-nft contract yet
        let tags = NearAppsTags::new("nftA", 1, "user0");
        let res = call!(
            &users[0],
            nftA.nft_transfer_call_logged(
                send_nftA.account_id(),
                tokens[0].into(),
                None,
                None,
                "".to_string(),
                tags.clone()
            ),
            deposit = parse_near!("1 yN")
        );
        print_vec(&res.all_logs());
        assert!(res.all_logs().contains(&tags.to_string()));
        // transfer failed because the receiver (send-nft) denied
        // receiving it
        let transfer_success = res.unwrap_json::<bool>();
        assert!(!transfer_success);

        // ok: confirm that user0 owns token0 on nftA
        let res = view!(nftA.nft_token(tokens[0].into()));
        assert_eq!(
            res.unwrap_json::<nft::Token>().owner_id,
            users[0].account_id()
        );

        // ok: confirm that nobody owns any token from nftA on
        // send-nft
        let res = view!(send_nftA.get_tokens_owned_by_users(nftA.account_id().into(), None, None));
        assert_eq!(res.unwrap_json::<Vec<(nft::TokenId, AccountId)>>(), vec![]);

        // ok: registers user0 on send-nft
        let res = call!(
            &root,
            send_nftA.add_user(
                //
                users[0].account_id().into()
            )
        );
        print_vec(&res.all_logs());
        res.assert_success();

        // fail: tries to registers user0 on send-nft again
        let res = call!(
            &root,
            send_nftA.add_user(
                //
                users[0].account_id().into()
            )
        );
        print_vec(&res.all_logs());
        res.assert_failure(0, Error::UserAlreadyRegistered);
    }

    // fail: user0 sends token0 to send-nft,
    // but user0 doesn't have nftA enabled for themselves on
    // send-nft yet
    {
        //
        // fail: user0 sends token0 to send-nft,
        // but user0 doesn't have nftA enabled for themselves on
        // send-nft yet
        let tags = NearAppsTags::new("nftA", 2, "user0");
        let res = call!(
            &users[0],
            nftA.nft_transfer_call_logged(
                send_nftA.account_id(),
                tokens[0].into(),
                None,
                None,
                "".to_string(),
                tags.clone()
            ),
            deposit = parse_near!("1 yN")
        );
        print_vec(&res.all_logs());
        assert!(res.all_logs().contains(&tags.to_string()));
        // transfer failed because the receiver (send-nft) denied
        // receiving it
        let transfer_success = res.unwrap_json::<bool>();
        assert!(!transfer_success);

        // ok: confirm that user0 owns token0 on nftA
        let res = view!(nftA.nft_token(tokens[0].into()));
        assert_eq!(
            res.unwrap_json::<nft::Token>().owner_id,
            users[0].account_id()
        );

        // ok: confirm that nobody owns any token from nftA on
        // send-nft
        let res = view!(send_nftA.get_tokens_owned_by_users(nftA.account_id().into(), None, None));
        assert_eq!(res.unwrap_json::<Vec<(nft::TokenId, AccountId)>>(), vec![]);

        // ok: enables nftA for user0 on send-nft
        let res = call!(
            &root,
            send_nftA.enable_nft_for_user(
                //
                users[0].account_id().into(),
                nftA.account_id().into()
            )
        );
        print_vec(&res.all_logs());
        res.assert_success();

        // fail: tries to enable nftA for user0 on send-nft again
        let res = call!(
            &root,
            send_nftA.enable_nft_for_user(
                //
                users[0].account_id().into(),
                nftA.account_id().into()
            )
        );
        print_vec(&res.all_logs());
        res.assert_failure(0, Error::NftContractAlreadyEnabled);

        // fail: tries to enable nftA for user1 (unregistered) on
        // send-nft
        let res = call!(
            &root,
            send_nftA.enable_nft_for_user(
                //
                users[1].account_id().into(),
                nftA.account_id().into()
            )
        );
        print_vec(&res.all_logs());
        res.assert_failure(0, Error::MissingUser);

        // fail: tries to enable nftB (unregistered) for user0
        // on send-nft
        let res = call!(
            &root,
            send_nftA.enable_nft_for_user(
                //
                users[0].account_id().into(),
                nftB.account_id().into()
            )
        );
        print_vec(&res.all_logs());
        res.assert_failure(0, Error::NftProtocolNotIncluded);
    }

    // ok: user0 sends the token0 to send-nft on nftA
    {
        // ok: user0 sends the token0 to send-nft
        let tags = NearAppsTags::new("nftA", 3, "user0");
        let res = call!(
            &users[0],
            nftA.nft_transfer_call_logged(
                send_nftA.account_id(),
                tokens[0].into(),
                None,
                None,
                "".to_string(),
                tags.clone()
            ),
            deposit = parse_near!("1 yN")
        );
        print_vec(&res.all_logs());
        assert!(res.all_logs().contains(&tags.to_string()));
        res.pretty_debug();
        let transfer_success = res.unwrap_json::<bool>();
        assert!(transfer_success);

        // ok: confirm that send-nft owns token0 on nftA
        let res = view!(nftA.nft_token(tokens[0].into()));
        assert_eq!(
            res.unwrap_json::<nft::Token>().owner_id,
            send_nftA.account_id()
        );

        // ok: confirm that user0 owns token0 on send-nft
        let res = view!(send_nftA.get_tokens_owned_by_users(nftA.account_id().into(), None, None));
        print_vec(&res.all_logs());
        assert_eq!(
            res.unwrap_json::<Vec<(nft::TokenId, AccountId)>>(),
            vec![(tokens[0].to_string(), users[0].account_id())]
        );
    }

    // ok: send-nft sends token0 to user1 on nftA
    {
        // ok: send-nft sends user0's token0 to user1 on nftA
        let tags = NearAppsTags::new("send-nft", 1, "root");
        let res = call!(
            &root,
            //
            parse_near!("1 yN"),
            parse_gas!("300 Tgas") as u64,
            // contract
            send_nftA,
            // method
            send_logged,
            // parameters
            nftA.account_id().into(),
            users[0].account_id().into(),
            users[1].account_id().into(),
            tokens[0].into(),
            None,
            None,
            tags.clone()
        );
        print_vec(&res.all_logs());
        res.pretty_debug();
        assert!(res.all_logs().contains(&tags.to_string()));
        let transfer_success = res.unwrap_json::<bool>();
        assert!(transfer_success);

        // ok: confirm that user1 owns token0 on nftA
        let res = view!(nftA.nft_token(tokens[0].into()));
        assert_eq!(
            res.unwrap_json::<nft::Token>().owner_id,
            users[1].account_id()
        );

        // ok: confirm that nobody owns any tokens on send-nft
        let res = view!(send_nftA.get_tokens_owned_by_users(nftA.account_id().into(), None, None));
        assert_eq!(res.unwrap_json::<Vec<(nft::TokenId, AccountId)>>(), vec![]);
    }

    // ok: user1 sends the token0 to send-nft on nftA
    {
        // ok: registers user1 on send-nft
        let res = call!(
            &root,
            send_nftA.add_user(
                //
                users[1].account_id().into()
            )
        );
        print_vec(&res.all_logs());
        res.assert_success();

        // ok: enables nftA for user1 on send-nft
        let res = call!(
            &root,
            send_nftA.enable_nft_for_user(
                //
                users[1].account_id().into(),
                nftA.account_id().into()
            )
        );
        print_vec(&res.all_logs());
        res.assert_success();

        // ok: user1 sends the token0 to send-nft
        let tags = NearAppsTags::new("nftA", 0, "user1");
        let res = call!(
            &users[1],
            nftA.nft_transfer_call_logged(
                send_nftA.account_id(),
                tokens[0].into(),
                None,
                None,
                "".to_string(),
                tags.clone()
            ),
            deposit = parse_near!("1 yN")
        );
        print_vec(&res.all_logs());
        assert!(res.all_logs().contains(&tags.to_string()));
        res.pretty_debug();
        let transfer_success = res.unwrap_json::<bool>();
        assert!(transfer_success);

        // ok: confirm that send-nft owns token0 on nftA
        let res = view!(nftA.nft_token(tokens[0].into()));
        assert_eq!(
            res.unwrap_json::<nft::Token>().owner_id,
            send_nftA.account_id()
        );

        // ok: confirm that user1 owns token0 on send-nft
        let res = view!(send_nftA.get_tokens_owned_by_users(nftA.account_id().into(), None, None));
        print_vec(&res.all_logs());
        assert_eq!(
            res.unwrap_json::<Vec<(nft::TokenId, AccountId)>>(),
            vec![(tokens[0].to_string(), users[1].account_id())]
        );
    }

    // fail: send-nft tries to send token0 to send-nft (itself)
    // on nftA
    {
        // fail: send-nft tries to send token0 to send-nft (itself)
        // on nftA
        let tags = NearAppsTags::new("send-nft", 2, "root");
        let res = call!(
            &root,
            //
            parse_near!("1 yN"),
            parse_gas!("300 Tgas") as u64,
            // contract
            send_nftA,
            // method
            send_logged,
            // parameters
            nftA.account_id().into(),
            users[1].account_id().into(),
            send_nftA.account_id().into(),
            tokens[0].into(),
            None,
            None,
            tags.clone()
        );
        print_vec(&res.all_logs());
        res.pretty_debug();
        assert!(!res.all_logs().contains(&tags.to_string()));
        res.assert_failure(0, Error::SelfReceiver);

        // ok: confirm that send-nft owns token0 on nftA
        let res = view!(nftA.nft_token(tokens[0].into()));
        assert_eq!(
            res.unwrap_json::<nft::Token>().owner_id,
            send_nftA.account_id()
        );

        // ok: confirm that user1 owns token0 on send-nft
        let res = view!(send_nftA.get_tokens_owned_by_users(nftA.account_id().into(), None, None));
        assert_eq!(
            res.unwrap_json::<Vec<(nft::TokenId, AccountId)>>(),
            vec![(tokens[0].to_string(), users[1].account_id())]
        );
    }

    // ok:
    // weird test:
    // send-nftA sends user1's nftA's token0 to send-nftB.
    // send-nftA will be treated as a user on send-nftB.
    //
    // this test can verify that the transfer_call functioned
    // properly
    {
        // ok: registers nftA on send-nftB
        let res = call!(
            &root,
            send_nftB.add_nft_protocol(
                //
                nftA.account_id().into(),
                nearapps_send_nft::NftProtocol::NearApps
            )
        );
        print_vec(&res.all_logs());
        res.assert_success();

        // ok: registers send-nftA (as a user) on send-nftB
        let res = call!(
            &root,
            send_nftB.add_user(
                //
                send_nftA.account_id().into()
            )
        );
        print_vec(&res.all_logs());
        res.assert_success();

        // ok: enables nftA for send-nftA (as a user) on send-nftB
        let res = call!(
            &root,
            send_nftB.enable_nft_for_user(
                //
                send_nftA.account_id().into(),
                nftA.account_id().into()
            )
        );
        print_vec(&res.all_logs());
        res.assert_success();

        // ok: send-nftA sends user1's nftA's token0 to send-nftB
        let tags = NearAppsTags::new("send-nft", 3, "root");
        let res = call!(
            &root,
            //
            parse_near!("1 yN"),
            parse_gas!("300 Tgas") as u64,
            // contract
            send_nftA,
            // method
            send_call_logged,
            // parameters
            nftA.account_id().into(),
            users[1].account_id().into(),
            send_nftB.account_id().into(),
            tokens[0].into(),
            None,
            None,
            "".to_string(),
            tags.clone()
        );
        print_vec(&res.all_logs());
        res.pretty_debug();
        assert!(res.all_logs().contains(&tags.to_string()));
        let transfer_success = res.unwrap_json::<bool>();
        assert!(transfer_success);

        // ok: confirm that send-nftB owns token0 on nftA
        let res = view!(nftA.nft_token(tokens[0].into()));
        assert_eq!(
            res.unwrap_json::<nft::Token>().owner_id,
            send_nftB.account_id()
        );

        // ok: confirm that send-nftA owns token0 on send-nftB
        let res = view!(send_nftB.get_tokens_owned_by_users(nftA.account_id().into(), None, None));
        assert_eq!(
            res.unwrap_json::<Vec<(nft::TokenId, AccountId)>>(),
            vec![(tokens[0].to_string(), send_nftA.account_id())]
        );
    }
}
