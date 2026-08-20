//! Release version type combining semver, build number, and build provenance.

use std::cmp::Ordering;
use std::fmt;

use semver::Version;

use crate::upgrade_decision::UpgradeDecision;

/// A parsed release version combining semantic version, build number,
/// and development-build provenance.
///
/// Two `BuildVersion` values are only meaningfully comparable when
/// both originate from the same build source (both CI or both dev).
/// When the semver components are equal but the build sources differ,
/// [`UpgradeDecision::DevBuild`] is returned instead of a numeric
/// build-number comparison.
#[derive(Debug, Clone)]
pub struct BuildVersion {
    semver: Version,
    build_number: u64,
    dev_build: bool,
}

impl BuildVersion {
    /// Construct from parsed components.
    #[must_use]
    pub fn new(semver: Version, build_number: u64, dev_build: bool) -> Self {
        Self {
            semver,
            build_number,
            dev_build,
        }
    }

    /// Parse a release manifest's version fields.
    ///
    /// Manifest builds are never development builds.
    ///
    /// # Errors
    ///
    /// Returns an error if `binary_version` is not valid semver.
    pub fn from_manifest(binary_version: &str, build_number: u64) -> Result<Self, semver::Error> {
        Ok(Self {
            semver: Version::parse(binary_version)?,
            build_number,
            dev_build: false,
        })
    }

    /// The semantic version component.
    #[must_use]
    pub fn semver(&self) -> &Version {
        &self.semver
    }

    /// The build number.
    #[must_use]
    pub fn build_number(&self) -> u64 {
        self.build_number
    }

    /// Whether this is a local development build.
    #[must_use]
    pub fn is_dev_build(&self) -> bool {
        self.dev_build
    }

    /// Compare this version against an available release to determine
    /// whether an upgrade should proceed.
    ///
    /// When `allow_override` is `true`, downgrades and development-build
    /// overrides are allowed.
    #[must_use]
    pub fn compare_for_upgrade(&self, available: &Self, allow_override: bool) -> UpgradeDecision {
        match self.semver.cmp(&available.semver) {
            Ordering::Less => return UpgradeDecision::Upgrade,
            Ordering::Greater => {
                return if allow_override {
                    UpgradeDecision::Upgrade
                } else {
                    UpgradeDecision::Downgrade
                };
            }
            Ordering::Equal => {}
        }

        // Semver is equal — check build provenance and number.
        if self.dev_build {
            return if allow_override {
                UpgradeDecision::Upgrade
            } else {
                UpgradeDecision::DevBuild
            };
        }

        // Both are release builds — compare build numbers numerically.
        match self.build_number.cmp(&available.build_number) {
            Ordering::Less => UpgradeDecision::Upgrade,
            Ordering::Equal => UpgradeDecision::UpToDate,
            Ordering::Greater => {
                if allow_override {
                    UpgradeDecision::Upgrade
                } else {
                    UpgradeDecision::Downgrade
                }
            }
        }
    }
}

impl fmt::Display for BuildVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.semver, self.build_number)
    }
}

impl Default for BuildVersion {
    /// A zero placeholder for applications that inject their current
    /// release stamp at startup.
    fn default() -> Self {
        Self {
            semver: Version::new(0, 0, 0),
            build_number: 0,
            dev_build: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_format() {
        let v = BuildVersion::new(Version::new(0, 11, 0), 207, false);
        assert_eq!(v.to_string(), "0.11.0.207");
    }

    #[test]
    fn from_manifest_valid() {
        let v = BuildVersion::from_manifest("0.11.0", 207).expect("valid semver");
        assert_eq!(v.semver(), &Version::new(0, 11, 0));
        assert_eq!(v.build_number(), 207);
        assert!(!v.is_dev_build());
    }

    #[test]
    fn from_manifest_invalid_semver() {
        assert!(BuildVersion::from_manifest("not.valid", 1).is_err());
    }
}
