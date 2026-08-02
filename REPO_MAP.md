# REPO_MAP.md

## Project Type

Standalone Common Reality constitutional core, currently Rust-only, with a
planned local-first React/TypeScript Packet Studio for Gate 2.

## Tech Stack

- Rust 2021 workspace
- `serde`/`serde_json` for typed JSON and strict fixture validation
- SHA-256 via `sha2` for canonical identities, fixture digests, and ledger hashes
- React/TypeScript PWA to be added under `apps/web`

## Package Manager

- Cargo for Rust packages
- npm workspaces for the Gate 2 web package (not present at the baseline)

## Commands

| Purpose | Command | Verified |
|---|---|---|
| Rust tests | `cargo test --workspace` | yes, baseline: 18 tests passed |
| Rust formatting check | `cargo fmt --all -- --check` | pending Gate 2 batch |
| Web tests | `npm test --workspace apps/web` | pending; app does not yet exist |
| Web build | `npm run build --workspace apps/web` | pending; app does not yet exist |
| Gate 2 verifier | `./scripts/verify-gate2.sh` | pending; script does not yet exist |
| Whitespace validation | `git diff --check` | pending after edits |

## Entry Points

- Rust schema: `crates/common-reality-schema/src/lib.rs`
- Canonical identity: `crates/common-reality-canonical/src/lib.rs`
- Replayable ledger and Merkle checkpoint: `crates/common-reality-ledger/src/lib.rs`
- Deny-by-default grants: `crates/common-reality-policy/src/lib.rs`
- Frozen synthetic packet: `fixtures/campus-ai-v1/packet.json`
- Gate 2 goal contract: `GOAL_Gate2.md`

## Major Folders

- `crates/`: typed schema, canonicalization, ledger, and policy crates
- `fixtures/campus-ai-v1/`: frozen synthetic Gate 0 corpus and attack fixtures
- `schemas/`: strict versioned packet schema artifacts
- `docs/plans/`: implementation and design plans
- `docs/evidence/`: saved verification receipts
- `apps/web/`: reserved for the Gate 2 Packet Studio UI
- `scripts/`: reserved for reproducible Gate 2 verification

## Existing Patterns To Preserve

- Fail-closed validation with explicit error text
- Closed serde objects and deterministic cross-reference checks
- Content IDs prefixed with `sha256:`
- Append-only state transitions (`draft -> source_frozen -> reviewed -> published`)
- Deny-by-default authorization bound to principal, action, resource, purpose,
  packet identity, state digest, expiry, and revocation
- Synthetic-only fixtures; no CAM Assistant or Pendoleum runtime coupling

## Tests and Verification

The baseline `cargo test --workspace` passed 18 tests across schema,
canonicalization, ledger, Merkle, and policy. Gate 2 must retain those tests and
add focused Rust and web tests plus a saved end-to-end verifier receipt.

## Likely Files For Current Task

- `crates/common-reality-studio/` for typed Packet Studio domain, review,
  publication, manifest/object storage, diff, and reconstruction
- `crates/common-reality-canonical/src/lib.rs` for generic canonical JSON bytes
- `apps/web/` for local-first React/TypeScript views and tests
- `scripts/verify-gate2.sh` for synthetic end-to-end proof
- `docs/evidence/gate-2-packet-studio.md` for current proof output

## Unknowns

- No JavaScript package manifest or dependency lockfile exists yet.
- Existing schema types deserialize the Gate 0 packet but do not serialize a
  Gate 2 typed object graph; the studio crate must own its versioned typed model
  and validate its legacy packet projection through the existing schema crate.
- No persistent database is required for Gate 2; a bounded content-addressed
  local object store is sufficient and safer for the proof surface.
