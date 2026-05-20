use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pool {
    /// Bech32 pool ID
    #[serde(rename = "pool_id")]
    pub pool_id: String,
    /// Hexadecimal pool ID.
    #[serde(rename = "hex")]
    pub hex: String,
    /// VRF key hash
    #[serde(rename = "vrf_key")]
    pub vrf_key: String,
    /// Total minted blocks
    #[serde(rename = "blocks_minted")]
    pub blocks_minted: i32,
    /// Number of blocks minted in the current epoch
    #[serde(rename = "blocks_epoch")]
    pub blocks_epoch: i32,
    #[serde(rename = "live_stake")]
    pub live_stake: String,
    #[serde(rename = "live_size")]
    pub live_size: f64,
    #[serde(rename = "live_saturation")]
    pub live_saturation: f64,
    #[serde(rename = "live_delegators")]
    pub live_delegators: f64,
    #[serde(rename = "active_stake")]
    pub active_stake: String,
    #[serde(rename = "active_size")]
    pub active_size: f64,
    /// Stake pool certificate pledge
    #[serde(rename = "declared_pledge")]
    pub declared_pledge: String,
    /// Stake pool current pledge
    #[serde(rename = "live_pledge")]
    pub live_pledge: String,
    /// Margin tax cost of the stake pool
    #[serde(rename = "margin_cost")]
    pub margin_cost: f64,
    /// Fixed tax cost of the stake pool
    #[serde(rename = "fixed_cost")]
    pub fixed_cost: String,
    /// Bech32 reward account of the stake pool
    #[serde(rename = "reward_account")]
    pub reward_account: String,
    #[serde(rename = "owners")]
    pub owners: Vec<String>,
    #[serde(rename = "registration")]
    pub registration: Vec<String>,
    #[serde(rename = "retirement")]
    pub retirement: Vec<String>,
    #[serde(rename = "calidus_key", deserialize_with = "Option::deserialize")]
    pub calidus_key: Option<Box<models::PoolCalidusKey>>,
}

impl Pool {
    pub fn new(pool_id: String, hex: String, vrf_key: String, blocks_minted: i32, blocks_epoch: i32, live_stake: String, live_size: f64, live_saturation: f64, live_delegators: f64, active_stake: String, active_size: f64, declared_pledge: String, live_pledge: String, margin_cost: f64, fixed_cost: String, reward_account: String, owners: Vec<String>, registration: Vec<String>, retirement: Vec<String>, calidus_key: Option<models::PoolCalidusKey>) -> Pool {
        Pool {
            pool_id,
            hex,
            vrf_key,
            blocks_minted,
            blocks_epoch,
            live_stake,
            live_size,
            live_saturation,
            live_delegators,
            active_stake,
            active_size,
            declared_pledge,
            live_pledge,
            margin_cost,
            fixed_cost,
            reward_account,
            owners,
            registration,
            retirement,
            calidus_key: if let Some(x) = calidus_key {Some(Box::new(x))} else {None},
        }
    }
}

