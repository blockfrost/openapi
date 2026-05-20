use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CardanoTransactionProofMessageCertifiedTransactionsInner {
    #[serde(rename = "transactions_hashes")]
    pub transactions_hashes: Vec<String>,
    /// Proof for the Cardano transactions
    #[serde(rename = "proof")]
    pub proof: String,
}

impl CardanoTransactionProofMessageCertifiedTransactionsInner {
    pub fn new(transactions_hashes: Vec<String>, proof: String) -> CardanoTransactionProofMessageCertifiedTransactionsInner {
        CardanoTransactionProofMessageCertifiedTransactionsInner {
            transactions_hashes,
            proof,
        }
    }
}

