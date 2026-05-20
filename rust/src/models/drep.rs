use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Drep {
    /// Bech32 encoded DRep address
    #[serde(rename = "drep_id")]
    pub drep_id: String,
    /// The raw bytes of the DRep
    #[serde(rename = "hex")]
    pub hex: String,
    /// The total amount of voting power this DRep is delegated.
    #[serde(rename = "amount")]
    pub amount: String,
    /// Registration state of the DRep
    #[serde(rename = "active")]
    pub active: bool,
    /// Epoch of the most recent registration
    #[serde(rename = "active_epoch", deserialize_with = "Option::deserialize")]
    pub active_epoch: Option<i32>,
    /// Flag which shows if this DRep credentials are a script hash
    #[serde(rename = "has_script")]
    pub has_script: bool,
    /// Registration state of the DRep. Set to `true` if the DRep has been deregistered; otherwise, `false`.
    #[serde(rename = "retired")]
    pub retired: bool,
    /// Whether the DRep has been inactive for a consecutive number of epochs (determined by a epoch parameter `drep_activity`)
    #[serde(rename = "expired")]
    pub expired: bool,
    /// Epoch of the most recent action - registration, update, deregistration or voting
    #[serde(rename = "last_active_epoch", deserialize_with = "Option::deserialize")]
    pub last_active_epoch: Option<i32>,
}

impl Drep {
    pub fn new(drep_id: String, hex: String, amount: String, active: bool, active_epoch: Option<i32>, has_script: bool, retired: bool, expired: bool, last_active_epoch: Option<i32>) -> Drep {
        Drep {
            drep_id,
            hex,
            amount,
            active,
            active_epoch,
            has_script,
            retired,
            expired,
            last_active_epoch,
        }
    }
}

