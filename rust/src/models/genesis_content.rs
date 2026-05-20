use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenesisContent {
    /// The proportion of slots in which blocks should be issued
    #[serde(rename = "active_slots_coefficient")]
    pub active_slots_coefficient: f64,
    /// Determines the quorum needed for votes on the protocol parameter updates
    #[serde(rename = "update_quorum")]
    pub update_quorum: i32,
    /// The total number of lovelace in the system
    #[serde(rename = "max_lovelace_supply")]
    pub max_lovelace_supply: String,
    /// Network identifier
    #[serde(rename = "network_magic")]
    pub network_magic: i32,
    /// Number of slots in an epoch
    #[serde(rename = "epoch_length")]
    pub epoch_length: i32,
    /// Time of slot 0 in UNIX time
    #[serde(rename = "system_start")]
    pub system_start: i32,
    /// Number of slots in an KES period
    #[serde(rename = "slots_per_kes_period")]
    pub slots_per_kes_period: i32,
    /// Duration of one slot in seconds
    #[serde(rename = "slot_length")]
    pub slot_length: i32,
    /// The maximum number of time a KES key can be evolved before a pool operator must create a new operational certificate
    #[serde(rename = "max_kes_evolutions")]
    pub max_kes_evolutions: i32,
    /// Security parameter k
    #[serde(rename = "security_param")]
    pub security_param: i32,
}

impl GenesisContent {
    pub fn new(active_slots_coefficient: f64, update_quorum: i32, max_lovelace_supply: String, network_magic: i32, epoch_length: i32, system_start: i32, slots_per_kes_period: i32, slot_length: i32, max_kes_evolutions: i32, security_param: i32) -> GenesisContent {
        GenesisContent {
            active_slots_coefficient,
            update_quorum,
            max_lovelace_supply,
            network_magic,
            epoch_length,
            system_start,
            slots_per_kes_period,
            slot_length,
            max_kes_evolutions,
            security_param,
        }
    }
}

