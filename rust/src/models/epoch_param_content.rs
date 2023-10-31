use serde::{Deserialize, Serialize};
/*
 * Blockfrost.io ~ API Documentation
 *
 * Blockfrost is an API as a service that allows users to interact with the Cardano blockchain and parts of its ecosystem.  ## Tokens  After signing up on https://blockfrost.io, a `project_id` token is automatically generated for each project. HTTP header of your request MUST include this `project_id` in order to authenticate against Blockfrost servers.  ## Available networks  At the moment, you can use the following networks. Please, note that each network has its own `project_id`.  <table>   <tbody>     <tr>       <td>           <b>Network</b>       </td>       <td>           <b>Endpoint</b>       </td>     </tr>     <tr>         <td>Cardano mainnet</td>         <td>             <code>https://cardano-mainnet.blockfrost.io/api/v0</code>         </td>     </tr>     <tr>         <td>Cardano preprod</td>         <td>             <code>https://cardano-preprod.blockfrost.io/api/v0</code>         </td>     </tr>     <tr>         <td>Cardano preview</td>         <td>             <code>https://cardano-preview.blockfrost.io/api/v0</code>         </td>     </tr>     <tr>         <td>InterPlanetary File System</td>         <td>             <code>https://ipfs.blockfrost.io/api/v0</code>         </td>     </tr>     <tr>         <td>Milkomeda mainnet</td>         <td>             <code>https://milkomeda-mainnet.blockfrost.io/api/v0</code>         </td>     </tr>     <tr>         <td>Milkomeda testnet</td>         <td>             <code>https://milkomeda-testnet.blockfrost.io/api/v0</code>         </td>     </tr>   </tbody> </table>  ## Milkomeda  <p>   <span>     For more information about how to use Milkomeda as well as the list of available endpoints, see the <a href=\"/start-building/milkomeda\">Milkomeda section</a>.   </span> </p>  ## Concepts  * All endpoints return either a JSON object or an array. * Data is returned in *ascending* (oldest first, newest last) order, if not stated otherwise.   * You might use the `?order=desc` query parameter to reverse this order. * By default, we return 100 results at a time. You have to use `?page=2` to list through the results. * All time and timestamp related fields (except `server_time`) are in seconds of UNIX time. * All amounts are returned in Lovelaces, where 1 ADA = 1 000 000 Lovelaces. * Addresses, accounts and pool IDs are in Bech32 format. * All values are case sensitive. * All hex encoded values are lower case. * Examples are not based on real data. Any resemblance to actual events is purely coincidental. * We allow to upload files up to 100MB of size to IPFS. This might increase in the future. * Only pinned IPFS files are counted towards the IPFS quota. * Non-pinned IPFS files are subject to regular garbage collection and will be removed unless pinned. * We allow maximum of 100 queued pins per IPFS user.  ## Errors  ### HTTP Status codes  The following are HTTP status code your application might receive when reaching Blockfrost endpoints and it should handle all of these cases.  * HTTP `400` return code is used when the request is not valid. * HTTP `402` return code is used when the projects exceed their daily request limit. * HTTP `403` return code is used when the request is not authenticated. * HTTP `404` return code is used when the resource doesn't exist. * HTTP `418` return code is used when the user has been auto-banned for flooding too much after previously receiving error code `402` or `429`. * HTTP `425` return code is used in Cardano networks, when the user has submitted a transaction when the mempool is already full, not accepting new txs straight away. * HTTP `425` return code is used in IPFS network, when the user has submitted a pin when the pin queue is already full, not accepting new pins straight away. * HTTP `429` return code is used when the user has sent too many requests in a given amount of time and therefore has been rate-limited. * HTTP `500` return code is used when our endpoints are having a problem.  ### Error codes  An internal error code number is used for better indication of the error in question. It is passed using the following payload.  ```json {   \"status_code\": 403,   \"error\": \"Forbidden\",   \"message\": \"Invalid project token.\" } ``` ## Limits  There are two types of limits we are enforcing:  The first depends on your plan and is the number of request we allow per day. We defined the day from midnight to midnight of UTC time.  The second is rate limiting. We limit an end user, distinguished by IP address, to 10 requests per second. On top of that, we allow each user to send burst of 500 requests, which cools off at rate of 10 requests per second. In essence, a user is allowed to make another whole burst after (currently) 500/10 = 50 seconds. E.g. if a user attempts to make a call 3 seconds after whole burst, 30 requests will be processed. We believe this should be sufficient for most of the use cases. If it is not and you have a specific use case, please get in touch with us, and we will make sure to take it into account as much as we can.  ## SDKs  We support a number of SDKs that will help you in developing your application on top of Blockfrost.  <table>   <tbody>     <tr>         <td><b>Programming language</b></td>         <td><b>SDK</b></td>     </tr>     <tr>         <td>JavaScript</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-js\">blockfrost-js</a>         </td>     </tr>     <tr>         <td>Haskell</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-haskell\">blockfrost-haskell</a>         </td>     </tr>     <tr>         <td>Python</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-python\">blockfrost-python</a>         </td>     </tr>     <tr>         <td>Rust</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-rust\">blockfrost-rust</a>         </td>     </tr>     <tr>         <td>Golang</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-go\">blockfrost-go</a>         </td>     </tr>     <tr>         <td>Ruby</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-ruby\">blockfrost-ruby</a>         </td>     </tr>     <tr>         <td>Java</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-java\">blockfrost-java</a>         </td>     </tr>     <tr>         <td>Scala</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-scala\">blockfrost-scala</a>         </td>     </tr>     <tr>         <td>Swift</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-swift\">blockfrost-swift</a>         </td>     </tr>     <tr>         <td>Kotlin</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-kotlin\">blockfrost-kotlin</a>         </td>     </tr>     <tr>         <td>Elixir</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-elixir\">blockfrost-elixir</a>         </td>     </tr>     <tr>         <td>.NET</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-dotnet\">blockfrost-dotnet</a>         </td>     </tr>     <tr>         <td>Arduino</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-arduino\">blockfrost-arduino</a>         </td>     </tr>     <tr>         <td>PHP</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-php\">blockfrost-php</a>         </td>     </tr>     <tr>         <td>Crystal</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-crystal\">blockfrost-crystal</a>         </td>     </tr>   </tbody> </table> 
 *
 * The version of the OpenAPI document: 0.1.60
 * Contact: contact@blockfrost.io
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EpochParamContent {
    /// Epoch number
    #[serde(rename = "epoch")]
    pub epoch: i32,
    /// The linear factor for the minimum fee calculation for given epoch
    #[serde(rename = "min_fee_a")]
    pub min_fee_a: i32,
    /// The constant factor for the minimum fee calculation
    #[serde(rename = "min_fee_b")]
    pub min_fee_b: i32,
    /// Maximum block body size in Bytes
    #[serde(rename = "max_block_size")]
    pub max_block_size: i32,
    /// Maximum transaction size
    #[serde(rename = "max_tx_size")]
    pub max_tx_size: i32,
    /// Maximum block header size
    #[serde(rename = "max_block_header_size")]
    pub max_block_header_size: i32,
    /// The amount of a key registration deposit in Lovelaces
    #[serde(rename = "key_deposit")]
    pub key_deposit: String,
    /// The amount of a pool registration deposit in Lovelaces
    #[serde(rename = "pool_deposit")]
    pub pool_deposit: String,
    /// Epoch bound on pool retirement
    #[serde(rename = "e_max")]
    pub e_max: i32,
    /// Desired number of pools
    #[serde(rename = "n_opt")]
    pub n_opt: i32,
    /// Pool pledge influence
    #[serde(rename = "a0")]
    pub a0: f32,
    /// Monetary expansion
    #[serde(rename = "rho")]
    pub rho: f32,
    /// Treasury expansion
    #[serde(rename = "tau")]
    pub tau: f32,
    /// Percentage of blocks produced by federated nodes
    #[serde(rename = "decentralisation_param")]
    pub decentralisation_param: f32,
    /// Seed for extra entropy
    #[serde(rename = "extra_entropy", deserialize_with = "Option::deserialize")]
    pub extra_entropy: Option<String>,
    /// Accepted protocol major version
    #[serde(rename = "protocol_major_ver")]
    pub protocol_major_ver: i32,
    /// Accepted protocol minor version
    #[serde(rename = "protocol_minor_ver")]
    pub protocol_minor_ver: i32,
    /// Minimum UTXO value
    #[serde(rename = "min_utxo")]
    pub min_utxo: String,
    /// Minimum stake cost forced on the pool
    #[serde(rename = "min_pool_cost")]
    pub min_pool_cost: String,
    /// Epoch number only used once
    #[serde(rename = "nonce")]
    pub nonce: String,
    /// Cost models parameters for Plutus Core scripts
    #[serde(rename = "cost_models", deserialize_with = "Option::deserialize")]
    pub cost_models: Option<::std::collections::HashMap<String, serde_json::Value>>,
    /// The per word cost of script memory usage
    #[serde(rename = "price_mem", deserialize_with = "Option::deserialize")]
    pub price_mem: Option<f32>,
    /// The cost of script execution step usage
    #[serde(rename = "price_step", deserialize_with = "Option::deserialize")]
    pub price_step: Option<f32>,
    /// The maximum number of execution memory allowed to be used in a single transaction
    #[serde(rename = "max_tx_ex_mem", deserialize_with = "Option::deserialize")]
    pub max_tx_ex_mem: Option<String>,
    /// The maximum number of execution steps allowed to be used in a single transaction
    #[serde(rename = "max_tx_ex_steps", deserialize_with = "Option::deserialize")]
    pub max_tx_ex_steps: Option<String>,
    /// The maximum number of execution memory allowed to be used in a single block
    #[serde(rename = "max_block_ex_mem", deserialize_with = "Option::deserialize")]
    pub max_block_ex_mem: Option<String>,
    /// The maximum number of execution steps allowed to be used in a single block
    #[serde(rename = "max_block_ex_steps", deserialize_with = "Option::deserialize")]
    pub max_block_ex_steps: Option<String>,
    /// The maximum Val size
    #[serde(rename = "max_val_size", deserialize_with = "Option::deserialize")]
    pub max_val_size: Option<String>,
    /// The percentage of the transactions fee which must be provided as collateral when including non-native scripts
    #[serde(rename = "collateral_percent", deserialize_with = "Option::deserialize")]
    pub collateral_percent: Option<i32>,
    /// The maximum number of collateral inputs allowed in a transaction
    #[serde(rename = "max_collateral_inputs", deserialize_with = "Option::deserialize")]
    pub max_collateral_inputs: Option<i32>,
    /// Cost per UTxO word for Alonzo. Cost per UTxO byte for Babbage and later.
    #[serde(rename = "coins_per_utxo_size", deserialize_with = "Option::deserialize")]
    pub coins_per_utxo_size: Option<String>,
    /// Cost per UTxO word for Alonzo. Cost per UTxO byte for Babbage and later.
    #[serde(rename = "coins_per_utxo_word", deserialize_with = "Option::deserialize")]
    pub coins_per_utxo_word: Option<String>,
}

impl EpochParamContent {
    pub fn new(epoch: i32, min_fee_a: i32, min_fee_b: i32, max_block_size: i32, max_tx_size: i32, max_block_header_size: i32, key_deposit: String, pool_deposit: String, e_max: i32, n_opt: i32, a0: f32, rho: f32, tau: f32, decentralisation_param: f32, extra_entropy: Option<String>, protocol_major_ver: i32, protocol_minor_ver: i32, min_utxo: String, min_pool_cost: String, nonce: String, cost_models: Option<::std::collections::HashMap<String, serde_json::Value>>, price_mem: Option<f32>, price_step: Option<f32>, max_tx_ex_mem: Option<String>, max_tx_ex_steps: Option<String>, max_block_ex_mem: Option<String>, max_block_ex_steps: Option<String>, max_val_size: Option<String>, collateral_percent: Option<i32>, max_collateral_inputs: Option<i32>, coins_per_utxo_size: Option<String>, coins_per_utxo_word: Option<String>) -> EpochParamContent {
        EpochParamContent {
            epoch,
            min_fee_a,
            min_fee_b,
            max_block_size,
            max_tx_size,
            max_block_header_size,
            key_deposit,
            pool_deposit,
            e_max,
            n_opt,
            a0,
            rho,
            tau,
            decentralisation_param,
            extra_entropy,
            protocol_major_ver,
            protocol_minor_ver,
            min_utxo,
            min_pool_cost,
            nonce,
            cost_models,
            price_mem,
            price_step,
            max_tx_ex_mem,
            max_tx_ex_steps,
            max_block_ex_mem,
            max_block_ex_steps,
            max_val_size,
            collateral_percent,
            max_collateral_inputs,
            coins_per_utxo_size,
            coins_per_utxo_word,
        }
    }
}


