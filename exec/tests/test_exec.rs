#![allow(clippy::ref_in_deref)]

use crate::utils::*;
use near_sdk_sim::call;

mod utils;

#[test]
fn test_exec() {
    use nearapps_exec::exec::{CallContext, ContractCall};

    let (root, exec) = setup_exec();
    let counter = setup_counter(&root);

    // ok: calls counter directly
    let res = call!(&root, counter.increment());
    let val: u8 = res.unwrap_json();
    assert_eq!(val, 1);

    // ok: calls counter through exec
    let ctx = ContractCall {
        contract_id: counter.account_id(),
        method_name: "increment".into(),
        args: "".into(),
    };
    let ctx = CallContext {
        contract_call: ctx,
        app_id: None,
        caller: None,
    };
    let res = call!(&root, exec.execute(ctx));
    let val: u8 = res.unwrap_json();
    assert_eq!(val, 2);
}
