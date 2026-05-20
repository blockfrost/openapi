use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UtilsTxsEvaluateUtxosPostRequestAdditionalUtxoSetInnerInner {
    /// Transaction hash for the input
    #[serde(rename = "txId", skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<String>,
    /// Index of the output within the transaction
    #[serde(rename = "index", skip_serializing_if = "Option::is_none")]
    pub index: Option<f64>,
    /// Output address
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "value")]
    pub value: Box<models::UtilsTxsEvaluateUtxosPostRequestAdditionalUtxoSetInnerInnerAnyOf1Value>,
    #[serde(rename = "datum_hash", skip_serializing_if = "Option::is_none")]
    pub datum_hash: Option<String>,
    #[serde(rename = "datum", skip_serializing_if = "Option::is_none")]
    pub datum: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "script", skip_serializing_if = "Option::is_none")]
    pub script: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl UtilsTxsEvaluateUtxosPostRequestAdditionalUtxoSetInnerInner {
    pub fn new(address: String, value: models::UtilsTxsEvaluateUtxosPostRequestAdditionalUtxoSetInnerInnerAnyOf1Value) -> UtilsTxsEvaluateUtxosPostRequestAdditionalUtxoSetInnerInner {
        UtilsTxsEvaluateUtxosPostRequestAdditionalUtxoSetInnerInner {
            tx_id: None,
            index: None,
            address,
            value: Box::new(value),
            datum_hash: None,
            datum: None,
            script: None,
        }
    }
}

