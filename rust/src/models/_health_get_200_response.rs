use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthGet200Response {
    #[serde(rename = "is_healthy")]
    pub is_healthy: bool,
}

impl HealthGet200Response {
    pub fn new(is_healthy: bool) -> HealthGet200Response {
        HealthGet200Response {
            is_healthy,
        }
    }
}

