use crate::Executor;
use near_sdk::{env, near_bindgen};

#[cfg(not(target_arch = "wasm32"))]
use crate::ExecutorContract;

/// Sha256 value.
///
/// Has a total size of 32 bytes.
#[derive(
    near_sdk::serde::Serialize, near_sdk::serde::Deserialize, Clone, PartialEq, Debug, Default,
)]
#[serde(crate = "near_sdk::serde")]
#[serde(transparent)]
pub struct Sha256(
    #[serde(with = "serde_big_array::BigArray")]
    //
    pub [u8; 32],
);

impl Sha256 {
    pub fn hash_bytes(msg_bytes: &[u8]) -> Self {
        // TODO: check if using `env` actually saves gas cost
        // (although it should save storage cost)
        let hash = near_sdk::env::sha256(msg_bytes);
        let hash = hash.as_slice();
        assert_eq!(hash.len(), 32);
        let mut res = [0u8; 32];
        res.copy_from_slice(hash);
        Sha256(res)
    }
}

// needed so this is a Digest
impl digest::BlockInput for Sha256 {
    type BlockSize = digest::consts::U64;
}

// needed so this is a Digest
// but this is not needed for sign verification
impl digest::Update for Sha256 {
    fn update(&mut self, _input: impl AsRef<[u8]>) {
        unimplemented!();
    }
}

// needed so this is a Digest
// but this is not needed for sign verification
impl digest::Reset for Sha256 {
    fn reset(&mut self) {
        unimplemented!();
    }
}

// needed so this is a Digest
// this is needed for sign verification
impl digest::FixedOutputDirty for Sha256 {
    type OutputSize = digest::consts::U32;

    fn finalize_into_dirty(&mut self, out: &mut digest::Output<Self>) {
        out.copy_from_slice(&self.0);
    }
}

/// Sha512 value.
///
/// Has a total size of 64 bytes.
#[derive(near_sdk::serde::Serialize, near_sdk::serde::Deserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
#[serde(transparent)]
pub struct Sha512(
    #[serde(with = "serde_big_array::BigArray")]
    //
    pub [u8; 64],
);

impl Sha512 {
    pub fn hash_bytes(msg_bytes: &[u8]) -> Self {
        use sha2::Digest;
        let mut hash = sha2::Sha512::new();
        hash.update(msg_bytes);
        let hash = hash.finalize();
        let mut res = [0u8; 64];
        res.copy_from_slice(hash.as_slice());
        Sha512(res)
    }
}

impl Default for Sha512 {
    fn default() -> Self {
        Sha512([0; 64])
    }
}

// needed so this is a Digest
impl digest::BlockInput for Sha512 {
    type BlockSize = digest::consts::U128;
}

// needed so this is a Digest
// but this is not needed for sign verification
impl digest::Update for Sha512 {
    fn update(&mut self, _input: impl AsRef<[u8]>) {
        unimplemented!();
    }
}

// needed so this is a Digest
// but this is not needed for sign verification
impl digest::Reset for Sha512 {
    fn reset(&mut self) {
        unimplemented!();
    }
}

// needed so this is a Digest
// this is not needed for sign verification
impl digest::FixedOutputDirty for Sha512 {
    type OutputSize = digest::consts::U64;

    fn finalize_into_dirty(&mut self, out: &mut digest::Output<Self>) {
        out.copy_from_slice(&self.0);
    }
}

#[near_bindgen]
impl Executor {
    /// Generates a `sha256` hash of the given bytes.
    ///
    /// The returned hash has a total size of 32-bytes.
    ///
    /// See also: [`Self::hash_sha256_msg`]
    pub fn hash_sha256(msg_bytes: Vec<u8>) -> Sha256 {
        let hash = env::sha256(&msg_bytes);
        let hash = hash.as_slice();
        assert_eq!(hash.len(), 32);
        let mut res = [0u8; 32];
        res.copy_from_slice(hash);
        Sha256(res)
    }

    /// Generates a `sha256` hash of the byte-repesentation of the
    /// given `msg`.
    ///
    /// The returned hash has a total size of 32-bytes.
    ///
    /// See also: [`Self::hash_sha256`]
    pub fn hash_sha256_msg(msg: String) -> Sha256 {
        let hash = env::sha256(msg.as_bytes());
        let hash = hash.as_slice();
        assert_eq!(hash.len(), 32);
        let mut res = [0u8; 32];
        res.copy_from_slice(hash);
        Sha256(res)
    }

    /// Generates a `sha512` hash of the given bytes.
    ///
    /// The returned hash has a total size of 64-bytes.
    ///
    /// See also: [`Self::hash_sha512_msg`]
    pub fn hash_sha512(msg_bytes: Vec<u8>) -> Sha512 {
        Sha512::hash_bytes(&msg_bytes)
    }

    /// Generates a `sha512` hash of the byte-repesentation of the
    /// given `msg`.
    ///
    /// The returned hash has a total size of 64-bytes.
    ///
    /// See also: [`Self::hash_sha512`]
    pub fn hash_sha512_msg(msg: String) -> Sha512 {
        Sha512::hash_bytes(msg.as_bytes())
    }
}
