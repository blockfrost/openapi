use crate::models;
use serde::{Deserialize, Serialize};

use serde_with::serde_as;

/// RegisterSingleSignatureMessage : This message holds a Signer Single Signature with the list of won indexes in the lottery. 
#[serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegisterSingleSignatureMessage {
    /// Entity type of the message that is signed
    #[serde(rename = "entity_type")]
    pub entity_type: std::collections::HashMap<String, serde_json::Value>,
    /// The unique identifier of the signer
    #[serde(rename = "party_id")]
    pub party_id: String,
    /// The single signature of the digest
    #[serde_as(as = "serde_with::base64::Base64")]
    #[serde(rename = "signature")]
    pub signature: Vec<u8>,
    /// The indexes of the lottery won that lead to the single signature
    #[serde(rename = "indexes")]
    pub indexes: Vec<i64>,
}

impl RegisterSingleSignatureMessage {
    /// This message holds a Signer Single Signature with the list of won indexes in the lottery. 
    pub fn new(entity_type: std::collections::HashMap<String, serde_json::Value>, party_id: String, signature: Vec<u8>, indexes: Vec<i64>) -> RegisterSingleSignatureMessage {
        RegisterSingleSignatureMessage {
            entity_type,
            party_id,
            signature,
            indexes,
        }
    }
}

