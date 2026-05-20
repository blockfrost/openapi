use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MempoolTxContent {
    #[serde(rename = "tx")]
    pub tx: Box<models::MempoolTxContentTx>,
    #[serde(rename = "inputs")]
    pub inputs: Vec<models::MempoolTxContentInputsInner>,
    #[serde(rename = "outputs")]
    pub outputs: Vec<models::MempoolTxContentOutputsInner>,
    #[serde(rename = "redeemers", skip_serializing_if = "Option::is_none")]
    pub redeemers: Option<Vec<models::MempoolTxContentRedeemersInner>>,
}

impl MempoolTxContent {
    pub fn new(tx: models::MempoolTxContentTx, inputs: Vec<models::MempoolTxContentInputsInner>, outputs: Vec<models::MempoolTxContentOutputsInner>) -> MempoolTxContent {
        MempoolTxContent {
            tx: Box::new(tx),
            inputs,
            outputs,
            redeemers: None,
        }
    }
}

