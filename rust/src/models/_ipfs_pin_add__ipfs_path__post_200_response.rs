use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpfsPinAddIpfsPathPost200Response {
    /// IPFS hash of the pinned object
    #[serde(rename = "ipfs_hash")]
    pub ipfs_hash: String,
    /// State of the pin action
    #[serde(rename = "state")]
    pub state: State,
    /// Whether filecoin was used to pin the resource.
    #[serde(rename = "filecoin")]
    pub filecoin: bool,
}

impl IpfsPinAddIpfsPathPost200Response {
    pub fn new(ipfs_hash: String, state: State, filecoin: bool) -> IpfsPinAddIpfsPathPost200Response {
        IpfsPinAddIpfsPathPost200Response {
            ipfs_hash,
            state,
            filecoin,
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

