# Gate 2 Packet Studio Evidence

## Status

`partial` — the Rust Packet Studio core and executable local integrity proof are
implemented. The React/TypeScript Packet Studio, npm tests/build, and final
aggregate Gate 2 evidence remain pending. This document must not be read as a
claim that Gate 2 is complete.

## Environment

- Repository: `/Volumes/WS4TB/pendoleum/common-reality`
- Branch at capture: `main`
- Rust: `rustc 1.89.0`, `cargo 1.89.0`
- Node/npm were inspected at `v26.5.0` / `11.17.0`; the web package does not
  exist yet.
- Campus AI fixture corpus: synthetic and local only.

## Commands run

```bash
cargo fmt --all -- --check
cargo test --workspace
cargo run -p common-reality-studio --example verify_gate2
./scripts/verify-gate2.sh
git diff --check
```

The workspace tests and formatter passed after the typed core batches. The
offline verifier command passed with `CARGO_NET_OFFLINE=true` inside the shell
script.

## Verifier output

Captured from `./scripts/verify-gate2.sh`:

```text
GATE2_STATUS=PASS
FIXTURE_MANIFEST=PASS
SOURCE_IMPORT=PASS digest=sha256:a51372e6aedbb5c7bc4b920f12e955cc38c4b583666c2b32113ab3a38e660d38 bytes=536
TYPED_OBJECTS=PASS count=12
REVIEW=PASS findings=0
PUBLICATION=PASS packet_content_id=sha256:44b82a7f98df10847ecb016538b9eba539d0a4b92db66010a9fa9f6d9be30270 version=1
RECONSTRUCTION=PASS manifest_digest=sha256:84bcfdd478a1d3731bf6eef5e742c7531de5552626d7b511f70337512e738ede objects=13
CORRUPTION_GATES=PASS missing altered wrong_version
DIFF=PASS changed_fields=1
LIMITATION=synthetic-local-integrity-only
```

The receipt contains only packet/version IDs, digests, object IDs, review and
ledger metadata, and the explicit local-integrity limitation. It does not
contain the imported source bytes or source text.

## Core proof coverage

- The frozen Gate 0 manifest is revalidated before the run.
- The bounded local source import produces a stable SHA-256 object ID and
  retains origin, retrieval time, type, limitations, and byte length in typed
  provenance metadata.
- The draft contains distinct `source`, `claim`, `evidence`, `counterevidence`,
  `uncertainty`, `interpretation`, `value`, `action`, and `minority_report`
  variants.
- Review rejects unsupported claims marked `supported`, missing material
  counterevidence, missing uncertainty, invalid references, and incomplete
  minority reports.
- Publication requires an exact local capability grant and rejects the absent
  grant.
- Published versions are immutable, successor versions record their parent, and
  the deterministic diff reports typed field changes without a truth score.
- Manifest/object reconstruction is byte-for-byte for the published draft and
  fails on missing, altered, or wrongly bound objects/version.

## Changed files in this partial capture

- `Cargo.toml`, `Cargo.lock`
- `crates/common-reality-canonical/src/lib.rs`
- `crates/common-reality-canonical/tests/generic_value.rs`
- `crates/common-reality-studio/Cargo.toml`
- `crates/common-reality-studio/src/lib.rs`
- `crates/common-reality-studio/tests/source_import.rs`
- `crates/common-reality-studio/tests/typed_objects.rs`
- `crates/common-reality-studio/tests/review.rs`
- `crates/common-reality-studio/tests/publication.rs`
- `crates/common-reality-studio/tests/reconstruction.rs`
- `crates/common-reality-studio/examples/verify_gate2.rs`
- `scripts/verify-gate2.sh`
- repository planning, README, risk, progress, and decision documents

## Remaining proof

- Add `apps/web` with inspectable local source, typed sections, review blockers,
  receipt/reconstruction, and deterministic diff views.
- Run and save `npm test --workspace apps/web` and
  `npm run build --workspace apps/web`.
- Re-run the complete Gate 2 command set from a clean worktree and update this
  document to `complete` only if every numbered `GOAL_Gate2.md` proof item is
  current and passes.

## Non-claims

This evidence does not establish truth, neutrality, human-pilot success, model
quality, external witnessing, signatures, encryption, democratic legitimacy,
or Gate 3/4 deliberation behavior.
