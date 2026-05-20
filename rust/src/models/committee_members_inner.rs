use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitteeMembersInner {
    /// CIP-129 bech32 encoded cold credential (`cc_cold1...`).
    #[serde(rename = "cc_cold_id")]
    pub cc_cold_id: String,
    /// Hex of the raw 28-byte cold key/script hash.
    #[serde(rename = "cc_cold_hex")]
    pub cc_cold_hex: String,
    #[serde(rename = "cc_cold_has_script")]
    pub cc_cold_has_script: bool,
    /// CIP-129 bech32 encoded current hot credential (`cc_hot1...`). `null` unless `status` is `authorized`.
    #[serde(rename = "cc_hot_id", deserialize_with = "Option::deserialize")]
    pub cc_hot_id: Option<String>,
    #[serde(rename = "cc_hot_hex", deserialize_with = "Option::deserialize")]
    pub cc_hot_hex: Option<String>,
    #[serde(rename = "cc_hot_has_script", deserialize_with = "Option::deserialize")]
    pub cc_hot_has_script: Option<bool>,
    /// `authorized` — member has a currently active hot key. `not_authorized` — member has never authorized a hot key. `resigned` — member's most recent on-chain event is a resignation certificate.
    #[serde(rename = "status")]
    pub status: Status,
    /// Epoch at which this member's term expires.
    #[serde(rename = "expiration_epoch")]
    pub expiration_epoch: i32,
}

impl CommitteeMembersInner {
    pub fn new(cc_cold_id: String, cc_cold_hex: String, cc_cold_has_script: bool, cc_hot_id: Option<String>, cc_hot_hex: Option<String>, cc_hot_has_script: Option<bool>, status: Status, expiration_epoch: i32) -> CommitteeMembersInner {
        CommitteeMembersInner {
            cc_cold_id,
            cc_cold_hex,
            cc_cold_has_script,
            cc_hot_id,
            cc_hot_hex,
            cc_hot_has_script,
            status,
            expiration_epoch,
        }
    }
}
/// `authorized` — member has a currently active hot key. `not_authorized` — member has never authorized a hot key. `resigned` — member's most recent on-chain event is a resignation certificate.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "authorized")]
    Authorized,
    #[serde(rename = "not_authorized")]
    NotAuthorized,
    #[serde(rename = "resigned")]
    Resigned,
}

impl Default for Status {
    fn default() -> Status {
        Self::Authorized
    }
}

