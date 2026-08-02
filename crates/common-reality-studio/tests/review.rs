use common_reality_studio::{PacketStudioDraft, TypedObject};

const PACKET: &str = include_str!("../../../fixtures/campus-ai-v1/packet.json");

fn draft() -> PacketStudioDraft {
    PacketStudioDraft::from_legacy_packet(PACKET).expect("fixture adapter")
}

#[test]
fn frozen_fixture_review_is_publishable_without_a_truth_score() {
    let report = draft().review();

    assert!(report.can_publish(), "findings: {:?}", report.findings);
    assert!(!serde_json::to_string(&report)
        .unwrap()
        .contains("truth_score"));
}

#[test]
fn unsupported_claim_cannot_be_marked_supported() {
    let mut draft = draft();
    let claim = draft
        .find_object_mut("claim-disclosure-boundary")
        .expect("claim");
    let TypedObject::Claim(claim) = claim else {
        panic!("typed claim")
    };
    claim.support_status = "supported".into();
    claim.evidence_refs.clear();

    let report = draft.review();
    assert!(report.has_code("unsupported_claim_marked_supported"));
    assert!(!report.can_publish());
}

#[test]
fn review_fails_closed_for_missing_material_counterevidence() {
    let mut draft = draft();
    assert!(draft.remove_object("counterevidence:claim-disclosure-boundary"));

    assert!(draft.review().has_code("missing_material_counterevidence"));
}

#[test]
fn review_fails_closed_for_missing_uncertainty() {
    let mut draft = draft();
    assert!(draft.remove_object("uncertainty:claim-disclosure-boundary"));

    assert!(draft.review().has_code("missing_uncertainty"));
}

#[test]
fn review_fails_closed_for_invalid_reference_and_incomplete_minority_report() {
    let mut invalid_reference = draft();
    let claim = invalid_reference
        .find_object_mut("claim-disclosure-boundary")
        .expect("claim");
    let TypedObject::Claim(claim) = claim else {
        panic!("typed claim")
    };
    claim.evidence_refs.push("evidence-does-not-exist".into());
    assert!(invalid_reference.review().has_code("invalid_reference"));

    let mut incomplete_minority = draft();
    let report = incomplete_minority
        .find_object_mut("minority-accessibility-appeal")
        .expect("minority report");
    let TypedObject::MinorityReport(report) = report else {
        panic!("typed minority report")
    };
    report.unresolved_risk.clear();
    assert!(incomplete_minority
        .review()
        .has_code("incomplete_minority_report"));
}

#[test]
fn typed_object_deserialization_rejects_kind_substitution() {
    let substituted = r#"{
        "object_type": "evidence",
        "data": {
            "claim_id": "claim-disclosure-boundary",
            "proposition": "not evidence"
        }
    }"#;

    assert!(serde_json::from_str::<TypedObject>(substituted).is_err());
}
