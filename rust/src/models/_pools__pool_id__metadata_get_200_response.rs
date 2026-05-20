use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PoolsPoolIdMetadataGet200Response {
    /// Bech32 pool ID
    #[serde(rename = "pool_id")]
    pub pool_id: String,
    /// Hexadecimal pool ID
    #[serde(rename = "hex")]
    pub hex: String,
    /// URL to the stake pool metadata
    #[serde(rename = "url", deserialize_with = "Option::deserialize")]
    pub url: Option<String>,
    /// Hash of the metadata file
    #[serde(rename = "hash", deserialize_with = "Option::deserialize")]
    pub hash: Option<String>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<Box<models::DrepMetadataError>>,
    /// Ticker of the stake pool
    #[serde(rename = "ticker", deserialize_with = "Option::deserialize")]
    pub ticker: Option<String>,
    /// Name of the stake pool
    #[serde(rename = "name", deserialize_with = "Option::deserialize")]
    pub name: Option<String>,
    /// Description of the stake pool
    #[serde(rename = "description", deserialize_with = "Option::deserialize")]
    pub description: Option<String>,
    /// Home page of the stake pool
    #[serde(rename = "homepage", deserialize_with = "Option::deserialize")]
    pub homepage: Option<String>,
}

impl PoolsPoolIdMetadataGet200Response {
    pub fn new(pool_id: String, hex: String, url: Option<String>, hash: Option<String>, ticker: Option<String>, name: Option<String>, description: Option<String>, homepage: Option<String>) -> PoolsPoolIdMetadataGet200Response {
        PoolsPoolIdMetadataGet200Response {
            pool_id,
            hex,
            url,
            hash,
            error: None,
            ticker,
            name,
            description,
            homepage,
        }
    }
}

