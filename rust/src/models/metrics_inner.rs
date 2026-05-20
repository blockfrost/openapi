use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricsInner {
    /// Starting time of the call count interval (ends midnight UTC) in UNIX time
    #[serde(rename = "time")]
    pub time: i32,
    /// Sum of all calls for a particular day
    #[serde(rename = "calls")]
    pub calls: i32,
}

impl MetricsInner {
    pub fn new(time: i32, calls: i32) -> MetricsInner {
        MetricsInner {
            time,
            calls,
        }
    }
}

