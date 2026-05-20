use crate::models;
use serde::{Deserialize, Serialize};

/// AssetOnchainMetadataCip68Ft333 : On-chain metadata stored in the datum of the reference NFT output which adheres to 333 FT Standard https://cips.cardano.org/cips/cip68/ 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetOnchainMetadataCip68Ft333 {
    /// Name of the asset
    #[serde(rename = "name")]
    pub name: String,
    /// Additional description
    #[serde(rename = "description")]
    pub description: String,
    /// URI(s) of the associated asset
    #[serde(rename = "logo", skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    /// Ticker
    #[serde(rename = "ticker", skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    /// Number of decimals
    #[serde(rename = "decimals", skip_serializing_if = "Option::is_none")]
    pub decimals: Option<f64>,
}

impl AssetOnchainMetadataCip68Ft333 {
    /// On-chain metadata stored in the datum of the reference NFT output which adheres to 333 FT Standard https://cips.cardano.org/cips/cip68/ 
    pub fn new(name: String, description: String) -> AssetOnchainMetadataCip68Ft333 {
        AssetOnchainMetadataCip68Ft333 {
            name,
            description,
            logo: None,
            ticker: None,
            decimals: None,
        }
    }
}

