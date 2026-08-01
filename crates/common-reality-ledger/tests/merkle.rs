use common_reality_ledger::{Ledger, MerkleCheckpoint, OperationState};

fn reviewed_ledger() -> Ledger {
    let mut ledger = Ledger::new("campus-ai-v1", 1);
    ledger
        .append(0, "campus-ai-v1", 1, OperationState::SourceFrozen)
        .unwrap();
    ledger
        .append(1, "campus-ai-v1", 1, OperationState::Reviewed)
        .unwrap();
    ledger
}

#[test]
fn identical_event_history_produces_an_identical_merkle_root() {
    let ledger = reviewed_ledger();

    assert_eq!(ledger.merkle_checkpoint(), ledger.merkle_checkpoint());
}

#[test]
fn a_changed_event_produces_a_new_merkle_root() {
    let ledger = reviewed_ledger();
    let original = ledger.merkle_checkpoint();
    let mut changed_events = ledger.events();
    changed_events[1].event_hash = "sha256:forged".into();

    assert_ne!(original, MerkleCheckpoint::from_events(&changed_events));
}
