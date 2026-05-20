use crate::models;
use serde::{Deserialize, Serialize};

/// CertificateListItemMessageMetadata : CertificateListItemMessageMetadata represents the metadata associated to a CertificateListItemMessage
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateListItemMessageMetadata {
    /// Cardano network
    #[serde(rename = "network")]
    pub network: String,
    /// Version of the protocol
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "parameters")]
    pub parameters: models::ProtocolParameters,
    /// Date and time at which the certificate was initialized and ready to accept single signatures from signers
    #[serde(rename = "initiated_at")]
    pub initiated_at: String,
    /// Date and time at which the certificate was sealed (when the quorum of single signatures was reached so that a multi signature could be aggregated from them)
    #[serde(rename = "sealed_at")]
    pub sealed_at: String,
    /// The number of the signers with their stakes and verification keys
    #[serde(rename = "total_signers")]
    pub total_signers: i64,
}

impl CertificateListItemMessageMetadata {
    /// CertificateListItemMessageMetadata represents the metadata associated to a CertificateListItemMessage
    pub fn new(network: String, version: String, parameters: models::ProtocolParameters, initiated_at: String, sealed_at: String, total_signers: i64) -> CertificateListItemMessageMetadata {
        CertificateListItemMessageMetadata {
            network,
            version,
            parameters,
            initiated_at,
            sealed_at,
            total_signers,
        }
    }
}

