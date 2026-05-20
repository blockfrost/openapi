use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PoolListExtendedInner {
    /// Bech32 encoded pool ID
    #[serde(rename = "pool_id")]
    pub pool_id: String,
    /// Hexadecimal pool ID.
    #[serde(rename = "hex")]
    pub hex: String,
    /// Active delegated amount
    #[serde(rename = "active_stake")]
    pub active_stake: String,
    /// Currently delegated amount
    #[serde(rename = "live_stake")]
    pub live_stake: String,
    #[serde(rename = "live_saturation")]
    pub live_saturation: f64,
    /// Total minted blocks
    #[serde(rename = "blocks_minted")]
    pub blocks_minted: i32,
    /// Stake pool certificate pledge
    #[serde(rename = "declared_pledge")]
    pub declared_pledge: String,
    /// Margin tax cost of the stake pool
    #[serde(rename = "margin_cost")]
    pub margin_cost: f64,
    /// Fixed tax cost of the stake pool
    #[serde(rename = "fixed_cost")]
    pub fixed_cost: String,
    #[serde(rename = "metadata", deserialize_with = "Option::deserialize")]
    pub metadata: Option<Box<models::PoolListExtendedInnerMetadata>>,
}

impl PoolListExtendedInner {
    pub fn new(pool_id: String, hex: String, active_stake: String, live_stake: String, live_saturation: f64, blocks_minted: i32, declared_pledge: String, margin_cost: f64, fixed_cost: String, metadata: Option<models::PoolListExtendedInnerMetadata>) -> PoolListExtendedInner {
        PoolListExtendedInner {
            pool_id,
            hex,
            active_stake,
            live_stake,
            live_saturation,
            blocks_minted,
            declared_pledge,
            margin_cost,
            fixed_cost,
            metadata: if let Some(x) = metadata {Some(Box::new(x))} else {None},
        }
    }
}

