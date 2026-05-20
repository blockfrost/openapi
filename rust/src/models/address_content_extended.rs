use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressContentExtended {
    /// Bech32 encoded addresses
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "amount")]
    pub amount: Vec<models::AddressContentExtendedAmountInner>,
    /// Stake address that controls the key
    #[serde(rename = "stake_address", deserialize_with = "Option::deserialize")]
    pub stake_address: Option<String>,
    /// Address era
    #[serde(rename = "type")]
    pub r#type: Type,
    /// True if this is a script address
    #[serde(rename = "script")]
    pub script: bool,
}

impl AddressContentExtended {
    pub fn new(address: String, amount: Vec<models::AddressContentExtendedAmountInner>, stake_address: Option<String>, r#type: Type, script: bool) -> AddressContentExtended {
        AddressContentExtended {
            address,
            amount,
            stake_address,
            r#type,
            script,
        }
    }
}
/// Address era
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "byron")]
    Byron,
    #[serde(rename = "shelley")]
    Shelley,
}

impl Default for Type {
    fn default() -> Type {
        Self::Byron
    }
}

