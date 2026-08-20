# release-version

[![Crates.io](https://img.shields.io/crates/v/release-version.svg)](https://crates.io/crates/release-version)
[![Documentation](https://docs.rs/release-version/badge.svg)](https://docs.rs/release-version)
[![CI](https://github.com/legra-ai/release-version/actions/workflows/ci.yml/badge.svg)](https://github.com/legra-ai/release-version/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/legra-ai/release-version#license)

Typed release-version comparison for applications that publish semantic
versions alongside monotonically increasing build numbers.

## What it models

`release-version` keeps three pieces of release identity together:

- a parsed `semver::Version`;
- a monotonically increasing build number;
- whether the value came from a development build or a release build.

`BuildVersion::compare_for_upgrade` applies a small, explicit policy:

- newer semantic versions are upgrades;
- older semantic versions are downgrades unless an explicit override was
  requested;
- equal semantic versions compare release build numbers;
- development builds do not compare their build number with a release build
  unless explicitly overridden.

The crate does not read the environment, contact a registry, parse a manifest
format, or perform an upgrade. Callers supply typed values and decide what to
do with the returned `UpgradeDecision`.

## Example

```rust
use release_version::{BuildVersion, UpgradeDecision};
use semver::Version;

let current = BuildVersion::new(Version::new(1, 4, 0), 12, false);
let available = BuildVersion::from_manifest("1.5.0", 3).expect("valid version");

assert_eq!(
    current.compare_for_upgrade(&available, false),
    UpgradeDecision::Upgrade,
);
```

## License

Licensed under either of:

- Apache License, Version 2.0
- MIT License

at your option.
