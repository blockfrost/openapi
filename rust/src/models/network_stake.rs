use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkStake {
    /// Current live stake in Lovelaces
    #[serde(rename = "live")]
    pub live: String,
    /// Current active stake in Lovelaces
    #[serde(rename = "active")]
    pub active: String,
}

impl NetworkStake {
    pub fn new(live: String, active: String) -> NetworkStake {
        NetworkStake {
            live,
            active,
        }
    }
}

