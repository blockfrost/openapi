use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TxContentMetadataInner {
    /// Metadata label
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "json_metadata")]
    pub json_metadata: Box<models::TxContentMetadataInnerJsonMetadata>,
}

impl TxContentMetadataInner {
    pub fn new(label: String, json_metadata: models::TxContentMetadataInnerJsonMetadata) -> TxContentMetadataInner {
        TxContentMetadataInner {
            label,
            json_metadata: Box::new(json_metadata),
        }
    }
}

