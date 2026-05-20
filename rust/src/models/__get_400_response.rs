use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Get400Response {
    #[serde(rename = "status_code")]
    pub status_code: i32,
    #[serde(rename = "error")]
    pub error: String,
    #[serde(rename = "message")]
    pub message: String,
}

impl Get400Response {
    pub fn new(status_code: i32, error: String, message: String) -> Get400Response {
        Get400Response {
            status_code,
            error,
            message,
        }
    }
}

