use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScriptUtxosInner {
    /// Bech32 encoded address of the UTXO holding the reference script
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
    /// The hash of the transaction output datum
    #[serde(rename = "data_hash", deserialize_with = "Option::deserialize")]
    pub data_hash: Option<String>,
    /// CBOR encoded inline datum
    #[serde(rename = "inline_datum", deserialize_with = "Option::deserialize")]
    pub inline_datum: Option<String>,
    /// The hash of the reference script of the output. Equals the queried script hash.
    #[serde(rename = "reference_script_hash")]
    pub reference_script_hash: String,
}

impl ScriptUtxosInner {
    pub fn new(address: String, tx_hash: String, output_index: i32, amount: Vec<models::TxContentOutputAmountInner>, block: String, data_hash: Option<String>, inline_datum: Option<String>, reference_script_hash: String) -> ScriptUtxosInner {
        ScriptUtxosInner {
            address,
            tx_hash,
            output_index,
            amount,
            block,
            data_hash,
            inline_datum,
            reference_script_hash,
        }
    }
}

