use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TxContentMetadataCborInner {
    /// Metadata label
    #[serde(rename = "label")]
    pub label: String,
    /// Content of the CBOR metadata
    #[serde(rename = "cbor_metadata", deserialize_with = "Option::deserialize")]
    pub cbor_metadata: Option<String>,
    /// Content of the CBOR metadata in hex
    #[serde(rename = "metadata", deserialize_with = "Option::deserialize")]
    pub metadata: Option<String>,
}

impl TxContentMetadataCborInner {
    pub fn new(label: String, cbor_metadata: Option<String>, metadata: Option<String>) -> TxContentMetadataCborInner {
        TxContentMetadataCborInner {
            label,
            cbor_metadata,
            metadata,
        }
    }
}

