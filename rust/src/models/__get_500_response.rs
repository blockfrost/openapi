use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Get500Response {
    #[serde(rename = "status_code")]
    pub status_code: i32,
    #[serde(rename = "error")]
    pub error: String,
    #[serde(rename = "message")]
    pub message: String,
}

impl Get500Response {
    pub fn new(status_code: i32, error: String, message: String) -> Get500Response {
        Get500Response {
            status_code,
            error,
            message,
        }
    }
}

