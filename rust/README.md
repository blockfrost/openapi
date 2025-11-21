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
  </tbody>
</table>

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

- API version: 0.1.83
- Package version: 0.1.83
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
 - [AccountTransactionsContentInner](docs/AccountTransactionsContentInner.md)
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
 - [PoolCalidusKey](docs/PoolCalidusKey.md)
 - [PoolDelegatorsInner](docs/PoolDelegatorsInner.md)
 - [PoolHistoryInner](docs/PoolHistoryInner.md)
 - [PoolListExtendedInner](docs/PoolListExtendedInner.md)
 - [PoolListExtendedInnerMetadata](docs/PoolListExtendedInnerMetadata.md)
 - [PoolListRetireInner](docs/PoolListRetireInner.md)
 - [PoolMetadata](docs/PoolMetadata.md)
 - [PoolUpdatesInner](docs/PoolUpdatesInner.md)
 - [PoolVotesInner](docs/PoolVotesInner.md)
 - [PoolsPoolIdMetadataGet200Response](docs/PoolsPoolIdMetadataGet200Response.md)
 - [Proposal](docs/Proposal.md)
 - [ProposalMetadata](docs/ProposalMetadata.md)
 - [ProposalMetadataV2](docs/ProposalMetadataV2.md)
 - [ProposalMetadataV2Error](docs/ProposalMetadataV2Error.md)
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
