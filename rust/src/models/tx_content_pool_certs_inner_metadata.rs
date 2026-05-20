use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TxContentPoolCertsInnerMetadata {
    /// URL to the stake pool metadata
    #[serde(rename = "url", deserialize_with = "Option::deserialize")]
    pub url: Option<String>,
    /// Hash of the metadata file
    #[serde(rename = "hash", deserialize_with = "Option::deserialize")]
    pub hash: Option<String>,
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

impl TxContentPoolCertsInnerMetadata {
    pub fn new(url: Option<String>, hash: Option<String>, ticker: Option<String>, name: Option<String>, description: Option<String>, homepage: Option<String>) -> TxContentPoolCertsInnerMetadata {
        TxContentPoolCertsInnerMetadata {
            url,
            hash,
            ticker,
            name,
            description,
            homepage,
        }
    }
}

