#![allow(clippy::ref_in_deref)]

use crate::utils::*;
use near_sdk_sim::call;

mod utils;

#[test]
fn test_exec() {
    use nearapps_exec::exec::{CallContext, ContractCall};
    use nearapps_exec::signing::eddsa_ed25519::types::{PubKey, Sign};

    let (root, exec) = setup_exec();
    let counter = setup_counter(&root);

    // ok: calls counter directly
    let res = call!(&root, counter.increment());
    let val: u8 = res.unwrap_json();
    assert_eq!(val, 1);

    // starts preparing the exec context
    let ctx = ContractCall {
        // "counter"
        contract_id: counter.account_id(),
        method_name: "increment".into(),
        args: "".into(),
    };
    // prepares signing info
    let (public_key, signature): (PubKey, Sign) = {
        use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature};
        let seckey_bytes: [u8; 32] = [
            62, 70, 27, 163, 92, 182, 11, 3, 77, 234, 98, 4, 11, 127, 79, 228, 243, 187, 150, 73,
            201, 137, 76, 22, 85, 251, 152, 2, 241, 42, 72, 54,
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
    };
    let ctx = CallContext {
        contract_call: ctx,
        app_id: None,
        caller: None,
        public_key,
        signature,
    };
    // ok: calls counter through exec
    let res = call!(&root, exec.execute(ctx));
    let val: u8 = res.unwrap_json();
    assert_eq!(val, 2);
}
