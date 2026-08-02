use common_reality_studio::{PacketStudioDraft, TypedObject};

const PACKET: &str = include_str!("../../../fixtures/campus-ai-v1/packet.json");

#[test]
fn frozen_packet_becomes_distinct_tagged_gate_two_objects() {
    let draft = PacketStudioDraft::from_legacy_packet(PACKET).expect("fixture adapter");
    let kinds: Vec<_> = draft.objects().iter().map(TypedObject::kind).collect();

    for expected in [
        "source",
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
            kinds.iter().any(|kind| *kind == expected),
            "missing {expected}"
        );
    }
    assert!(draft.claim_count() > 0);
    assert!(draft.counterevidence_count() > 0);
    assert!(draft.uncertainty_count() == draft.claim_count());
}
