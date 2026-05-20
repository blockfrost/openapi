use crate::models;
use serde::{Deserialize, Serialize};

/// TxContentMetadataInnerJsonMetadata : Content of the metadata
/// Content of the metadata
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TxContentMetadataInnerJsonMetadata {
    String(String),
    Object(std::collections::HashMap<String, serde_json::Value>),
}

impl Default for TxContentMetadataInnerJsonMetadata {
    fn default() -> Self {
        Self::String(Default::default())
    }
}

