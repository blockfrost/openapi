use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpfsPinListIpfsPathGet200Response {
    /// Time of the creation of the IPFS object on our backends
    #[serde(rename = "time_created")]
    pub time_created: i32,
    /// Time of the pin of the IPFS object on our backends
    #[serde(rename = "time_pinned")]
    pub time_pinned: i32,
    /// IPFS hash of the pinned object
    #[serde(rename = "ipfs_hash")]
    pub ipfs_hash: String,
    /// Size of the object in Bytes
    #[serde(rename = "size")]
    pub size: String,
    /// State of the pinned object. We define 5 states: `queued`, `pinned`, `unpinned`, `failed`, `gc`. When the object is pending retrieval (i.e. after `/ipfs/pin/add/{IPFS_path}`), the state is `queued`. If the object is already successfully retrieved, state is changed to `pinned` or `failed` otherwise. When object is unpinned (i.e. after `/ipfs/pin/remove/{IPFS_path}`) it is marked for garbage collection. State `gc` means that a previously `unpinned` item has been garbage collected due to account being over storage quota. 
    #[serde(rename = "state")]
    pub state: State,
    /// Whether filecoin was used to pin the resource.
    #[serde(rename = "filecoin")]
    pub filecoin: bool,
}

impl IpfsPinListIpfsPathGet200Response {
    pub fn new(time_created: i32, time_pinned: i32, ipfs_hash: String, size: String, state: State, filecoin: bool) -> IpfsPinListIpfsPathGet200Response {
        IpfsPinListIpfsPathGet200Response {
            time_created,
            time_pinned,
            ipfs_hash,
            size,
            state,
            filecoin,
        }
    }
}
/// State of the pinned object. We define 5 states: `queued`, `pinned`, `unpinned`, `failed`, `gc`. When the object is pending retrieval (i.e. after `/ipfs/pin/add/{IPFS_path}`), the state is `queued`. If the object is already successfully retrieved, state is changed to `pinned` or `failed` otherwise. When object is unpinned (i.e. after `/ipfs/pin/remove/{IPFS_path}`) it is marked for garbage collection. State `gc` means that a previously `unpinned` item has been garbage collected due to account being over storage quota. 
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

