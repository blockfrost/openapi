use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TxMetadataLabelJsonInner {
    /// Transaction hash that contains the specific metadata
    #[serde(rename = "tx_hash")]
    pub tx_hash: String,
    /// Content of the JSON metadata
    #[serde(rename = "json_metadata", deserialize_with = "Option::deserialize")]
    pub json_metadata: Option<serde_json::Value>,
}

impl TxMetadataLabelJsonInner {
    pub fn new(tx_hash: String, json_metadata: Option<serde_json::Value>) -> TxMetadataLabelJsonInner {
        TxMetadataLabelJsonInner {
            tx_hash,
            json_metadata,
        }
    }
}

