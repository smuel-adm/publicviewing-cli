# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Update Cargo.toml manifest with description, license, license-file, documentation, homepage and repository.
- Better error handling, remove most of the unwraps
- add LTO optimisation on release builds

## [1.3.1] - 2024-11-11

### Fixed

- missing Changelog from v1.3.0


## [1.3.0] - 2024-11-11

### Added

- update dependencies (anyhow, clap, instant, tao and wry )

## [1.3.1] - 2024-11-11

### Fixed

- missing Changelog from v1.3.0


## [1.3.0] - 2024-11-11

### Added

- update dependencies (anyhow, clap, instant, tao and wry )


## [1.2.0] - 2024-02-14

### Added

- refactor cycle_sec parameter
- The code has been modularized
- improved error handling

### Fixed

- lots of typos
- `cargo fmt` github action


## [1.1.0] - 2024-02-11

### Added

- Multi monitor support `--monitor`
- Keep windows always on top parameter `-a, --above`
- Linux support

### Fixed

- Error handling
- Formatting with `cargo fmt`


## [1.0.0] - 2024-02-03

### Added

- first version
