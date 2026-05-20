use crate::models;
use serde::{Deserialize, Serialize};

/// SnapshotMessage : This message represents a snapshot file and its metadata.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SnapshotMessage {
    /// Digest that is signed by the signer participants
    #[serde(rename = "digest")]
    pub digest: String,
    #[serde(rename = "beacon")]
    pub beacon: models::CardanoDbBeacon,
    /// Hash of the associated certificate
    #[serde(rename = "certificate_hash")]
    pub certificate_hash: String,
    /// Size of the snapshot file in Bytes
    #[serde(rename = "size")]
    pub size: i64,
    /// Date and time at which the snapshot was created
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// Locations where the binary content of the snapshot can be retrieved
    #[serde(rename = "locations")]
    pub locations: Vec<String>,
    /// Compression algorithm for the snapshot archive
    #[serde(rename = "compression_algorithm", skip_serializing_if = "Option::is_none")]
    pub compression_algorithm: Option<String>,
    /// Version of the Cardano node which is used to create snapshot archives.
    #[serde(rename = "cardano_node_version", skip_serializing_if = "Option::is_none")]
    pub cardano_node_version: Option<String>,
}

impl SnapshotMessage {
    /// This message represents a snapshot file and its metadata.
    pub fn new(digest: String, beacon: models::CardanoDbBeacon, certificate_hash: String, size: i64, created_at: String, locations: Vec<String>) -> SnapshotMessage {
        SnapshotMessage {
            digest,
            beacon,
            certificate_hash,
            size,
            created_at,
            locations,
            compression_algorithm: None,
            cardano_node_version: None,
        }
    }
}

