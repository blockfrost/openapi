use crate::models;
use serde::{Deserialize, Serialize};

/// AssetMetadata : Off-chain metadata fetched from GitHub based on network. Mainnet: https://github.com/cardano-foundation/cardano-token-registry/ Testnet: https://github.com/input-output-hk/metadata-registry-testnet/ 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetMetadata {
    /// Asset name
    #[serde(rename = "name")]
    pub name: String,
    /// Asset description
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "ticker", deserialize_with = "Option::deserialize")]
    pub ticker: Option<String>,
    /// Asset website
    #[serde(rename = "url", deserialize_with = "Option::deserialize")]
    pub url: Option<String>,
    /// Base64 encoded logo of the asset
    #[serde(rename = "logo", deserialize_with = "Option::deserialize")]
    pub logo: Option<String>,
    /// Number of decimal places of the asset unit
    #[serde(rename = "decimals", deserialize_with = "Option::deserialize")]
    pub decimals: Option<i32>,
}

impl AssetMetadata {
    /// Off-chain metadata fetched from GitHub based on network. Mainnet: https://github.com/cardano-foundation/cardano-token-registry/ Testnet: https://github.com/input-output-hk/metadata-registry-testnet/ 
    pub fn new(name: String, description: String, ticker: Option<String>, url: Option<String>, logo: Option<String>, decimals: Option<i32>) -> AssetMetadata {
        AssetMetadata {
            name,
            description,
            ticker,
            url,
            logo,
            decimals,
        }
    }
}

