use crate::models;
use serde::{Deserialize, Serialize};

/// CommitteeQuorum : Voting threshold of the committee.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitteeQuorum {
    #[serde(rename = "numerator")]
    pub numerator: i32,
    #[serde(rename = "denominator")]
    pub denominator: i32,
}

impl CommitteeQuorum {
    /// Voting threshold of the committee.
    pub fn new(numerator: i32, denominator: i32) -> CommitteeQuorum {
        CommitteeQuorum {
            numerator,
            denominator,
        }
    }
}

