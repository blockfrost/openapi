/*
 * Blockfrost.io ~ API Documentation
 *
 * Blockfrost is an API as a service that allows users to interact with the Cardano blockchain and parts of its ecosystem.  ## Tokens  After signing up on https://blockfrost.io, a `project_id` token is automatically generated for each project. HTTP header of your request MUST include this `project_id` in order to authenticate against Blockfrost servers.  ## Available networks  At the moment, you can use the following networks. Please, note that each network has its own `project_id`.  <table>   <tbody>     <tr>       <td>           <b>Network</b>       </td>       <td>           <b>Endpoint</b>       </td>     </tr>     <tr>         <td>Cardano mainnet</td>         <td>             <code>https://cardano-mainnet.blockfrost.io/api/v0</code>         </td>     </tr>     <tr>         <td>Cardano preprod</td>         <td>             <code>https://cardano-preprod.blockfrost.io/api/v0</code>         </td>     </tr>     <tr>         <td>Cardano preview</td>         <td>             <code>https://cardano-preview.blockfrost.io/api/v0</code>         </td>     </tr>     <tr>         <td>InterPlanetary File System</td>         <td>             <code>https://ipfs.blockfrost.io/api/v0</code>         </td>     </tr>     <tr>         <td>Milkomeda mainnet</td>         <td>             <code>https://milkomeda-mainnet.blockfrost.io/api/v0</code>         </td>     </tr>     <tr>         <td>Milkomeda testnet</td>         <td>             <code>https://milkomeda-testnet.blockfrost.io/api/v0</code>         </td>     </tr>   </tbody> </table>  ## Milkomeda  <p>   <span>     For more information about how to use Milkomeda as well as the list of available endpoints, see the <a href=\"https://blockfrost.dev/start-building/milkomeda\" target=\"_blank\">Milkomeda section</a>.   </span> </p>   ## Concepts  * All endpoints return either a JSON object or an array. * Data is returned in *ascending* (oldest first, newest last) order, if not stated otherwise.   * You might use the `?order=desc` query parameter to reverse this order. * By default, we return 100 results at a time. You have to use `?page=2` to list through the results. * All time and timestamp related fields (except `server_time`) are in seconds of UNIX time. * All amounts are returned in Lovelaces, where 1 ADA = 1 000 000 Lovelaces. * Addresses, accounts and pool IDs are in Bech32 format. * All values are case sensitive. * All hex encoded values are lower case. * Examples are not based on real data. Any resemblance to actual events is purely coincidental. * We allow to upload files up to 100MB of size to IPFS. This might increase in the future. * Only pinned IPFS files are counted towards the IPFS quota. * Non-pinned IPFS files are subject to regular garbage collection and will be removed unless pinned. * We allow maximum of 100 queued pins per IPFS user.  ## Errors  ### HTTP Status codes  The following are HTTP status code your application might receive when reaching Blockfrost endpoints and it should handle all of these cases.  * HTTP `400` return code is used when the request is not valid. * HTTP `402` return code is used when the projects exceed their daily request limit. * HTTP `403` return code is used when the request is not authenticated. * HTTP `404` return code is used when the resource doesn't exist. * HTTP `418` return code is used when the user has been auto-banned for flooding too much after previously receiving error code `402` or `429`. * HTTP `425` return code is used in Cardano networks, when the user has submitted a transaction when the mempool is already full, not accepting new txs straight away. * HTTP `425` return code is used in IPFS network, when the user has submitted a pin when the pin queue is already full, not accepting new pins straight away. * HTTP `429` return code is used when the user has sent too many requests in a given amount of time and therefore has been rate-limited. * HTTP `500` return code is used when our endpoints are having a problem.  ### Error codes  An internal error code number is used for better indication of the error in question. It is passed using the following payload.  ```json {   \"status_code\": 403,   \"error\": \"Forbidden\",   \"message\": \"Invalid project token.\" } ``` ## Limits  There are two types of limits we are enforcing:  The first depends on your plan and is the number of request we allow per day. We defined the day from midnight to midnight of UTC time.  The second is rate limiting. We limit an end user, distinguished by IP address, to 10 requests per second. On top of that, we allow each user to send burst of 500 requests, which cools off at rate of 10 requests per second. In essence, a user is allowed to make another whole burst after (currently) 500/10 = 50 seconds. E.g. if a user attempts to make a call 3 seconds after whole burst, 30 requests will be processed. We believe this should be sufficient for most of the use cases. If it is not and you have a specific use case, please get in touch with us, and we will make sure to take it into account as much as we can.  ## SDKs  We support a number of SDKs that will help you in developing your application on top of Blockfrost.  <table>   <tbody>     <tr>         <td><b>Programming language</b></td>         <td><b>SDK</b></td>     </tr>     <tr>         <td>JavaScript</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-js\">blockfrost-js</a>         </td>     </tr>     <tr>         <td>Haskell</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-haskell\">blockfrost-haskell</a>         </td>     </tr>     <tr>         <td>Python</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-python\">blockfrost-python</a>         </td>     </tr>     <tr>         <td>Rust</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-rust\">blockfrost-rust</a>         </td>     </tr>     <tr>         <td>Golang</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-go\">blockfrost-go</a>         </td>     </tr>     <tr>         <td>Ruby</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-ruby\">blockfrost-ruby</a>         </td>     </tr>     <tr>         <td>Java</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-java\">blockfrost-java</a>         </td>     </tr>     <tr>         <td>Scala</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-scala\">blockfrost-scala</a>         </td>     </tr>     <tr>         <td>Swift</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-swift\">blockfrost-swift</a>         </td>     </tr>     <tr>         <td>Kotlin</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-kotlin\">blockfrost-kotlin</a>         </td>     </tr>     <tr>         <td>Elixir</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-elixir\">blockfrost-elixir</a>         </td>     </tr>     <tr>         <td>.NET</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-dotnet\">blockfrost-dotnet</a>         </td>     </tr>     <tr>         <td>Arduino</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-arduino\">blockfrost-arduino</a>         </td>     </tr>     <tr>         <td>PHP</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-php\">blockfrost-php</a>         </td>     </tr>     <tr>         <td>Crystal</td>         <td>             <a href=\"https://github.com/blockfrost/blockfrost-crystal\">blockfrost-crystal</a>         </td>     </tr>   </tbody> </table> 
 *
 * The version of the OpenAPI document: 0.1.69
 * Contact: contact@blockfrost.io
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`governance_dreps_drep_id_delegators_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GovernanceDrepsDrepIdDelegatorsGetError {
    Status400(models::Get400Response),
    Status403(models::Get403Response),
    Status418(models::Get418Response),
    Status429(models::Get429Response),
    Status500(models::Get500Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`governance_dreps_drep_id_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GovernanceDrepsDrepIdGetError {
    Status400(models::Get400Response),
    Status403(models::Get403Response),
    Status418(models::Get418Response),
    Status429(models::Get429Response),
    Status500(models::Get500Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`governance_dreps_drep_id_metadata_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GovernanceDrepsDrepIdMetadataGetError {
    Status400(models::Get400Response),
    Status403(models::Get403Response),
    Status418(models::Get418Response),
    Status429(models::Get429Response),
    Status500(models::Get500Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`governance_dreps_drep_id_updates_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GovernanceDrepsDrepIdUpdatesGetError {
    Status400(models::Get400Response),
    Status403(models::Get403Response),
    Status418(models::Get418Response),
    Status429(models::Get429Response),
    Status500(models::Get500Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`governance_dreps_drep_id_votes_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GovernanceDrepsDrepIdVotesGetError {
    Status400(models::Get400Response),
    Status403(models::Get403Response),
    Status418(models::Get418Response),
    Status429(models::Get429Response),
    Status500(models::Get500Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`governance_dreps_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GovernanceDrepsGetError {
    Status400(models::Get400Response),
    Status403(models::Get403Response),
    Status418(models::Get418Response),
    Status429(models::Get429Response),
    Status500(models::Get500Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`governance_proposals_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GovernanceProposalsGetError {
    Status400(models::Get400Response),
    Status403(models::Get403Response),
    Status418(models::Get418Response),
    Status429(models::Get429Response),
    Status500(models::Get500Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`governance_proposals_tx_hash_cert_index_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GovernanceProposalsTxHashCertIndexGetError {
    Status400(models::Get400Response),
    Status403(models::Get403Response),
    Status418(models::Get418Response),
    Status429(models::Get429Response),
    Status500(models::Get500Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`governance_proposals_tx_hash_cert_index_metadata_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GovernanceProposalsTxHashCertIndexMetadataGetError {
    Status400(models::Get400Response),
    Status403(models::Get403Response),
    Status418(models::Get418Response),
    Status429(models::Get429Response),
    Status500(models::Get500Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`governance_proposals_tx_hash_cert_index_parameters_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GovernanceProposalsTxHashCertIndexParametersGetError {
    Status400(models::Get400Response),
    Status403(models::Get403Response),
    Status404(models::BlocksLatestGet404Response),
    Status418(models::Get418Response),
    Status429(models::Get429Response),
    Status500(models::Get500Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`governance_proposals_tx_hash_cert_index_votes_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GovernanceProposalsTxHashCertIndexVotesGetError {
    Status400(models::Get400Response),
    Status403(models::Get403Response),
    Status418(models::Get418Response),
    Status429(models::Get429Response),
    Status500(models::Get500Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`governance_proposals_tx_hash_cert_index_withdrawals_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GovernanceProposalsTxHashCertIndexWithdrawalsGetError {
    Status400(models::Get400Response),
    Status403(models::Get403Response),
    Status404(models::BlocksLatestGet404Response),
    Status418(models::Get418Response),
    Status429(models::Get429Response),
    Status500(models::Get500Response),
    UnknownValue(serde_json::Value),
}


/// List of Drep delegators.
pub async fn governance_dreps_drep_id_delegators_get(configuration: &configuration::Configuration, drep_id: &str, count: Option<i32>, page: Option<i32>, order: Option<&str>) -> Result<Vec<models::DrepDelegatorsInner>, Error<GovernanceDrepsDrepIdDelegatorsGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/governance/dreps/{drep_id}/delegators", local_var_configuration.base_path, drep_id=crate::apis::urlencode(drep_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = count {
        local_var_req_builder = local_var_req_builder.query(&[("count", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = order {
        local_var_req_builder = local_var_req_builder.query(&[("order", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("project_id", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GovernanceDrepsDrepIdDelegatorsGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// DRep information.
pub async fn governance_dreps_drep_id_get(configuration: &configuration::Configuration, drep_id: &str) -> Result<models::Drep, Error<GovernanceDrepsDrepIdGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/governance/dreps/{drep_id}", local_var_configuration.base_path, drep_id=crate::apis::urlencode(drep_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("project_id", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GovernanceDrepsDrepIdGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// DRep metadata information.
pub async fn governance_dreps_drep_id_metadata_get(configuration: &configuration::Configuration, drep_id: &str) -> Result<models::DrepMetadata, Error<GovernanceDrepsDrepIdMetadataGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/governance/dreps/{drep_id}/metadata", local_var_configuration.base_path, drep_id=crate::apis::urlencode(drep_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("project_id", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GovernanceDrepsDrepIdMetadataGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List of certificate updates to the DRep.
pub async fn governance_dreps_drep_id_updates_get(configuration: &configuration::Configuration, drep_id: &str, count: Option<i32>, page: Option<i32>, order: Option<&str>) -> Result<Vec<models::DrepUpdatesInner>, Error<GovernanceDrepsDrepIdUpdatesGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/governance/dreps/{drep_id}/updates", local_var_configuration.base_path, drep_id=crate::apis::urlencode(drep_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = count {
        local_var_req_builder = local_var_req_builder.query(&[("count", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = order {
        local_var_req_builder = local_var_req_builder.query(&[("order", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("project_id", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GovernanceDrepsDrepIdUpdatesGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// History of Drep votes.
pub async fn governance_dreps_drep_id_votes_get(configuration: &configuration::Configuration, drep_id: &str, count: Option<i32>, page: Option<i32>, order: Option<&str>) -> Result<Vec<models::DrepVotesInner>, Error<GovernanceDrepsDrepIdVotesGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/governance/dreps/{drep_id}/votes", local_var_configuration.base_path, drep_id=crate::apis::urlencode(drep_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = count {
        local_var_req_builder = local_var_req_builder.query(&[("count", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = order {
        local_var_req_builder = local_var_req_builder.query(&[("order", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("project_id", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GovernanceDrepsDrepIdVotesGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Return the information about Delegate Representatives (DReps)
pub async fn governance_dreps_get(configuration: &configuration::Configuration, count: Option<i32>, page: Option<i32>, order: Option<&str>) -> Result<Vec<models::DrepsInner>, Error<GovernanceDrepsGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/governance/dreps", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = count {
        local_var_req_builder = local_var_req_builder.query(&[("count", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = order {
        local_var_req_builder = local_var_req_builder.query(&[("order", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("project_id", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GovernanceDrepsGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Return the information about Proposals
pub async fn governance_proposals_get(configuration: &configuration::Configuration, count: Option<i32>, page: Option<i32>, order: Option<&str>) -> Result<Vec<models::ProposalsInner>, Error<GovernanceProposalsGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/governance/proposals", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = count {
        local_var_req_builder = local_var_req_builder.query(&[("count", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = order {
        local_var_req_builder = local_var_req_builder.query(&[("order", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("project_id", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GovernanceProposalsGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Proposal information.
pub async fn governance_proposals_tx_hash_cert_index_get(configuration: &configuration::Configuration, tx_hash: &str, cert_index: i32) -> Result<models::Proposal, Error<GovernanceProposalsTxHashCertIndexGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/governance/proposals/{tx_hash}/{cert_index}", local_var_configuration.base_path, tx_hash=crate::apis::urlencode(tx_hash), cert_index=cert_index);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("project_id", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GovernanceProposalsTxHashCertIndexGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Proposal metadata information.
pub async fn governance_proposals_tx_hash_cert_index_metadata_get(configuration: &configuration::Configuration, tx_hash: &str, cert_index: i32) -> Result<models::ProposalMetadata, Error<GovernanceProposalsTxHashCertIndexMetadataGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/governance/proposals/{tx_hash}/{cert_index}/metadata", local_var_configuration.base_path, tx_hash=crate::apis::urlencode(tx_hash), cert_index=cert_index);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("project_id", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GovernanceProposalsTxHashCertIndexMetadataGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Parameters proposal details.
pub async fn governance_proposals_tx_hash_cert_index_parameters_get(configuration: &configuration::Configuration, tx_hash: &str, cert_index: i32) -> Result<models::ProposalParameters, Error<GovernanceProposalsTxHashCertIndexParametersGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/governance/proposals/{tx_hash}/{cert_index}/parameters", local_var_configuration.base_path, tx_hash=crate::apis::urlencode(tx_hash), cert_index=cert_index);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("project_id", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GovernanceProposalsTxHashCertIndexParametersGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// History of Proposal votes.
pub async fn governance_proposals_tx_hash_cert_index_votes_get(configuration: &configuration::Configuration, tx_hash: &str, cert_index: i32, count: Option<i32>, page: Option<i32>, order: Option<&str>) -> Result<Vec<models::ProposalVotesInner>, Error<GovernanceProposalsTxHashCertIndexVotesGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/governance/proposals/{tx_hash}/{cert_index}/votes", local_var_configuration.base_path, tx_hash=crate::apis::urlencode(tx_hash), cert_index=cert_index);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = count {
        local_var_req_builder = local_var_req_builder.query(&[("count", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = order {
        local_var_req_builder = local_var_req_builder.query(&[("order", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("project_id", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GovernanceProposalsTxHashCertIndexVotesGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Parameters withdrawals details.
pub async fn governance_proposals_tx_hash_cert_index_withdrawals_get(configuration: &configuration::Configuration, tx_hash: &str, cert_index: i32) -> Result<Vec<models::ProposalWithdrawalsInner>, Error<GovernanceProposalsTxHashCertIndexWithdrawalsGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/governance/proposals/{tx_hash}/{cert_index}/withdrawals", local_var_configuration.base_path, tx_hash=crate::apis::urlencode(tx_hash), cert_index=cert_index);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("project_id", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GovernanceProposalsTxHashCertIndexWithdrawalsGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

