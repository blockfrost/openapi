use crate::models;
use serde::{Deserialize, Serialize};

/// AddressContentExtendedAmountInner : The sum of all the UTXO per asset
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressContentExtendedAmountInner {
    /// The unit of the value
    #[serde(rename = "unit")]
    pub unit: String,
    /// The quantity of the unit
    #[serde(rename = "quantity")]
    pub quantity: String,
    /// Number of decimal places of the asset unit. Primary data source is CIP68 reference NFT with a fallback to off-chain metadata.
    #[serde(rename = "decimals", deserialize_with = "Option::deserialize")]
    pub decimals: Option<i32>,
    /// True if the latest minting transaction includes metadata (best-effort)
    #[serde(rename = "has_nft_onchain_metadata")]
    pub has_nft_onchain_metadata: bool,
}

impl AddressContentExtendedAmountInner {
    /// The sum of all the UTXO per asset
    pub fn new(unit: String, quantity: String, decimals: Option<i32>, has_nft_onchain_metadata: bool) -> AddressContentExtendedAmountInner {
        AddressContentExtendedAmountInner {
            unit,
            quantity,
            decimals,
            has_nft_onchain_metadata,
        }
    }
}

