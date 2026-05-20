use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TxContentUtxoOutputsInner {
    /// Output address
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "amount")]
    pub amount: Vec<models::TxContentOutputAmountInner>,
    /// UTXO index in the transaction
    #[serde(rename = "output_index")]
    pub output_index: i32,
    /// The hash of the transaction output datum
    #[serde(rename = "data_hash", deserialize_with = "Option::deserialize")]
    pub data_hash: Option<String>,
    /// CBOR encoded inline datum
    #[serde(rename = "inline_datum", deserialize_with = "Option::deserialize")]
    pub inline_datum: Option<String>,
    /// Whether the output is a collateral output
    #[serde(rename = "collateral")]
    pub collateral: bool,
    /// The hash of the reference script of the output
    #[serde(rename = "reference_script_hash", deserialize_with = "Option::deserialize")]
    pub reference_script_hash: Option<String>,
    /// Transaction hash that consumed the UTXO or null for unconsumed UTXOs. Always null for collateral outputs.
    #[serde(rename = "consumed_by_tx", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub consumed_by_tx: Option<Option<String>>,
}

impl TxContentUtxoOutputsInner {
    pub fn new(address: String, amount: Vec<models::TxContentOutputAmountInner>, output_index: i32, data_hash: Option<String>, inline_datum: Option<String>, collateral: bool, reference_script_hash: Option<String>) -> TxContentUtxoOutputsInner {
        TxContentUtxoOutputsInner {
            address,
            amount,
            output_index,
            data_hash,
            inline_datum,
            collateral,
            reference_script_hash,
            consumed_by_tx: None,
        }
    }
}

