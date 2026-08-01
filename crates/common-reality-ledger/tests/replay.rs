use common_reality_ledger::{Ledger, LedgerIntegrity, OperationState};

fn published_ledger() -> Ledger {
    let mut ledger = Ledger::new("campus-ai-v1", 1);
    ledger
        .append(0, "campus-ai-v1", 1, OperationState::SourceFrozen)
        .unwrap();
    ledger
        .append(1, "campus-ai-v1", 1, OperationState::Reviewed)
        .unwrap();
    ledger
        .append(2, "campus-ai-v1", 1, OperationState::Published)
        .unwrap();
    ledger
}

#[test]
fn replay_reconstructs_the_exact_operation_state() {
    let ledger = published_ledger();

    let replayed =
        Ledger::from_events("campus-ai-v1", 1, ledger.events()).expect("valid ledger replay");

    assert_eq!(replayed.state(), OperationState::Published);
    assert_eq!(replayed.events().len(), 3);
}

#[test]
fn replay_detects_mutation_deletion_and_reordering() {
    let ledger = published_ledger();

    let mut mutated = ledger.events();
    mutated[1].state = OperationState::Published;
    assert!(Ledger::from_events("campus-ai-v1", 1, mutated).is_err());

    let mut deleted = ledger.events();
    deleted.remove(1);
    assert!(Ledger::from_events("campus-ai-v1", 1, deleted).is_err());

    let mut reordered = ledger.events();
    reordered.swap(0, 1);
    assert!(Ledger::from_events("campus-ai-v1", 1, reordered).is_err());
}

#[test]
fn stale_writers_and_packet_version_mismatches_fail_closed() {
    let mut ledger = Ledger::new("campus-ai-v1", 1);
    ledger
        .append(0, "campus-ai-v1", 1, OperationState::SourceFrozen)
        .unwrap();

    assert!(ledger
        .append(0, "campus-ai-v1", 1, OperationState::Reviewed)
        .is_err());
    assert!(ledger
        .append(1, "campus-ai-v1", 2, OperationState::Reviewed)
        .is_err());
}

#[test]
fn terminal_history_without_an_external_witness_is_labeled_honestly() {
    let ledger = published_ledger();

    assert_eq!(
        ledger.integrity_status(),
        LedgerIntegrity::TerminalTruncationUnwitnessed
    );
}
