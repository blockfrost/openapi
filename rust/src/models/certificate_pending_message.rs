use crate::models;
use serde::{Deserialize, Serialize};

/// CertificatePendingMessage : CertificatePendingMessage represents all the information related to the certificate currently expecting to receive quorum of single signatures
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificatePendingMessage {
    /// Cardano chain epoch number
    #[serde(rename = "epoch")]
    pub epoch: i64,
    #[serde(rename = "beacon", skip_serializing_if = "Option::is_none")]
    pub beacon: Option<models::CardanoDbBeacon>,
    /// Entity type of the message that is signed
    #[serde(rename = "entity_type")]
    pub entity_type: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "protocol")]
    pub protocol: models::ProtocolParameters,
    #[serde(rename = "next_protocol")]
    pub next_protocol: models::ProtocolParameters,
    #[serde(rename = "signers")]
    pub signers: Vec<models::Signer>,
    #[serde(rename = "next_signers")]
    pub next_signers: Vec<models::Signer>,
}

impl CertificatePendingMessage {
    /// CertificatePendingMessage represents all the information related to the certificate currently expecting to receive quorum of single signatures
    pub fn new(epoch: i64, entity_type: std::collections::HashMap<String, serde_json::Value>, protocol: models::ProtocolParameters, next_protocol: models::ProtocolParameters, signers: Vec<models::Signer>, next_signers: Vec<models::Signer>) -> CertificatePendingMessage {
        CertificatePendingMessage {
            epoch,
            beacon: None,
            entity_type,
            protocol,
            next_protocol,
            signers,
            next_signers,
        }
    }
}

