# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

## [0.2.1] - 2024-06-13

### Changed

- Replace `anyhow!` with `bail!` macro for better error reporting in
  `main.rs`
- Update `README.md` to document the new `--verbose` flag
- Update dependencies in Cargo.lock
- Improve error handling by logging specific errors using `GetLastError`

### Added

- Add `log` and `simple_logger` dependencies in `Cargo.toml`
- Introduce `-v, --verbose` flag to enable logging file operations and
  error handling

## [0.2.0] - 2024-03-12

### Changed

- Replace StructOpt with Clap.
- Change repository name from "cat-win-rs" to "wincat-rs".
- Update the dependencies in Cargo.toml and Cargo.lock.
- Updated the error handling using `anyhow`.
- Changes in `README.md` file:
  - Updated the installation instructions to use `wincat.exe` instead of `cat-win.exe`.
  - Updated the project name.
  - Updated the examples to use `wincat.exe` instead of `cat-win.exe`.
  - Move the support and roadmap section inside the contributing guidelines.

## [0.1.1] - 2023-12-??

### Added

  - Print the file contents of more than one file.

## [0.1.0] - 2023-12-03

### Added

  - Initial release: Basic functionality to print file contents.
