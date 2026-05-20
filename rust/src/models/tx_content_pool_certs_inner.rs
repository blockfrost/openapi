use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TxContentPoolCertsInner {
    /// Index of the certificate within the transaction
    #[serde(rename = "cert_index")]
    pub cert_index: i32,
    /// Bech32 encoded pool ID
    #[serde(rename = "pool_id")]
    pub pool_id: String,
    /// VRF key hash
    #[serde(rename = "vrf_key")]
    pub vrf_key: String,
    /// Stake pool certificate pledge in Lovelaces
    #[serde(rename = "pledge")]
    pub pledge: String,
    /// Margin tax cost of the stake pool
    #[serde(rename = "margin_cost")]
    pub margin_cost: f64,
    /// Fixed tax cost of the stake pool in Lovelaces
    #[serde(rename = "fixed_cost")]
    pub fixed_cost: String,
    /// Bech32 reward account of the stake pool
    #[serde(rename = "reward_account")]
    pub reward_account: String,
    #[serde(rename = "owners")]
    pub owners: Vec<String>,
    #[serde(rename = "metadata", deserialize_with = "Option::deserialize")]
    pub metadata: Option<Box<models::TxContentPoolCertsInnerMetadata>>,
    #[serde(rename = "relays")]
    pub relays: Vec<models::TxContentPoolCertsInnerRelaysInner>,
    /// Epoch in which the update becomes active
    #[serde(rename = "active_epoch")]
    pub active_epoch: i32,
}

impl TxContentPoolCertsInner {
    pub fn new(cert_index: i32, pool_id: String, vrf_key: String, pledge: String, margin_cost: f64, fixed_cost: String, reward_account: String, owners: Vec<String>, metadata: Option<models::TxContentPoolCertsInnerMetadata>, relays: Vec<models::TxContentPoolCertsInnerRelaysInner>, active_epoch: i32) -> TxContentPoolCertsInner {
        TxContentPoolCertsInner {
            cert_index,
            pool_id,
            vrf_key,
            pledge,
            margin_cost,
            fixed_cost,
            reward_account,
            owners,
            metadata: if let Some(x) = metadata {Some(Box::new(x))} else {None},
            relays,
            active_epoch,
        }
    }
}

