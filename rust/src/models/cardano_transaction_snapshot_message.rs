use crate::models;
use serde::{Deserialize, Serialize};

/// CardanoTransactionSnapshotMessage : This message represents a Cardano transactions set snapshot.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CardanoTransactionSnapshotMessage {
    /// Hash of the Cardano transactions set
    #[serde(rename = "hash")]
    pub hash: String,
    /// Hash of the associated certificate
    #[serde(rename = "certificate_hash")]
    pub certificate_hash: String,
    /// Merkle root of the Cardano transactions set
    #[serde(rename = "merkle_root")]
    pub merkle_root: String,
    /// Cardano chain epoch number
    #[serde(rename = "epoch")]
    pub epoch: i64,
    /// Cardano block number
    #[serde(rename = "block_number")]
    pub block_number: i64,
    /// Date and time at which the Cardano transactions set was created
    #[serde(rename = "created_at")]
    pub created_at: String,
}

impl CardanoTransactionSnapshotMessage {
    /// This message represents a Cardano transactions set snapshot.
    pub fn new(hash: String, certificate_hash: String, merkle_root: String, epoch: i64, block_number: i64, created_at: String) -> CardanoTransactionSnapshotMessage {
        CardanoTransactionSnapshotMessage {
            hash,
            certificate_hash,
            merkle_root,
            epoch,
            block_number,
            created_at,
        }
    }
}

