use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TxContentPoolCertsInnerRelaysInner {
    /// IPv4 address of the relay
    #[serde(rename = "ipv4", deserialize_with = "Option::deserialize")]
    pub ipv4: Option<String>,
    /// IPv6 address of the relay
    #[serde(rename = "ipv6", deserialize_with = "Option::deserialize")]
    pub ipv6: Option<String>,
    /// DNS name of the relay
    #[serde(rename = "dns", deserialize_with = "Option::deserialize")]
    pub dns: Option<String>,
    /// DNS SRV entry of the relay
    #[serde(rename = "dns_srv", deserialize_with = "Option::deserialize")]
    pub dns_srv: Option<String>,
    /// Network port of the relay
    #[serde(rename = "port")]
    pub port: i32,
}

impl TxContentPoolCertsInnerRelaysInner {
    pub fn new(ipv4: Option<String>, ipv6: Option<String>, dns: Option<String>, dns_srv: Option<String>, port: i32) -> TxContentPoolCertsInnerRelaysInner {
        TxContentPoolCertsInnerRelaysInner {
            ipv4,
            ipv6,
            dns,
            dns_srv,
            port,
        }
    }
}

