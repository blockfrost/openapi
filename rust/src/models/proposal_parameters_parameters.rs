use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProposalParametersParameters {
    /// Epoch number
    #[serde(rename = "epoch", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub epoch: Option<Option<i32>>,
    /// The linear factor for the minimum fee calculation for given epoch
    #[serde(rename = "min_fee_a", deserialize_with = "Option::deserialize")]
    pub min_fee_a: Option<i32>,
    /// The constant factor for the minimum fee calculation
    #[serde(rename = "min_fee_b", deserialize_with = "Option::deserialize")]
    pub min_fee_b: Option<i32>,
    /// Maximum block body size in Bytes
    #[serde(rename = "max_block_size", deserialize_with = "Option::deserialize")]
    pub max_block_size: Option<i32>,
    /// Maximum transaction size
    #[serde(rename = "max_tx_size", deserialize_with = "Option::deserialize")]
    pub max_tx_size: Option<i32>,
    /// Maximum block header size
    #[serde(rename = "max_block_header_size", deserialize_with = "Option::deserialize")]
    pub max_block_header_size: Option<i32>,
    /// The amount of a key registration deposit in Lovelaces
    #[serde(rename = "key_deposit", deserialize_with = "Option::deserialize")]
    pub key_deposit: Option<String>,
    /// The amount of a pool registration deposit in Lovelaces
    #[serde(rename = "pool_deposit", deserialize_with = "Option::deserialize")]
    pub pool_deposit: Option<String>,
    /// Epoch bound on pool retirement
    #[serde(rename = "e_max", deserialize_with = "Option::deserialize")]
    pub e_max: Option<i32>,
    /// Desired number of pools
    #[serde(rename = "n_opt", deserialize_with = "Option::deserialize")]
    pub n_opt: Option<i32>,
    /// Pool pledge influence
    #[serde(rename = "a0", deserialize_with = "Option::deserialize")]
    pub a0: Option<f64>,
    /// Monetary expansion
    #[serde(rename = "rho", deserialize_with = "Option::deserialize")]
    pub rho: Option<f64>,
    /// Treasury expansion
    #[serde(rename = "tau", deserialize_with = "Option::deserialize")]
    pub tau: Option<f64>,
    /// Percentage of blocks produced by federated nodes
    #[serde(rename = "decentralisation_param", deserialize_with = "Option::deserialize")]
    pub decentralisation_param: Option<f64>,
    /// Seed for extra entropy
    #[serde(rename = "extra_entropy", deserialize_with = "Option::deserialize")]
    pub extra_entropy: Option<String>,
    /// Accepted protocol major version
    #[serde(rename = "protocol_major_ver", deserialize_with = "Option::deserialize")]
    pub protocol_major_ver: Option<i32>,
    /// Accepted protocol minor version
    #[serde(rename = "protocol_minor_ver", deserialize_with = "Option::deserialize")]
    pub protocol_minor_ver: Option<i32>,
    /// Minimum UTXO value
    #[serde(rename = "min_utxo", deserialize_with = "Option::deserialize")]
    pub min_utxo: Option<String>,
    /// Minimum stake cost forced on the pool
    #[serde(rename = "min_pool_cost", deserialize_with = "Option::deserialize")]
    pub min_pool_cost: Option<String>,
    /// Cost models parameters for Plutus Core scripts in raw list form
    #[serde(rename = "cost_models", deserialize_with = "Option::deserialize")]
    pub cost_models: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// The per word cost of script memory usage
    #[serde(rename = "price_mem", deserialize_with = "Option::deserialize")]
    pub price_mem: Option<f64>,
    /// The cost of script execution step usage
    #[serde(rename = "price_step", deserialize_with = "Option::deserialize")]
    pub price_step: Option<f64>,
    /// The maximum number of execution memory allowed to be used in a single transaction
    #[serde(rename = "max_tx_ex_mem", deserialize_with = "Option::deserialize")]
    pub max_tx_ex_mem: Option<String>,
    /// The maximum number of execution steps allowed to be used in a single transaction
    #[serde(rename = "max_tx_ex_steps", deserialize_with = "Option::deserialize")]
    pub max_tx_ex_steps: Option<String>,
    /// The maximum number of execution memory allowed to be used in a single block
    #[serde(rename = "max_block_ex_mem", deserialize_with = "Option::deserialize")]
    pub max_block_ex_mem: Option<String>,
    /// The maximum number of execution steps allowed to be used in a single block
    #[serde(rename = "max_block_ex_steps", deserialize_with = "Option::deserialize")]
    pub max_block_ex_steps: Option<String>,
    /// The maximum Val size
    #[serde(rename = "max_val_size", deserialize_with = "Option::deserialize")]
    pub max_val_size: Option<String>,
    /// The percentage of the transactions fee which must be provided as collateral when including non-native scripts
    #[serde(rename = "collateral_percent", deserialize_with = "Option::deserialize")]
    pub collateral_percent: Option<i32>,
    /// The maximum number of collateral inputs allowed in a transaction
    #[serde(rename = "max_collateral_inputs", deserialize_with = "Option::deserialize")]
    pub max_collateral_inputs: Option<i32>,
    /// Cost per UTxO word for Alonzo. Cost per UTxO byte for Babbage and later.
    #[serde(rename = "coins_per_utxo_size", deserialize_with = "Option::deserialize")]
    pub coins_per_utxo_size: Option<String>,
    /// Cost per UTxO word for Alonzo. Cost per UTxO byte for Babbage and later.
    #[serde(rename = "coins_per_utxo_word", deserialize_with = "Option::deserialize")]
    pub coins_per_utxo_word: Option<String>,
    /// Pool Voting threshold for motion of no-confidence. New in 13.2-Conway.
    #[serde(rename = "pvt_motion_no_confidence", deserialize_with = "Option::deserialize")]
    pub pvt_motion_no_confidence: Option<f64>,
    /// Pool Voting threshold for new committee/threshold (normal state). New in 13.2-Conway.
    #[serde(rename = "pvt_committee_normal", deserialize_with = "Option::deserialize")]
    pub pvt_committee_normal: Option<f64>,
    /// Pool Voting threshold for new committee/threshold (state of no-confidence). New in 13.2-Conway.
    #[serde(rename = "pvt_committee_no_confidence", deserialize_with = "Option::deserialize")]
    pub pvt_committee_no_confidence: Option<f64>,
    /// Pool Voting threshold for hard-fork initiation. New in 13.2-Conway.
    #[serde(rename = "pvt_hard_fork_initiation", deserialize_with = "Option::deserialize")]
    pub pvt_hard_fork_initiation: Option<f64>,
    /// DRep Vote threshold for motion of no-confidence. New in 13.2-Conway.
    #[serde(rename = "dvt_motion_no_confidence", deserialize_with = "Option::deserialize")]
    pub dvt_motion_no_confidence: Option<f64>,
    /// DRep Vote threshold for new committee/threshold (normal state). New in 13.2-Conway.
    #[serde(rename = "dvt_committee_normal", deserialize_with = "Option::deserialize")]
    pub dvt_committee_normal: Option<f64>,
    /// DRep Vote threshold for new committee/threshold (state of no-confidence). New in 13.2-Conway.
    #[serde(rename = "dvt_committee_no_confidence", deserialize_with = "Option::deserialize")]
    pub dvt_committee_no_confidence: Option<f64>,
    /// DRep Vote threshold for update to the Constitution. New in 13.2-Conway.
    #[serde(rename = "dvt_update_to_constitution", deserialize_with = "Option::deserialize")]
    pub dvt_update_to_constitution: Option<f64>,
    /// DRep Vote threshold for hard-fork initiation. New in 13.2-Conway.
    #[serde(rename = "dvt_hard_fork_initiation", deserialize_with = "Option::deserialize")]
    pub dvt_hard_fork_initiation: Option<f64>,
    /// DRep Vote threshold for protocol parameter changes, network group. New in 13.2-Conway.
    #[serde(rename = "dvt_p_p_network_group", deserialize_with = "Option::deserialize")]
    pub dvt_p_p_network_group: Option<f64>,
    /// DRep Vote threshold for protocol parameter changes, economic group. New in 13.2-Conway.
    #[serde(rename = "dvt_p_p_economic_group", deserialize_with = "Option::deserialize")]
    pub dvt_p_p_economic_group: Option<f64>,
    /// DRep Vote threshold for protocol parameter changes, technical group. New in 13.2-Conway.
    #[serde(rename = "dvt_p_p_technical_group", deserialize_with = "Option::deserialize")]
    pub dvt_p_p_technical_group: Option<f64>,
    /// DRep Vote threshold for protocol parameter changes, governance group. New in 13.2-Conway.
    #[serde(rename = "dvt_p_p_gov_group", deserialize_with = "Option::deserialize")]
    pub dvt_p_p_gov_group: Option<f64>,
    /// DRep Vote threshold for treasury withdrawal. New in 13.2-Conway.
    #[serde(rename = "dvt_treasury_withdrawal", deserialize_with = "Option::deserialize")]
    pub dvt_treasury_withdrawal: Option<f64>,
    /// Minimal constitutional committee size. New in 13.2-Conway.
    #[serde(rename = "committee_min_size", deserialize_with = "Option::deserialize")]
    pub committee_min_size: Option<String>,
    /// Constitutional committee term limits. New in 13.2-Conway.
    #[serde(rename = "committee_max_term_length", deserialize_with = "Option::deserialize")]
    pub committee_max_term_length: Option<String>,
    /// Governance action expiration. New in 13.2-Conway.
    #[serde(rename = "gov_action_lifetime", deserialize_with = "Option::deserialize")]
    pub gov_action_lifetime: Option<String>,
    /// Governance action deposit. New in 13.2-Conway.
    #[serde(rename = "gov_action_deposit", deserialize_with = "Option::deserialize")]
    pub gov_action_deposit: Option<String>,
    /// DRep deposit amount. New in 13.2-Conway.
    #[serde(rename = "drep_deposit", deserialize_with = "Option::deserialize")]
    pub drep_deposit: Option<String>,
    /// DRep activity period. New in 13.2-Conway.
    #[serde(rename = "drep_activity", deserialize_with = "Option::deserialize")]
    pub drep_activity: Option<String>,
    /// Pool Voting threshold for security-relevant protocol parameters changes. Renamed to pvt_p_p_security_group.
    #[serde(rename = "pvtpp_security_group", deserialize_with = "Option::deserialize")]
    pub pvtpp_security_group: Option<f64>,
    /// Pool Voting threshold for security-relevant protocol parameters changes.
    #[serde(rename = "pvt_p_p_security_group", deserialize_with = "Option::deserialize")]
    pub pvt_p_p_security_group: Option<f64>,
    #[serde(rename = "min_fee_ref_script_cost_per_byte", deserialize_with = "Option::deserialize")]
    pub min_fee_ref_script_cost_per_byte: Option<f64>,
}

impl ProposalParametersParameters {
    pub fn new(min_fee_a: Option<i32>, min_fee_b: Option<i32>, max_block_size: Option<i32>, max_tx_size: Option<i32>, max_block_header_size: Option<i32>, key_deposit: Option<String>, pool_deposit: Option<String>, e_max: Option<i32>, n_opt: Option<i32>, a0: Option<f64>, rho: Option<f64>, tau: Option<f64>, decentralisation_param: Option<f64>, extra_entropy: Option<String>, protocol_major_ver: Option<i32>, protocol_minor_ver: Option<i32>, min_utxo: Option<String>, min_pool_cost: Option<String>, cost_models: Option<std::collections::HashMap<String, serde_json::Value>>, price_mem: Option<f64>, price_step: Option<f64>, max_tx_ex_mem: Option<String>, max_tx_ex_steps: Option<String>, max_block_ex_mem: Option<String>, max_block_ex_steps: Option<String>, max_val_size: Option<String>, collateral_percent: Option<i32>, max_collateral_inputs: Option<i32>, coins_per_utxo_size: Option<String>, coins_per_utxo_word: Option<String>, pvt_motion_no_confidence: Option<f64>, pvt_committee_normal: Option<f64>, pvt_committee_no_confidence: Option<f64>, pvt_hard_fork_initiation: Option<f64>, dvt_motion_no_confidence: Option<f64>, dvt_committee_normal: Option<f64>, dvt_committee_no_confidence: Option<f64>, dvt_update_to_constitution: Option<f64>, dvt_hard_fork_initiation: Option<f64>, dvt_p_p_network_group: Option<f64>, dvt_p_p_economic_group: Option<f64>, dvt_p_p_technical_group: Option<f64>, dvt_p_p_gov_group: Option<f64>, dvt_treasury_withdrawal: Option<f64>, committee_min_size: Option<String>, committee_max_term_length: Option<String>, gov_action_lifetime: Option<String>, gov_action_deposit: Option<String>, drep_deposit: Option<String>, drep_activity: Option<String>, pvtpp_security_group: Option<f64>, pvt_p_p_security_group: Option<f64>, min_fee_ref_script_cost_per_byte: Option<f64>) -> ProposalParametersParameters {
        ProposalParametersParameters {
            epoch: None,
            min_fee_a,
            min_fee_b,
            max_block_size,
            max_tx_size,
            max_block_header_size,
            key_deposit,
            pool_deposit,
            e_max,
            n_opt,
            a0,
            rho,
            tau,
            decentralisation_param,
            extra_entropy,
            protocol_major_ver,
            protocol_minor_ver,
            min_utxo,
            min_pool_cost,
            cost_models,
            price_mem,
            price_step,
            max_tx_ex_mem,
            max_tx_ex_steps,
            max_block_ex_mem,
            max_block_ex_steps,
            max_val_size,
            collateral_percent,
            max_collateral_inputs,
            coins_per_utxo_size,
            coins_per_utxo_word,
            pvt_motion_no_confidence,
            pvt_committee_normal,
            pvt_committee_no_confidence,
            pvt_hard_fork_initiation,
            dvt_motion_no_confidence,
            dvt_committee_normal,
            dvt_committee_no_confidence,
            dvt_update_to_constitution,
            dvt_hard_fork_initiation,
            dvt_p_p_network_group,
            dvt_p_p_economic_group,
            dvt_p_p_technical_group,
            dvt_p_p_gov_group,
            dvt_treasury_withdrawal,
            committee_min_size,
            committee_max_term_length,
            gov_action_lifetime,
            gov_action_deposit,
            drep_deposit,
            drep_activity,
            pvtpp_security_group,
            pvt_p_p_security_group,
            min_fee_ref_script_cost_per_byte,
        }
    }
}

