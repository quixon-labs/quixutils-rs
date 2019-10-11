# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.13.0] - 2019-10-11

- Clean up `error` module
- Rename -> `error::print_*` to `error::write_*` except for top level
- Introduce `error::log_error*`
- Discourage the use of `::Result` alias, as it's provided by anyhow.

## [0.12.5] - 2019-10-11

- Add `error` mod

## [0.12.0] - 2019-10-11

- Removed `api` for now.
- Removed actix and tide support for now.
- Moved away from `failure` to `anyhow`
- Removed features: `actix_utils` and `tide_utils`
- Added feature `tokio_utils`

## [0.11.2] - 2019-05-14

- Updated deps to latest
- Fix compilation on latest nightly and futures_api

## [0.11.0] - 2019-04-22

### Changed

- Switch to `futures 0.3`
- Bring tide utils in sync with upstream tide - `0.1.1`
- Bring actix utils in sync with upstream actix - `1.0-beta`
- Switch to `futures::compat` instead of tokio async await preview.

## [0.10.0] - 2019-04-13

### Added

- `quixutils::timer` mod

### Changed

- `sleep` => `delay` to be more in line with std
- Similarly, `sleep_ms` => `delay_ms`
- `delay` function have now been moved into `timer` mod

## [0.9.1] - 2019-04-11

### Changed

- Fix typo for `split_with_delimiter`

## [0.9.0] - 2019-04-11

### Changed

- `LOG_LOCALTIME` removed. Use `LOG_UTC` instead.