# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
  - `txs_count` → `tx_count`
  - `txs_sum` → `output`
  - `fees_sum` → `fees`
- Rename `acronym` → `ticker` in metadata of `/assets/{asset}`

## [0.1.0] - 2021-02-23

### Added

- Initial release
