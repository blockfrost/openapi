use crate::models;
use serde::{Deserialize, Serialize};

/// MithrilStakeDistributionMessage : This message represents a Mithril stake distribution.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MithrilStakeDistributionMessage {
    /// Cardano chain epoch number
    #[serde(rename = "epoch")]
    pub epoch: i64,
    /// Hash of the Mithril stake distribution
    #[serde(rename = "hash")]
    pub hash: String,
    /// Hash of the associated certificate
    #[serde(rename = "certificate_hash", skip_serializing_if = "Option::is_none")]
    pub certificate_hash: Option<String>,
    /// The list of the signers with their stakes and verification keys
    #[serde(rename = "signers")]
    pub signers: Vec<models::SignerWithStake>,
    /// Date and time of the entity creation
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "protocol_parameters")]
    pub protocol_parameters: models::ProtocolParameters,
}

impl MithrilStakeDistributionMessage {
    /// This message represents a Mithril stake distribution.
    pub fn new(epoch: i64, hash: String, signers: Vec<models::SignerWithStake>, created_at: String, protocol_parameters: models::ProtocolParameters) -> MithrilStakeDistributionMessage {
        MithrilStakeDistributionMessage {
            epoch,
            hash,
            certificate_hash: None,
            signers,
            created_at,
            protocol_parameters,
        }
    }
}

