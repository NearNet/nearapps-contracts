#![allow(clippy::ref_in_deref)]

use near_sdk::json_types::U128;
use near_sdk::AccountId;
use near_sdk_sim::{call, to_yocto, view};

mod utils;

use crate::utils::*;

#[test]
fn test_sign() {
    let (root, contract) = setup_contract();

    let res = view!(contract.test_sign());
    let _ = res.unwrap();

    panic!("VIEW OK");

    let res = call!(&root, contract.test_sign());
    res.assert_success();
}
