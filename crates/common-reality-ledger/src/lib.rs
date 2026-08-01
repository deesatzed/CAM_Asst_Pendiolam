use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub enum OperationState {
    Draft,
    SourceFrozen,
    Reviewed,
    Published,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct LedgerEvent {
    pub sequence: u64,
    pub packet_id: String,
    pub packet_version: u64,
    pub state: OperationState,
    pub previous_hash: String,
    pub event_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LedgerIntegrity {
    LocalReplayOnly,
    TerminalTruncationUnwitnessed,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MerkleCheckpoint {
    pub leaf_count: u64,
    pub root: String,
}

impl MerkleCheckpoint {
    pub fn from_events(events: &[LedgerEvent]) -> Self {
        let mut nodes: Vec<String> = events
            .iter()
            .map(|event| event.event_hash.clone())
            .collect();
        if nodes.is_empty() {
            return Self {
                leaf_count: 0,
                root: hash_node(""),
            };
        }

        while nodes.len() > 1 {
            let mut parents = Vec::with_capacity(nodes.len().div_ceil(2));
            for pair in nodes.chunks(2) {
                let right = pair.get(1).unwrap_or(&pair[0]);
                parents.push(hash_node(&format!("{}|{}", pair[0], right)));
            }
            nodes = parents;
        }

        Self {
            leaf_count: events.len() as u64,
            root: nodes.pop().expect("non-empty Merkle nodes"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LedgerError(String);

impl fmt::Display for LedgerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for LedgerError {}

pub struct Ledger {
    packet_id: String,
    packet_version: u64,
    state: OperationState,
    events: Vec<LedgerEvent>,
}

impl Ledger {
    pub fn new(packet_id: impl Into<String>, packet_version: u64) -> Self {
        Self {
            packet_id: packet_id.into(),
            packet_version,
            state: OperationState::Draft,
            events: Vec::new(),
        }
    }

    pub fn append(
        &mut self,
        expected_sequence: u64,
        packet_id: &str,
        packet_version: u64,
        state: OperationState,
    ) -> Result<(), LedgerError> {
        if expected_sequence != self.events.len() as u64 {
            return Err(LedgerError("stale writer sequence".to_owned()));
        }
        if packet_id != self.packet_id || packet_version != self.packet_version {
            return Err(LedgerError(
                "packet identity or version mismatch".to_owned(),
            ));
        }
        ensure_transition(&self.state, &state)?;

        let sequence = expected_sequence + 1;
        let previous_hash = self
            .events
            .last()
            .map(|event| event.event_hash.clone())
            .unwrap_or_else(|| genesis_hash(&self.packet_id, self.packet_version));
        let event_hash = hash_event(sequence, packet_id, packet_version, &state, &previous_hash);
        self.events.push(LedgerEvent {
            sequence,
            packet_id: packet_id.to_owned(),
            packet_version,
            state: state.clone(),
            previous_hash,
            event_hash,
        });
        self.state = state;
        Ok(())
    }

    pub fn from_events(
        packet_id: impl Into<String>,
        packet_version: u64,
        events: Vec<LedgerEvent>,
    ) -> Result<Self, LedgerError> {
        let mut replayed = Self::new(packet_id, packet_version);
        for event in events {
            let expected_sequence = replayed.events.len() as u64 + 1;
            if event.sequence != expected_sequence
                || event.packet_id != replayed.packet_id
                || event.packet_version != replayed.packet_version
            {
                return Err(LedgerError(
                    "invalid ledger event identity or sequence".to_owned(),
                ));
            }
            let expected_previous_hash = replayed
                .events
                .last()
                .map(|prior| prior.event_hash.clone())
                .unwrap_or_else(|| genesis_hash(&replayed.packet_id, replayed.packet_version));
            if event.previous_hash != expected_previous_hash {
                return Err(LedgerError("ledger chain discontinuity".to_owned()));
            }
            let expected_event_hash = hash_event(
                event.sequence,
                &event.packet_id,
                event.packet_version,
                &event.state,
                &event.previous_hash,
            );
            if event.event_hash != expected_event_hash {
                return Err(LedgerError("ledger event mutation detected".to_owned()));
            }
            ensure_transition(&replayed.state, &event.state)?;
            replayed.state = event.state.clone();
            replayed.events.push(event);
        }
        Ok(replayed)
    }

    pub fn state(&self) -> OperationState {
        self.state.clone()
    }

    pub fn events(&self) -> Vec<LedgerEvent> {
        self.events.clone()
    }

    pub fn integrity_status(&self) -> LedgerIntegrity {
        if self.state == OperationState::Published {
            LedgerIntegrity::TerminalTruncationUnwitnessed
        } else {
            LedgerIntegrity::LocalReplayOnly
        }
    }

    pub fn merkle_checkpoint(&self) -> MerkleCheckpoint {
        MerkleCheckpoint::from_events(&self.events)
    }
}

fn ensure_transition(current: &OperationState, next: &OperationState) -> Result<(), LedgerError> {
    if matches!(
        (current, next),
        (OperationState::Draft, OperationState::SourceFrozen)
            | (OperationState::SourceFrozen, OperationState::Reviewed)
            | (OperationState::Reviewed, OperationState::Published)
    ) {
        Ok(())
    } else {
        Err(LedgerError("invalid operation state transition".to_owned()))
    }
}

fn genesis_hash(packet_id: &str, packet_version: u64) -> String {
    format!(
        "sha256:{:x}",
        Sha256::digest(format!("{packet_id}:{packet_version}:genesis"))
    )
}

fn hash_event(
    sequence: u64,
    packet_id: &str,
    packet_version: u64,
    state: &OperationState,
    previous_hash: &str,
) -> String {
    #[derive(Serialize)]
    struct HashMaterial<'a> {
        sequence: u64,
        packet_id: &'a str,
        packet_version: u64,
        state: &'a OperationState,
        previous_hash: &'a str,
    }

    let bytes = serde_json::to_vec(&HashMaterial {
        sequence,
        packet_id,
        packet_version,
        state,
        previous_hash,
    })
    .expect("fixed ledger hash material serializes");
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn hash_node(value: &str) -> String {
    format!("sha256:{:x}", Sha256::digest(value))
}
