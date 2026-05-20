use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TxMetadataLabelCborInner {
    /// Transaction hash that contains the specific metadata
    #[serde(rename = "tx_hash")]
    pub tx_hash: String,
    /// Content of the CBOR metadata
    #[serde(rename = "cbor_metadata", deserialize_with = "Option::deserialize")]
    pub cbor_metadata: Option<String>,
    /// Content of the CBOR metadata in hex
    #[serde(rename = "metadata", deserialize_with = "Option::deserialize")]
    pub metadata: Option<String>,
}

impl TxMetadataLabelCborInner {
    pub fn new(tx_hash: String, cbor_metadata: Option<String>, metadata: Option<String>) -> TxMetadataLabelCborInner {
        TxMetadataLabelCborInner {
            tx_hash,
            cbor_metadata,
            metadata,
        }
    }
}

