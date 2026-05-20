use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressUtxoContentInner {
    /// Bech32 encoded addresses - useful when querying by payment_cred
    #[serde(rename = "address")]
    pub address: String,
    /// Transaction hash of the UTXO
    #[serde(rename = "tx_hash")]
    pub tx_hash: String,
    /// UTXO index in the transaction
    #[serde(rename = "tx_index")]
    pub tx_index: i32,
    /// UTXO index in the transaction
    #[serde(rename = "output_index")]
    pub output_index: i32,
    #[serde(rename = "amount")]
    pub amount: Vec<models::TxContentOutputAmountInner>,
    /// Block hash of the UTXO
    #[serde(rename = "block")]
    pub block: String,
    /// The hash of the transaction output datum
    #[serde(rename = "data_hash", deserialize_with = "Option::deserialize")]
    pub data_hash: Option<String>,
    /// CBOR encoded inline datum
    #[serde(rename = "inline_datum", deserialize_with = "Option::deserialize")]
    pub inline_datum: Option<String>,
    /// The hash of the reference script of the output
    #[serde(rename = "reference_script_hash", deserialize_with = "Option::deserialize")]
    pub reference_script_hash: Option<String>,
}

impl AddressUtxoContentInner {
    pub fn new(address: String, tx_hash: String, tx_index: i32, output_index: i32, amount: Vec<models::TxContentOutputAmountInner>, block: String, data_hash: Option<String>, inline_datum: Option<String>, reference_script_hash: Option<String>) -> AddressUtxoContentInner {
        AddressUtxoContentInner {
            address,
            tx_hash,
            tx_index,
            output_index,
            amount,
            block,
            data_hash,
            inline_datum,
            reference_script_hash,
        }
    }
}

