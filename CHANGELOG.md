# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.5.0 (2026-03-01)

### Added

- **Stateless Verification Feature**: New opt-in `stateless` feature using JWT and SHA-256 for serverless captcha validation.
- Added `Captcha::as_token` and `Captcha::as_tuple` to generate secure, time-bound verification tokens.
- Added `verify` function to validate solutions against tokens without server-side storage.
- New `stateless` example demonstrating the end-to-end verification flow.

## v0.4.2 (2026-03-01)

### Changed

- Optimized array character random selection for better performance.
- Avoided unnecessary image cloning in wavy distortion.

### Fixed

- Fixed Denial of Service vulnerability via unbounded parameters.
- Limited distortion level in `CaptchaBuilder`.
- Added edge case tests for small dimensions in interference drawing.

## v0.4.1 (2026-02-25)

### Added

- Support for passing a custom alphabet (`.chars()`) to generate the captcha text.
- Deferred text generation to `.build()` to allow flexible builder ordering.

## v0.4.0 (2026-02-25)

### Added

- Support for dark mode, visual enhancements, and captcha distortion.
- Added parameters to enhance captcha complexity and make it harder for bots to bypass.
- Example generator application to generate and save captcha images.
- Performance benchmarks using Criterion.

### Changed

- Refactored the library to implement a correct builder pattern.
- Cleaned the codebase and upgraded dependencies.
- Updated documentation to reflect the new features and benchmark results.

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

