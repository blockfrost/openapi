use crate::models;
use serde::{Deserialize, Serialize};

/// ProtocolMessage : ProtocolMessage represents a message that is signed (or verified) by the Mithril protocol
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProtocolMessage {
    #[serde(rename = "message_parts")]
    pub message_parts: models::ProtocolMessageParts,
}

impl ProtocolMessage {
    /// ProtocolMessage represents a message that is signed (or verified) by the Mithril protocol
    pub fn new(message_parts: models::ProtocolMessageParts) -> ProtocolMessage {
        ProtocolMessage {
            message_parts,
        }
    }
}

