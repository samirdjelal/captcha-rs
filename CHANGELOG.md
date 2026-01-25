# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.3.0 (2026-01-25)

### Added

- New edge-case tests.

### Changed

- Dependency upgrades and input-validation tweaks.
- Required Rust 2024 edition.
- Refactored font handling with `FontArc` and updated `rand`.

### Fixed

- Improved error handling.

## v0.2.11 (2024-09-08)

### Changed

- Randomized captcha noise generation.

## v0.2.10 (2023-06-01)

### Changed

- Disabled default features for the `image` crate to trim dependencies.

## v0.2.9 (2023-05-26)

### Added

- Adjustable JPEG compression rate.

### Fixed

- Compression setter behavior.

### Changed

- README updates.

## v0.2.7 (2023-03-13)

### Added

- Captcha complexity with noise.
- Custom text support.
- Exposed raw image output.

### Changed

- Switched default image output to JPEG.
- Noise only applies when complexity is above 1 for better performance.

### Fixed

- Test reliability improvements.

