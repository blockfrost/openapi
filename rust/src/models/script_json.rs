use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScriptJson {
    /// JSON contents of the `timelock` script, null for `plutus` scripts
    #[serde(rename = "json", deserialize_with = "Option::deserialize")]
    pub json: Option<serde_json::Value>,
}

impl ScriptJson {
    pub fn new(json: Option<serde_json::Value>) -> ScriptJson {
        ScriptJson {
            json,
        }
    }
}

