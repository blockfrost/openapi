use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NutlinkAddressTickersInner {
    /// Name of the ticker
    #[serde(rename = "name")]
    pub name: String,
    /// Number of ticker records
    #[serde(rename = "count")]
    pub count: i32,
    /// Block height of the latest record
    #[serde(rename = "latest_block")]
    pub latest_block: i32,
}

impl NutlinkAddressTickersInner {
    pub fn new(name: String, count: i32, latest_block: i32) -> NutlinkAddressTickersInner {
        NutlinkAddressTickersInner {
            name,
            count,
            latest_block,
        }
    }
}

