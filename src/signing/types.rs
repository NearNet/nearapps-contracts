// use digest::consts::{U32, U64};

pub mod secp256k1 {
    /// Private Key value.
    ///
    /// Has a total size of 32 bytes.
    #[derive(near_sdk::serde::Serialize, near_sdk::serde::Deserialize, Clone, PartialEq, Debug)]
    #[serde(crate = "near_sdk::serde")]
    #[serde(transparent)]
    pub struct SecKey(
        #[serde(with = "serde_big_array::BigArray")]
        //
        pub [u8; 32],
    );

    // TODO: replace by extended pubkey, as this is what the
    // nearcore uses
    //
    /// Public Key serialized in compressed form.  
    /// Instead of having both `x` and `y` values, only `x` is present,
    /// as `y` can be derived from that.
    ///
    /// Has a total size of 33 bytes, containing:
    ///
    /// - `header` (1-byte);
    ///   - If `y` was even, the `header` is `0x02`;
    ///   - If `y` was odd, the `header` is `0x03`.
    /// - `x` (32-bytes).
    #[derive(near_sdk::serde::Serialize, near_sdk::serde::Deserialize, Clone, PartialEq, Debug)]
    #[serde(crate = "near_sdk::serde")]
    #[serde(transparent)]
    pub struct PubKeyCompact(
        #[serde(with = "serde_big_array::BigArray")]
        //
        pub [u8; 33],
    );

    /// Signature in serialized compact form.
    ///
    /// Has a total size of 64 bytes, containing:
    ///
    /// - `r` (32-bytes big-endian);
    /// - `s` (32-bytes big-endian).
    ///
    /// See also: [`k256::ecdsa::Signature`].
    #[derive(near_sdk::serde::Serialize, near_sdk::serde::Deserialize, Clone, PartialEq, Debug)]
    #[serde(crate = "near_sdk::serde")]
    #[serde(transparent)]
    pub struct SignCompact(
        #[serde(with = "serde_big_array::BigArray")]
        //
        pub [u8; 64],
    );
}

pub mod ed25519 {
    // todo: replace this by an extended key, as this
    // is what nearcore uses
    //
    /// Private Key value.
    ///
    /// Has a total size of 32 bytes.
    #[derive(near_sdk::serde::Serialize, near_sdk::serde::Deserialize, Clone, PartialEq, Debug)]
    #[serde(crate = "near_sdk::serde")]
    #[serde(transparent)]
    pub struct SecKey(
        #[serde(with = "serde_big_array::BigArray")]
        //
        pub [u8; ed25519_dalek::SECRET_KEY_LENGTH],
    );

    /// Public Key value.  
    ///
    /// Has a total size of 32 bytes.
    #[derive(near_sdk::serde::Serialize, near_sdk::serde::Deserialize, Clone, PartialEq, Debug)]
    #[serde(crate = "near_sdk::serde")]
    #[serde(transparent)]
    pub struct PubKey(
        #[serde(with = "serde_big_array::BigArray")]
        //
        pub [u8; ed25519_dalek::PUBLIC_KEY_LENGTH],
    );

    // TODO: replace by a recoverable signature,
    // which has an additional byte at msb (last byte),
    // as this is what nearcore uses
    //
    /// Signature in serialized form.
    ///
    /// Has a total size of 64 bytes.
    #[derive(near_sdk::serde::Serialize, near_sdk::serde::Deserialize, Clone, PartialEq, Debug)]
    #[serde(crate = "near_sdk::serde")]
    #[serde(transparent)]
    pub struct Sign(
        #[serde(with = "serde_big_array::BigArray")]
        //
        pub [u8; 64],
    );
}

pub mod hash {

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
            //
            // TODO: it could return a [u8; 32] instead of Vec
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
}
