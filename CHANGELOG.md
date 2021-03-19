# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.6] - unreleased

### Changed

- The `/tx/submit` endpoint now accepts CBOR encoded serialized transaction instread of a binary blob.
- Adding `/accounts/{account}/addresses` which returns the list of on-chain addresses associated with a specific stake key.

## [0.1.5] - 2021-03-12

### Added

- `/assets/{asset}/txs` endpoint to list all transactions of a given asset
- Owners to `/txs/{hash}/pool_updates` endpoint

### Changed

- Altered functionality and description of `active_epoch` in `/accounts/{stake_address}` to better match its changed functionality. When account is deregistered (`active` field is `false`), this field contains the epoch number of deregistration.
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
