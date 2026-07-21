use crate::models;
use serde::{Deserialize, Serialize};

/// DrepsInnerMetadataError : Present when metadata could not be fetched or validated.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DrepsInnerMetadataError {
    /// Stable machine-readable error code.
    #[serde(rename = "code")]
    pub code: Code,
    /// Human-readable description of the error.
    #[serde(rename = "message")]
    pub message: String,
}

impl DrepsInnerMetadataError {
    /// Present when metadata could not be fetched or validated.
    pub fn new(code: Code, message: String) -> DrepsInnerMetadataError {
        DrepsInnerMetadataError {
            code,
            message,
        }
    }
}
/// Stable machine-readable error code.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Code {
    #[serde(rename = "HASH_MISMATCH")]
    HashMismatch,
    #[serde(rename = "CONNECTION_ERROR")]
    ConnectionError,
    #[serde(rename = "HTTP_RESPONSE_ERROR")]
    HttpResponseError,
    #[serde(rename = "DECODE_ERROR")]
    DecodeError,
    #[serde(rename = "SIZE_EXCEEDED")]
    SizeExceeded,
    #[serde(rename = "UNKNOWN_ERROR")]
    UnknownError,
}

impl Default for Code {
    fn default() -> Code {
        Self::HashMismatch
    }
}

