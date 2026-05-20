use crate::models;
use serde::{Deserialize, Serialize};

/// SignerRegistrationsMessage : This message holds the registered signers at a given epoch. 
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SignerRegistrationsMessage {
    /// Cardano chain epoch number
    #[serde(rename = "registered_at", skip_serializing_if = "Option::is_none")]
    pub registered_at: Option<i64>,
    /// Cardano chain epoch number
    #[serde(rename = "signing_at", skip_serializing_if = "Option::is_none")]
    pub signing_at: Option<i64>,
    #[serde(rename = "registrations", skip_serializing_if = "Option::is_none")]
    pub registrations: Option<Vec<models::SignerRegistrationsListItemMessage>>,
}

impl SignerRegistrationsMessage {
    /// This message holds the registered signers at a given epoch. 
    pub fn new() -> SignerRegistrationsMessage {
        SignerRegistrationsMessage {
            registered_at: None,
            signing_at: None,
            registrations: None,
        }
    }
}

