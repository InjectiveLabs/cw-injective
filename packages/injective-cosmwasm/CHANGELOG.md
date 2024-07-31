# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [0.2.24] - 2024-07-31

### Changed

- Updated cosmwasm_std to `2.1.0`

## [0.2.22] - 2024-03-21

### Added

- Tests queries (injective-cosmwasm-mock) covering functionality of querier.rs

### Fixed

- Exchange aggregate volume query to use the correct parsing.

### Removed

- Grants related queries.
- Exchange denom decimal query.
