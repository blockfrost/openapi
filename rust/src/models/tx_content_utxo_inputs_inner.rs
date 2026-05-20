use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TxContentUtxoInputsInner {
    /// Input address
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "amount")]
    pub amount: Vec<models::TxContentOutputAmountInner>,
    /// Hash of the UTXO transaction
    #[serde(rename = "tx_hash")]
    pub tx_hash: String,
    /// UTXO index in the transaction
    #[serde(rename = "output_index")]
    pub output_index: i32,
    /// The hash of the transaction output datum
    #[serde(rename = "data_hash", deserialize_with = "Option::deserialize")]
    pub data_hash: Option<String>,
    /// CBOR encoded inline datum
    #[serde(rename = "inline_datum", deserialize_with = "Option::deserialize")]
    pub inline_datum: Option<String>,
    /// The hash of the reference script of the input
    #[serde(rename = "reference_script_hash", deserialize_with = "Option::deserialize")]
    pub reference_script_hash: Option<String>,
    /// Whether the input is a collateral consumed on script validation failure
    #[serde(rename = "collateral")]
    pub collateral: bool,
    /// Whether the input is a reference transaction input
    #[serde(rename = "reference", skip_serializing_if = "Option::is_none")]
    pub reference: Option<bool>,
}

impl TxContentUtxoInputsInner {
    pub fn new(address: String, amount: Vec<models::TxContentOutputAmountInner>, tx_hash: String, output_index: i32, data_hash: Option<String>, inline_datum: Option<String>, reference_script_hash: Option<String>, collateral: bool) -> TxContentUtxoInputsInner {
        TxContentUtxoInputsInner {
            address,
            amount,
            tx_hash,
            output_index,
            data_hash,
            inline_datum,
            reference_script_hash,
            collateral,
            reference: None,
        }
    }
}

