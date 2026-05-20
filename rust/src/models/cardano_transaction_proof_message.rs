use crate::models;
use serde::{Deserialize, Serialize};

/// CardanoTransactionProofMessage : This message represents proofs for Cardano Transactions.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CardanoTransactionProofMessage {
    /// Hash of the certificate that validate the merkle root of this proof
    #[serde(rename = "certificate_hash")]
    pub certificate_hash: String,
    /// Proofs for certified Cardano transactions
    #[serde(rename = "certified_transactions")]
    pub certified_transactions: Vec<models::CardanoTransactionProofMessageCertifiedTransactionsInner>,
    #[serde(rename = "non_certified_transactions")]
    pub non_certified_transactions: Vec<String>,
    /// Last block number
    #[serde(rename = "latest_block_number")]
    pub latest_block_number: i64,
}

impl CardanoTransactionProofMessage {
    /// This message represents proofs for Cardano Transactions.
    pub fn new(certificate_hash: String, certified_transactions: Vec<models::CardanoTransactionProofMessageCertifiedTransactionsInner>, non_certified_transactions: Vec<String>, latest_block_number: i64) -> CardanoTransactionProofMessage {
        CardanoTransactionProofMessage {
            certificate_hash,
            certified_transactions,
            non_certified_transactions,
            latest_block_number,
        }
    }
}

