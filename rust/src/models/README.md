# Rust API client for openapi

Blockfrost is an API as a service that allows users to interact with the Cardano blockchain and parts of its ecosystem.

## Tokens

After signing up on https://blockfrost.io, a `project_id` token is automatically generated for each project.
HTTP header of your request MUST include this `project_id` in order to authenticate against Blockfrost servers.

## Available networks

At the moment, you can use the following networks. Please, note that each network has its own `project_id`.

<table>
  <tbody>
    <tr>
      <td>
          <b>Network</b>
      </td>
      <td>
          <b>Endpoint</b>
      </td>
    </tr>
    <tr>
        <td>Cardano mainnet</td>
        <td>
            <code>https://cardano-mainnet.blockfrost.io/api/v0</code>
        </td>
    </tr>
    <tr>
        <td>Cardano preprod</td>
        <td>
            <code>https://cardano-preprod.blockfrost.io/api/v0</code>
        </td>
    </tr>
    <tr>
        <td>Cardano preview</td>
        <td>
            <code>https://cardano-preview.blockfrost.io/api/v0</code>
        </td>
    </tr>
    <tr>
        <td>InterPlanetary File System</td>
        <td>
            <code>https://ipfs.blockfrost.io/api/v0</code>
        </td>
    </tr>
    <tr>
        <td>Milkomeda mainnet</td>
        <td>
            <code>https://milkomeda-mainnet.blockfrost.io/api/v0</code>
        </td>
    </tr>
    <tr>
        <td>Milkomeda testnet</td>
        <td>
            <code>https://milkomeda-testnet.blockfrost.io/api/v0</code>
        </td>
    </tr>
  </tbody>
</table>

## Milkomeda

<p>
  <span>
    For more information about how to use Milkomeda as well as the list of available endpoints, see the <a href=\"https://blockfrost.dev/start-building/milkomeda\" target=\"_blank\">Milkomeda section</a>.
  </span>
</p>


## Concepts

* All endpoints return either a JSON object or an array.
* Data is returned in *ascending* (oldest first, newest last) order, if not stated otherwise.
  * You might use the `?order=desc` query parameter to reverse this order.
* By default, we return 100 results at a time. You have to use `?page=2` to list through the results.
* All time and timestamp related fields (except `server_time`) are in seconds of UNIX time.
* All amounts are returned in Lovelaces, where 1 ADA = 1 000 000 Lovelaces.
* Addresses, accounts and pool IDs are in Bech32 format.
* All values are case sensitive.
* All hex encoded values are lower case.
* Examples are not based on real data. Any resemblance to actual events is purely coincidental.
* We allow to upload files up to 100MB of size to IPFS. This might increase in the future.
* Only pinned IPFS files are counted towards the IPFS quota.
* Non-pinned IPFS files are subject to regular garbage collection and will be removed unless pinned.
* We allow maximum of 100 queued pins per IPFS user.

## Errors

### HTTP Status codes

The following are HTTP status code your application might receive when reaching Blockfrost endpoints and
it should handle all of these cases.

* HTTP `400` return code is used when the request is not valid.
* HTTP `402` return code is used when the projects exceed their daily request limit.
* HTTP `403` return code is used when the request is not authenticated.
* HTTP `404` return code is used when the resource doesn't exist.
* HTTP `418` return code is used when the user has been auto-banned for flooding too much after previously receiving error code `402` or `429`.
* HTTP `425` return code is used in Cardano networks, when the user has submitted a transaction when the mempool is already full, not accepting new txs straight away.
* HTTP `425` return code is used in IPFS network, when the user has submitted a pin when the pin queue is already full, not accepting new pins straight away.
* HTTP `429` return code is used when the user has sent too many requests in a given amount of time and therefore has been rate-limited.
* HTTP `500` return code is used when our endpoints are having a problem.

### Error codes

An internal error code number is used for better indication of the error in question. It is passed using the following payload.

```json
{
  \"status_code\": 403,
  \"error\": \"Forbidden\",
  \"message\": \"Invalid project token.\"
}
```
## Limits

There are two types of limits we are enforcing:

The first depends on your plan and is the number of request we allow per day. We defined the day from midnight to midnight of UTC time.

The second is rate limiting. We limit an end user, distinguished by IP address, to 10 requests per second. On top of that, we allow
each user to send burst of 500 requests, which cools off at rate of 10 requests per second. In essence, a user is allowed to make another
whole burst after (currently) 500/10 = 50 seconds. E.g. if a user attempts to make a call 3 seconds after whole burst, 30 requests will be processed.
We believe this should be sufficient for most of the use cases. If it is not and you have a specific use case, please get in touch with us, and
we will make sure to take it into account as much as we can.

## SDKs

We support a number of SDKs that will help you in developing your application on top of Blockfrost.

<table>
  <tbody>
    <tr>
        <td><b>Programming language</b></td>
        <td><b>SDK</b></td>
    </tr>
    <tr>
        <td>JavaScript</td>
        <td>
            <a href=\"https://github.com/blockfrost/blockfrost-js\">blockfrost-js</a>
        </td>
    </tr>
    <tr>
        <td>Haskell</td>
        <td>
            <a href=\"https://github.com/blockfrost/blockfrost-haskell\">blockfrost-haskell</a>
        </td>
    </tr>
    <tr>
        <td>Python</td>
        <td>
            <a href=\"https://github.com/blockfrost/blockfrost-python\">blockfrost-python</a>
        </td>
    </tr>
    <tr>
        <td>Rust</td>
        <td>
            <a href=\"https://github.com/blockfrost/blockfrost-rust\">blockfrost-rust</a>
        </td>
    </tr>
    <tr>
        <td>Golang</td>
        <td>
            <a href=\"https://github.com/blockfrost/blockfrost-go\">blockfrost-go</a>
        </td>
    </tr>
    <tr>
        <td>Ruby</td>
        <td>
            <a href=\"https://github.com/blockfrost/blockfrost-ruby\">blockfrost-ruby</a>
        </td>
    </tr>
    <tr>
        <td>Java</td>
        <td>
            <a href=\"https://github.com/blockfrost/blockfrost-java\">blockfrost-java</a>
        </td>
    </tr>
    <tr>
        <td>Scala</td>
        <td>
            <a href=\"https://github.com/blockfrost/blockfrost-scala\">blockfrost-scala</a>
        </td>
    </tr>
    <tr>
        <td>Swift</td>
        <td>
            <a href=\"https://github.com/blockfrost/blockfrost-swift\">blockfrost-swift</a>
        </td>
    </tr>
    <tr>
        <td>Kotlin</td>
        <td>
            <a href=\"https://github.com/blockfrost/blockfrost-kotlin\">blockfrost-kotlin</a>
        </td>
    </tr>
    <tr>
        <td>Elixir</td>
        <td>
            <a href=\"https://github.com/blockfrost/blockfrost-elixir\">blockfrost-elixir</a>
        </td>
    </tr>
    <tr>
        <td>.NET</td>
        <td>
            <a href=\"https://github.com/blockfrost/blockfrost-dotnet\">blockfrost-dotnet</a>
        </td>
    </tr>
    <tr>
        <td>Arduino</td>
        <td>
            <a href=\"https://github.com/blockfrost/blockfrost-arduino\">blockfrost-arduino</a>
        </td>
    </tr>
    <tr>
        <td>PHP</td>
        <td>
            <a href=\"https://github.com/blockfrost/blockfrost-php\">blockfrost-php</a>
        </td>
    </tr>
    <tr>
        <td>Crystal</td>
        <td>
            <a href=\"https://github.com/blockfrost/blockfrost-crystal\">blockfrost-crystal</a>
        </td>
    </tr>
  </tbody>
</table>


For more information, please visit [https://blockfrost.io](https://blockfrost.io)

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 0.1.75
- Package version: 0.1.75
- Generator version: 7.12.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `openapi` and add the following to `Cargo.toml` under `[dependencies]`:

```
openapi = { path = "./openapi" }
```

## Documentation for API Endpoints

All URIs are relative to *https://cardano-mainnet.blockfrost.io/api/v0*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*CardanoAccountsApi* | [**accounts_stake_address_addresses_assets_get**](docs/CardanoAccountsApi.md#accounts_stake_address_addresses_assets_get) | **GET** /accounts/{stake_address}/addresses/assets | Assets associated with the account addresses
*CardanoAccountsApi* | [**accounts_stake_address_addresses_get**](docs/CardanoAccountsApi.md#accounts_stake_address_addresses_get) | **GET** /accounts/{stake_address}/addresses | Account associated addresses
*CardanoAccountsApi* | [**accounts_stake_address_addresses_total_get**](docs/CardanoAccountsApi.md#accounts_stake_address_addresses_total_get) | **GET** /accounts/{stake_address}/addresses/total | Detailed information about account associated addresses
*CardanoAccountsApi* | [**accounts_stake_address_delegations_get**](docs/CardanoAccountsApi.md#accounts_stake_address_delegations_get) | **GET** /accounts/{stake_address}/delegations | Account delegation history
*CardanoAccountsApi* | [**accounts_stake_address_get**](docs/CardanoAccountsApi.md#accounts_stake_address_get) | **GET** /accounts/{stake_address} | Specific account address
*CardanoAccountsApi* | [**accounts_stake_address_history_get**](docs/CardanoAccountsApi.md#accounts_stake_address_history_get) | **GET** /accounts/{stake_address}/history | Account history
*CardanoAccountsApi* | [**accounts_stake_address_mirs_get**](docs/CardanoAccountsApi.md#accounts_stake_address_mirs_get) | **GET** /accounts/{stake_address}/mirs | Account MIR history
*CardanoAccountsApi* | [**accounts_stake_address_registrations_get**](docs/CardanoAccountsApi.md#accounts_stake_address_registrations_get) | **GET** /accounts/{stake_address}/registrations | Account registration history
*CardanoAccountsApi* | [**accounts_stake_address_rewards_get**](docs/CardanoAccountsApi.md#accounts_stake_address_rewards_get) | **GET** /accounts/{stake_address}/rewards | Account reward history
*CardanoAccountsApi* | [**accounts_stake_address_utxos_get**](docs/CardanoAccountsApi.md#accounts_stake_address_utxos_get) | **GET** /accounts/{stake_address}/utxos | Account UTXOs
*CardanoAccountsApi* | [**accounts_stake_address_withdrawals_get**](docs/CardanoAccountsApi.md#accounts_stake_address_withdrawals_get) | **GET** /accounts/{stake_address}/withdrawals | Account withdrawal history
*CardanoAddressesApi* | [**addresses_address_extended_get**](docs/CardanoAddressesApi.md#addresses_address_extended_get) | **GET** /addresses/{address}/extended | Extended information of a specific address
*CardanoAddressesApi* | [**addresses_address_get**](docs/CardanoAddressesApi.md#addresses_address_get) | **GET** /addresses/{address} | Specific address
*CardanoAddressesApi* | [**addresses_address_total_get**](docs/CardanoAddressesApi.md#addresses_address_total_get) | **GET** /addresses/{address}/total | Address details
*CardanoAddressesApi* | [**addresses_address_transactions_get**](docs/CardanoAddressesApi.md#addresses_address_transactions_get) | **GET** /addresses/{address}/transactions | Address transactions
*CardanoAddressesApi* | [**addresses_address_txs_get**](docs/CardanoAddressesApi.md#addresses_address_txs_get) | **GET** /addresses/{address}/txs | Address txs
*CardanoAddressesApi* | [**addresses_address_utxos_asset_get**](docs/CardanoAddressesApi.md#addresses_address_utxos_asset_get) | **GET** /addresses/{address}/utxos/{asset} | Address UTXOs of a given asset
*CardanoAddressesApi* | [**addresses_address_utxos_get**](docs/CardanoAddressesApi.md#addresses_address_utxos_get) | **GET** /addresses/{address}/utxos | Address UTXOs
*CardanoAssetsApi* | [**assets_asset_addresses_get**](docs/CardanoAssetsApi.md#assets_asset_addresses_get) | **GET** /assets/{asset}/addresses | Asset addresses
*CardanoAssetsApi* | [**assets_asset_get**](docs/CardanoAssetsApi.md#assets_asset_get) | **GET** /assets/{asset} | Specific asset
*CardanoAssetsApi* | [**assets_asset_history_get**](docs/CardanoAssetsApi.md#assets_asset_history_get) | **GET** /assets/{asset}/history | Asset history
*CardanoAssetsApi* | [**assets_asset_transactions_get**](docs/CardanoAssetsApi.md#assets_asset_transactions_get) | **GET** /assets/{asset}/transactions | Asset transactions
*CardanoAssetsApi* | [**assets_asset_txs_get**](docs/CardanoAssetsApi.md#assets_asset_txs_get) | **GET** /assets/{asset}/txs | Asset txs
*CardanoAssetsApi* | [**assets_get**](docs/CardanoAssetsApi.md#assets_get) | **GET** /assets | Assets
*CardanoAssetsApi* | [**assets_policy_policy_id_get**](docs/CardanoAssetsApi.md#assets_policy_policy_id_get) | **GET** /assets/policy/{policy_id} | Assets of a specific policy
*CardanoBlocksApi* | [**blocks_epoch_epoch_number_slot_slot_number_get**](docs/CardanoBlocksApi.md#blocks_epoch_epoch_number_slot_slot_number_get) | **GET** /blocks/epoch/{epoch_number}/slot/{slot_number} | Specific block in a slot in an epoch
*CardanoBlocksApi* | [**blocks_hash_or_number_addresses_get**](docs/CardanoBlocksApi.md#blocks_hash_or_number_addresses_get) | **GET** /blocks/{hash_or_number}/addresses | Addresses affected in a specific block
*CardanoBlocksApi* | [**blocks_hash_or_number_get**](docs/CardanoBlocksApi.md#blocks_hash_or_number_get) | **GET** /blocks/{hash_or_number} | Specific block
*CardanoBlocksApi* | [**blocks_hash_or_number_next_get**](docs/CardanoBlocksApi.md#blocks_hash_or_number_next_get) | **GET** /blocks/{hash_or_number}/next | Listing of next blocks
*CardanoBlocksApi* | [**blocks_hash_or_number_previous_get**](docs/CardanoBlocksApi.md#blocks_hash_or_number_previous_get) | **GET** /blocks/{hash_or_number}/previous | Listing of previous blocks
*CardanoBlocksApi* | [**blocks_hash_or_number_txs_cbor_get**](docs/CardanoBlocksApi.md#blocks_hash_or_number_txs_cbor_get) | **GET** /blocks/{hash_or_number}/txs/cbor | Block transactions with CBOR data
*CardanoBlocksApi* | [**blocks_hash_or_number_txs_get**](docs/CardanoBlocksApi.md#blocks_hash_or_number_txs_get) | **GET** /blocks/{hash_or_number}/txs | Block transactions
*CardanoBlocksApi* | [**blocks_latest_get**](docs/CardanoBlocksApi.md#blocks_latest_get) | **GET** /blocks/latest | Latest block
*CardanoBlocksApi* | [**blocks_latest_txs_cbor_get**](docs/CardanoBlocksApi.md#blocks_latest_txs_cbor_get) | **GET** /blocks/latest/txs/cbor | Latest block transactions with CBOR data
*CardanoBlocksApi* | [**blocks_latest_txs_get**](docs/CardanoBlocksApi.md#blocks_latest_txs_get) | **GET** /blocks/latest/txs | Latest block transactions
*CardanoBlocksApi* | [**blocks_slot_slot_number_get**](docs/CardanoBlocksApi.md#blocks_slot_slot_number_get) | **GET** /blocks/slot/{slot_number} | Specific block in a slot
*CardanoEpochsApi* | [**epochs_latest_get**](docs/CardanoEpochsApi.md#epochs_latest_get) | **GET** /epochs/latest | Latest epoch
*CardanoEpochsApi* | [**epochs_latest_parameters_get**](docs/CardanoEpochsApi.md#epochs_latest_parameters_get) | **GET** /epochs/latest/parameters | Latest epoch protocol parameters
*CardanoEpochsApi* | [**epochs_number_blocks_get**](docs/CardanoEpochsApi.md#epochs_number_blocks_get) | **GET** /epochs/{number}/blocks | Block distribution
*CardanoEpochsApi* | [**epochs_number_blocks_pool_id_get**](docs/CardanoEpochsApi.md#epochs_number_blocks_pool_id_get) | **GET** /epochs/{number}/blocks/{pool_id} | Block distribution by pool
*CardanoEpochsApi* | [**epochs_number_get**](docs/CardanoEpochsApi.md#epochs_number_get) | **GET** /epochs/{number} | Specific epoch
*CardanoEpochsApi* | [**epochs_number_next_get**](docs/CardanoEpochsApi.md#epochs_number_next_get) | **GET** /epochs/{number}/next | Listing of next epochs
*CardanoEpochsApi* | [**epochs_number_parameters_get**](docs/CardanoEpochsApi.md#epochs_number_parameters_get) | **GET** /epochs/{number}/parameters | Protocol parameters
*CardanoEpochsApi* | [**epochs_number_previous_get**](docs/CardanoEpochsApi.md#epochs_number_previous_get) | **GET** /epochs/{number}/previous | Listing of previous epochs
*CardanoEpochsApi* | [**epochs_number_stakes_get**](docs/CardanoEpochsApi.md#epochs_number_stakes_get) | **GET** /epochs/{number}/stakes | Stake distribution
*CardanoEpochsApi* | [**epochs_number_stakes_pool_id_get**](docs/CardanoEpochsApi.md#epochs_number_stakes_pool_id_get) | **GET** /epochs/{number}/stakes/{pool_id} | Stake distribution by pool
*CardanoGovernanceApi* | [**governance_dreps_drep_id_delegators_get**](docs/CardanoGovernanceApi.md#governance_dreps_drep_id_delegators_get) | **GET** /governance/dreps/{drep_id}/delegators | DRep delegators
*CardanoGovernanceApi* | [**governance_dreps_drep_id_get**](docs/CardanoGovernanceApi.md#governance_dreps_drep_id_get) | **GET** /governance/dreps/{drep_id} | Specific DRep
*CardanoGovernanceApi* | [**governance_dreps_drep_id_metadata_get**](docs/CardanoGovernanceApi.md#governance_dreps_drep_id_metadata_get) | **GET** /governance/dreps/{drep_id}/metadata | DRep metadata
*CardanoGovernanceApi* | [**governance_dreps_drep_id_updates_get**](docs/CardanoGovernanceApi.md#governance_dreps_drep_id_updates_get) | **GET** /governance/dreps/{drep_id}/updates | DRep updates
*CardanoGovernanceApi* | [**governance_dreps_drep_id_votes_get**](docs/CardanoGovernanceApi.md#governance_dreps_drep_id_votes_get) | **GET** /governance/dreps/{drep_id}/votes | DRep votes
*CardanoGovernanceApi* | [**governance_dreps_get**](docs/CardanoGovernanceApi.md#governance_dreps_get) | **GET** /governance/dreps | Delegate Representatives (DReps)
*CardanoGovernanceApi* | [**governance_proposals_get**](docs/CardanoGovernanceApi.md#governance_proposals_get) | **GET** /governance/proposals | Proposals
*CardanoGovernanceApi* | [**governance_proposals_tx_hash_cert_index_get**](docs/CardanoGovernanceApi.md#governance_proposals_tx_hash_cert_index_get) | **GET** /governance/proposals/{tx_hash}/{cert_index} | Specific proposal
*CardanoGovernanceApi* | [**governance_proposals_tx_hash_cert_index_metadata_get**](docs/CardanoGovernanceApi.md#governance_proposals_tx_hash_cert_index_metadata_get) | **GET** /governance/proposals/{tx_hash}/{cert_index}/metadata | Specific proposal metadata
*CardanoGovernanceApi* | [**governance_proposals_tx_hash_cert_index_parameters_get**](docs/CardanoGovernanceApi.md#governance_proposals_tx_hash_cert_index_parameters_get) | **GET** /governance/proposals/{tx_hash}/{cert_index}/parameters | Specific parameters proposal
*CardanoGovernanceApi* | [**governance_proposals_tx_hash_cert_index_votes_get**](docs/CardanoGovernanceApi.md#governance_proposals_tx_hash_cert_index_votes_get) | **GET** /governance/proposals/{tx_hash}/{cert_index}/votes | Proposal votes
*CardanoGovernanceApi* | [**governance_proposals_tx_hash_cert_index_withdrawals_get**](docs/CardanoGovernanceApi.md#governance_proposals_tx_hash_cert_index_withdrawals_get) | **GET** /governance/proposals/{tx_hash}/{cert_index}/withdrawals | Specific withdrawals proposal
*CardanoLedgerApi* | [**genesis_get**](docs/CardanoLedgerApi.md#genesis_get) | **GET** /genesis | Blockchain genesis
*CardanoMempoolApi* | [**mempool_addresses_address_get**](docs/CardanoMempoolApi.md#mempool_addresses_address_get) | **GET** /mempool/addresses/{address} | Mempool by address
*CardanoMempoolApi* | [**mempool_get**](docs/CardanoMempoolApi.md#mempool_get) | **GET** /mempool | Mempool
*CardanoMempoolApi* | [**mempool_hash_get**](docs/CardanoMempoolApi.md#mempool_hash_get) | **GET** /mempool/{hash} | Specific transaction in the mempool
*CardanoMetadataApi* | [**metadata_txs_labels_get**](docs/CardanoMetadataApi.md#metadata_txs_labels_get) | **GET** /metadata/txs/labels | Transaction metadata labels
*CardanoMetadataApi* | [**metadata_txs_labels_label_cbor_get**](docs/CardanoMetadataApi.md#metadata_txs_labels_label_cbor_get) | **GET** /metadata/txs/labels/{label}/cbor | Transaction metadata content in CBOR
*CardanoMetadataApi* | [**metadata_txs_labels_label_get**](docs/CardanoMetadataApi.md#metadata_txs_labels_label_get) | **GET** /metadata/txs/labels/{label} | Transaction metadata content in JSON
*CardanoNetworkApi* | [**network_eras_get**](docs/CardanoNetworkApi.md#network_eras_get) | **GET** /network/eras | Query summary of blockchain eras
*CardanoNetworkApi* | [**network_get**](docs/CardanoNetworkApi.md#network_get) | **GET** /network | Network information
*CardanoPoolsApi* | [**pools_extended_get**](docs/CardanoPoolsApi.md#pools_extended_get) | **GET** /pools/extended | List of stake pools with additional information
*CardanoPoolsApi* | [**pools_get**](docs/CardanoPoolsApi.md#pools_get) | **GET** /pools | List of stake pools
*CardanoPoolsApi* | [**pools_pool_id_blocks_get**](docs/CardanoPoolsApi.md#pools_pool_id_blocks_get) | **GET** /pools/{pool_id}/blocks | Stake pool blocks
*CardanoPoolsApi* | [**pools_pool_id_delegators_get**](docs/CardanoPoolsApi.md#pools_pool_id_delegators_get) | **GET** /pools/{pool_id}/delegators | Stake pool delegators
*CardanoPoolsApi* | [**pools_pool_id_get**](docs/CardanoPoolsApi.md#pools_pool_id_get) | **GET** /pools/{pool_id} | Specific stake pool
*CardanoPoolsApi* | [**pools_pool_id_history_get**](docs/CardanoPoolsApi.md#pools_pool_id_history_get) | **GET** /pools/{pool_id}/history | Stake pool history
*CardanoPoolsApi* | [**pools_pool_id_metadata_get**](docs/CardanoPoolsApi.md#pools_pool_id_metadata_get) | **GET** /pools/{pool_id}/metadata | Stake pool metadata
*CardanoPoolsApi* | [**pools_pool_id_relays_get**](docs/CardanoPoolsApi.md#pools_pool_id_relays_get) | **GET** /pools/{pool_id}/relays | Stake pool relays
*CardanoPoolsApi* | [**pools_pool_id_updates_get**](docs/CardanoPoolsApi.md#pools_pool_id_updates_get) | **GET** /pools/{pool_id}/updates | Stake pool updates
*CardanoPoolsApi* | [**pools_pool_id_votes_get**](docs/CardanoPoolsApi.md#pools_pool_id_votes_get) | **GET** /pools/{pool_id}/votes | Stake pool votes
*CardanoPoolsApi* | [**pools_retired_get**](docs/CardanoPoolsApi.md#pools_retired_get) | **GET** /pools/retired | List of retired stake pools
*CardanoPoolsApi* | [**pools_retiring_get**](docs/CardanoPoolsApi.md#pools_retiring_get) | **GET** /pools/retiring | List of retiring stake pools
*CardanoScriptsApi* | [**scripts_datum_datum_hash_cbor_get**](docs/CardanoScriptsApi.md#scripts_datum_datum_hash_cbor_get) | **GET** /scripts/datum/{datum_hash}/cbor | Datum CBOR value
*CardanoScriptsApi* | [**scripts_datum_datum_hash_get**](docs/CardanoScriptsApi.md#scripts_datum_datum_hash_get) | **GET** /scripts/datum/{datum_hash} | Datum value
*CardanoScriptsApi* | [**scripts_get**](docs/CardanoScriptsApi.md#scripts_get) | **GET** /scripts | Scripts
*CardanoScriptsApi* | [**scripts_script_hash_cbor_get**](docs/CardanoScriptsApi.md#scripts_script_hash_cbor_get) | **GET** /scripts/{script_hash}/cbor | Script CBOR
*CardanoScriptsApi* | [**scripts_script_hash_get**](docs/CardanoScriptsApi.md#scripts_script_hash_get) | **GET** /scripts/{script_hash} | Specific script
*CardanoScriptsApi* | [**scripts_script_hash_json_get**](docs/CardanoScriptsApi.md#scripts_script_hash_json_get) | **GET** /scripts/{script_hash}/json | Script JSON
*CardanoScriptsApi* | [**scripts_script_hash_redeemers_get**](docs/CardanoScriptsApi.md#scripts_script_hash_redeemers_get) | **GET** /scripts/{script_hash}/redeemers | Redeemers of a specific script
*CardanoTransactionsApi* | [**tx_submit_post**](docs/CardanoTransactionsApi.md#tx_submit_post) | **POST** /tx/submit | Submit a transaction
*CardanoTransactionsApi* | [**txs_hash_cbor_get**](docs/CardanoTransactionsApi.md#txs_hash_cbor_get) | **GET** /txs/{hash}/cbor | Transaction CBOR
*CardanoTransactionsApi* | [**txs_hash_delegations_get**](docs/CardanoTransactionsApi.md#txs_hash_delegations_get) | **GET** /txs/{hash}/delegations | Transaction delegation certificates
*CardanoTransactionsApi* | [**txs_hash_get**](docs/CardanoTransactionsApi.md#txs_hash_get) | **GET** /txs/{hash} | Specific transaction
*CardanoTransactionsApi* | [**txs_hash_metadata_cbor_get**](docs/CardanoTransactionsApi.md#txs_hash_metadata_cbor_get) | **GET** /txs/{hash}/metadata/cbor | Transaction metadata in CBOR
*CardanoTransactionsApi* | [**txs_hash_metadata_get**](docs/CardanoTransactionsApi.md#txs_hash_metadata_get) | **GET** /txs/{hash}/metadata | Transaction metadata
*CardanoTransactionsApi* | [**txs_hash_mirs_get**](docs/CardanoTransactionsApi.md#txs_hash_mirs_get) | **GET** /txs/{hash}/mirs | Transaction MIRs
*CardanoTransactionsApi* | [**txs_hash_pool_retires_get**](docs/CardanoTransactionsApi.md#txs_hash_pool_retires_get) | **GET** /txs/{hash}/pool_retires | Transaction stake pool retirement certificates
*CardanoTransactionsApi* | [**txs_hash_pool_updates_get**](docs/CardanoTransactionsApi.md#txs_hash_pool_updates_get) | **GET** /txs/{hash}/pool_updates | Transaction stake pool registration and update certificates
*CardanoTransactionsApi* | [**txs_hash_redeemers_get**](docs/CardanoTransactionsApi.md#txs_hash_redeemers_get) | **GET** /txs/{hash}/redeemers | Transaction redeemers
*CardanoTransactionsApi* | [**txs_hash_required_signers_get**](docs/CardanoTransactionsApi.md#txs_hash_required_signers_get) | **GET** /txs/{hash}/required_signers | Transaction required signers
*CardanoTransactionsApi* | [**txs_hash_stakes_get**](docs/CardanoTransactionsApi.md#txs_hash_stakes_get) | **GET** /txs/{hash}/stakes | Transaction stake addresses certificates
*CardanoTransactionsApi* | [**txs_hash_utxos_get**](docs/CardanoTransactionsApi.md#txs_hash_utxos_get) | **GET** /txs/{hash}/utxos | Transaction UTXOs
*CardanoTransactionsApi* | [**txs_hash_withdrawals_get**](docs/CardanoTransactionsApi.md#txs_hash_withdrawals_get) | **GET** /txs/{hash}/withdrawals | Transaction withdrawal
*CardanoUtilitiesApi* | [**utils_addresses_xpub_xpub_role_index_get**](docs/CardanoUtilitiesApi.md#utils_addresses_xpub_xpub_role_index_get) | **GET** /utils/addresses/xpub/{xpub}/{role}/{index} | Derive an address
*CardanoUtilitiesApi* | [**utils_txs_evaluate_post**](docs/CardanoUtilitiesApi.md#utils_txs_evaluate_post) | **POST** /utils/txs/evaluate | Submit a transaction for execution units evaluation
*CardanoUtilitiesApi* | [**utils_txs_evaluate_utxos_post**](docs/CardanoUtilitiesApi.md#utils_txs_evaluate_utxos_post) | **POST** /utils/txs/evaluate/utxos | Submit a transaction for execution units evaluation (additional UTXO set)
*HealthApi* | [**health_clock_get**](docs/HealthApi.md#health_clock_get) | **GET** /health/clock | Current backend time
*HealthApi* | [**health_get**](docs/HealthApi.md#health_get) | **GET** /health | Backend health status
*HealthApi* | [**root_get**](docs/HealthApi.md#root_get) | **GET** / | Root endpoint
*IpfsAddApi* | [**ipfs_add**](docs/IpfsAddApi.md#ipfs_add) | **POST** /ipfs/add | Add a file to IPFS
*IpfsGatewayApi* | [**ipfs_gateway_ipfs_path_get**](docs/IpfsGatewayApi.md#ipfs_gateway_ipfs_path_get) | **GET** /ipfs/gateway/{IPFS_path} | Relay to an IPFS gateway
*IpfsPinsApi* | [**ipfs_pin_add_ipfs_path_post**](docs/IpfsPinsApi.md#ipfs_pin_add_ipfs_path_post) | **POST** /ipfs/pin/add/{IPFS_path} | Pin an object
*IpfsPinsApi* | [**ipfs_pin_list_get**](docs/IpfsPinsApi.md#ipfs_pin_list_get) | **GET** /ipfs/pin/list | List pinned objects
*IpfsPinsApi* | [**ipfs_pin_list_ipfs_path_get**](docs/IpfsPinsApi.md#ipfs_pin_list_ipfs_path_get) | **GET** /ipfs/pin/list/{IPFS_path} | Get details about pinned object
*IpfsPinsApi* | [**ipfs_pin_remove_ipfs_path_post**](docs/IpfsPinsApi.md#ipfs_pin_remove_ipfs_path_post) | **POST** /ipfs/pin/remove/{IPFS_path} | Remove a IPFS pin
*MetricsApi* | [**metrics_endpoints_get**](docs/MetricsApi.md#metrics_endpoints_get) | **GET** /metrics/endpoints | Blockfrost endpoint usage metrics
*MetricsApi* | [**metrics_get**](docs/MetricsApi.md#metrics_get) | **GET** /metrics | Blockfrost usage metrics
*NutLinkApi* | [**nutlink_address_get**](docs/NutLinkApi.md#nutlink_address_get) | **GET** /nutlink/{address} | Specific nut.link address
*NutLinkApi* | [**nutlink_address_tickers_get**](docs/NutLinkApi.md#nutlink_address_tickers_get) | **GET** /nutlink/{address}/tickers | List of tickers of an oracle
*NutLinkApi* | [**nutlink_address_tickers_ticker_get**](docs/NutLinkApi.md#nutlink_address_tickers_ticker_get) | **GET** /nutlink/{address}/tickers/{ticker} | Specific ticker for an address
*NutLinkApi* | [**nutlink_tickers_ticker_get**](docs/NutLinkApi.md#nutlink_tickers_ticker_get) | **GET** /nutlink/tickers/{ticker} | Specific ticker


## Documentation For Models

 - [AccountAddressesAssetsInner](docs/AccountAddressesAssetsInner.md)
 - [AccountAddressesContentInner](docs/AccountAddressesContentInner.md)
 - [AccountAddressesTotal](docs/AccountAddressesTotal.md)
 - [AccountAddressesTotalReceivedSumInner](docs/AccountAddressesTotalReceivedSumInner.md)
 - [AccountContent](docs/AccountContent.md)
 - [AccountDelegationContentInner](docs/AccountDelegationContentInner.md)
 - [AccountHistoryContentInner](docs/AccountHistoryContentInner.md)
 - [AccountMirContentInner](docs/AccountMirContentInner.md)
 - [AccountRegistrationContentInner](docs/AccountRegistrationContentInner.md)
 - [AccountRewardContentInner](docs/AccountRewardContentInner.md)
 - [AccountUtxoContentInner](docs/AccountUtxoContentInner.md)
 - [AccountWithdrawalContentInner](docs/AccountWithdrawalContentInner.md)
 - [AddressContent](docs/AddressContent.md)
 - [AddressContentExtended](docs/AddressContentExtended.md)
 - [AddressContentExtendedAmountInner](docs/AddressContentExtendedAmountInner.md)
 - [AddressContentTotal](docs/AddressContentTotal.md)
 - [AddressTransactionsContentInner](docs/AddressTransactionsContentInner.md)
 - [AddressUtxoContentInner](docs/AddressUtxoContentInner.md)
 - [AggregatorFeaturesMessage](docs/AggregatorFeaturesMessage.md)
 - [AggregatorFeaturesMessageCapabilities](docs/AggregatorFeaturesMessageCapabilities.md)
 - [AggregatorFeaturesMessageCapabilitiesCardanoTransactionsProver](docs/AggregatorFeaturesMessageCapabilitiesCardanoTransactionsProver.md)
 - [Asset](docs/Asset.md)
 - [AssetAddressesInner](docs/AssetAddressesInner.md)
 - [AssetHistoryInner](docs/AssetHistoryInner.md)
 - [AssetMetadata](docs/AssetMetadata.md)
 - [AssetOnchainMetadataCip25](docs/AssetOnchainMetadataCip25.md)
 - [AssetOnchainMetadataCip25Description](docs/AssetOnchainMetadataCip25Description.md)
 - [AssetOnchainMetadataCip25FilesInner](docs/AssetOnchainMetadataCip25FilesInner.md)
 - [AssetOnchainMetadataCip25FilesInnerSrc](docs/AssetOnchainMetadataCip25FilesInnerSrc.md)
 - [AssetOnchainMetadataCip25Image](docs/AssetOnchainMetadataCip25Image.md)
 - [AssetOnchainMetadataCip68Ft333](docs/AssetOnchainMetadataCip68Ft333.md)
 - [AssetOnchainMetadataCip68Nft222](docs/AssetOnchainMetadataCip68Nft222.md)
 - [AssetOnchainMetadataCip68Rft444](docs/AssetOnchainMetadataCip68Rft444.md)
 - [AssetPolicyInner](docs/AssetPolicyInner.md)
 - [AssetTransactionsInner](docs/AssetTransactionsInner.md)
 - [AssetsInner](docs/AssetsInner.md)
 - [BlockContent](docs/BlockContent.md)
 - [BlockContentAddressesInner](docs/BlockContentAddressesInner.md)
 - [BlockContentAddressesInnerTransactionsInner](docs/BlockContentAddressesInnerTransactionsInner.md)
 - [BlockContentTxsCborInner](docs/BlockContentTxsCborInner.md)
 - [BlocksLatestGet404Response](docs/BlocksLatestGet404Response.md)
 - [CardanoDbBeacon](docs/CardanoDbBeacon.md)
 - [CardanoTransactionProofMessage](docs/CardanoTransactionProofMessage.md)
 - [CardanoTransactionProofMessageCertifiedTransactionsInner](docs/CardanoTransactionProofMessageCertifiedTransactionsInner.md)
 - [CardanoTransactionSnapshotListMessageInner](docs/CardanoTransactionSnapshotListMessageInner.md)
 - [CardanoTransactionSnapshotMessage](docs/CardanoTransactionSnapshotMessage.md)
 - [CertificateListItemMessage](docs/CertificateListItemMessage.md)
 - [CertificateListItemMessageMetadata](docs/CertificateListItemMessageMetadata.md)
 - [CertificateMessage](docs/CertificateMessage.md)
 - [CertificateMetadata](docs/CertificateMetadata.md)
 - [CertificatePendingMessage](docs/CertificatePendingMessage.md)
 - [Drep](docs/Drep.md)
 - [DrepDelegatorsInner](docs/DrepDelegatorsInner.md)
 - [DrepMetadata](docs/DrepMetadata.md)
 - [DrepUpdatesInner](docs/DrepUpdatesInner.md)
 - [DrepVotesInner](docs/DrepVotesInner.md)
 - [DrepsInner](docs/DrepsInner.md)
 - [EpochContent](docs/EpochContent.md)
 - [EpochParamContent](docs/EpochParamContent.md)
 - [EpochSettingsMessage](docs/EpochSettingsMessage.md)
 - [EpochStakeContentInner](docs/EpochStakeContentInner.md)
 - [EpochStakePoolContentInner](docs/EpochStakePoolContentInner.md)
 - [Error](docs/Error.md)
 - [GenesisContent](docs/GenesisContent.md)
 - [Get200Response](docs/Get200Response.md)
 - [Get400Response](docs/Get400Response.md)
 - [Get403Response](docs/Get403Response.md)
 - [Get418Response](docs/Get418Response.md)
 - [Get429Response](docs/Get429Response.md)
 - [Get500Response](docs/Get500Response.md)
 - [HealthClockGet200Response](docs/HealthClockGet200Response.md)
 - [HealthGet200Response](docs/HealthGet200Response.md)
 - [IpfsAdd200Response](docs/IpfsAdd200Response.md)
 - [IpfsPinAddIpfsPathPost200Response](docs/IpfsPinAddIpfsPathPost200Response.md)
 - [IpfsPinAddIpfsPathPost425Response](docs/IpfsPinAddIpfsPathPost425Response.md)
 - [IpfsPinListGet200ResponseInner](docs/IpfsPinListGet200ResponseInner.md)
 - [IpfsPinListIpfsPathGet200Response](docs/IpfsPinListIpfsPathGet200Response.md)
 - [IpfsPinRemoveIpfsPathPost200Response](docs/IpfsPinRemoveIpfsPathPost200Response.md)
 - [MempoolContentInner](docs/MempoolContentInner.md)
 - [MempoolTxContent](docs/MempoolTxContent.md)
 - [MempoolTxContentInputsInner](docs/MempoolTxContentInputsInner.md)
 - [MempoolTxContentOutputsInner](docs/MempoolTxContentOutputsInner.md)
 - [MempoolTxContentRedeemersInner](docs/MempoolTxContentRedeemersInner.md)
 - [MempoolTxContentTx](docs/MempoolTxContentTx.md)
 - [MetricsEndpointsInner](docs/MetricsEndpointsInner.md)
 - [MetricsInner](docs/MetricsInner.md)
 - [MithrilStakeDistributionListMessageInner](docs/MithrilStakeDistributionListMessageInner.md)
 - [MithrilStakeDistributionMessage](docs/MithrilStakeDistributionMessage.md)
 - [Network](docs/Network.md)
 - [NetworkErasInner](docs/NetworkErasInner.md)
 - [NetworkErasInnerEnd](docs/NetworkErasInnerEnd.md)
 - [NetworkErasInnerParameters](docs/NetworkErasInnerParameters.md)
 - [NetworkErasInnerStart](docs/NetworkErasInnerStart.md)
 - [NetworkStake](docs/NetworkStake.md)
 - [NetworkSupply](docs/NetworkSupply.md)
 - [NutlinkAddress](docs/NutlinkAddress.md)
 - [NutlinkAddressTickerInner](docs/NutlinkAddressTickerInner.md)
 - [NutlinkAddressTickersInner](docs/NutlinkAddressTickersInner.md)
 - [NutlinkTickersTickerInner](docs/NutlinkTickersTickerInner.md)
 - [Pool](docs/Pool.md)
 - [PoolDelegatorsInner](docs/PoolDelegatorsInner.md)
 - [PoolHistoryInner](docs/PoolHistoryInner.md)
 - [PoolListExtendedInner](docs/PoolListExtendedInner.md)
 - [PoolListExtendedInnerMetadata](docs/PoolListExtendedInnerMetadata.md)
 - [PoolListRetireInner](docs/PoolListRetireInner.md)
 - [PoolMetadata](docs/PoolMetadata.md)
 - [PoolUpdatesInner](docs/PoolUpdatesInner.md)
 - [PoolsPoolIdMetadataGet200Response](docs/PoolsPoolIdMetadataGet200Response.md)
 - [Proposal](docs/Proposal.md)
 - [ProposalMetadata](docs/ProposalMetadata.md)
 - [ProposalParameters](docs/ProposalParameters.md)
 - [ProposalParametersParameters](docs/ProposalParametersParameters.md)
 - [ProposalVotesInner](docs/ProposalVotesInner.md)
 - [ProposalWithdrawalsInner](docs/ProposalWithdrawalsInner.md)
 - [ProposalsInner](docs/ProposalsInner.md)
 - [ProtocolMessage](docs/ProtocolMessage.md)
 - [ProtocolMessageParts](docs/ProtocolMessageParts.md)
 - [ProtocolParameters](docs/ProtocolParameters.md)
 - [RegisterSignerMessage](docs/RegisterSignerMessage.md)
 - [RegisterSingleSignatureMessage](docs/RegisterSingleSignatureMessage.md)
 - [Script](docs/Script.md)
 - [ScriptCbor](docs/ScriptCbor.md)
 - [ScriptDatum](docs/ScriptDatum.md)
 - [ScriptDatumCbor](docs/ScriptDatumCbor.md)
 - [ScriptJson](docs/ScriptJson.md)
 - [ScriptRedeemersInner](docs/ScriptRedeemersInner.md)
 - [ScriptsInner](docs/ScriptsInner.md)
 - [Signer](docs/Signer.md)
 - [SignerRegistrationsListItemMessage](docs/SignerRegistrationsListItemMessage.md)
 - [SignerRegistrationsMessage](docs/SignerRegistrationsMessage.md)
 - [SignerTickerListItemMessage](docs/SignerTickerListItemMessage.md)
 - [SignerWithStake](docs/SignerWithStake.md)
 - [SignersTickersMessage](docs/SignersTickersMessage.md)
 - [Snapshot](docs/Snapshot.md)
 - [SnapshotDownloadMessage](docs/SnapshotDownloadMessage.md)
 - [SnapshotMessage](docs/SnapshotMessage.md)
 - [Stake](docs/Stake.md)
 - [StakeDistributionParty](docs/StakeDistributionParty.md)
 - [TxContent](docs/TxContent.md)
 - [TxContentCbor](docs/TxContentCbor.md)
 - [TxContentDelegationsInner](docs/TxContentDelegationsInner.md)
 - [TxContentMetadataCborInner](docs/TxContentMetadataCborInner.md)
 - [TxContentMetadataInner](docs/TxContentMetadataInner.md)
 - [TxContentMetadataInnerJsonMetadata](docs/TxContentMetadataInnerJsonMetadata.md)
 - [TxContentMirsInner](docs/TxContentMirsInner.md)
 - [TxContentOutputAmountInner](docs/TxContentOutputAmountInner.md)
 - [TxContentPoolCertsInner](docs/TxContentPoolCertsInner.md)
 - [TxContentPoolCertsInnerMetadata](docs/TxContentPoolCertsInnerMetadata.md)
 - [TxContentPoolCertsInnerRelaysInner](docs/TxContentPoolCertsInnerRelaysInner.md)
 - [TxContentPoolRetiresInner](docs/TxContentPoolRetiresInner.md)
 - [TxContentRedeemersInner](docs/TxContentRedeemersInner.md)
 - [TxContentRequiredSignersInner](docs/TxContentRequiredSignersInner.md)
 - [TxContentStakeAddrInner](docs/TxContentStakeAddrInner.md)
 - [TxContentUtxo](docs/TxContentUtxo.md)
 - [TxContentUtxoInputsInner](docs/TxContentUtxoInputsInner.md)
 - [TxContentUtxoOutputsInner](docs/TxContentUtxoOutputsInner.md)
 - [TxContentWithdrawalsInner](docs/TxContentWithdrawalsInner.md)
 - [TxMetadataLabelCborInner](docs/TxMetadataLabelCborInner.md)
 - [TxMetadataLabelJsonInner](docs/TxMetadataLabelJsonInner.md)
 - [TxMetadataLabelsInner](docs/TxMetadataLabelsInner.md)
 - [TxSubmitPost425Response](docs/TxSubmitPost425Response.md)
 - [UtilsAddressesXpub](docs/UtilsAddressesXpub.md)
 - [UtilsTxsEvaluateUtxosPostRequest](docs/UtilsTxsEvaluateUtxosPostRequest.md)
 - [UtilsTxsEvaluateUtxosPostRequestAdditionalUtxoSetInnerInner](docs/UtilsTxsEvaluateUtxosPostRequestAdditionalUtxoSetInnerInner.md)
 - [UtilsTxsEvaluateUtxosPostRequestAdditionalUtxoSetInnerInnerAnyOf](docs/UtilsTxsEvaluateUtxosPostRequestAdditionalUtxoSetInnerInnerAnyOf.md)
 - [UtilsTxsEvaluateUtxosPostRequestAdditionalUtxoSetInnerInnerAnyOf1](docs/UtilsTxsEvaluateUtxosPostRequestAdditionalUtxoSetInnerInnerAnyOf1.md)
 - [UtilsTxsEvaluateUtxosPostRequestAdditionalUtxoSetInnerInnerAnyOf1Value](docs/UtilsTxsEvaluateUtxosPostRequestAdditionalUtxoSetInnerInnerAnyOf1Value.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

contact@blockfrost.io

