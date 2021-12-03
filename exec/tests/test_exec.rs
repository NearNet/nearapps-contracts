#![allow(clippy::ref_in_deref)]

use crate::utils::{setup_counter, setup_exec};
use near_sdk::serde_json::json;
use near_sdk_sim::call;
use nearapps_exec::exec::{CallContext, ContractCall};
use nearapps_exec::signing::eddsa_ed25519::types::{PubKey, Sign};

mod utils;

fn sign(ctx: &ContractCall) -> (PubKey, Sign) {
    use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature};
    let seckey_bytes: [u8; 32] = [
        62, 70, 27, 163, 92, 182, 11, 3, 77, 234, 98, 4, 11, 127, 79, 228, 243, 187, 150, 73, 201,
        137, 76, 22, 85, 251, 152, 2, 241, 42, 72, 54,
    ];

    let secret: SecretKey = SecretKey::from_bytes(&seckey_bytes).unwrap();
    let public: PublicKey = PublicKey::from(&secret);
    let keypair: Keypair = Keypair { secret, public };

    // TODO: confirm what information should be signed
    let msg_bytes = near_sdk::serde_json::to_string(&ctx).unwrap();

    let sign: Signature = {
        use ed25519_dalek::Signer;
        keypair.sign(msg_bytes.as_bytes())
    };

    (public.into(), sign.into())
}

fn into_callctx(ctx: ContractCall) -> CallContext {
    let (public_key, signature) = sign(&ctx);
    CallContext {
        contract_call: ctx,
        app_id: None,
        caller: None,
        public_key,
        signature,
    }
}

#[test]
fn test_exec_basic() {
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
    let ctx = into_callctx(ctx);
    let res = call!(&root, exec.execute(ctx));
    let val: u8 = res.unwrap_json();
    assert_eq!(val, 2);
}
