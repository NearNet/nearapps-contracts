#![allow(clippy::ref_in_deref)]
#![allow(unused_imports)]
#![allow(unused_variables)]

pub use near_sdk::json_types::{Base64VecU8, U64};
use near_sdk_sim::{call, init_simulator};
use near_units::parse_near;
use nearapps_log::{print_vec, NearAppsTags};
use nearapps_near_ext::ExecutionExt;
use nearapps_send_near::error::Error;

pub mod utils;

use crate::utils::user;

#[test]
fn test_nft() {
    let root = init_simulator(None);
    let exec = utils::setup_exec(&root);
    let nft = utils::setup_send_near(&root, exec.account_id());

    let users: Vec<_> = (0..10)
        .into_iter()
        .map(|i| root.create_user(user(i), parse_near!("100 N")))
        .collect();
}
