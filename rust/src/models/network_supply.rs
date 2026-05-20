use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkSupply {
    /// Maximum supply in Lovelaces
    #[serde(rename = "max")]
    pub max: String,
    /// Current total (max supply - reserves) supply in Lovelaces
    #[serde(rename = "total")]
    pub total: String,
    /// Current circulating (UTXOs + withdrawables) supply in Lovelaces
    #[serde(rename = "circulating")]
    pub circulating: String,
    /// Current supply locked by scripts in Lovelaces
    #[serde(rename = "locked")]
    pub locked: String,
    /// Current supply locked in treasury
    #[serde(rename = "treasury")]
    pub treasury: String,
    /// Current supply locked in reserves
    #[serde(rename = "reserves")]
    pub reserves: String,
}

impl NetworkSupply {
    pub fn new(max: String, total: String, circulating: String, locked: String, treasury: String, reserves: String) -> NetworkSupply {
        NetworkSupply {
            max,
            total,
            circulating,
            locked,
            treasury,
            reserves,
        }
    }
}

