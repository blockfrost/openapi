use crate::models;
use serde::{Deserialize, Serialize};

/// ProtocolParameters : Protocol cryptographic parameters
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProtocolParameters {
    /// Quorum parameter
    #[serde(rename = "k")]
    pub k: i64,
    /// Security parameter (number of lotteries)
    #[serde(rename = "m")]
    pub m: i64,
    /// f in phi(w) = 1 - (1 - f)^w, where w is the stake of a participant
    #[serde(rename = "phi_f")]
    pub phi_f: f64,
}

impl ProtocolParameters {
    /// Protocol cryptographic parameters
    pub fn new(k: i64, m: i64, phi_f: f64) -> ProtocolParameters {
        ProtocolParameters {
            k,
            m,
            phi_f,
        }
    }
}

