use crate::models;
use serde::{Deserialize, Serialize};

/// SnapshotDownloadMessage : SnapshotDownloadMessage represents a downloaded snapshot event
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SnapshotDownloadMessage {
    /// Digest that is signed by the signer participants
    #[serde(rename = "digest")]
    pub digest: String,
    #[serde(rename = "beacon")]
    pub beacon: models::CardanoDbBeacon,
    /// Size of the snapshot file in Bytes
    #[serde(rename = "size")]
    pub size: i64,
    /// Locations where the binary content of the snapshot can be retrieved
    #[serde(rename = "locations")]
    pub locations: Vec<String>,
    /// Compression algorithm for the snapshot archive
    #[serde(rename = "compression_algorithm")]
    pub compression_algorithm: String,
    /// Version of the Cardano node which is used to create snapshot archives.
    #[serde(rename = "cardano_node_version")]
    pub cardano_node_version: String,
}

impl SnapshotDownloadMessage {
    /// SnapshotDownloadMessage represents a downloaded snapshot event
    pub fn new(digest: String, beacon: models::CardanoDbBeacon, size: i64, locations: Vec<String>, compression_algorithm: String, cardano_node_version: String) -> SnapshotDownloadMessage {
        SnapshotDownloadMessage {
            digest,
            beacon,
            size,
            locations,
            compression_algorithm,
            cardano_node_version,
        }
    }
}

