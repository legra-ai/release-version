//! Upgrade decision logic for comparing build versions.

/// The outcome of comparing the current build version against a
/// release manifest.
#[derive(Debug, PartialEq, Eq)]
pub enum UpgradeDecision {
    /// The manifest version is newer. Upgrade should proceed.
    Upgrade,
    /// The versions are identical. Already up to date.
    UpToDate,
    /// The available version is older. Downgrade refused unless the
    /// caller explicitly allows an override.
    Downgrade,
    /// The current value is a development build; build numbers are not
    /// comparable with release builds. Upgrade refused unless the
    /// caller explicitly allows an override.
    DevBuild,
}

#[cfg(test)]
mod tests {
    use semver::Version;

    use super::*;
    use crate::BuildVersion;

    fn ci(major: u64, minor: u64, patch: u64, build: u64) -> BuildVersion {
        BuildVersion::new(Version::new(major, minor, patch), build, false)
    }

    fn dev(major: u64, minor: u64, patch: u64, build: u64) -> BuildVersion {
        BuildVersion::new(Version::new(major, minor, patch), build, true)
    }

    #[test]
    fn semver_upgrade() {
        let current = ci(0, 10, 0, 100);
        let manifest = ci(0, 11, 0, 50);
        assert_eq!(
            current.compare_for_upgrade(&manifest, false),
            UpgradeDecision::Upgrade
        );
    }

    #[test]
    fn semver_downgrade_refused() {
        let current = ci(0, 11, 0, 100);
        let manifest = ci(0, 10, 0, 200);
        assert_eq!(
            current.compare_for_upgrade(&manifest, false),
            UpgradeDecision::Downgrade
        );
    }

    #[test]
    fn semver_downgrade_explicit() {
        let current = ci(0, 11, 0, 100);
        let manifest = ci(0, 10, 0, 200);
        assert_eq!(
            current.compare_for_upgrade(&manifest, true),
            UpgradeDecision::Upgrade
        );
    }

    #[test]
    fn same_semver_higher_build() {
        let current = ci(0, 11, 0, 100);
        let manifest = ci(0, 11, 0, 200);
        assert_eq!(
            current.compare_for_upgrade(&manifest, false),
            UpgradeDecision::Upgrade
        );
    }

    #[test]
    fn same_semver_same_build() {
        let current = ci(0, 11, 0, 200);
        let manifest = ci(0, 11, 0, 200);
        assert_eq!(
            current.compare_for_upgrade(&manifest, false),
            UpgradeDecision::UpToDate
        );
    }

    #[test]
    fn same_semver_lower_build_refused() {
        let current = ci(0, 11, 0, 200);
        let manifest = ci(0, 11, 0, 100);
        assert_eq!(
            current.compare_for_upgrade(&manifest, false),
            UpgradeDecision::Downgrade
        );
    }

    #[test]
    fn same_semver_lower_build_explicit() {
        let current = ci(0, 11, 0, 200);
        let manifest = ci(0, 11, 0, 100);
        assert_eq!(
            current.compare_for_upgrade(&manifest, true),
            UpgradeDecision::Upgrade
        );
    }

    #[test]
    fn dev_build_same_semver_refused() {
        let current = dev(0, 11, 0, 1221);
        let manifest = ci(0, 11, 0, 207);
        assert_eq!(
            current.compare_for_upgrade(&manifest, false),
            UpgradeDecision::DevBuild
        );
    }

    #[test]
    fn dev_build_same_semver_explicit() {
        let current = dev(0, 11, 0, 1221);
        let manifest = ci(0, 11, 0, 207);
        assert_eq!(
            current.compare_for_upgrade(&manifest, true),
            UpgradeDecision::Upgrade
        );
    }

    #[test]
    fn dev_build_older_semver_upgrades() {
        let current = dev(0, 10, 0, 1221);
        let manifest = ci(0, 11, 0, 207);
        assert_eq!(
            current.compare_for_upgrade(&manifest, false),
            UpgradeDecision::Upgrade
        );
    }

    #[test]
    fn dev_build_newer_semver_refuses() {
        let current = dev(0, 12, 0, 1221);
        let manifest = ci(0, 11, 0, 207);
        assert_eq!(
            current.compare_for_upgrade(&manifest, false),
            UpgradeDecision::Downgrade
        );
    }

    #[test]
    fn semver_ordering_0_9_lt_0_10() {
        let current = ci(0, 9, 0, 1);
        let manifest = ci(0, 10, 0, 1);
        assert_eq!(
            current.compare_for_upgrade(&manifest, false),
            UpgradeDecision::Upgrade
        );
    }
}
