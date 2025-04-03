/*
 * Blockfrost.io ~ API Documentation
 *
 * Blockfrost is an API as a service that allows users to interact with the Cardano blockchain and parts of its ecosystem.  ## Tokens  After signing up on https://blockfrost.io, a `project_id` token is automatically generated for each project. HTTP header of your request MUST include this `project_id` in order to authenticate against Blockfrost servers.  ## Available networks  At the moment, you can use the following networks. Please, note that each network has its own `project_id`.  <table>   <tbody>     <tr>       <td>           <b>Network</b>       </td>       <td>           <b>Endpoint</b>       </td>     </tr>     <tr>         <td>Cardano mainnet</td>         <td>             <code>https://cardano-mainnet.blockfrost.io/api/v0</code>         </td>     </tr>     <tr>         <td>Cardano preprod</td>         <td>             <code>https://cardano-preprod.blockfrost.io/api/v0</code>         </td>     </tr>     <tr>         <td>Cardano preview</td>         <td>             <code>https://cardano-preview.blockfrost.io/api/v0</code>         </td>     </tr>     <tr>         <td>InterPlanetary File System</td>         <td>             <code>https://ipfs.blockfrost.io/api/v0</code>         </td>     </tr>     <tr>         <td>Milkomeda mainnet</td>         <td>             <code>https://milkomeda-mainnet.blockfrost.io/api/v0</code>         </td>     </tr>     <tr>         <td>Milkomeda testnet</td>         <td>             <code>https://milkomeda-testnet.blockfrost.io/api/v0</code>         </td>     </tr>   </tbody> </table>  ## Milkomeda  <p>   <span>     For more information about how to use Milkomeda as well as the list of available endpoints, see the <a href=\"https://blockfrost.dev/start-building/milkomeda\" target=\"_blank\">Milkomeda section</a>.   </span> </p>   ## Concepts  * All endpoints return either a JSON object or an array. * Data is returned in *ascending* (oldest first, newest last) order, if not stated otherwise.   * You might use the `?order=desc` query parameter to reverse this order. * By default, we return 100 results at a time. You have to use `?page=2` to list through the results. * All time and timestamp related fields (except `server_time`) are in seconds of UNIX time. * All amounts are returned in Lovelaces, where 1 ADA = 1 000 000 Lovelaces. * Addresses, accounts and pool IDs are in Bech32 format. * All values are case sensitive. * All hex encoded values are lower case. * Examples are not based on real data. Any resemblance to actual events is purely coincidental. * We allow to upload files up to 100MB of size to IPFS. This might increase in the future. * Only pinned IPFS files are counted towards the IPFS quota. * Non-pinned IPFS files are subject to regular garbage collection and will be removed unless pinned. * We allow maximum of 100 queued pins per IPFS user.  ## Errors  ### HTTP Status codes  The following are HTTP status code your application might receive when reaching Blockfrost endpoints and it should handle all of these cases.  * HTTP `400` return code is used when the request is not valid. * HTTP `402` return code is used when the projects exceed their daily request limit. * HTTP `403` return code is used when the request is not authenticated. * HTTP `404` return code is used when the resource doesn't exist. * HTTP `418` return code is used when the user has been auto-banned for flooding too much after previously receiving error code `402` or `429`. * HTTP `425` return code is used in Cardano networks, when the user has submitted a transaction when the mempool is already full, not accepting new txs straight away. * HTTP `425` return code is used in IPFS network, when the user has submitted a pin when the pin queue is already full, not accepting new pins straight away. * HTTP `429` return code is used when the user has sent too many requests in a given amount of time and therefore has been rate-limited. * HTTP `500` return code is used when our endpoints are having a problem.  ### Error codes  An internal error code number is used for better indication of the error in question. It is passed using the following payload.  ```json {   \"status_code\": 403,   \"error\": \"Forbidden\",   \"message\": \"Invalid project token.\" } ``` ## Limits  There are two types of limits we are enforcing:  The first depends on your plan and is the number of request we allow per day. We defined the day from midnight to midnight of UTC time.  The second is rate limiting. We limit an end user, distinguished by IP address, to 10 requests per second. On top of that, we allow each user to send burst of 500 requests, which cools off at rate of 10 requests per second. In essence, a user is allowed to make another whole burst after (currently) 500/10 = 50 seconds. E.g. if a user attempts to make a call 3 seconds after whole burst, 30 requests will be processed. We believe this should be sufficient for most of the use cases. If it is not and you have a specific use case, please get in touch with us, and we will make sure to take it into account as much as we can.  ## SDKs  We support a number of SDKs that will help you in developing your application on top of Blockfrost.  <table>   <tbody>     <tr>         <td><b>Programming language</b></td>         <td><b>SDK</b></td>     </tr>     <tr>         <td>JavaScript</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-js\">blockfrost-js</a>         </td>     </tr>     <tr>         <td>Haskell</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-haskell\">blockfrost-haskell</a>         </td>     </tr>     <tr>         <td>Python</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-python\">blockfrost-python</a>         </td>     </tr>     <tr>         <td>Rust</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-rust\">blockfrost-rust</a>         </td>     </tr>     <tr>         <td>Golang</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-go\">blockfrost-go</a>         </td>     </tr>     <tr>         <td>Ruby</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-ruby\">blockfrost-ruby</a>         </td>     </tr>     <tr>         <td>Java</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-java\">blockfrost-java</a>         </td>     </tr>     <tr>         <td>Scala</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-scala\">blockfrost-scala</a>         </td>     </tr>     <tr>         <td>Swift</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-swift\">blockfrost-swift</a>         </td>     </tr>     <tr>         <td>Kotlin</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-kotlin\">blockfrost-kotlin</a>         </td>     </tr>     <tr>         <td>Elixir</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-elixir\">blockfrost-elixir</a>         </td>     </tr>     <tr>         <td>.NET</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-dotnet\">blockfrost-dotnet</a>         </td>     </tr>     <tr>         <td>Arduino</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-arduino\">blockfrost-arduino</a>         </td>     </tr>     <tr>         <td>PHP</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-php\">blockfrost-php</a>         </td>     </tr>     <tr>         <td>Crystal</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-crystal\">blockfrost-crystal</a>         </td>     </tr>   </tbody> </table> 
 *
 * The version of the OpenAPI document: 0.1.75
 * Contact: contact@blockfrost.io
 * Generated by: https://openapi-generator.tech
 */

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

