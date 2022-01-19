use crate::error::Error;
use crate::SendNft;
use near_sdk::{env, near_bindgen, AccountId};
use nearapps_near_ext::ensure;

#[cfg(not(target_arch = "wasm32"))]
use crate::SendNftContract;

pub trait Owner {
    fn assert_owner(&self);
}

impl Owner for SendNft {
    fn assert_owner(&self) {
        ensure(env::predecessor_account_id() == self.owner, Error::NotOwner)
    }
}

#[near_bindgen]
impl SendNft {
    pub fn get_owner(&self) -> AccountId {
        self.owner.clone()
    }

    pub fn change_owner(&mut self, new_owner: AccountId) {
        self.assert_owner();
        self.owner = new_owner;
    }
}
