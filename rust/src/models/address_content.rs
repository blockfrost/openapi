use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressContent {
    /// Bech32 encoded addresses
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "amount")]
    pub amount: Vec<models::TxContentOutputAmountInner>,
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

impl AddressContent {
    pub fn new(address: String, amount: Vec<models::TxContentOutputAmountInner>, stake_address: Option<String>, r#type: Type, script: bool) -> AddressContent {
        AddressContent {
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

