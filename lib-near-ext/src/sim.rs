pub use near_sdk::json_types::{Base64VecU8, U64};
use near_sdk::Gas;
use near_sdk_sim::transaction::ExecutionStatus;
use near_sdk_sim::{ExecutionResult, ViewResult};

/// Extensions that can be added on ExecutionResult.
pub trait ExecutionExt {
    fn assert_failure<E: ToString>(&self, action: impl Into<Option<u32>>, err: E);
    fn total_gas_burnt(&self) -> Gas;
    fn pretty_debug(&self);
    fn all_logs(&self) -> Vec<String>;
}

impl ExecutionExt for ExecutionResult {
    fn assert_failure<E: ToString>(&self, action: impl Into<Option<u32>>, err: E) {
        let err = format!(
            "Action #{}: Smart contract panicked: {}",
            action.into().expect("expecting action number"),
            err.to_string()
        );
        match self.status() {
            ExecutionStatus::Failure(txerr_) => {
                assert_eq!(txerr_.to_string(), err)
            }
            ExecutionStatus::Unknown => panic!("Got Unknown. Should have failed with {}", err),
            ExecutionStatus::SuccessValue(_v) => {
                panic!("Got SuccessValue. Should have failed with {}", err)
            }
            ExecutionStatus::SuccessReceiptId(_id) => {
                panic!("Got SuccessReceiptId. Should have failed with {}", err)
            }
        }
    }
    fn total_gas_burnt(&self) -> Gas {
        let res = self
            // .get_receipt_results()
            // .into_iter()
            .promise_results()
            .iter()
            .flatten()
            .map(|o| o.gas_burnt().0)
            .sum::<u64>();
        res.into()
    }
    fn pretty_debug(&self) {
        use std::fmt::Write;

        let mut f = String::new();

        writeln!(f, "---").unwrap();
        writeln!(
            f,
            "{:?} - status: {:?} - total gas burnt: {}",
            self.executor_id(),
            pretty_status(self.status()),
            pretty_gas(self.total_gas_burnt())
        )
        .unwrap();
        let logs = self.logs();
        for l in logs {
            writeln!(f, "- log: {}", l).unwrap();
        }
        writeln!(f, "--- call stack ---").unwrap();

        for r in self.promise_results().into_iter().flatten() {
            writeln!(
                f,
                "{:?} - status: {:?} - gas burnt: {}",
                r.executor_id(),
                pretty_status(r.status()),
                pretty_gas(r.gas_burnt())
            )
            .unwrap();

            let logs = r.logs();

            for l in logs {
                writeln!(f, "- log: {}", l).unwrap();
            }
        }
        writeln!(f, "---").unwrap();

        println!("{}", f);
    }
    fn all_logs(&self) -> Vec<String> {
        let mut logs = vec![];
        for res in self.promise_results().into_iter().flatten() {
            logs.extend(res.logs().clone());
        }
        logs
    }
}

impl ExecutionExt for ViewResult {
    fn assert_failure<E: ToString>(&self, _action: impl Into<Option<u32>>, err: E) {
        let err = format!(
            "wasm execution failed with error: FunctionCallError(HostError(GuestPanic {{ panic_msg: \"{}\" }}))",
            err.to_string()
        );
        let view_err = self.unwrap_err();
        assert_eq!(view_err.to_string(), err)
    }
    fn total_gas_burnt(&self) -> Gas {
        panic!("View method doesn't burn any gas")
    }
    fn pretty_debug(&self) {
        unimplemented!()
    }
    fn all_logs(&self) -> Vec<String> {
        self.logs().clone()
    }
}

fn pretty_gas(gas: Gas) -> String {
    const TERA: u64 = 1_000_000_000_000;
    let tgas = gas.0 / TERA;
    let rem = gas.0 % TERA;
    if rem == 0 {
        format!("{} TGas", tgas)
    } else {
        // ignore whats below milli_tera
        let milli_tera = TERA / 1000;
        let milli_tera_gas = rem / milli_tera;

        format!("~{}.{:0>3} TGas", tgas, milli_tera_gas)
    }
}

fn pretty_status(status: ExecutionStatus) -> String {
    match status {
        ExecutionStatus::Unknown => "Unknown".into(),
        ExecutionStatus::Failure(e) => format!("Failure({})", e),
        ExecutionStatus::SuccessValue(v) => {
            format!("SuccessValue({})", pretty_utf8(&v))
        }
        ExecutionStatus::SuccessReceiptId(receipt_id) => {
            let receipt_id = format!("{}", receipt_id);
            format!(
                "SuccessReceiptId({}…)",
                receipt_id.chars().take(4).collect::<String>()
            )
        }
    }
}

/// Baed on `near_primitives_core::logging::pretty_utf8`.
pub fn pretty_utf8(buf: &[u8]) -> String {
    match std::str::from_utf8(buf) {
        Ok(s) => s.into(),
        Err(_) => {
            format!("{:?}", buf)
        }
    }
}
