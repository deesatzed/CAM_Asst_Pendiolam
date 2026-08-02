use common_reality_policy::CapabilityGrant;
use common_reality_schema::verify_fixture_manifest;
use common_reality_studio::{
    ObjectStore, PacketStudioDraft, StudioError, TypedObject, PUBLICATION_PURPOSE, PUBLISH_ACTION,
};
use std::path::PathBuf;

const PACKET: &str = include_str!("../../../fixtures/campus-ai-v1/packet.json");

fn grant_for(draft: &PacketStudioDraft, action: &str) -> CapabilityGrant {
    CapabilityGrant {
        issuer: "local-reviewer".into(),
        principal: "local-editor".into(),
        action: action.into(),
        resource: format!("packet:{}", draft.packet_id),
        purpose: PUBLICATION_PURPOSE.into(),
        packet_id: draft.packet_id.clone(),
        state_digest: draft.state_digest().expect("state digest"),
        expires_at: 1_000,
        revoked: false,
    }
}

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/campus-ai-v1")
        .join(name)
}

fn main() {
    let manifest = verify_fixture_manifest().expect("frozen Gate 0 manifest");
    assert!(manifest.fixture_ids().iter().any(|id| id == "packet"));

    let source_path = fixture_path("source-corpus.json");
    let metadata = common_reality_studio::SourceMetadata::synthetic(
        "source-corpus",
        "Synthetic source corpus",
        "fixture",
        "2026-08-01T00:00:00Z",
    );
    let mut source_store = ObjectStore::new();
    let source = source_store
        .import_source_file(&source_path, 16_384, metadata.clone())
        .expect("bounded local source import");
    let mut second_source_store = ObjectStore::new();
    let second_source = second_source_store
        .import_source_file(&source_path, 16_384, metadata)
        .expect("repeat local source import");
    assert_eq!(source.content_digest, second_source.content_digest);
    assert!(source_store.contains(&source.object_id));

    let draft = PacketStudioDraft::from_legacy_packet(PACKET).expect("typed fixture draft");
    for kind in [
        "claim",
        "evidence",
        "counterevidence",
        "uncertainty",
        "interpretation",
        "value",
        "action",
        "minority_report",
    ] {
        assert!(
            draft.objects().iter().any(|object| object.kind() == kind),
            "missing typed object kind {kind}"
        );
    }

    let mut unsupported = draft.clone();
    let claim = unsupported
        .find_object_mut("claim-disclosure-boundary")
        .expect("claim");
    let TypedObject::Claim(claim) = claim else {
        panic!("typed claim")
    };
    claim.support_status = "supported".into();
    claim.evidence_refs.clear();
    assert!(unsupported
        .review()
        .has_code("unsupported_claim_marked_supported"));

    let mut missing_counterevidence = draft.clone();
    assert!(missing_counterevidence.remove_object("counterevidence:claim-disclosure-boundary"));
    assert!(missing_counterevidence
        .review()
        .has_code("missing_material_counterevidence"));

    let mut missing_uncertainty = draft.clone();
    assert!(missing_uncertainty.remove_object("uncertainty:claim-disclosure-boundary"));
    assert!(missing_uncertainty.review().has_code("missing_uncertainty"));

    let mut invalid_reference = draft.clone();
    let claim = invalid_reference
        .find_object_mut("claim-disclosure-boundary")
        .expect("claim");
    let TypedObject::Claim(claim) = claim else {
        panic!("typed claim")
    };
    claim.evidence_refs.push("evidence-missing".into());
    assert!(invalid_reference.review().has_code("invalid_reference"));

    let mut incomplete_minority = draft.clone();
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

    assert!(matches!(
        draft.review_authorized(None, 100),
        Err(StudioError::AuthorizationDenied(action)) if action == "review_packet"
    ));
    let review = draft
        .review_authorized(Some(&grant_for(&draft, "review_packet")), 100)
        .expect("exact review grant");
    assert!(review.can_publish());

    let mut store = ObjectStore::new();
    let published = draft
        .publish(
            &mut store,
            Some(&grant_for(&draft, PUBLISH_ACTION)),
            100,
            None,
        )
        .expect("first local publication");
    assert!(!published.receipt.raw_source_bytes_included);
    let receipt_json = serde_json::to_string(&published.receipt).expect("receipt JSON");
    assert!(!receipt_json.contains("Synthetic source corpus"));

    let reconstructed = store
        .reconstruct(&published.manifest, "campus-ai-v1", 1)
        .expect("byte-for-byte reconstruction");
    assert_eq!(
        serde_json::to_vec(&published.draft).expect("draft bytes"),
        serde_json::to_vec(&reconstructed).expect("reconstructed bytes")
    );

    let mut missing_store = store.clone();
    let claim_object_id = published
        .manifest
        .objects
        .iter()
        .find(|entry| entry.kind == "claim")
        .expect("claim object entry")
        .object_id
        .clone();
    assert!(missing_store.remove_object(&claim_object_id));
    assert!(missing_store
        .reconstruct(&published.manifest, "campus-ai-v1", 1)
        .is_err());

    let mut altered_store = store.clone();
    altered_store
        .object_bytes_mut(&published.manifest.root_object_id)
        .expect("root object")
        .push(b' ');
    assert!(altered_store
        .reconstruct(&published.manifest, "campus-ai-v1", 1)
        .is_err());
    assert!(store
        .reconstruct(&published.manifest, "campus-ai-v1", 2)
        .is_err());

    let mut second_draft = draft.next_version();
    let claim = second_draft
        .find_object_mut("claim-disclosure-boundary")
        .expect("claim");
    let TypedObject::Claim(claim) = claim else {
        panic!("typed claim")
    };
    claim.proposition.push_str(" The pilot remains reversible.");
    let second = second_draft
        .publish(
            &mut store,
            Some(&grant_for(&second_draft, PUBLISH_ACTION)),
            100,
            Some(&published),
        )
        .expect("successor local publication");
    let diff = second.diff_from_parent(&published);
    assert!(diff
        .changes
        .iter()
        .any(|change| change.field == "proposition"));
    assert!(!serde_json::to_string(&diff)
        .expect("diff JSON")
        .contains("truth_score"));

    println!("GATE2_STATUS=PASS");
    println!("FIXTURE_MANIFEST=PASS");
    println!(
        "SOURCE_IMPORT=PASS digest={} bytes={}",
        source.content_digest, source.byte_length
    );
    println!("TYPED_OBJECTS=PASS count={}", draft.objects().len());
    println!("REVIEW=PASS findings={}", review.findings.len());
    println!(
        "PUBLICATION=PASS packet_content_id={} version={}",
        published.receipt.packet_content_id, published.receipt.packet_version
    );
    println!(
        "RECONSTRUCTION=PASS manifest_digest={} objects={}",
        published.receipt.manifest_digest,
        published.receipt.object_ids.len()
    );
    println!("CORRUPTION_GATES=PASS missing altered wrong_version");
    println!("DIFF=PASS changed_fields={}", diff.changes.len());
    println!("LIMITATION=synthetic-local-integrity-only");
}
