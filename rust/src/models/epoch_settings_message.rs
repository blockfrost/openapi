use crate::models;
use serde::{Deserialize, Serialize};

/// EpochSettingsMessage : Epoch settings
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EpochSettingsMessage {
    /// Cardano chain epoch number
    #[serde(rename = "epoch")]
    pub epoch: i64,
    #[serde(rename = "protocol")]
    pub protocol: models::ProtocolParameters,
    #[serde(rename = "next_protocol")]
    pub next_protocol: models::ProtocolParameters,
}

impl EpochSettingsMessage {
    /// Epoch settings
    pub fn new(epoch: i64, protocol: models::ProtocolParameters, next_protocol: models::ProtocolParameters) -> EpochSettingsMessage {
        EpochSettingsMessage {
            epoch,
            protocol,
            next_protocol,
        }
    }
}

