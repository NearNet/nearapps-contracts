#![allow(clippy::ref_in_deref)]

use crate::utils::*;
use near_sdk_sim::call;

mod utils;

#[test]
fn test_exec() {
    use nearapps_exec::exec;

    let (root, contract) = setup_exec();
}
