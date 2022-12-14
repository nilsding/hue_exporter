# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2022-12-14

### Changed
- Light states `bri`, `hue`, `sat`, and `ct` are now optional as not all lights emit these properties (@martinlindhe)
- `/metrics` now sets the utf-8 charset in the response header (@martinlindhe)

## [0.2.0] - 2021-05-03

### Added
- Metrics for lights

### Changed
- Rewrote it in Rust
- Configuration is now done via the following env variables:
  - `HUE_TOKEN`: set this to the token received from authorizing with the Hue bridge
  - `HUE_BRIDGE_URL`: set this to the URL of the Hue bridge.  Default value: `http://hue-bridge.local`
  - `BIND_ADDR`: set this to the address+port to bind to.  Default value: `127.0.0.1:9369`

## [0.1.0] - 2018-05-28

### Added
- Initial release
- Metrics for sensors

[0.1.0]: https://github.com/nilsding/hue_exporter/releases/tag/v0.1.0
