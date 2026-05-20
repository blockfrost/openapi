use crate::models;
use serde::{Deserialize, Serialize};

/// NetworkErasInnerStart : Start of the blockchain era, relative to the start of the network 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkErasInnerStart {
    /// Time in seconds relative to the start time of the network
    #[serde(rename = "time")]
    pub time: i32,
    /// Absolute slot number
    #[serde(rename = "slot")]
    pub slot: i32,
    /// Epoch number
    #[serde(rename = "epoch")]
    pub epoch: i32,
}

impl NetworkErasInnerStart {
    /// Start of the blockchain era, relative to the start of the network 
    pub fn new(time: i32, slot: i32, epoch: i32) -> NetworkErasInnerStart {
        NetworkErasInnerStart {
            time,
            slot,
            epoch,
        }
    }
}

