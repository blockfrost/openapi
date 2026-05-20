use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricsEndpointsInner {
    /// Starting time of the call count interval (ends midnight UTC) in UNIX time
    #[serde(rename = "time")]
    pub time: i32,
    /// Sum of all calls for a particular day and endpoint
    #[serde(rename = "calls")]
    pub calls: i32,
    /// Endpoint parent name
    #[serde(rename = "endpoint")]
    pub endpoint: String,
}

impl MetricsEndpointsInner {
    pub fn new(time: i32, calls: i32, endpoint: String) -> MetricsEndpointsInner {
        MetricsEndpointsInner {
            time,
            calls,
            endpoint,
        }
    }
}

