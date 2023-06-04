# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Types of changes:

    Added for new features.
    Changed for changes in existing functionality.
    Deprecated for soon-to-be removed features.
    Removed for now removed features.
    Fixed for any bug fixes.
    Security in case of vulnerabilities.


## Unreleased

## [1.0.0] - 2023-06-04

### Added
- Added default values for `data` and `template` args. The default values are `data.json` and `template.mustache` respectively.
- Added `output` arg to specify output file path.
- Added `verbose` arg to enable verbose logging.
- Environment variables are now available in templates.
- The following environment variables can now be used to specify args:
    - `PBL_DATA`
    - `PBL_TEMPLATE`
    - `PBL_OUTPUT`
- Added augmented data. (BREAKING CHANGE) The following struct is now used to represent augmented data:
```rust
let augmented_data = AllData {
    data: val.clone(),
    env: std::env::vars().collect(),
    config: render_config.clone(),
    formatted_date: formatted_date(),
};
```    


## [0.2.0] - 2022-01-17

### Added
- Now supports arbitrary JSON data with the need for specifying types for deserialization.

### Changed
- Moved from `ramhorns` to `mustache` library.

## [0.1.0] - 2022-01-14

### Added
- Added credit to [README.md](README.md)

### Changed
- `google-authenticator` library version from `0.2.1` to `0.3.0`
- GitHub action from `actions/checkout@v1` to `actions/checkout@v2`
- Rust edition from `2018` to `2021`

## [0.0.5] - 2020-11-22

### Changed

- Changed `google-authenticator` to version `0.2.1`

## [0.0.4] - 2020-06-30

- Initial release.

## [0.0.1-0.0.3]

- Stabilization.
