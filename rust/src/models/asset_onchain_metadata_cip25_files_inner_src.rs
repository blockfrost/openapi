use crate::models;
use serde::{Deserialize, Serialize};

/// AssetOnchainMetadataCip25FilesInnerSrc : URI pointing to a resource of this mime type
/// URI pointing to a resource of this mime type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AssetOnchainMetadataCip25FilesInnerSrc {
    String(String),
    ArrayVecString(Vec<String>),
}

impl Default for AssetOnchainMetadataCip25FilesInnerSrc {
    fn default() -> Self {
        Self::String(Default::default())
    }
}

