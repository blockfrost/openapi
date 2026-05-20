use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScriptCbor {
    /// CBOR contents of the `plutus` script, null for `timelocks`
    #[serde(rename = "cbor", deserialize_with = "Option::deserialize")]
    pub cbor: Option<String>,
}

impl ScriptCbor {
    pub fn new(cbor: Option<String>) -> ScriptCbor {
        ScriptCbor {
            cbor,
        }
    }
}

