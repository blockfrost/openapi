use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UtilsTxsEvaluateUtxosPostRequestAdditionalUtxoSetInnerInnerAnyOf1Value {
    /// Lovelace amount
    #[serde(rename = "coins")]
    pub coins: f64,
    /// Assets amount
    #[serde(rename = "assets", skip_serializing_if = "Option::is_none")]
    pub assets: Option<std::collections::HashMap<String, f64>>,
}

impl UtilsTxsEvaluateUtxosPostRequestAdditionalUtxoSetInnerInnerAnyOf1Value {
    pub fn new(coins: f64) -> UtilsTxsEvaluateUtxosPostRequestAdditionalUtxoSetInnerInnerAnyOf1Value {
        UtilsTxsEvaluateUtxosPostRequestAdditionalUtxoSetInnerInnerAnyOf1Value {
            coins,
            assets: None,
        }
    }
}

