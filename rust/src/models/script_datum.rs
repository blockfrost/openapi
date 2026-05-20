use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScriptDatum {
    /// JSON content of the datum
    #[serde(rename = "json_value")]
    pub json_value: std::collections::HashMap<String, serde_json::Value>,
}

impl ScriptDatum {
    pub fn new(json_value: std::collections::HashMap<String, serde_json::Value>) -> ScriptDatum {
        ScriptDatum {
            json_value,
        }
    }
}

