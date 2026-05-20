use crate::models;
use serde::{Deserialize, Serialize};

/// NetworkErasInnerParameters : Era parameters
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkErasInnerParameters {
    /// Epoch length in number of slots
    #[serde(rename = "epoch_length")]
    pub epoch_length: i32,
    /// Slot length in seconds
    #[serde(rename = "slot_length")]
    pub slot_length: i32,
    /// Zone in which it is guaranteed that no hard fork can take place
    #[serde(rename = "safe_zone")]
    pub safe_zone: i32,
}

impl NetworkErasInnerParameters {
    /// Era parameters
    pub fn new(epoch_length: i32, slot_length: i32, safe_zone: i32) -> NetworkErasInnerParameters {
        NetworkErasInnerParameters {
            epoch_length,
            slot_length,
            safe_zone,
        }
    }
}

