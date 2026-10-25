# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project follows
[Semantic Versioning](https://semver.org/) (while `0.x`: a breaking change bumps the minor version).

## [Unreleased]

### Changed

- **Breaking:** public functions now return `PathAsUriError` (a thiserror enum) instead of
  `anyhow::Error`, so callers can match on the failure without depending on `anyhow`. Its one variant,
  `NotAbsolute`, carries the path (`PathBuf`); the enum is `#[non_exhaustive]`.

## [0.1.1] - 2026-10-25

### Changed

- Crate documentation on docs.rs: an overview, `# Errors` / `# Panics` sections and tested examples on every function.
- The error message prints the path with `Display` (`Path: '/rel/path'`) instead of `Debug` (`Path: '"/rel/path"'`).
- `get_path_as_uri_string` is `#[must_use]`.
- Dependency requirements relaxed to `anyhow = "1.0"` and `url = "2.5"`; `rust-version = "1.85"` (edition 2024) is now
  declared in `Cargo.toml`.
- The package contains only the sources, README, LICENSE and CHANGELOG.

### Fixed

- The README example did not compile (`PathBuf::new` takes no argument).
- The tests passed only on Unix; they now run on Windows too.

## [0.1.0] - 2025-04-20

First release on crates.io. Its source is the commit tagged `v0.0.1` (that tag was made before the version was set to
`0.1.0`); the tag `v0.1.0` points to the same commit.

[Unreleased]: https://github.com/Alirex/rs_lib_i_path_as_uri/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/Alirex/rs_lib_i_path_as_uri/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/Alirex/rs_lib_i_path_as_uri/releases/tag/v0.1.0
