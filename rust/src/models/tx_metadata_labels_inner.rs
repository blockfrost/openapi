use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TxMetadataLabelsInner {
    /// Metadata label
    #[serde(rename = "label")]
    pub label: String,
    /// CIP10 defined description
    #[serde(rename = "cip10", deserialize_with = "Option::deserialize")]
    pub cip10: Option<String>,
    /// The count of metadata entries with a specific label
    #[serde(rename = "count")]
    pub count: String,
}

impl TxMetadataLabelsInner {
    pub fn new(label: String, cip10: Option<String>, count: String) -> TxMetadataLabelsInner {
        TxMetadataLabelsInner {
            label,
            cip10,
            count,
        }
    }
}

