use crate::models;
use serde::{Deserialize, Serialize};

/// CertificateMetadata : CertificateMetadata represents the metadata associated to a Certificate
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateMetadata {
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
    /// The list of the signers identifiers with their stakes and verification keys
    #[serde(rename = "signers")]
    pub signers: Vec<models::StakeDistributionParty>,
}

impl CertificateMetadata {
    /// CertificateMetadata represents the metadata associated to a Certificate
    pub fn new(network: String, version: String, parameters: models::ProtocolParameters, initiated_at: String, sealed_at: String, signers: Vec<models::StakeDistributionParty>) -> CertificateMetadata {
        CertificateMetadata {
            network,
            version,
            parameters,
            initiated_at,
            sealed_at,
            signers,
        }
    }
}

