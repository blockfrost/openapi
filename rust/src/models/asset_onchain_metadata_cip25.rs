use crate::models;
use serde::{Deserialize, Serialize};

/// AssetOnchainMetadataCip25 : On-chain metadata stored in the minting transaction under label 721, which adheres to https://cips.cardano.org/cips/cip25/ 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetOnchainMetadataCip25 {
    /// Name of the asset
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "image")]
    pub image: Box<models::AssetOnchainMetadataCip25Image>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<models::AssetOnchainMetadataCip25Description>>,
    /// Mime sub-type of image
    #[serde(rename = "mediaType", skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[serde(rename = "files", skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<models::AssetOnchainMetadataCip25FilesInner>>,
}

impl AssetOnchainMetadataCip25 {
    /// On-chain metadata stored in the minting transaction under label 721, which adheres to https://cips.cardano.org/cips/cip25/ 
    pub fn new(name: String, image: models::AssetOnchainMetadataCip25Image) -> AssetOnchainMetadataCip25 {
        AssetOnchainMetadataCip25 {
            name,
            image: Box::new(image),
            description: None,
            media_type: None,
            files: None,
        }
    }
}

