use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountAddressesContentInner {
    /// Address associated with the stake key
    #[serde(rename = "address")]
    pub address: String,
}

impl AccountAddressesContentInner {
    pub fn new(address: String) -> AccountAddressesContentInner {
        AccountAddressesContentInner {
            address,
        }
    }
}

