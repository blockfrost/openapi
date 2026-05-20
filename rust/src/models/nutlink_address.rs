use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NutlinkAddress {
    /// Bech32 encoded address
    #[serde(rename = "address")]
    pub address: String,
    /// URL of the specific metadata file
    #[serde(rename = "metadata_url")]
    pub metadata_url: String,
    /// Hash of the metadata file
    #[serde(rename = "metadata_hash")]
    pub metadata_hash: String,
    /// The cached metadata of the `metadata_url` file.
    #[serde(rename = "metadata", deserialize_with = "Option::deserialize")]
    pub metadata: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl NutlinkAddress {
    pub fn new(address: String, metadata_url: String, metadata_hash: String, metadata: Option<std::collections::HashMap<String, serde_json::Value>>) -> NutlinkAddress {
        NutlinkAddress {
            address,
            metadata_url,
            metadata_hash,
            metadata,
        }
    }
}

