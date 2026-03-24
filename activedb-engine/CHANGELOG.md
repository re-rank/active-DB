# Changelog

All notable changes to this project will be documented in this file.

## [2.0.0] - 2026-03-24

### Changed
- Rebranded from HelixDB to ActiveDB
- Renamed query language from HQL to AQL (ActiveQL)
- Configuration file: `helix.toml` → `activedb.toml`
- Credentials directory: `~/.helix/` → `~/.activedb/`
- CLI binary: `helix` → `activedb`
- Crate names: `helix-db` → `activedb-core`, `helix-cli` → `activedb-cli`

### Migration
- `helix.toml` and `~/.helix/` still work with deprecation warnings
- See project documentation for detailed upgrade guide

## [1.3.3] - Previous

- See original HelixDB repository for prior changelog
