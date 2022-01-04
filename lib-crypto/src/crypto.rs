pub mod ecdsa_secp256k1;
pub mod eddsa_ed25519;

pub use ecdsa_secp256k1::types::PubKeyUncompressedNoHeader as EcdsaSecp256k1PublicKey;
pub use ecdsa_secp256k1::types::SignRecoverable as EcdsaSecp256k1Signature;

pub use eddsa_ed25519::types::PubKey as EddsaEd25519PublicKey;
pub use eddsa_ed25519::types::Sign as EddsaEd25519Signature;

impl Bs58EncodedSignature {
    // TODO: write comments
    //
    // signature verification that is compatible to Near
    pub fn verify_hashed_msg(
        self,
        pubkey: NearEncodedPubkey,
        msg_hash: crate::hash::Sha256,
    ) -> bool {
        let pubkey = pubkey.parse();
        Self::verify_inner(self, pubkey, msg_hash)
    }

    /// Verifies if `pubkey` matches `sign` with the `sha256` hash of
    /// the `msg`.
    ///
    /// Note: Internally the hashed msg is hashed again by the
    /// signature verification algorithm. This is compatible with
    /// Near's behavior.
    pub fn verify_msg(self, pubkey: NearEncodedPubkey, msg: String) -> bool {
        let msg_hash = {
            use digest::Digest;
            let mut sha2_hash = sha2::Sha256::new();
            sha2_hash.update(msg.as_bytes());
            let sha2_hash = sha2_hash.finalize();

            let mut arr = [0; 32];
            arr.copy_from_slice(sha2_hash.as_slice());
            crate::hash::Sha256(arr)
        };
        let pubkey = pubkey.parse();
        Self::verify_inner(self, pubkey, msg_hash)
    }

    /// Note: Internally the hashed msg is hashed again by the
    /// signature verification algorithm. This is compatible with
    /// Near's behavior.
    pub fn verify_inner(self, pubkey: near_sdk::PublicKey, msg_hash: crate::hash::Sha256) -> bool {
        use near_sdk::CurveType;
        let sign = self.decode();
        match (pubkey.curve_type(), sign.len()) {
            (CurveType::ED25519, 64) => {
                let mut sign_raw = [0; 64];
                sign_raw.copy_from_slice(&sign[0..64]);
                let sign = EddsaEd25519Signature(sign_raw);

                let pubkey: EddsaEd25519PublicKey = pubkey.into();

                // note: msg_hash will be hashed again internally, this is
                // compatible with Near's behavior.
                sign.verify(pubkey, &msg_hash.0)
            }
            (CurveType::SECP256K1, 65) => {
                let mut sign_raw = [0; 65];
                sign_raw.copy_from_slice(&sign[0..65]);
                let sign = EcdsaSecp256k1Signature(sign_raw);
                let sign: ecdsa_secp256k1::types::SignCompact = sign.into();

                let pubkey: EcdsaSecp256k1PublicKey = pubkey.into();
                let pubkey: ecdsa_secp256k1::types::PubKeyUncompressed = pubkey.into();

                // note: msg_hash will be hashed again internally, this is
                // compatible with Near's behavior.
                sign.verify_uncompressed_msg_bytes(pubkey, &msg_hash.0)
            }
            (curve_type, sign_len) => panic!(
                "Wrong sign length of {} for the curve type {:?}",
                sign_len, curve_type
            ),
        }
    }
}

#[derive(
    near_sdk::serde::Serialize, near_sdk::serde::Deserialize, Clone, PartialEq, Debug, Default,
)]
#[serde(crate = "near_sdk::serde")]
#[serde(transparent)]
pub struct NearEncodedPubkey(pub String);

impl NearEncodedPubkey {
    pub fn parse(&self) -> near_sdk::PublicKey {
        self.0.parse().unwrap()
    }
}

impl From<String> for NearEncodedPubkey {
    fn from(s: String) -> Self {
        Self(s)
    }
}

#[derive(
    near_sdk::serde::Serialize, near_sdk::serde::Deserialize, Clone, PartialEq, Debug, Default,
)]
#[serde(crate = "near_sdk::serde")]
#[serde(transparent)]
pub struct Bs58EncodedSignature(pub String);

impl Bs58EncodedSignature {
    pub fn decode(&self) -> Vec<u8> {
        near_sdk::bs58::decode(&self.0).into_vec().unwrap()
    }
    pub fn encode(bytes: &[u8]) -> Self {
        Self(near_sdk::bs58::encode(bytes).into_string())
    }
}

impl From<String> for Bs58EncodedSignature {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<EddsaEd25519Signature> for Bs58EncodedSignature {
    fn from(sign: EddsaEd25519Signature) -> Self {
        Self::encode(&sign.0)
    }
}
