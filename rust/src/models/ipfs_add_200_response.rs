use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpfsAdd200Response {
    /// Name of the file
    #[serde(rename = "name")]
    pub name: String,
    /// IPFS hash of the file
    #[serde(rename = "ipfs_hash")]
    pub ipfs_hash: String,
    /// IPFS node size in Bytes
    #[serde(rename = "size")]
    pub size: String,
}

impl IpfsAdd200Response {
    pub fn new(name: String, ipfs_hash: String, size: String) -> IpfsAdd200Response {
        IpfsAdd200Response {
            name,
            ipfs_hash,
            size,
        }
    }
}

