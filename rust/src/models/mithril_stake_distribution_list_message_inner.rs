use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MithrilStakeDistributionListMessageInner {
    /// Cardano chain epoch number
    #[serde(rename = "epoch")]
    pub epoch: i64,
    /// Hash of the Mithril stake distribution
    #[serde(rename = "hash")]
    pub hash: String,
    /// Hash of the associated certificate
    #[serde(rename = "certificate_hash", skip_serializing_if = "Option::is_none")]
    pub certificate_hash: Option<String>,
    /// Date and time at which the Mithril stake distribution was created
    #[serde(rename = "created_at")]
    pub created_at: String,
}

impl MithrilStakeDistributionListMessageInner {
    pub fn new(epoch: i64, hash: String, created_at: String) -> MithrilStakeDistributionListMessageInner {
        MithrilStakeDistributionListMessageInner {
            epoch,
            hash,
            certificate_hash: None,
            created_at,
        }
    }
}

