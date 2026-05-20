use crate::models;
use serde::{Deserialize, Serialize};

/// SignerTickerListItemMessage : represents a known signer with its pool ticker
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SignerTickerListItemMessage {
    /// The unique identifier of the signer
    #[serde(rename = "party_id")]
    pub party_id: String,
    /// The signer pool ticker
    #[serde(rename = "pool_ticker", skip_serializing_if = "Option::is_none")]
    pub pool_ticker: Option<String>,
    /// The signer has registered at least once
    #[serde(rename = "has_registered")]
    pub has_registered: bool,
}

impl SignerTickerListItemMessage {
    /// represents a known signer with its pool ticker
    pub fn new(party_id: String, has_registered: bool) -> SignerTickerListItemMessage {
        SignerTickerListItemMessage {
            party_id,
            pool_ticker: None,
            has_registered,
        }
    }
}

