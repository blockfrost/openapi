use crate::models;
use serde::{Deserialize, Serialize};

/// Error : Internal error representation
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    /// optional label
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// error message
    #[serde(rename = "message")]
    pub message: String,
}

impl Error {
    /// Internal error representation
    pub fn new(message: String) -> Error {
        Error {
            label: None,
            message,
        }
    }
}

