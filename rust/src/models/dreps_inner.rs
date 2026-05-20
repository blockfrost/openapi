use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DrepsInner {
    /// The Bech32 encoded DRep address
    #[serde(rename = "drep_id")]
    pub drep_id: String,
    /// The raw bytes of the DRep
    #[serde(rename = "hex")]
    pub hex: String,
}

impl DrepsInner {
    pub fn new(drep_id: String, hex: String) -> DrepsInner {
        DrepsInner {
            drep_id,
            hex,
        }
    }
}

