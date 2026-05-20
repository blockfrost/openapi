use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UtilsTxsEvaluateUtxosPostRequest {
    /// Transaction CBOR (encoded using base64 or base16).
    #[serde(rename = "cbor")]
    pub cbor: String,
    /// Additional UTXO as an array of tuples [TxIn, TxOut]. See https://ogmios.dev/mini-protocols/local-tx-submission/#additional-utxo-set.
    #[serde(rename = "additionalUtxoSet", skip_serializing_if = "Option::is_none")]
    pub additional_utxo_set: Option<Vec<Vec<models::UtilsTxsEvaluateUtxosPostRequestAdditionalUtxoSetInnerInner>>>,
}

impl UtilsTxsEvaluateUtxosPostRequest {
    pub fn new(cbor: String) -> UtilsTxsEvaluateUtxosPostRequest {
        UtilsTxsEvaluateUtxosPostRequest {
            cbor,
            additional_utxo_set: None,
        }
    }
}

