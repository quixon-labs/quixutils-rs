# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- None

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