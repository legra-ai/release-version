//! Public manifest and upgrade-decision contracts shipped in the registry archive.

use release_version::{BuildVersion, UpgradeDecision};
use semver::Version;

#[test]
fn a_release_requires_explicit_override_to_accept_a_development_build() {
    let installed = BuildVersion::from_manifest("1.2.3", 10).expect("release stamp");
    let development = BuildVersion::new(Version::new(1, 2, 3), 999, true);
    assert_eq!(
        installed.compare_for_upgrade(&development, false),
        UpgradeDecision::DevBuild
    );
    assert_eq!(
        installed.compare_for_upgrade(&development, true),
        UpgradeDecision::Upgrade
    );
}

#[test]
fn manifest_identity_and_semver_order_survive_the_public_boundary() {
    let installed = BuildVersion::from_manifest("0.9.0", u64::MAX).expect("current release");
    let available = BuildVersion::from_manifest("0.10.0", 0).expect("next release");
    assert_eq!(
        installed.compare_for_upgrade(&available, false),
        UpgradeDecision::Upgrade
    );
    assert_eq!(available.semver(), &Version::new(0, 10, 0));
    assert_eq!(available.build_number(), 0);
    assert!(!available.is_dev_build());
    assert!(BuildVersion::from_manifest("0.10", 0).is_err());
}
