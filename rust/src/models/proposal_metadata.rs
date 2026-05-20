use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProposalMetadata {
    /// Governance Action Identifier (CIP-0129)
    #[serde(rename = "id")]
    pub id: String,
    /// Off-chain metadata of a proposal with a specific transaction hash
    #[serde(rename = "tx_hash")]
    pub tx_hash: String,
    /// Off-chain metadata of a proposal with a specific transaction cert_index
    #[serde(rename = "cert_index")]
    pub cert_index: i32,
    /// URL to the proposal metadata
    #[serde(rename = "url")]
    pub url: String,
    /// Hash of the metadata file
    #[serde(rename = "hash")]
    pub hash: String,
    /// Content of the JSON metadata (validated CIP-108)
    #[serde(rename = "json_metadata", deserialize_with = "Option::deserialize")]
    pub json_metadata: Option<serde_json::Value>,
    /// Content of the metadata (raw)
    #[serde(rename = "bytes")]
    pub bytes: String,
}

impl ProposalMetadata {
    pub fn new(id: String, tx_hash: String, cert_index: i32, url: String, hash: String, json_metadata: Option<serde_json::Value>, bytes: String) -> ProposalMetadata {
        ProposalMetadata {
            id,
            tx_hash,
            cert_index,
            url,
            hash,
            json_metadata,
            bytes,
        }
    }
}

