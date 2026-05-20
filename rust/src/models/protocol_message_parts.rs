use crate::models;
use serde::{Deserialize, Serialize};

/// ProtocolMessageParts : ProtocolMessage represents a message that is signed (or verified) by the Mithril protocol
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProtocolMessageParts {
    /// Digest of the snapshot archive
    #[serde(rename = "snapshot_digest", skip_serializing_if = "Option::is_none")]
    pub snapshot_digest: Option<String>,
    /// Aggregate verification key (AVK) that will be used to create the next multi signature
    #[serde(rename = "next_aggregate_verification_key")]
    pub next_aggregate_verification_key: String,
    /// The latest signed block number
    #[serde(rename = "latest_block_number", skip_serializing_if = "Option::is_none")]
    pub latest_block_number: Option<String>,
}

impl ProtocolMessageParts {
    /// ProtocolMessage represents a message that is signed (or verified) by the Mithril protocol
    pub fn new(next_aggregate_verification_key: String) -> ProtocolMessageParts {
        ProtocolMessageParts {
            snapshot_digest: None,
            next_aggregate_verification_key,
            latest_block_number: None,
        }
    }
}

