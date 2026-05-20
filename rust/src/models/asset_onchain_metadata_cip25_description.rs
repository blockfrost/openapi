use crate::models;
use serde::{Deserialize, Serialize};

/// AssetOnchainMetadataCip25Description : Additional description
/// Additional description
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AssetOnchainMetadataCip25Description {
    String(String),
    ArrayVecString(Vec<String>),
}

impl Default for AssetOnchainMetadataCip25Description {
    fn default() -> Self {
        Self::String(Default::default())
    }
}

