use crate::models;
use serde::{Deserialize, Serialize};

/// CertificateMessage : Certificate represents a Mithril certificate embedding a Mithril STM multi signature
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateMessage {
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
    pub metadata: Box<models::CertificateMetadata>,
    #[serde(rename = "protocol_message")]
    pub protocol_message: Box<models::ProtocolMessage>,
    /// Hash of the protocol message that is signed by the signer participants
    #[serde(rename = "signed_message")]
    pub signed_message: String,
    /// Aggregate verification key used to verify the multi signature
    #[serde(rename = "aggregate_verification_key")]
    pub aggregate_verification_key: String,
    /// STM multi signature created from a quorum of single signatures from the signers
    #[serde(rename = "multi_signature")]
    pub multi_signature: String,
    /// Genesis signature created to bootstrap the certificate chain with the Cardano Genesis Keys
    #[serde(rename = "genesis_signature")]
    pub genesis_signature: String,
}

impl CertificateMessage {
    /// Certificate represents a Mithril certificate embedding a Mithril STM multi signature
    pub fn new(hash: String, previous_hash: String, epoch: i64, signed_entity_type: std::collections::HashMap<String, serde_json::Value>, metadata: models::CertificateMetadata, protocol_message: models::ProtocolMessage, signed_message: String, aggregate_verification_key: String, multi_signature: String, genesis_signature: String) -> CertificateMessage {
        CertificateMessage {
            hash,
            previous_hash,
            epoch,
            beacon: None,
            signed_entity_type,
            metadata: Box::new(metadata),
            protocol_message: Box::new(protocol_message),
            signed_message,
            aggregate_verification_key,
            multi_signature,
            genesis_signature,
        }
    }
}

