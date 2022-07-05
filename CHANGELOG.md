# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project will adhere to [Semantic Versioning](https://semver.org/spec/v2.0.0.html) once it reaches maturity in v1.

## [Unreleased]

## [0.1.39] - 2021-07-01

### Added

- `/txs/{hash}/utxos`
  - `inputs` and `outputs` now contain extra
    - `inline_datum` field
    - `reference_script_hash` field
  - `inputs` now has a boolean `reference` field, `true` if the input is a reference input
- `/addresses/{address}/utxos` and `/addresses/{address}/utxos/{asset}`
  - `inline_datum` field
  - `reference_script_hash` field
- `/scripts/datum/{datum-hash}/cbor` endpoint

### Changed

- `/txs/{hash}/redeemers` - `datum_hash` field deprecated, prefer `redeemer_data_hash`
- `/scripts/{hash}/redeemers` - `datum_hash` field deprecated, prefer `redeemer_data_hash`
- `/epochs/{number}/parameters` and `/epochs/latest/parameters`
  - `coins_per_utxo_word` field deprecated, prefer `coins_per_utxo_size`
    - `coins_per_utxo_size` is now
      - Cost per UTxO **word** for Alonzo.
      - Cost per UTxO **byte** for Babbage and later.
- `/scripts/{hash}`
  - `type` field now uses `plutusV1` and `plutusV2` instead of just `plutus` to be able
    to differentiate between two `PlutusScript` versions.

### Fixed

- `/scripts/{script-hash}/cbor` - `cbor` field type changed to `string`

## [0.1.37] - 2021-03-24

### Added

- `type` property to `/accounts/{stake_address}/rewards` endpoint
- `/utils/txs/evaluate` endpoint

## [0.1.36] - 2021-12-21

### Added

- `blocks_epoch` property to `/pools/{pool_id}` endpoint

### Fixed

- mark `tx_hash` in `/blocks/{hash_or_number}/addresses` as required field

## [0.1.35] - 2021-12-10

### Added

- `/blocks/{hash_or_number}/addresses`

## [0.1.34] - 2021-12-07

### Added

- `/pools/extended` endpoint
- `/utils/addresses/xpub/{xpub}/{role}/{index}` endpoint

## [0.1.33] - 2021-11-24

### Added

- `/addresses/{address}/extended` endpoint
- `/accounts/{stake_address}/addresses/total` endpoint

### Fixed

- multiple descriptions

## [0.1.32] - 2021-10-30

### Added

- `block_time` property to `/assets/{asset}/transactions`

## [0.1.31] - 2021-10-30

### Added

- `block_time` property to `/addresses/{address}/transactions` and `/txs/{hash}`

## [0.1.30] - 2021-10-14

### Added

- HTTP `425` - `Mempool Full` to `Errors` for better handling of full mempool in `/tx/submit` endpoint
- Alonzo support related additions
  - `valid_contract` property to `/txs/{hash}` endpoint, `true` when attached
    script passed validation, `false` if it failed phase 2 validation
  - `datum_hash` and `script_hash` properties to `/txs/{hash}/redeemers`
  - `datum_hash` property to `/scripts/{hash}/redeemers`
  - `/scripts/datum/{datum-hash}` endpoint
  - `/scripts/{script_hash}/json` endpoint for dumping `timelock` scripts
  - `/scripts/{script_hash}/cbor` endpoint for dumping `plutus` script contents

### Changed

- `/epochs/{number}/parameters` - `collateral_percent` type from `number` to `integer`

### Fixed

- `/network` supply descriptions

## [0.1.29] - 2021-10-07

### Added

- `/addresses/{address}/utxos/{asset}` endpoint

### Changed

- enforce pagination limits

### Fixed

- description of `stake_cert_count` in `/txs/{hash}`

## [0.1.28] - 2021-09-29

### Added

- `output_index` to each `output` item for `/txs/{hash}/utxos` endpoint

### Changed

- `/txs/{hash}/metadata/cbor` and `metadata/txs/labels/{label}/cbor` property `cbor_metadata` is now deprecated in favour of `metadata` property

### Fixed

- `/txs/{hash}/utxos` - `data_hash` field moved up from `amount` to `output` item
- `/txs/{hash}/utxos` - `output_index` of `input` items switch from `number` to `integer`

## [0.1.27] - 2021-09-12

### Added

- Alonzo support related additions

  - `/scripts` endpoint for listing all scripts
  - `/scripts/{hash}` endpoint for script details
  - `/scripts/{hash}/redeemers` endpoint for listing reedemers of a script
  - `/txs/{hash}/redeemers` endpoint for querying transaction redeemers
  - `locked` property to `/network` endpoint, representing total supply locked
    by scripts
  - `script` property to `/addresses/{hash}`, which is `true` when
    the address is a script address.
  - `redeemer_count` property to `/txs/{hash}` endpoint
  - Boolean `collateral` property to `inputs` object
    of `/txs/{hash}/utxos` endpoint
  - `data_hash` property to both `inputs` and `outputs` objects of
    `/txs/{hash}/utxos` endpoint
  - `data_hash` property to `/addressess/{hash}/utxos` endpoint
  - `/epoch/latest/parameters` and `/epoch/{number}/parameters`
    extended with cost model fields.

### Fixed

- `/ipfs/add` size response type fixed from integer to string
- types of IPFS size examples
- description of `ipfs/pin/list/<object>`
- `onchain_metadata` type of `/assets/{asset}` endpoint

## [0.1.26] - 2021-08-12

### Fixed

- multiple descriptions and examples, mainly related to time/date

## [0.1.25] - 2021-07-27

### Added

- `network` endpoint

### Fixed

- typo in `/addresses/{address}/txs` description - thanks @papacarp

## [0.1.24] - 2021-07-16

### Added

- properties `tx_hash` and `output_index` to `inputs` of `/txs/{hash}/utxos` endpoint
- `stake_address` property to `/accounts/{stake_address}` endpoint
- `address` property to `/addresses/{address}` and `/addresses/{address}/total` endpoints
- `asset` property to `/assets/{asset}` endpoint
- `epoch` property to `/epochs/{number}/parameters` and `/epochs/latest/parameters` endpoints
- `pool_id` and `hex` properties to `/pools/{pool_id}` and `/pools/{pool_id}/metadata` endpoints
- `hash` property to `/txs/{hash}` and `/txs/{hash}/utxos` endpoints
- `address` property to `/nutlink/{address}` endpoint

### Fixed

- clarified active stake `amount` in `/epochs/{number}/stakes` and `/epochs/{number}/stakes/{pool_id}`
- pagination of `accounts/{stake_address}/addresses/assets` endpoint

## [0.1.23] - 2021-07-01

### Added

- `mint_or_burn_count` property to `/assets/{asset}` endpoint
- `asset_mint_or_burn_count` property to `/txs/{hash}` endpoint
- `decimals` property to `/assets/{asset}` endpoint

## [0.1.22] - 2021-07-01

### Added

- `nutlink` endpoints

### Changed

- authentication documentation

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
