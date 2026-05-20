use crate::models;
use serde::{Deserialize, Serialize};

/// UtilsTxsEvaluateUtxosPostRequestAdditionalUtxoSetInnerInnerAnyOf1 : TxOut
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UtilsTxsEvaluateUtxosPostRequestAdditionalUtxoSetInnerInnerAnyOf1 {
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

impl UtilsTxsEvaluateUtxosPostRequestAdditionalUtxoSetInnerInnerAnyOf1 {
    /// TxOut
    pub fn new(address: String, value: models::UtilsTxsEvaluateUtxosPostRequestAdditionalUtxoSetInnerInnerAnyOf1Value) -> UtilsTxsEvaluateUtxosPostRequestAdditionalUtxoSetInnerInnerAnyOf1 {
        UtilsTxsEvaluateUtxosPostRequestAdditionalUtxoSetInnerInnerAnyOf1 {
            address,
            value: Box::new(value),
            datum_hash: None,
            datum: None,
            script: None,
        }
    }
}

