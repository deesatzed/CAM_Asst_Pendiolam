use common_reality_schema::{
    fixture_manifest_path, verify_fixture_manifest, REQUIRED_ATTACK_FIXTURE_IDS,
};

#[test]
fn exposes_the_frozen_campus_fixture_manifest_path() {
    assert!(fixture_manifest_path().exists());
}

#[test]
fn frozen_manifest_hashes_the_packet_and_every_required_attack_fixture() {
    let manifest = verify_fixture_manifest().expect("the Gate 0 manifest must validate");

    assert!(manifest.fixture_ids().contains(&"packet".to_owned()));
    for attack_id in REQUIRED_ATTACK_FIXTURE_IDS {
        assert!(
            manifest
                .fixture_ids()
                .iter()
                .any(|fixture_id| fixture_id == attack_id),
            "missing required attack fixture: {attack_id}"
        );
    }
}
