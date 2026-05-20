use crate::models;
use serde::{Deserialize, Serialize};

/// Stake : Stake represents the stakes of a participant in the Cardano chain
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Stake {
    /// Stake share as computed in the 'stake distribution' by the Cardano Node, multiplied by a billion (1.0e9)
    #[serde(rename = "stake")]
    pub stake: i64,
}

impl Stake {
    /// Stake represents the stakes of a participant in the Cardano chain
    pub fn new(stake: i64) -> Stake {
        Stake {
            stake,
        }
    }
}

