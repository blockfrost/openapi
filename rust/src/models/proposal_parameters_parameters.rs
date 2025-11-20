/*
 * Blockfrost.io ~ API Documentation
 *
 * Blockfrost is an API as a service that allows users to interact with the Cardano blockchain and parts of its ecosystem.  ## Tokens  After signing up on https://blockfrost.io, a `project_id` token is automatically generated for each project. HTTP header of your request MUST include this `project_id` in order to authenticate against Blockfrost servers.  ## Available networks  At the moment, you can use the following networks. Please, note that each network has its own `project_id`.  <table>   <tbody>     <tr>       <td>           <b>Network</b>       </td>       <td>           <b>Endpoint</b>       </td>     </tr>     <tr>         <td>Cardano mainnet</td>         <td>             <code>https://cardano-mainnet.blockfrost.io/api/v0</code>         </td>     </tr>     <tr>         <td>Cardano preprod</td>         <td>             <code>https://cardano-preprod.blockfrost.io/api/v0</code>         </td>     </tr>     <tr>         <td>Cardano preview</td>         <td>             <code>https://cardano-preview.blockfrost.io/api/v0</code>         </td>     </tr>     <tr>         <td>InterPlanetary File System</td>         <td>             <code>https://ipfs.blockfrost.io/api/v0</code>         </td>     </tr>   </tbody> </table>  ## Concepts  * All endpoints return either a JSON object or an array. * Data is returned in *ascending* (oldest first, newest last) order, if not stated otherwise.   * You might use the `?order=desc` query parameter to reverse this order. * By default, we return 100 results at a time. You have to use `?page=2` to list through the results. * All time and timestamp related fields (except `server_time`) are in seconds of UNIX time. * All amounts are returned in Lovelaces, where 1 ADA = 1 000 000 Lovelaces. * Addresses, accounts and pool IDs are in Bech32 format. * All values are case sensitive. * All hex encoded values are lower case. * Examples are not based on real data. Any resemblance to actual events is purely coincidental. * We allow to upload files up to 100MB of size to IPFS. This might increase in the future. * Only pinned IPFS files are counted towards the IPFS quota. * Non-pinned IPFS files are subject to regular garbage collection and will be removed unless pinned. * We allow maximum of 100 queued pins per IPFS user.  ## Errors  ### HTTP Status codes  The following are HTTP status code your application might receive when reaching Blockfrost endpoints and it should handle all of these cases.  * HTTP `400` return code is used when the request is not valid. * HTTP `402` return code is used when the projects exceed their daily request limit. * HTTP `403` return code is used when the request is not authenticated. * HTTP `404` return code is used when the resource doesn't exist. * HTTP `418` return code is used when the user has been auto-banned for flooding too much after previously receiving error code `402` or `429`. * HTTP `425` return code is used in Cardano networks, when the user has submitted a transaction when the mempool is already full, not accepting new txs straight away. * HTTP `425` return code is used in IPFS network, when the user has submitted a pin when the pin queue is already full, not accepting new pins straight away. * HTTP `429` return code is used when the user has sent too many requests in a given amount of time and therefore has been rate-limited. * HTTP `500` return code is used when our endpoints are having a problem.  ### Error codes  An internal error code number is used for better indication of the error in question. It is passed using the following payload.  ```json {   \"status_code\": 403,   \"error\": \"Forbidden\",   \"message\": \"Invalid project token.\" } ``` ## Limits  There are two types of limits we are enforcing:  The first depends on your plan and is the number of request we allow per day. We defined the day from midnight to midnight of UTC time.  The second is rate limiting. We limit an end user, distinguished by IP address, to 10 requests per second. On top of that, we allow each user to send burst of 500 requests, which cools off at rate of 10 requests per second. In essence, a user is allowed to make another whole burst after (currently) 500/10 = 50 seconds. E.g. if a user attempts to make a call 3 seconds after whole burst, 30 requests will be processed. We believe this should be sufficient for most of the use cases. If it is not and you have a specific use case, please get in touch with us, and we will make sure to take it into account as much as we can.  ## SDKs  We support a number of SDKs that will help you in developing your application on top of Blockfrost.  <table>   <tbody>     <tr>         <td><b>Programming language</b></td>         <td><b>SDK</b></td>     </tr>     <tr>         <td>JavaScript</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-js\">blockfrost-js</a>         </td>     </tr>     <tr>         <td>Haskell</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-haskell\">blockfrost-haskell</a>         </td>     </tr>     <tr>         <td>Python</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-python\">blockfrost-python</a>         </td>     </tr>     <tr>         <td>Rust</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-rust\">blockfrost-rust</a>         </td>     </tr>     <tr>         <td>Golang</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-go\">blockfrost-go</a>         </td>     </tr>     <tr>         <td>Ruby</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-ruby\">blockfrost-ruby</a>         </td>     </tr>     <tr>         <td>Java</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-java\">blockfrost-java</a>         </td>     </tr>     <tr>         <td>Scala</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-scala\">blockfrost-scala</a>         </td>     </tr>     <tr>         <td>Swift</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-swift\">blockfrost-swift</a>         </td>     </tr>     <tr>         <td>Kotlin</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-kotlin\">blockfrost-kotlin</a>         </td>     </tr>     <tr>         <td>Elixir</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-elixir\">blockfrost-elixir</a>         </td>     </tr>     <tr>         <td>.NET</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-dotnet\">blockfrost-dotnet</a>         </td>     </tr>     <tr>         <td>Arduino</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-arduino\">blockfrost-arduino</a>         </td>     </tr>     <tr>         <td>PHP</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-php\">blockfrost-php</a>         </td>     </tr>     <tr>         <td>Crystal</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-crystal\">blockfrost-crystal</a>         </td>     </tr>   </tbody> </table> 
 *
 * The version of the OpenAPI document: 0.1.82
 * Contact: contact@blockfrost.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProposalParametersParameters {
    /// Epoch number
    #[serde(rename = "epoch", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub epoch: Option<Option<i32>>,
    /// The linear factor for the minimum fee calculation for given epoch
    #[serde(rename = "min_fee_a", deserialize_with = "Option::deserialize")]
    pub min_fee_a: Option<i32>,
    /// The constant factor for the minimum fee calculation
    #[serde(rename = "min_fee_b", deserialize_with = "Option::deserialize")]
    pub min_fee_b: Option<i32>,
    /// Maximum block body size in Bytes
    #[serde(rename = "max_block_size", deserialize_with = "Option::deserialize")]
    pub max_block_size: Option<i32>,
    /// Maximum transaction size
    #[serde(rename = "max_tx_size", deserialize_with = "Option::deserialize")]
    pub max_tx_size: Option<i32>,
    /// Maximum block header size
    #[serde(rename = "max_block_header_size", deserialize_with = "Option::deserialize")]
    pub max_block_header_size: Option<i32>,
    /// The amount of a key registration deposit in Lovelaces
    #[serde(rename = "key_deposit", deserialize_with = "Option::deserialize")]
    pub key_deposit: Option<String>,
    /// The amount of a pool registration deposit in Lovelaces
    #[serde(rename = "pool_deposit", deserialize_with = "Option::deserialize")]
    pub pool_deposit: Option<String>,
    /// Epoch bound on pool retirement
    #[serde(rename = "e_max", deserialize_with = "Option::deserialize")]
    pub e_max: Option<i32>,
    /// Desired number of pools
    #[serde(rename = "n_opt", deserialize_with = "Option::deserialize")]
    pub n_opt: Option<i32>,
    /// Pool pledge influence
    #[serde(rename = "a0", deserialize_with = "Option::deserialize")]
    pub a0: Option<f64>,
    /// Monetary expansion
    #[serde(rename = "rho", deserialize_with = "Option::deserialize")]
    pub rho: Option<f64>,
    /// Treasury expansion
    #[serde(rename = "tau", deserialize_with = "Option::deserialize")]
    pub tau: Option<f64>,
    /// Percentage of blocks produced by federated nodes
    #[serde(rename = "decentralisation_param", deserialize_with = "Option::deserialize")]
    pub decentralisation_param: Option<f64>,
    /// Seed for extra entropy
    #[serde(rename = "extra_entropy", deserialize_with = "Option::deserialize")]
    pub extra_entropy: Option<String>,
    /// Accepted protocol major version
    #[serde(rename = "protocol_major_ver", deserialize_with = "Option::deserialize")]
    pub protocol_major_ver: Option<i32>,
    /// Accepted protocol minor version
    #[serde(rename = "protocol_minor_ver", deserialize_with = "Option::deserialize")]
    pub protocol_minor_ver: Option<i32>,
    /// Minimum UTXO value
    #[serde(rename = "min_utxo", deserialize_with = "Option::deserialize")]
    pub min_utxo: Option<String>,
    /// Minimum stake cost forced on the pool
    #[serde(rename = "min_pool_cost", deserialize_with = "Option::deserialize")]
    pub min_pool_cost: Option<String>,
    /// Cost models parameters for Plutus Core scripts in raw list form
    #[serde(rename = "cost_models", deserialize_with = "Option::deserialize")]
    pub cost_models: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// The per word cost of script memory usage
    #[serde(rename = "price_mem", deserialize_with = "Option::deserialize")]
    pub price_mem: Option<f64>,
    /// The cost of script execution step usage
    #[serde(rename = "price_step", deserialize_with = "Option::deserialize")]
    pub price_step: Option<f64>,
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
    /// Pool Voting threshold for motion of no-confidence. New in 13.2-Conway.
    #[serde(rename = "pvt_motion_no_confidence", deserialize_with = "Option::deserialize")]
    pub pvt_motion_no_confidence: Option<f64>,
    /// Pool Voting threshold for new committee/threshold (normal state). New in 13.2-Conway.
    #[serde(rename = "pvt_committee_normal", deserialize_with = "Option::deserialize")]
    pub pvt_committee_normal: Option<f64>,
    /// Pool Voting threshold for new committee/threshold (state of no-confidence). New in 13.2-Conway.
    #[serde(rename = "pvt_committee_no_confidence", deserialize_with = "Option::deserialize")]
    pub pvt_committee_no_confidence: Option<f64>,
    /// Pool Voting threshold for hard-fork initiation. New in 13.2-Conway.
    #[serde(rename = "pvt_hard_fork_initiation", deserialize_with = "Option::deserialize")]
    pub pvt_hard_fork_initiation: Option<f64>,
    /// DRep Vote threshold for motion of no-confidence. New in 13.2-Conway.
    #[serde(rename = "dvt_motion_no_confidence", deserialize_with = "Option::deserialize")]
    pub dvt_motion_no_confidence: Option<f64>,
    /// DRep Vote threshold for new committee/threshold (normal state). New in 13.2-Conway.
    #[serde(rename = "dvt_committee_normal", deserialize_with = "Option::deserialize")]
    pub dvt_committee_normal: Option<f64>,
    /// DRep Vote threshold for new committee/threshold (state of no-confidence). New in 13.2-Conway.
    #[serde(rename = "dvt_committee_no_confidence", deserialize_with = "Option::deserialize")]
    pub dvt_committee_no_confidence: Option<f64>,
    /// DRep Vote threshold for update to the Constitution. New in 13.2-Conway.
    #[serde(rename = "dvt_update_to_constitution", deserialize_with = "Option::deserialize")]
    pub dvt_update_to_constitution: Option<f64>,
    /// DRep Vote threshold for hard-fork initiation. New in 13.2-Conway.
    #[serde(rename = "dvt_hard_fork_initiation", deserialize_with = "Option::deserialize")]
    pub dvt_hard_fork_initiation: Option<f64>,
    /// DRep Vote threshold for protocol parameter changes, network group. New in 13.2-Conway.
    #[serde(rename = "dvt_p_p_network_group", deserialize_with = "Option::deserialize")]
    pub dvt_p_p_network_group: Option<f64>,
    /// DRep Vote threshold for protocol parameter changes, economic group. New in 13.2-Conway.
    #[serde(rename = "dvt_p_p_economic_group", deserialize_with = "Option::deserialize")]
    pub dvt_p_p_economic_group: Option<f64>,
    /// DRep Vote threshold for protocol parameter changes, technical group. New in 13.2-Conway.
    #[serde(rename = "dvt_p_p_technical_group", deserialize_with = "Option::deserialize")]
    pub dvt_p_p_technical_group: Option<f64>,
    /// DRep Vote threshold for protocol parameter changes, governance group. New in 13.2-Conway.
    #[serde(rename = "dvt_p_p_gov_group", deserialize_with = "Option::deserialize")]
    pub dvt_p_p_gov_group: Option<f64>,
    /// DRep Vote threshold for treasury withdrawal. New in 13.2-Conway.
    #[serde(rename = "dvt_treasury_withdrawal", deserialize_with = "Option::deserialize")]
    pub dvt_treasury_withdrawal: Option<f64>,
    /// Minimal constitutional committee size. New in 13.2-Conway.
    #[serde(rename = "committee_min_size", deserialize_with = "Option::deserialize")]
    pub committee_min_size: Option<String>,
    /// Constitutional committee term limits. New in 13.2-Conway.
    #[serde(rename = "committee_max_term_length", deserialize_with = "Option::deserialize")]
    pub committee_max_term_length: Option<String>,
    /// Governance action expiration. New in 13.2-Conway.
    #[serde(rename = "gov_action_lifetime", deserialize_with = "Option::deserialize")]
    pub gov_action_lifetime: Option<String>,
    /// Governance action deposit. New in 13.2-Conway.
    #[serde(rename = "gov_action_deposit", deserialize_with = "Option::deserialize")]
    pub gov_action_deposit: Option<String>,
    /// DRep deposit amount. New in 13.2-Conway.
    #[serde(rename = "drep_deposit", deserialize_with = "Option::deserialize")]
    pub drep_deposit: Option<String>,
    /// DRep activity period. New in 13.2-Conway.
    #[serde(rename = "drep_activity", deserialize_with = "Option::deserialize")]
    pub drep_activity: Option<String>,
    /// Pool Voting threshold for security-relevant protocol parameters changes. Renamed to pvt_p_p_security_group.
    #[serde(rename = "pvtpp_security_group", deserialize_with = "Option::deserialize")]
    pub pvtpp_security_group: Option<f64>,
    /// Pool Voting threshold for security-relevant protocol parameters changes.
    #[serde(rename = "pvt_p_p_security_group", deserialize_with = "Option::deserialize")]
    pub pvt_p_p_security_group: Option<f64>,
    #[serde(rename = "min_fee_ref_script_cost_per_byte", deserialize_with = "Option::deserialize")]
    pub min_fee_ref_script_cost_per_byte: Option<f64>,
}

impl ProposalParametersParameters {
    pub fn new(min_fee_a: Option<i32>, min_fee_b: Option<i32>, max_block_size: Option<i32>, max_tx_size: Option<i32>, max_block_header_size: Option<i32>, key_deposit: Option<String>, pool_deposit: Option<String>, e_max: Option<i32>, n_opt: Option<i32>, a0: Option<f64>, rho: Option<f64>, tau: Option<f64>, decentralisation_param: Option<f64>, extra_entropy: Option<String>, protocol_major_ver: Option<i32>, protocol_minor_ver: Option<i32>, min_utxo: Option<String>, min_pool_cost: Option<String>, cost_models: Option<std::collections::HashMap<String, serde_json::Value>>, price_mem: Option<f64>, price_step: Option<f64>, max_tx_ex_mem: Option<String>, max_tx_ex_steps: Option<String>, max_block_ex_mem: Option<String>, max_block_ex_steps: Option<String>, max_val_size: Option<String>, collateral_percent: Option<i32>, max_collateral_inputs: Option<i32>, coins_per_utxo_size: Option<String>, coins_per_utxo_word: Option<String>, pvt_motion_no_confidence: Option<f64>, pvt_committee_normal: Option<f64>, pvt_committee_no_confidence: Option<f64>, pvt_hard_fork_initiation: Option<f64>, dvt_motion_no_confidence: Option<f64>, dvt_committee_normal: Option<f64>, dvt_committee_no_confidence: Option<f64>, dvt_update_to_constitution: Option<f64>, dvt_hard_fork_initiation: Option<f64>, dvt_p_p_network_group: Option<f64>, dvt_p_p_economic_group: Option<f64>, dvt_p_p_technical_group: Option<f64>, dvt_p_p_gov_group: Option<f64>, dvt_treasury_withdrawal: Option<f64>, committee_min_size: Option<String>, committee_max_term_length: Option<String>, gov_action_lifetime: Option<String>, gov_action_deposit: Option<String>, drep_deposit: Option<String>, drep_activity: Option<String>, pvtpp_security_group: Option<f64>, pvt_p_p_security_group: Option<f64>, min_fee_ref_script_cost_per_byte: Option<f64>) -> ProposalParametersParameters {
        ProposalParametersParameters {
            epoch: None,
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
            pvt_motion_no_confidence,
            pvt_committee_normal,
            pvt_committee_no_confidence,
            pvt_hard_fork_initiation,
            dvt_motion_no_confidence,
            dvt_committee_normal,
            dvt_committee_no_confidence,
            dvt_update_to_constitution,
            dvt_hard_fork_initiation,
            dvt_p_p_network_group,
            dvt_p_p_economic_group,
            dvt_p_p_technical_group,
            dvt_p_p_gov_group,
            dvt_treasury_withdrawal,
            committee_min_size,
            committee_max_term_length,
            gov_action_lifetime,
            gov_action_deposit,
            drep_deposit,
            drep_activity,
            pvtpp_security_group,
            pvt_p_p_security_group,
            min_fee_ref_script_cost_per_byte,
        }
    }
}

