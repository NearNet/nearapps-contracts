use crate::Crypto;
use near_sdk::near_bindgen;
use nearapps_near_ext::{version_from_env, IVersion, Version};

#[cfg(not(target_arch = "wasm32"))]
use crate::CryptoContract;

#[near_bindgen]
impl IVersion for Crypto {
    fn version(&self) -> Version {
        version_from_env!()
    }
}
