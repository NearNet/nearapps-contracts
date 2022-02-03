use crate::SendNft;
use near_sdk::near_bindgen;
use nearapps_near_ext::{version_from_env, IVersion, Version};

#[cfg(not(target_arch = "wasm32"))]
use crate::SendNftContract;

#[near_bindgen]
impl IVersion for SendNft {
    fn version(&self) -> Version {
        version_from_env!()
    }
}
