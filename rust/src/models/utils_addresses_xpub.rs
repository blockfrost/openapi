use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UtilsAddressesXpub {
    /// Script hash
    #[serde(rename = "xpub")]
    pub xpub: String,
    /// Account role
    #[serde(rename = "role")]
    pub role: i32,
    /// Address index
    #[serde(rename = "index")]
    pub index: i32,
    /// Derived address
    #[serde(rename = "address")]
    pub address: String,
}

impl UtilsAddressesXpub {
    pub fn new(xpub: String, role: i32, index: i32, address: String) -> UtilsAddressesXpub {
        UtilsAddressesXpub {
            xpub,
            role,
            index,
            address,
        }
    }
}

