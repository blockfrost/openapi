use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Get200Response {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "version")]
    pub version: String,
}

impl Get200Response {
    pub fn new(url: String, version: String) -> Get200Response {
        Get200Response {
            url,
            version,
        }
    }
}

