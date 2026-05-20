use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScriptDatumCbor {
    /// CBOR serialized datum
    #[serde(rename = "cbor")]
    pub cbor: String,
}

impl ScriptDatumCbor {
    pub fn new(cbor: String) -> ScriptDatumCbor {
        ScriptDatumCbor {
            cbor,
        }
    }
}

