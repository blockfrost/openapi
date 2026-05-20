use crate::models;
use serde::{Deserialize, Serialize};

/// AggregatorFeaturesMessageCapabilities : Capabilities of the aggregator
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AggregatorFeaturesMessageCapabilities {
    /// Signed entity types that are signed by the aggregator
    #[serde(rename = "signed_entity_types")]
    pub signed_entity_types: Vec<SignedEntityTypes>,
    #[serde(rename = "cardano_transactions_prover", skip_serializing_if = "Option::is_none")]
    pub cardano_transactions_prover: Option<Box<models::AggregatorFeaturesMessageCapabilitiesCardanoTransactionsProver>>,
}

impl AggregatorFeaturesMessageCapabilities {
    /// Capabilities of the aggregator
    pub fn new(signed_entity_types: Vec<SignedEntityTypes>) -> AggregatorFeaturesMessageCapabilities {
        AggregatorFeaturesMessageCapabilities {
            signed_entity_types,
            cardano_transactions_prover: None,
        }
    }
}
/// Signed entity types that are signed by the aggregator
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SignedEntityTypes {
    #[serde(rename = "MithrilStakeDistribution")]
    MithrilStakeDistribution,
    #[serde(rename = "CardanoStakeDistribution")]
    CardanoStakeDistribution,
    #[serde(rename = "CardanoImmutableFilesFull")]
    CardanoImmutableFilesFull,
    #[serde(rename = "CardanoTransactions")]
    CardanoTransactions,
}

impl Default for SignedEntityTypes {
    fn default() -> SignedEntityTypes {
        Self::MithrilStakeDistribution
    }
}

