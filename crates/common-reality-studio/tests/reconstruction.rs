use common_reality_policy::CapabilityGrant;
use common_reality_studio::{ObjectStore, PacketStudioDraft, PUBLICATION_PURPOSE, PUBLISH_ACTION};

const PACKET: &str = include_str!("../../../fixtures/campus-ai-v1/packet.json");

fn grant_for(draft: &PacketStudioDraft) -> CapabilityGrant {
    CapabilityGrant {
        issuer: "local-reviewer".into(),
        principal: "local-editor".into(),
        action: PUBLISH_ACTION.into(),
        resource: format!("packet:{}", draft.packet_id),
        purpose: PUBLICATION_PURPOSE.into(),
        packet_id: draft.packet_id.clone(),
        state_digest: draft.state_digest().expect("state digest"),
        expires_at: 1_000,
        revoked: false,
    }
}

fn published() -> (ObjectStore, common_reality_studio::PublishedPacket) {
    let draft = PacketStudioDraft::from_legacy_packet(PACKET).expect("fixture adapter");
    let mut store = ObjectStore::new();
    let packet = draft
        .publish(&mut store, Some(&grant_for(&draft)), 100, None)
        .expect("publication");
    (store, packet)
}

#[test]
fn published_packet_reconstructs_byte_for_byte_from_manifest_objects() {
    let (store, packet) = published();
    let reconstructed = store
        .reconstruct(&packet.manifest, "campus-ai-v1", 1)
        .expect("manifest reconstruction");

    let original = serde_json::to_value(&packet.draft).unwrap();
    let rebuilt = serde_json::to_value(&reconstructed).unwrap();
    assert_eq!(original, rebuilt);
}

#[test]
fn reconstruction_fails_for_missing_altered_replaced_or_wrongly_bound_objects() {
    let (mut missing_store, packet) = published();
    let object_to_remove = packet
        .manifest
        .objects
        .iter()
        .find(|entry| entry.kind == "claim")
        .expect("claim entry")
        .object_id
        .clone();
    assert!(missing_store.remove_object(&object_to_remove));
    assert!(missing_store
        .reconstruct(&packet.manifest, "campus-ai-v1", 1)
        .is_err());

    let (mut altered_store, packet) = published();
    let root = altered_store
        .object_bytes_mut(&packet.manifest.root_object_id)
        .expect("root object");
    root.push(b' ');
    assert!(altered_store
        .reconstruct(&packet.manifest, "campus-ai-v1", 1)
        .is_err());

    let (mut replaced_store, packet) = published();
    let entries: Vec<_> = packet
        .manifest
        .objects
        .iter()
        .filter(|entry| entry.kind != "packet_draft")
        .collect();
    let first_id = entries[0].object_id.clone();
    let second_id = entries[1].object_id.clone();
    let replacement = replaced_store.object_bytes(&second_id).unwrap().to_vec();
    *replaced_store.object_bytes_mut(&first_id).unwrap() = replacement;
    assert!(replaced_store
        .reconstruct(&packet.manifest, "campus-ai-v1", 1)
        .is_err());

    let (store, packet) = published();
    assert!(store
        .reconstruct(&packet.manifest, "campus-ai-v1", 2)
        .is_err());
    assert!(store
        .reconstruct(&packet.manifest, "other-packet", 1)
        .is_err());
}
