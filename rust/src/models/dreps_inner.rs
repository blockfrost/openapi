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
    /// The total amount of voting power this DRep is delegated.
    #[serde(rename = "amount")]
    pub amount: String,
    /// Flag indicating whether this DRep's credential is a script hash
    #[serde(rename = "has_script")]
    pub has_script: bool,
    /// Registration state of the DRep. Set to `true` if the DRep has been deregistered; otherwise, `false`.
    #[serde(rename = "retired")]
    pub retired: bool,
    /// Whether the DRep has been inactive for a consecutive number of epochs (determined by an epoch parameter `drep_activity`)
    #[serde(rename = "expired")]
    pub expired: bool,
    /// Epoch of the most recent action - registration, update, deregistration or voting
    #[serde(rename = "last_active_epoch", deserialize_with = "Option::deserialize")]
    pub last_active_epoch: Option<i32>,
    #[serde(rename = "metadata", deserialize_with = "Option::deserialize")]
    pub metadata: Option<Box<models::DrepsInnerMetadata>>,
}

impl DrepsInner {
    pub fn new(drep_id: String, hex: String, amount: String, has_script: bool, retired: bool, expired: bool, last_active_epoch: Option<i32>, metadata: Option<models::DrepsInnerMetadata>) -> DrepsInner {
        DrepsInner {
            drep_id,
            hex,
            amount,
            has_script,
            retired,
            expired,
            last_active_epoch,
            metadata: if let Some(x) = metadata {Some(Box::new(x))} else {None},
        }
    }
}

