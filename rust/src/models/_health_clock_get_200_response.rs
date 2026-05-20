use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthClockGet200Response {
    #[serde(rename = "server_time")]
    pub server_time: i64,
}

impl HealthClockGet200Response {
    pub fn new(server_time: i64) -> HealthClockGet200Response {
        HealthClockGet200Response {
            server_time,
        }
    }
}

