#![allow(clippy::ref_in_deref)]

use crate::utils::*;
use near_sdk_sim::call;

mod utils;

#[test]
fn test_exec() {
    use nearapps_exec::exec as exec_lib;

    let (root, exec) = setup_exec();
    let counter = setup_counter(&root);

    let res = call!(&root, counter.increment());
    let val: u8 = res.unwrap_json();
    assert_eq!(val, 1);
}
