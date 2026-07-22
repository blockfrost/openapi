use crate::models;
use serde::{Deserialize, Serialize};

/// DrepsInnerMetadata : Off-chain metadata associated with the DRep's latest registration. `null` when the DRep has no registration anchor (e.g. special DReps such as `drep_always_abstain` / `drep_always_no_confidence`). When an anchor exists but the off-chain content could not be fetched or validated, `error` is populated and `json_metadata` / `bytes` are `null`. 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DrepsInnerMetadata {
    /// URL to the drep metadata
    #[serde(rename = "url")]
    pub url: String,
    /// Hash of the metadata file
    #[serde(rename = "hash")]
    pub hash: String,
    /// Content of the JSON metadata (validated CIP-119)
    #[serde(rename = "json_metadata", deserialize_with = "Option::deserialize")]
    pub json_metadata: Option<serde_json::Value>,
    /// Content of the metadata (raw)
    #[serde(rename = "bytes", deserialize_with = "Option::deserialize")]
    pub bytes: Option<String>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<Box<models::DrepsInnerMetadataError>>,
}

impl DrepsInnerMetadata {
    /// Off-chain metadata associated with the DRep's latest registration. `null` when the DRep has no registration anchor (e.g. special DReps such as `drep_always_abstain` / `drep_always_no_confidence`). When an anchor exists but the off-chain content could not be fetched or validated, `error` is populated and `json_metadata` / `bytes` are `null`. 
    pub fn new(url: String, hash: String, json_metadata: Option<serde_json::Value>, bytes: Option<String>) -> DrepsInnerMetadata {
        DrepsInnerMetadata {
            url,
            hash,
            json_metadata,
            bytes,
            error: None,
        }
    }
}

