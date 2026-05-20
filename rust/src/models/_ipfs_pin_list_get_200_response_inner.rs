use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpfsPinListGet200ResponseInner {
    /// Creation time of the IPFS object on our backends
    #[serde(rename = "time_created")]
    pub time_created: i32,
    /// Pin time of the IPFS object on our backends
    #[serde(rename = "time_pinned")]
    pub time_pinned: i32,
    /// IPFS hash of the pinned object
    #[serde(rename = "ipfs_hash")]
    pub ipfs_hash: String,
    /// Size of the object in Bytes
    #[serde(rename = "size")]
    pub size: String,
    /// State of the pinned object, which is `queued` when we are retriving object. If this is successful the state is changed to `pinned` or `failed` if not. The state `gc` means the pinned item has been garbage collected due to account being over storage quota or after it has been moved to `unpinned` state by removing the object pin. 
    #[serde(rename = "state")]
    pub state: State,
    /// Whether filecoin was used to pin the resource.
    #[serde(rename = "filecoin")]
    pub filecoin: bool,
}

impl IpfsPinListGet200ResponseInner {
    pub fn new(time_created: i32, time_pinned: i32, ipfs_hash: String, size: String, state: State, filecoin: bool) -> IpfsPinListGet200ResponseInner {
        IpfsPinListGet200ResponseInner {
            time_created,
            time_pinned,
            ipfs_hash,
            size,
            state,
            filecoin,
        }
    }
}
/// State of the pinned object, which is `queued` when we are retriving object. If this is successful the state is changed to `pinned` or `failed` if not. The state `gc` means the pinned item has been garbage collected due to account being over storage quota or after it has been moved to `unpinned` state by removing the object pin. 
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

