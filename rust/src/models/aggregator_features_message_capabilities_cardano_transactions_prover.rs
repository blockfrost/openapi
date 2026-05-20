use crate::models;
use serde::{Deserialize, Serialize};

/// AggregatorFeaturesMessageCapabilitiesCardanoTransactionsProver : Cardano transactions prover capabilities
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AggregatorFeaturesMessageCapabilitiesCardanoTransactionsProver {
    /// Maximum number of hashes allowed for a single request
    #[serde(rename = "max_hashes_allowed_by_request")]
    pub max_hashes_allowed_by_request: i64,
}

impl AggregatorFeaturesMessageCapabilitiesCardanoTransactionsProver {
    /// Cardano transactions prover capabilities
    pub fn new(max_hashes_allowed_by_request: i64) -> AggregatorFeaturesMessageCapabilitiesCardanoTransactionsProver {
        AggregatorFeaturesMessageCapabilitiesCardanoTransactionsProver {
            max_hashes_allowed_by_request,
        }
    }
}

