use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpfsPinRemoveIpfsPathPost200Response {
    /// IPFS hash of the pinned object
    #[serde(rename = "ipfs_hash")]
    pub ipfs_hash: String,
    /// State of the pin action
    #[serde(rename = "state")]
    pub state: State,
}

impl IpfsPinRemoveIpfsPathPost200Response {
    pub fn new(ipfs_hash: String, state: State) -> IpfsPinRemoveIpfsPathPost200Response {
        IpfsPinRemoveIpfsPathPost200Response {
            ipfs_hash,
            state,
        }
    }
}
/// State of the pin action
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "pinned")]
    Pinned,
    #[serde(rename = "unpinned")]
    Unpinned,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "gc")]
    Gc,
}

impl Default for State {
    fn default() -> State {
        Self::Queued
    }
}

