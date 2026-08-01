use common_reality_schema::fixture_manifest_path;

#[test]
fn exposes_the_frozen_campus_fixture_manifest_path() {
    assert!(fixture_manifest_path().exists());
}
