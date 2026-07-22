use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DrepMetadata {
    /// Bech32 encoded addresses
    #[serde(rename = "drep_id")]
    pub drep_id: String,
    /// The raw bytes of the DRep
    #[serde(rename = "hex")]
    pub hex: String,
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

impl DrepMetadata {
    pub fn new(drep_id: String, hex: String, url: String, hash: String, json_metadata: Option<serde_json::Value>, bytes: Option<String>) -> DrepMetadata {
        DrepMetadata {
            drep_id,
            hex,
            url,
            hash,
            json_metadata,
            bytes,
            error: None,
        }
    }
}

