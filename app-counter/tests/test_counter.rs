#![allow(clippy::ref_in_deref)]

pub use near_sdk::json_types::{Base64VecU8, U64};
use near_sdk_sim::{self as sim, call, view};
use nearapps_log::{print_vec, NearAppsTags};
use nearapps_near_ext::ExecutionExt;

pub mod utils;

#[test]
fn simulate_increment() {
    let root = sim::init_simulator(None);
    let no_exec: near_sdk::AccountId = "no-exec".parse().unwrap();
    let counter = utils::setup_counter(&root, no_exec);

    let mut current_num: u32 = view!(counter.get()).unwrap_json();
    assert_eq!(&current_num, &0);

    let tags = NearAppsTags::new("counter", 0, "root");
    let res = call!(root, counter.increment(tags.clone()));
    print_vec(&res.all_logs());
    // logs are not contained because the registered exec
    // contract doesn't actually exists
    assert!(!res.all_logs().contains(&tags.to_string()));
    res.assert_success();

    current_num = view!(counter.get()).unwrap_json();
    assert_eq!(&current_num, &1);
}
