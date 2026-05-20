use crate::models;
use serde::{Deserialize, Serialize};

/// CertificateListItemMessage : CertificateListItemMessage represents an item of a list of Mithril certificates
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateListItemMessage {
    /// Hash of the current certificate
    #[serde(rename = "hash")]
    pub hash: String,
    /// Hash of the previous certificate
    #[serde(rename = "previous_hash")]
    pub previous_hash: String,
    /// Cardano chain epoch number
    #[serde(rename = "epoch")]
    pub epoch: i64,
    #[serde(rename = "beacon", skip_serializing_if = "Option::is_none")]
    pub beacon: Option<models::CardanoDbBeacon>,
    /// Entity type of the message that is signed
    #[serde(rename = "signed_entity_type")]
    pub signed_entity_type: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "metadata")]
    pub metadata: Box<models::CertificateListItemMessageMetadata>,
    #[serde(rename = "protocol_message")]
    pub protocol_message: Box<models::ProtocolMessage>,
    /// Hash of the protocol message that is signed by the signer participants
    #[serde(rename = "signed_message")]
    pub signed_message: String,
    /// Aggregate verification key used to verify the multi signature
    #[serde(rename = "aggregate_verification_key")]
    pub aggregate_verification_key: String,
}

impl CertificateListItemMessage {
    /// CertificateListItemMessage represents an item of a list of Mithril certificates
    pub fn new(hash: String, previous_hash: String, epoch: i64, signed_entity_type: std::collections::HashMap<String, serde_json::Value>, metadata: models::CertificateListItemMessageMetadata, protocol_message: models::ProtocolMessage, signed_message: String, aggregate_verification_key: String) -> CertificateListItemMessage {
        CertificateListItemMessage {
            hash,
            previous_hash,
            epoch,
            beacon: None,
            signed_entity_type,
            metadata: Box::new(metadata),
            protocol_message: Box::new(protocol_message),
            signed_message,
            aggregate_verification_key,
        }
    }
}

