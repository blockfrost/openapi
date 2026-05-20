use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Asset {
    /// Hex-encoded asset full name
    #[serde(rename = "asset")]
    pub asset: String,
    /// Policy ID of the asset
    #[serde(rename = "policy_id")]
    pub policy_id: String,
    /// Hex-encoded asset name of the asset
    #[serde(rename = "asset_name", deserialize_with = "Option::deserialize")]
    pub asset_name: Option<String>,
    /// CIP14 based user-facing fingerprint
    #[serde(rename = "fingerprint")]
    pub fingerprint: String,
    /// Current asset quantity
    #[serde(rename = "quantity")]
    pub quantity: String,
    /// ID of the initial minting transaction
    #[serde(rename = "initial_mint_tx_hash")]
    pub initial_mint_tx_hash: String,
    /// Count of mint and burn transactions
    #[serde(rename = "mint_or_burn_count")]
    pub mint_or_burn_count: i32,
    /// On-chain metadata which SHOULD adhere to the valid standards, based on which we perform the look up and display the asset (best effort) 
    #[serde(rename = "onchain_metadata", deserialize_with = "Option::deserialize")]
    pub onchain_metadata: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// If on-chain metadata passes validation, we display the standard under which it is valid 
    #[serde(rename = "onchain_metadata_standard", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub onchain_metadata_standard: Option<Option<OnchainMetadataStandard>>,
    /// Arbitrary plutus data (CIP68). 
    #[serde(rename = "onchain_metadata_extra", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub onchain_metadata_extra: Option<Option<String>>,
    #[serde(rename = "metadata", deserialize_with = "Option::deserialize")]
    pub metadata: Option<Box<models::AssetMetadata>>,
}

impl Asset {
    pub fn new(asset: String, policy_id: String, asset_name: Option<String>, fingerprint: String, quantity: String, initial_mint_tx_hash: String, mint_or_burn_count: i32, onchain_metadata: Option<std::collections::HashMap<String, serde_json::Value>>, metadata: Option<models::AssetMetadata>) -> Asset {
        Asset {
            asset,
            policy_id,
            asset_name,
            fingerprint,
            quantity,
            initial_mint_tx_hash,
            mint_or_burn_count,
            onchain_metadata,
            onchain_metadata_standard: None,
            onchain_metadata_extra: None,
            metadata: if let Some(x) = metadata {Some(Box::new(x))} else {None},
        }
    }
}
/// If on-chain metadata passes validation, we display the standard under which it is valid 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OnchainMetadataStandard {
    #[serde(rename = "CIP25v1")]
    Cip25v1,
    #[serde(rename = "CIP25v2")]
    Cip25v2,
    #[serde(rename = "CIP68v1")]
    Cip68v1,
    #[serde(rename = "CIP68v2")]
    Cip68v2,
    #[serde(rename = "CIP68v3")]
    Cip68v3,
}

impl Default for OnchainMetadataStandard {
    fn default() -> OnchainMetadataStandard {
        Self::Cip25v1
    }
}

