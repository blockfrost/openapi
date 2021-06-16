# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project will adhere to [Semantic Versioning](https://semver.org/spec/v2.0.0.html) once it reaches maturity in v1.

## [] - Unreleased

### Added

- `nutlink` endpoints

## [0.1.19] - 2021-06-11

### Added

- `mir_cert_count` property to `/txs/{hash}`
- additional query parameters `from` and `to` into `/addresses/{address}/transactions` endpoint
- `/txs/{hash}/mirs` endpoint to display MIR details
- `/accounts/{stake_address}/withdrawals` and `/accounts/{stake_address}/mirs` endpoints

### Fixed

- `txs/{hash}/stakes` default ordering

## [0.1.18] - 2021-06-06

### Fixed

- properties of `/addresses/{address}/transactions` and `/assets/{asset}/transactions` to be required

## [0.1.17] - 2021-06-02

### Added

- `/addresses/{address}/transactions` and `/assets/{asset}/transactions` endpoints to provide more details about transactions and make the endpoint extendable (object) for future needs
- `/blocks/slot/{slot_number}` and `/blocks/epoch/{epoch_number}/slot/{slot_number}` endpoints to provide block details for a specific slot

### Changed

- `/addresses/{address}/txs` and `/assets/{asset}/txs` are deprecated (but still functional) in favour of newly added endpoints

### Fixed

- type of `number` parameter in `/epochs/{number}/` string -> integer

## [0.1.16] - 2021-05-25

### Added

- `/accounts/{stake_address}/addresses/assets` endpoint to list all assets on addresses related to a given stake_address (account)
- `/epochs/latest/parameters` and `/blocks/latest/txs` endpoints to list the current information about latest epoch and block

### Changed

- implementation (increase allowed burst) of rate limiting and clarified documentation

### Fixed

- `onchain_metadata` in `/assets/{asset}/total` endpoint that could have been missing in some special cases
- `onchain_metadata` in `/assets/:asset` endpoint to return null on malformed data which do not follow https://github.com/cardano-foundation/CIPs/pull/85/files

## [0.1.15] - 2021-05-19

### Fixed

- `registration` and `retirement` arrays in `/pools/{pool_id}/` endpoint doesn't need to be `Nullable`

## [0.1.14] - 2021-05-19

### Fixed

- `amount` in `/accounts/{stake_address}/rewards`, `/accounts/{stake_address}/delegations` and `/accounts/{stake_address}/history` endpoints doesn't need to be `Nullable`

## [0.1.13] - 2021-05-06

### Fixed

- inconsistency in `/txs` endpoints when array was used outside of ref (moved array inside the referenced schema)

## [0.1.12] - 2021-04-29

### Added

- slightly improved README

### Fixed

- all occurences of `epoch` to `active_epoch` in `/accounts/{stake_address}/history` endpoint
- required properties of multiple endpoints and error messages

## [0.1.11] - 2021-04-23

### Added

- `block_height` to `/txs/{hash}` endpoint

### Fixed

- alphabetical ordering in Cardano subcategory documentation

## [0.1.10] - 2021-04-17

### Changed

- deprecated `index` in favour of `cert_index` in `/txs/{hash}/delegations`

### Added

- `onchain_metadata` to `/assets/{asset}` endpoint
- `slot` parameter to `/txs/{hash}` endpoint
- `cert_index` parameter to all relevant endpoints:
  - `/txs/{hash}/stakes`
  - `/txs/{hash}/pool_updates`
  - `/txs/{hash}/pool_retires`

### Fixed

- `/pools` trailing slash in documentation

## [0.1.9] - 2021-04-04

### Added

- `/assets/{asset}/addresses` lists of the addresses holding a specific asset
- `/assets/policy/{policy_id}` lists the assets minted under a specific policy

### Changed

- `tx_index` in `/addresses/{address}/utxos` - it has been deprecated in favour of `output_index`
- `/` endpoint property `version` type number -> string

## [0.1.8] - 2021-03-31

### Added

- `/ipfs/` endpoints for IPFS cluster

### Changed

- `/assets/{asset}` specification of `metadata` to adhere to upstream

### Fixed

- `/tx/submit` error handling (standardized error reply for 400)
- `/assets/{asset}` handling of `metadata`

## [0.1.6] - 2021-03-19

### Added

- `/accounts/{account}/addresses` which returns the list of on-chain addresses associated with a specific stake key
- `402` response to mark oversubscription (projects exceeded their daily subscription plan)

### Changed

- `/tx/submit` endpoint, which now accepts CBOR encoded serialized transaction instead of a binary blob

### Fixed

- functionality and description of rate limiting (429), oversubscription (402) and banning (418)

## [0.1.5] - 2021-03-12

### Added

- `/assets/{asset}/txs` endpoint to list all transactions of a given asset
- Owners to `/txs/{hash}/pool_updates` endpoint

### Changed

- Altered functionality and description of `active_epoch` in `/accounts/{stake_address}` to better match its changed functionality. When account is deregistered (`active` field is `false`), this field contains the epoch number of deregistration
- `reward_address` -> `reward_account` in `/pools/{pool_id}` and `/txs/{hash}/pool_updates` endpoints as the previous name was misleading and incorrect
- Deprecated `unit` field of not yet used `metadata` in `/assets/{asset}` endpoint
- Descriptions of few fields

### Fixed

- Multiple endpoints impacted by [Cardano DB Sync issue](https://github.com/input-output-hk/cardano-db-sync/issues/474). We have addressed associated issues with a temporary fix which will be in effect until the issue is addressed in the upstream. Please note that this mostly involved retired pools and their associated reward accounts. Affected endpoints and their parameters:
  - `/pools/{pool_id}`
    - `live_stake`
    - `live_size`
    - `live_saturation`
    - `live_pledge`
  - `/pools/{pool_id}/delegators`
    - `live_stake`
  - `/accounts/{stake_address}`
    - `controlled_amount`
    - `withdrawable_amount`
- Missing treasury in calculation of `live_stake` in `/pools/{pool_id}/delegators` endpoint. Other endpoints were not affected
- Pool hopping account issues which were causing that some pools were displaying slightly higher `live_stake` values and thus also very slightly skewed `live_size` and `live_saturation` calculations in `/pools/{pool_id}` endpoint
- `active_epoch` in `/accounts/{stake_address}` was previously displaying epoch of delegation, not epoch of de/registration

## [0.1.4] - 2021-03-08

### Fixed

- `/txs/{hash}` types of `invalid_before` and `invalid_hereafter` changed from integer to string as the value may overflow

## [0.1.2] - 2021-03-05

### Fixed

- `/pools/retired` and `/pools/retiring` are now sorted first by `epoch` and then `tx_id` as it was causing inconsistencies when pool announced its retirement way back in the past (in this case, the retired pool could be inserted in the middle of already retired pools)
- `/pools` was showing duplicate values in some special cases
- `json_metadata` type in `tx_metadata_label_json` OpenAPI scheme so it would correctly accept all values - thanks @sorki

### Changed

- `/pools` endpoint now also includes retiring (not yet retired) pools
- All `/addresses/{address}` endpoints now also accept `payment_cred` in Bech32 format
- All `/pools/{pool_id}` endpoints now also accept `pool_id` in Hex format

## [0.1.1] - 2021-02-26

### Added

- New items to multiple `/epochs` endpoints
  - `first_block_time` - Marking first block of the epoch
  - `last_block_time` - Marking last block of the epoch
  - `active_stake` - Sum of all active stakes of the epoch
- Pagination for `/metadata/txs/labels` endpoint

### Fixed

- Rename `active_pledge` → `live_pledge` in `/pools/{pool_id}`
- `start_time` and `end_time` now display correct values (previously `first_block_time` and `last_block_time` values were shown)
- Specification for `tx_metadata_labels`
- Unify pagination documentation

### Changed

- Unify item names in `/epochs/`
  - `blocks_count` → `block_count`
  - `txs_count` → `tx_count`
  - `txs_sum` → `output`
  - `fees_sum` → `fees`
- Rename `acronym` → `ticker` in metadata of `/assets/{asset}`

## [0.1.0] - 2021-02-23

### Added

- Initial release
