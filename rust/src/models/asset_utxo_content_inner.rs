use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetUtxoContentInner {
    /// Bech32 encoded address holding the UTxO
    #[serde(rename = "address")]
    pub address: String,
    /// Transaction hash of the UTXO
    #[serde(rename = "tx_hash")]
    pub tx_hash: String,
    /// UTXO index in the transaction
    #[serde(rename = "output_index")]
    pub output_index: i32,
    #[serde(rename = "amount")]
    pub amount: Vec<models::TxContentOutputAmountInner>,
    /// Block hash of the UTXO
    #[serde(rename = "block")]
    pub block: String,
    /// Block number of the UTXO
    #[serde(rename = "block_height")]
    pub block_height: i32,
    /// UNIX time of the block
    #[serde(rename = "block_time")]
    pub block_time: i32,
    /// The hash of the transaction output datum
    #[serde(rename = "data_hash", deserialize_with = "Option::deserialize")]
    pub data_hash: Option<String>,
    /// CBOR encoded inline datum
    #[serde(rename = "inline_datum", deserialize_with = "Option::deserialize")]
    pub inline_datum: Option<String>,
    /// JSON representation of the inline datum
    #[serde(rename = "inline_datum_json", deserialize_with = "Option::deserialize")]
    pub inline_datum_json: Option<serde_json::Value>,
    /// The hash of the reference script of the output
    #[serde(rename = "reference_script_hash", deserialize_with = "Option::deserialize")]
    pub reference_script_hash: Option<String>,
}

impl AssetUtxoContentInner {
    pub fn new(address: String, tx_hash: String, output_index: i32, amount: Vec<models::TxContentOutputAmountInner>, block: String, block_height: i32, block_time: i32, data_hash: Option<String>, inline_datum: Option<String>, inline_datum_json: Option<serde_json::Value>, reference_script_hash: Option<String>) -> AssetUtxoContentInner {
        AssetUtxoContentInner {
            address,
            tx_hash,
            output_index,
            amount,
            block,
            block_height,
            block_time,
            data_hash,
            inline_datum,
            inline_datum_json,
            reference_script_hash,
        }
    }
}

