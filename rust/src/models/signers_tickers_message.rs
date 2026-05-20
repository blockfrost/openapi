use crate::models;
use serde::{Deserialize, Serialize};

/// SignersTickersMessage : represents the list of signers known by the aggregator
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SignersTickersMessage {
    /// Cardano network of the aggregator
    #[serde(rename = "network")]
    pub network: String,
    /// Known signers
    #[serde(rename = "signers")]
    pub signers: Vec<models::SignerTickerListItemMessage>,
}

impl SignersTickersMessage {
    /// represents the list of signers known by the aggregator
    pub fn new(network: String, signers: Vec<models::SignerTickerListItemMessage>) -> SignersTickersMessage {
        SignersTickersMessage {
            network,
            signers,
        }
    }
}

