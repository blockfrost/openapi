use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProposalParameters {
    /// Governance Action Identifier (CIP-0129)
    #[serde(rename = "id")]
    pub id: String,
    /// Off-chain metadata of a proposal with a specific transaction hash
    #[serde(rename = "tx_hash")]
    pub tx_hash: String,
    /// Off-chain metadata of a proposal with a specific transaction cert_index
    #[serde(rename = "cert_index")]
    pub cert_index: i32,
    #[serde(rename = "parameters")]
    pub parameters: Box<models::ProposalParametersParameters>,
}

impl ProposalParameters {
    pub fn new(id: String, tx_hash: String, cert_index: i32, parameters: models::ProposalParametersParameters) -> ProposalParameters {
        ProposalParameters {
            id,
            tx_hash,
            cert_index,
            parameters: Box::new(parameters),
        }
    }
}

