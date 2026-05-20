use crate::models;
use serde::{Deserialize, Serialize};

use serde_with::serde_as;

/// AggregatorFeaturesMessage : Represents general information about Aggregator public information and signing capabilities
#[serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AggregatorFeaturesMessage {
    /// Open API version
    #[serde_as(as = "serde_with::base64::Base64")]
    #[serde(rename = "open_api_version")]
    pub open_api_version: Vec<u8>,
    /// Mithril documentation
    #[serde_as(as = "serde_with::base64::Base64")]
    #[serde(rename = "documentation_url")]
    pub documentation_url: Vec<u8>,
    #[serde(rename = "capabilities")]
    pub capabilities: Box<models::AggregatorFeaturesMessageCapabilities>,
}

impl AggregatorFeaturesMessage {
    /// Represents general information about Aggregator public information and signing capabilities
    pub fn new(open_api_version: Vec<u8>, documentation_url: Vec<u8>, capabilities: models::AggregatorFeaturesMessageCapabilities) -> AggregatorFeaturesMessage {
        AggregatorFeaturesMessage {
            open_api_version,
            documentation_url,
            capabilities: Box::new(capabilities),
        }
    }
}

