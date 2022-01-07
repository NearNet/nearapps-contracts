use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use serde_with::{serde_as, FromInto};

#[serde_as]
#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, Copy)]
#[serde(transparent)]
#[serde(crate = "near_sdk::serde")]
pub struct JBalance(
    #[serde_as(as = "FromInto<near_sdk::json_types::U128>")]
    //
    pub u128,
);

impl From<u128> for JBalance {
    fn from(balance: u128) -> Self {
        JBalance(balance)
    }
}

impl From<near_sdk::json_types::U128> for JBalance {
    fn from(balance: near_sdk::json_types::U128) -> Self {
        JBalance(balance.0)
    }
}
