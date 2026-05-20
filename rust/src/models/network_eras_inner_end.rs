use crate::models;
use serde::{Deserialize, Serialize};

/// NetworkErasInnerEnd : End of the blockchain era, relative to the start of the network 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkErasInnerEnd {
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

impl NetworkErasInnerEnd {
    /// End of the blockchain era, relative to the start of the network 
    pub fn new(time: i32, slot: i32, epoch: i32) -> NetworkErasInnerEnd {
        NetworkErasInnerEnd {
            time,
            slot,
            epoch,
        }
    }
}

