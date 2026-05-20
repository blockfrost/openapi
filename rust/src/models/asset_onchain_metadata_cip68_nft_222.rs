use crate::models;
use serde::{Deserialize, Serialize};

/// AssetOnchainMetadataCip68Nft222 : On-chain metadata stored in the datum of the reference NFT output which adheres to 222 NFT Standard https://cips.cardano.org/cips/cip68/ 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetOnchainMetadataCip68Nft222 {
    /// Name of the asset
    #[serde(rename = "name")]
    pub name: String,
    /// URI(s) of the associated asset
    #[serde(rename = "image")]
    pub image: String,
    /// Additional description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Mime sub-type of image
    #[serde(rename = "mediaType", skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[serde(rename = "files", skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<models::AssetOnchainMetadataCip25FilesInner>>,
}

impl AssetOnchainMetadataCip68Nft222 {
    /// On-chain metadata stored in the datum of the reference NFT output which adheres to 222 NFT Standard https://cips.cardano.org/cips/cip68/ 
    pub fn new(name: String, image: String) -> AssetOnchainMetadataCip68Nft222 {
        AssetOnchainMetadataCip68Nft222 {
            name,
            image,
            description: None,
            media_type: None,
            files: None,
        }
    }
}

