use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScriptsInner {
    /// Script hash
    #[serde(rename = "script_hash")]
    pub script_hash: String,
}

impl ScriptsInner {
    pub fn new(script_hash: String) -> ScriptsInner {
        ScriptsInner {
            script_hash,
        }
    }
}

