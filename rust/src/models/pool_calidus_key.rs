use crate::models;
use serde::{Deserialize, Serialize};

/// PoolCalidusKey : Last valid Calidus key for the pool
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PoolCalidusKey {
    /// A Bech32-encoded identifier derived from the calidus public key
    #[serde(rename = "id")]
    pub id: String,
    /// The raw hexadecimal-encoded calidus public key used for verification purposes
    #[serde(rename = "pub_key")]
    pub pub_key: String,
    /// A unique number used once to prevent replay attacks and ensure the uniqueness of the key registration
    #[serde(rename = "nonce")]
    pub nonce: i32,
    /// The transaction hash that submitted the Calidus key registration
    #[serde(rename = "tx_hash")]
    pub tx_hash: String,
    /// The block height at which this key registration was recorded
    #[serde(rename = "block_height")]
    pub block_height: i32,
    /// Block time of the key registration
    #[serde(rename = "block_time")]
    pub block_time: i32,
    /// Epoch number of the key registration
    #[serde(rename = "epoch")]
    pub epoch: i32,
}

impl PoolCalidusKey {
    /// Last valid Calidus key for the pool
    pub fn new(id: String, pub_key: String, nonce: i32, tx_hash: String, block_height: i32, block_time: i32, epoch: i32) -> PoolCalidusKey {
        PoolCalidusKey {
            id,
            pub_key,
            nonce,
            tx_hash,
            block_height,
            block_time,
            epoch,
        }
    }
}

