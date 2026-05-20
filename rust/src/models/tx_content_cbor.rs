use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TxContentCbor {
    /// CBOR serialized transaction
    #[serde(rename = "cbor")]
    pub cbor: String,
}

impl TxContentCbor {
    pub fn new(cbor: String) -> TxContentCbor {
        TxContentCbor {
            cbor,
        }
    }
}

