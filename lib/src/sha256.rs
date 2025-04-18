use crate::U256;
use serde::{Deserialize, Serialize};
use sha256::digest;
use std::fmt;
#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Hash(U256);

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Hash {
    // hash anything that can be serde Serialized via ciborium
    #[allow(clippy::self_named_constructors)]
    pub fn hash<T: serde::Serialize>(data: &T) -> Self {
        let mut serialized: Vec<u8> = vec![];
        if let Err(e) = ciborium::into_writer(data, &mut serialized) {
            panic!(
                "Falied to serialize data: {:?}. \
            This should not happen",
                e
            );
        }
        let hash = digest(&serialized);
        let hash_bytes = hex::decode(hash).unwrap();
        let hash_array: [u8; 32] = hash_bytes.as_slice().try_into().unwrap();
        Hash(U256::from(hash_array))
    }

    // check if a hash matches a target
    pub fn matches_target(&self, target: U256) -> bool {
        self.0 <= target
    }

    // zero hash
    pub fn zero() -> Self {
        Hash(U256::zero())
    }
}
