# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 4.0.1 - 2025-03-29

### Changed

- Updated PyO3 to 0.23.5, Cargo to Edition 2021, and all main dependencies.
- Provided maturin build CI for linux/macos/windows environments producing respective artifacts.

## 4.0.0 - 2020-02-26

### Changed

- Updated bitmap color functions to accept hex value rather than tuple.

## 3.1.0 - 2020-01-17

### Added

- Added constant for tab key.

### Fixed

- Updated `autopilot-rs` to 0.4.0, including the following fixes:
- Fixed typing of apostrophe and quote characters on Linux.

### Changed

- This project has been relicensed under the MIT or Apache 2.0 license.
- Updated image, libc, rand, quickcheck, pkg-config, core-foundation,
  core-graphics, and cocoa crates to latest versions.

## 3.0.1 - 2019-11-12

### Added

- Added support for Python 3.8.
- Added support for setting modifier delay in `key.tap`.

### Changed

- Updated modifiers param in `key.tap` and `key.toggle` to be optional (note
  that this is not a breaking change).

### Fixed

- Updated `autopilot-rs` to 0.3.1, including the following fixes:
- Fixed issue with point scaling in `mouse.location` and `mouse.move`.
- Fixed typing of "^" character on Linux.
- Fixed typing of "_" character on Linux.

## 3.0.0 - 2019-09-16

### Added

- Added support for missing function keys F13 – F24.

### Changed

- Updated color functions to return hex value rather than tuple.
- Updated `autopilot-rs` to the latest version, including the following fixes:
- Updated image, libc, rand, quickcheck, pkg-config, and cocoa crates to latest
  versions.
  
### Fixed

- Fixed warnings of using try operator on latest nightly.

## 2.1.0 - 2019-05-15

### Changed

- Updated to the latest version of pyo3.

## 2.0.0 - 2019-05-13

### Changed

- Updated for latest Python versions and `cibuildwheel`.
- Updated `autopilot-rs` to the latest version, including the following fixes:
- Updated image, libc, scopeguard, quickcheck, pkg-config, core-foundation,
  core-graphics, and cocoa crates to latest versions.

### Removed

- Removed handling of unsupported image formats. AutoPy now only supports
  direct saving of PNG, GIF, BMP and JPEG files.

## 1.1.1 - 2018-09-26

### Fixed

- Updated scale factor on x11 to be rounded to the nearest hundredth.

## 1.1.0 - 2018-09-19

### Added

- Added constant for spacebar key.
- Added support for passing a delay into `mouse.click`.
- Added constant for tab key.
- Added support for passing a delay into `key.tap`.
- Added support for faster typing with `key.type_string`.

### Changed

- Updated `autopilot-rs` to the latest version, including the following fixes:
- Updated Cocoa and other macOS dependencies.
- Updated x11 dependency.
- Updated function signatures with delay parameters to be consistent.
- Updated `key.tap` delay to be passed through to modifier key toggles.
- Updated `mouse.smooth_move` to accept a duration.

### Fixed

- Fixed compilation error on 32-bit Linux.
- Fixed compilation error on 32-bit Windows.
- Fixed linux arrow keycode constant definitions.
- Fixed colon showing up as semicolon on Windows.
- Fixed `mouse.click` to release at end of function.

## 1.0.1 - 2018-05-01

### Fixed

- Fixed packaging issue on Linux.

## 1.0.0 - 2018-04-30

### Added

- Initial release of new fork.
