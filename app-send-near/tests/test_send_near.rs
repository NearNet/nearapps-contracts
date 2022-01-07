#![allow(clippy::ref_in_deref)]
#![allow(unused_imports)]
#![allow(unused_variables)]

pub use near_sdk::json_types::{Base64VecU8, U128, U64};
use near_sdk_sim::{call, init_simulator, view};
use near_units::parse_near;
use nearapps_log::{print_vec, NearAppsTags};
use nearapps_near_ext::ExecutionExt;
use nearapps_send_near::error::Error;

pub mod utils;

use crate::utils::user;

#[allow(clippy::bool_assert_comparison)]
#[test]
fn test_nft() {
    let root = init_simulator(None);
    let exec = utils::setup_exec(&root);
    let send_near = utils::setup_send_near(&root, exec.account_id());

    let users: Vec<_> = (0..10)
        .into_iter()
        .map(|i| root.create_user(user(i), parse_near!("100 N")))
        .collect();

    // ok: user0 sends 1 Near to user1
    assert_eq!(users[1].account().unwrap().amount, parse_near!("100 N"));
    let tags = &NearAppsTags::new("send_near", 0, "user0");
    let res = call!(
        users[0],
        send_near.send_attached_logged(user(1), tags.clone()),
        deposit = parse_near!("1 N")
    );
    print_vec(&res.all_logs());
    assert!(res.all_logs().contains(&tags.to_string()));
    assert_eq!(res.unwrap_json::<bool>(), true);
    res.assert_success();
    assert_eq!(users[1].account().unwrap().amount, parse_near!("101 N"));

    // fail: user0 sends 1 Near to user11 (inexistent user)
    let tags = &NearAppsTags::new("send_near", 1, "user0");
    let res = call!(
        users[0],
        send_near.send_attached_logged(user(11), tags.clone()),
        deposit = parse_near!("1 N")
    );
    print_vec(&res.all_logs());
    // success, but a response of false
    assert_eq!(res.unwrap_json::<bool>(), false);

    // ok: confirm that the user0 balance is 1 N
    let res = view!(send_near.get_balance(user(0)));
    assert_eq!(res.unwrap_json::<U128>().0, parse_near!("1 N"));

    // ok: user0 sends 0 Near to user1 but doesn't attach anything
    assert_eq!(users[1].account().unwrap().amount, parse_near!("101 N"));
    let tags = &NearAppsTags::new("send_near", 2, "user0");
    let res = call!(
        users[0],
        send_near.send_attached_logged(user(1), tags.clone()),
        deposit = parse_near!("0 N")
    );
    print_vec(&res.all_logs());
    assert!(res.all_logs().contains(&tags.to_string()));
    assert_eq!(res.unwrap_json::<bool>(), true);
    res.assert_success();
    assert_eq!(users[1].account().unwrap().amount, parse_near!("101 N"));

    // ok: confirm that the user0 balance is 1 N
    let res = view!(send_near.get_balance(user(0)));
    assert_eq!(res.unwrap_json::<U128>().0, parse_near!("1 N"));

    // ok: user0 sends 1 Near to user1 (using explicit amount)
    assert_eq!(users[1].account().unwrap().amount, parse_near!("101 N"));
    let tags = &NearAppsTags::new("send_near", 3, "user0");
    let res = call!(
        users[0],
        send_near.send_logged(user(1), parse_near!("1 N").into(), tags.clone()),
        deposit = parse_near!("1 N")
    );
    print_vec(&res.all_logs());
    assert!(res.all_logs().contains(&tags.to_string()));
    assert_eq!(res.unwrap_json::<bool>(), true);
    res.assert_success();
    assert_eq!(users[1].account().unwrap().amount, parse_near!("102 N"));

    // ok: user0 sends 1 Near to user1
    // using explicit amount; attaches more than needed
    assert_eq!(users[1].account().unwrap().amount, parse_near!("102 N"));
    let tags = &NearAppsTags::new("send_near", 4, "user0");
    let res = call!(
        users[0],
        send_near.send_logged(user(1), parse_near!("1 N").into(), tags.clone()),
        // 9 extra Near attached
        deposit = parse_near!("10 N")
    );
    print_vec(&res.all_logs());
    assert!(res.all_logs().contains(&tags.to_string()));
    assert_eq!(res.unwrap_json::<bool>(), true);
    res.assert_success();
    assert_eq!(users[1].account().unwrap().amount, parse_near!("103 N"));

    // ok: confirm that the user0 balance is 10 N
    let res = view!(send_near.get_balance(user(0)));
    assert_eq!(res.unwrap_json::<U128>().0, parse_near!("10 N"));

    // fail: user0 sends 11 Near to user1 (not enought balance)
    // using explicit amount; uses tracked balance
    assert_eq!(users[1].account().unwrap().amount, parse_near!("103 N"));
    let tags = &NearAppsTags::new("send_near", 5, "user0");
    let res = call!(
        users[0],
        send_near.send_logged(user(1), parse_near!("11 N").into(), tags.clone()),
        deposit = parse_near!("0 N")
    );
    res.assert_failure(0, Error::InsufficientFunds);
    assert_eq!(users[1].account().unwrap().amount, parse_near!("103 N"));

    // ok: user0 sends 1 Near to user1
    // using explicit amount; uses tracked balance
    assert_eq!(users[1].account().unwrap().amount, parse_near!("103 N"));
    let tags = &NearAppsTags::new("send_near", 6, "user0");
    let res = call!(
        users[0],
        send_near.send_logged(user(1), parse_near!("1 N").into(), tags.clone()),
        deposit = parse_near!("0 N")
    );
    print_vec(&res.all_logs());
    assert!(res.all_logs().contains(&tags.to_string()));
    assert_eq!(res.unwrap_json::<bool>(), true);
    res.assert_success();
    assert_eq!(users[1].account().unwrap().amount, parse_near!("104 N"));

    let res = view!(send_near.get_balance(user(0)));
    assert_eq!(res.unwrap_json::<U128>().0, parse_near!("9 N"));

    // ok: user0 sends 2 Near to user1
    // using explicit amount; uses attached and tracked balance
    assert_eq!(users[1].account().unwrap().amount, parse_near!("104 N"));
    let tags = &NearAppsTags::new("send_near", 7, "user0");
    let res = call!(
        users[0],
        send_near.send_logged(user(1), parse_near!("2 N").into(), tags.clone()),
        deposit = parse_near!("1 N")
    );
    print_vec(&res.all_logs());
    assert_eq!(res.unwrap_json::<bool>(), true);
    assert!(res.all_logs().contains(&tags.to_string()));
    res.assert_success();
    assert_eq!(users[1].account().unwrap().amount, parse_near!("106 N"));

    // ok: user0 sends 8 Near to user1
    // using explicit amount; uses all of the tracked balance
    assert_eq!(users[1].account().unwrap().amount, parse_near!("106 N"));
    let tags = &NearAppsTags::new("send_near", 8, "user0");
    let res = call!(
        users[0],
        send_near.send_logged(user(1), parse_near!("8 N").into(), tags.clone()),
        deposit = parse_near!("0 N")
    );
    print_vec(&res.all_logs());
    assert!(res.all_logs().contains(&tags.to_string()));
    assert_eq!(res.unwrap_json::<bool>(), true);
    res.assert_success();
    assert_eq!(users[1].account().unwrap().amount, parse_near!("114 N"));

    // ok: user0 sends 0 Near to itself
    // using explicit amount; attaches more than needed
    let tags = &NearAppsTags::new("send_near", 9, "user0");
    let res = call!(
        users[0],
        send_near.send_logged(user(0), parse_near!("0 N").into(), tags.clone()),
        deposit = parse_near!("1 N")
    );
    print_vec(&res.all_logs());
    assert!(res.all_logs().contains(&tags.to_string()));
    assert_eq!(res.unwrap_json::<bool>(), true);
    res.assert_success();
    assert_eq!(users[1].account().unwrap().amount, parse_near!("114 N"));

    // ok: user0 sends 2 Near to user1
    // using explicit amount; uses all of the tracked balance
    // and the attached amount
    assert_eq!(users[1].account().unwrap().amount, parse_near!("114 N"));
    let tags = &NearAppsTags::new("send_near", 10, "user0");
    let res = call!(
        users[0],
        send_near.send_logged(user(1), parse_near!("2 N").into(), tags.clone()),
        deposit = parse_near!("1 N")
    );
    print_vec(&res.all_logs());
    assert!(res.all_logs().contains(&tags.to_string()));
    assert_eq!(res.unwrap_json::<bool>(), true);
    res.assert_success();
    assert_eq!(users[1].account().unwrap().amount, parse_near!("116 N"));

    // ok: user0 sends 0 Near to itself
    // using explicit amount; attaches more than needed
    let user0_amount = users[0].account().unwrap().amount;
    let tags = &NearAppsTags::new("send_near", 11, "user0");
    let res = call!(
        users[0],
        send_near.send_logged(user(0), parse_near!("0 N").into(), tags.clone()),
        deposit = parse_near!("1 N")
    );
    print_vec(&res.all_logs());
    assert!(res.all_logs().contains(&tags.to_string()));
    assert_eq!(res.unwrap_json::<bool>(), true);
    res.assert_success();
    assert_eq!(
        users[0].account().unwrap().amount,
        user0_amount - res.total_gas_burnt().0 as u128 * 100_000_000 - parse_near!("1 N")
    );

    // ok: user0 withdraws
    let user0_amount = users[0].account().unwrap().amount;
    let tags = &NearAppsTags::new("send_near", 12, "user0");
    let res = call!(users[0], send_near.withdraw_logged(tags.clone()));
    print_vec(&res.all_logs());
    assert!(res.all_logs().contains(&tags.to_string()));
    assert_eq!(res.unwrap_json::<bool>(), true);
    res.assert_success();
    assert_eq!(
        users[0].account().unwrap().amount,
        user0_amount - res.total_gas_burnt().0 as u128 * 100_000_000 + parse_near!("1 N")
    );

    // ok: user0 balance is zeroed
    let res = view!(send_near.get_balance(user(0)));
    res.assert_failure(None, Error::MissingUser);
}
