use crate::models;
use serde::{Deserialize, Serialize};

use serde_with::serde_as;

#[serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegisterSignerMessage {
    /// The unique identifier of the signer
    #[serde(rename = "party_id")]
    pub party_id: String,
    /// The public key used to authenticate signer signature
    #[serde_as(as = "serde_with::base64::Base64")]
    #[serde(rename = "verification_key")]
    pub verification_key: Vec<u8>,
    /// The signature of the verification_key (signed by the Cardano node KES secret key)
    #[serde_as(as = "Option<serde_with::base64::Base64>")]
    #[serde(rename = "verification_key_signature", skip_serializing_if = "Option::is_none")]
    pub verification_key_signature: Option<Vec<u8>>,
    /// The operational certificate of the stake pool operator attached to the signer node
    #[serde_as(as = "Option<serde_with::base64::Base64>")]
    #[serde(rename = "operational_certificate", skip_serializing_if = "Option::is_none")]
    pub operational_certificate: Option<Vec<u8>>,
    /// The number of updates of the KES secret key that signed the verification key
    #[serde(rename = "kes_period", skip_serializing_if = "Option::is_none")]
    pub kes_period: Option<i64>,
    /// Cardano chain epoch number
    #[serde(rename = "epoch", skip_serializing_if = "Option::is_none")]
    pub epoch: Option<i64>,
}

impl RegisterSignerMessage {
    pub fn new(party_id: String, verification_key: Vec<u8>) -> RegisterSignerMessage {
        RegisterSignerMessage {
            party_id,
            verification_key,
            verification_key_signature: None,
            operational_certificate: None,
            kes_period: None,
            epoch: None,
        }
    }
}

