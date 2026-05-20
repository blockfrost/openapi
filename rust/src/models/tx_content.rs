use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TxContent {
    /// Transaction hash
    #[serde(rename = "hash")]
    pub hash: String,
    /// Block hash
    #[serde(rename = "block")]
    pub block: String,
    /// Block number
    #[serde(rename = "block_height")]
    pub block_height: i32,
    /// Block creation time in UNIX time
    #[serde(rename = "block_time")]
    pub block_time: i32,
    /// Slot number
    #[serde(rename = "slot")]
    pub slot: i32,
    /// Transaction index within the block
    #[serde(rename = "index")]
    pub index: i32,
    #[serde(rename = "output_amount")]
    pub output_amount: Vec<models::TxContentOutputAmountInner>,
    /// Fees of the transaction in Lovelaces
    #[serde(rename = "fees")]
    pub fees: String,
    /// Deposit within the transaction in Lovelaces
    #[serde(rename = "deposit")]
    pub deposit: String,
    /// Size of the transaction in Bytes
    #[serde(rename = "size")]
    pub size: i32,
    /// Left (included) endpoint of the timelock validity intervals
    #[serde(rename = "invalid_before", deserialize_with = "Option::deserialize")]
    pub invalid_before: Option<String>,
    /// Right (excluded) endpoint of the timelock validity intervals
    #[serde(rename = "invalid_hereafter", deserialize_with = "Option::deserialize")]
    pub invalid_hereafter: Option<String>,
    /// Count of UTXOs within the transaction
    #[serde(rename = "utxo_count")]
    pub utxo_count: i32,
    /// Count of the withdrawals within the transaction
    #[serde(rename = "withdrawal_count")]
    pub withdrawal_count: i32,
    /// Count of the MIR certificates within the transaction
    #[serde(rename = "mir_cert_count")]
    pub mir_cert_count: i32,
    /// Count of the delegations within the transaction
    #[serde(rename = "delegation_count")]
    pub delegation_count: i32,
    /// Count of the stake keys (de)registration within the transaction
    #[serde(rename = "stake_cert_count")]
    pub stake_cert_count: i32,
    /// Count of the stake pool registration and update certificates within the transaction
    #[serde(rename = "pool_update_count")]
    pub pool_update_count: i32,
    /// Count of the stake pool retirement certificates within the transaction
    #[serde(rename = "pool_retire_count")]
    pub pool_retire_count: i32,
    /// Count of asset mints and burns within the transaction
    #[serde(rename = "asset_mint_or_burn_count")]
    pub asset_mint_or_burn_count: i32,
    /// Count of redeemers within the transaction
    #[serde(rename = "redeemer_count")]
    pub redeemer_count: i32,
    /// True if contract script passed validation
    #[serde(rename = "valid_contract")]
    pub valid_contract: bool,
    /// Treasury donation in Lovelaces
    #[serde(rename = "treasury_donation")]
    pub treasury_donation: String,
}

impl TxContent {
    pub fn new(hash: String, block: String, block_height: i32, block_time: i32, slot: i32, index: i32, output_amount: Vec<models::TxContentOutputAmountInner>, fees: String, deposit: String, size: i32, invalid_before: Option<String>, invalid_hereafter: Option<String>, utxo_count: i32, withdrawal_count: i32, mir_cert_count: i32, delegation_count: i32, stake_cert_count: i32, pool_update_count: i32, pool_retire_count: i32, asset_mint_or_burn_count: i32, redeemer_count: i32, valid_contract: bool, treasury_donation: String) -> TxContent {
        TxContent {
            hash,
            block,
            block_height,
            block_time,
            slot,
            index,
            output_amount,
            fees,
            deposit,
            size,
            invalid_before,
            invalid_hereafter,
            utxo_count,
            withdrawal_count,
            mir_cert_count,
            delegation_count,
            stake_cert_count,
            pool_update_count,
            pool_retire_count,
            asset_mint_or_burn_count,
            redeemer_count,
            valid_contract,
            treasury_donation,
        }
    }
}

