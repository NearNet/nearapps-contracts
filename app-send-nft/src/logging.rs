use crate::SendNft;
use near_sdk::{near_bindgen, AccountId};
use nearapps_log::{ILoggerAccount, LoggerAccount};

#[cfg(not(target_arch = "wasm32"))]
use crate::SendNftContract;

impl LoggerAccount for SendNft {}

#[near_bindgen]
impl ILoggerAccount for SendNft {
    fn set_logger_account(&mut self, account: AccountId) {
        self.assert_owner();
        self.nearapps_logger = account;
    }

    fn is_logger_account(&self, account: AccountId) -> bool {
        account == self.get_logger_account()
    }

    fn get_logger_account(&self) -> near_sdk::AccountId {
        self.nearapps_logger.clone()
    }
}
