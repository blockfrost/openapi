use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Script {
    /// Script hash
    #[serde(rename = "script_hash")]
    pub script_hash: String,
    /// Type of the script language
    #[serde(rename = "type")]
    pub r#type: Type,
    /// The size of the CBOR serialised script, if a Plutus script
    #[serde(rename = "serialised_size", deserialize_with = "Option::deserialize")]
    pub serialised_size: Option<i32>,
}

impl Script {
    pub fn new(script_hash: String, r#type: Type, serialised_size: Option<i32>) -> Script {
        Script {
            script_hash,
            r#type,
            serialised_size,
        }
    }
}
/// Type of the script language
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "timelock")]
    Timelock,
    #[serde(rename = "plutusV1")]
    PlutusV1,
    #[serde(rename = "plutusV2")]
    PlutusV2,
    #[serde(rename = "plutusV3")]
    PlutusV3,
}

impl Default for Type {
    fn default() -> Type {
        Self::Timelock
    }
}

