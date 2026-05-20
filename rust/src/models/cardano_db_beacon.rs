use crate::models;
use serde::{Deserialize, Serialize};

/// CardanoDbBeacon : A point in the Cardano chain at which a Mithril certificate of the Cardano Database should be produced
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CardanoDbBeacon {
    /// Cardano network
    #[serde(rename = "network")]
    pub network: String,
    /// Cardano chain epoch number
    #[serde(rename = "epoch")]
    pub epoch: i64,
    /// Number of the last immutable file that should be included the snapshot
    #[serde(rename = "immutable_file_number")]
    pub immutable_file_number: i64,
}

impl CardanoDbBeacon {
    /// A point in the Cardano chain at which a Mithril certificate of the Cardano Database should be produced
    pub fn new(network: String, epoch: i64, immutable_file_number: i64) -> CardanoDbBeacon {
        CardanoDbBeacon {
            network,
            epoch,
            immutable_file_number,
        }
    }
}

