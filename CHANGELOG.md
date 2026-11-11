# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-11-11

### Added

- `init` command storing lessor details, default rent/charges and receipt
  numbering in a TOML config file.
- `new` command generating a rent receipt PDF from an embedded Typst template.
- `show` command printing the config path and current values.
- French amount-to-words conversion and locale-aware currency wording.
- Configurable rental period, issue date, payment date, receipt number and
  optional payment method.
- MIT license, documentation and CI checks.
