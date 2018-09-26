# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
