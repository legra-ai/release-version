#![doc = include_str!("../README.md")]
//
//! Pure release-version comparison types with no build-script coupling.

mod build_version;
mod upgrade_decision;

pub use build_version::BuildVersion;
pub use upgrade_decision::UpgradeDecision;
