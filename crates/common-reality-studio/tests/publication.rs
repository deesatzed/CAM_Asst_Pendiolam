use common_reality_policy::CapabilityGrant;
use common_reality_studio::{
    ObjectStore, PacketStudioDraft, StudioError, TypedObject, PUBLICATION_PURPOSE, PUBLISH_ACTION,
};

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

fn draft() -> PacketStudioDraft {
    PacketStudioDraft::from_legacy_packet(PACKET).expect("fixture adapter")
}

#[test]
fn publication_is_denied_without_an_exact_capability_and_receipt_is_redacted() {
    let draft = draft();
    let mut store = ObjectStore::new();
    assert!(matches!(
        draft.publish(&mut store, None, 100, None),
        Err(StudioError::AuthorizationDenied(action)) if action == PUBLISH_ACTION
    ));

    let published = draft
        .publish(
            &mut store,
            Some(&grant_for(&draft, PUBLISH_ACTION)),
            100,
            None,
        )
        .expect("exact local publication grant");
    assert_eq!(published.receipt.status, "published");
    assert_eq!(published.receipt.packet_id, draft.packet_id);
    assert_eq!(published.receipt.packet_version, 1);
    assert_eq!(
        published.receipt.packet_content_id,
        published.manifest.root_object_id
    );
    assert!(!published.receipt.raw_source_bytes_included);
    let receipt_json = serde_json::to_string(&published.receipt).unwrap();
    assert!(!receipt_json.contains("Synthetic source corpus"));
    assert!(!receipt_json.contains("source-assessment-integrity"));
}

#[test]
fn a_published_version_is_immutable_and_successors_record_parent_and_diff() {
    let first_draft = draft();
    let mut store = ObjectStore::new();
    let first = first_draft
        .publish(
            &mut store,
            Some(&grant_for(&first_draft, PUBLISH_ACTION)),
            100,
            None,
        )
        .expect("first publication");

    let duplicate = first_draft.publish(
        &mut store,
        Some(&grant_for(&first_draft, PUBLISH_ACTION)),
        100,
        None,
    );
    assert!(matches!(
        duplicate,
        Err(StudioError::ImmutableVersion { .. })
    ));

    let mut second_draft = first_draft.next_version();
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
            Some(&first),
        )
        .expect("successor publication");
    assert_eq!(second.manifest.parent_version, Some(1));
    let diff = second.diff_from_parent(&first);
    assert_eq!(diff.from_version, 1);
    assert_eq!(diff.to_version, 2);
    assert!(diff
        .changes
        .iter()
        .any(|change| change.field == "proposition"));
    assert!(!serde_json::to_string(&diff)
        .unwrap()
        .contains("truth_score"));
}
