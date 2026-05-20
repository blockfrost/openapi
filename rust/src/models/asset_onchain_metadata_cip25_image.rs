use crate::models;
use serde::{Deserialize, Serialize};

/// AssetOnchainMetadataCip25Image : URI(s) of the associated asset
/// URI(s) of the associated asset
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AssetOnchainMetadataCip25Image {
    String(String),
    ArrayVecString(Vec<String>),
}

impl Default for AssetOnchainMetadataCip25Image {
    fn default() -> Self {
        Self::String(Default::default())
    }
}

