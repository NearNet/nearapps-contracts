use std::marker::PhantomData;

use near_contract_standards::non_fungible_token as nft;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{TreeMap, UnorderedMap, UnorderedSet};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, AccountId};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum NftProtocol {
    Unknown,
    Standard,
    NearApps,
}

/// The AccountId of a Nft contract.
#[derive(Clone, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[serde(transparent)]
pub struct NftContractId(pub AccountId);

impl From<AccountId> for NftContractId {
    fn from(acc: AccountId) -> Self {
        Self(acc)
    }
}

impl From<NftContractId> for AccountId {
    fn from(acc: NftContractId) -> Self {
        acc.0
    }
}

impl AsRef<[u8]> for NftContractId {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref().as_bytes()
    }
}

struct TokenIdOwner {}

/// The accountId of a Nft token owner.
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, PartialEq, Clone)]
#[serde(crate = "near_sdk::serde")]
#[serde(transparent)]
pub struct NftUserAccountId(pub AccountId);

impl From<AccountId> for NftUserAccountId {
    fn from(acc: AccountId) -> Self {
        Self(acc)
    }
}

impl From<NftUserAccountId> for AccountId {
    fn from(acc: NftUserAccountId) -> Self {
        acc.0
    }
}

impl AsRef<[u8]> for NftUserAccountId {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref().as_bytes()
    }
}

/// Maps a Nft token owner per [`nft::TokenId`] of a certain Nft contract.
pub type UserByTokenId = TreeMap<nft::TokenId, NftUserAccountId>;

/// Maps a set of [`nft::TokenId`] per [`NftContractId`].
pub type TokenSetForNftContract = UnorderedMap<NftContractId, UnorderedSet<nft::TokenId>>;

/// Sha256 result from the byte representation of some value.
pub struct Sha256From<T> {
    pub value: [u8; 32],
    _phantom: PhantomData<T>,
}

impl<T> borsh::ser::BorshSerialize for Sha256From<T>
where
    [u8; 32]: borsh::ser::BorshSerialize,
{
    fn serialize<W: borsh::maybestd::io::Write>(
        &self,
        writer: &mut W,
    ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
        borsh::BorshSerialize::serialize(&self.value, writer)?;
        Ok(())
    }
}

impl<T> borsh::de::BorshDeserialize for Sha256From<T>
where
    [u8; 32]: borsh::BorshDeserialize,
{
    fn deserialize(buf: &mut &[u8]) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
            value: borsh::BorshDeserialize::deserialize(buf)?,
            _phantom: PhantomData,
        })
    }
}

impl<T> Sha256From<T> {
    pub fn new(val: &T) -> Self
    where
        T: AsRef<[u8]>,
    {
        let mut hash_fixed = [0u8; 32];
        let hash = env::sha256(val.as_ref());
        hash_fixed.copy_from_slice(hash.as_slice());
        Self {
            value: hash_fixed,
            _phantom: PhantomData,
        }
    }
}

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

impl<T> From<T> for Sha256From<T>
where
    T: ToBytes,
{
    fn from(t: T) -> Self {
        let bytes = t.to_bytes();
        let mut hash_fixed = [0u8; 32];
        let hash = env::sha256(&bytes);
        hash_fixed.copy_from_slice(hash.as_slice());
        Self {
            value: hash_fixed,
            _phantom: PhantomData,
        }
    }
}
