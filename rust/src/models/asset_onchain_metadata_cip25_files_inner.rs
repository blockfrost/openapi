use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetOnchainMetadataCip25FilesInner {
    /// Name of the file
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Mime sub-type of image
    #[serde(rename = "mediaType")]
    pub media_type: String,
    #[serde(rename = "src")]
    pub src: Box<models::AssetOnchainMetadataCip25FilesInnerSrc>,
}

impl AssetOnchainMetadataCip25FilesInner {
    pub fn new(media_type: String, src: models::AssetOnchainMetadataCip25FilesInnerSrc) -> AssetOnchainMetadataCip25FilesInner {
        AssetOnchainMetadataCip25FilesInner {
            name: None,
            media_type,
            src: Box::new(src),
        }
    }
}

