use crate::models;
use serde::{Deserialize, Serialize};

/// UtilsTxsEvaluateUtxosPostRequestAdditionalUtxoSetInnerInnerAnyOf : TxIn
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UtilsTxsEvaluateUtxosPostRequestAdditionalUtxoSetInnerInnerAnyOf {
    /// Transaction hash for the input
    #[serde(rename = "txId", skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<String>,
    /// Index of the output within the transaction
    #[serde(rename = "index", skip_serializing_if = "Option::is_none")]
    pub index: Option<f64>,
}

impl UtilsTxsEvaluateUtxosPostRequestAdditionalUtxoSetInnerInnerAnyOf {
    /// TxIn
    pub fn new() -> UtilsTxsEvaluateUtxosPostRequestAdditionalUtxoSetInnerInnerAnyOf {
        UtilsTxsEvaluateUtxosPostRequestAdditionalUtxoSetInnerInnerAnyOf {
            tx_id: None,
            index: None,
        }
    }
}

