# Gate 2 Packet Studio Evidence

## Status

`complete` — the Rust Packet Studio core, executable local integrity proof, and
React/TypeScript inspection surface are implemented and verified below. The
evidence remains limited to synthetic/local integrity semantics; it is not a
human-pilot, truth, neutrality, or later-gate claim.

## Environment

- Repository: `/Volumes/WS4TB/pendoleum/common-reality`
- Branch at capture: `main`
- Rust: `rustc 1.89.0`, `cargo 1.89.0`
- Node/npm: `v26.5.0` / `11.17.0`.
- Campus AI fixture corpus: synthetic and local only.

## Commands run

```bash
cargo fmt --all -- --check
cargo test --workspace
NPM_CONFIG_OFFLINE=true npm test --workspace apps/web
NPM_CONFIG_OFFLINE=true npm run build --workspace apps/web
./scripts/verify-gate2.sh
npm audit --omit=dev
npm audit
git diff --check
```

The formatter, Rust workspace (32 integration tests, 0 failures), web suite (2
tests, 0 failures), web production build, offline verifier, both npm audits,
and whitespace check passed. The verifier command uses `CARGO_NET_OFFLINE=true`
inside the shell script. The npm commands use the lockfile-installed local
toolchain; they make no application network calls.

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
- The web surface renders source-frozen provenance, packet identity, all typed
  sections, local editors, blocked review reasons, receipt redaction,
  reconstruction state, and deterministic diff without truth-score language.

## Changed files in this capture

The following files are the complete `git diff --name-only 39cdf9d..HEAD`
inventory at the final audit:

- `.gitignore`
- `Cargo.lock`
- `Cargo.toml`
- `DECISIONS.md`
- `GOAL_Gate2.md`
- `PROGRESS.md`
- `README.md`
- `REPO_MAP.md`
- `RISK_NOTES.md`
- `apps/web/index.html`
- `apps/web/package.json`
- `apps/web/src/App.tsx`
- `apps/web/src/main.tsx`
- `apps/web/src/studioModel.ts`
- `apps/web/src/styles.css`
- `apps/web/tests/studioModel.test.tsx`
- `apps/web/tsconfig.json`
- `apps/web/vite.config.ts`
- `crates/common-reality-canonical/src/lib.rs`
- `crates/common-reality-canonical/tests/generic_value.rs`
- `crates/common-reality-studio/Cargo.toml`
- `crates/common-reality-studio/examples/verify_gate2.rs`
- `crates/common-reality-studio/src/lib.rs`
- `crates/common-reality-studio/tests/publication.rs`
- `crates/common-reality-studio/tests/reconstruction.rs`
- `crates/common-reality-studio/tests/review.rs`
- `crates/common-reality-studio/tests/source_import.rs`
- `crates/common-reality-studio/tests/typed_objects.rs`
- `docs/evidence/gate-2-packet-studio.md`
- `docs/plans/2026-08-01-common-reality-gate2-packet-studio.md`
- `package-lock.json`
- `package.json`
- `scripts/verify-gate2.sh`

## Completion audit

All numbered `GOAL_Gate2.md` proof items have current command evidence. The
final pushed worktree is clean, the frozen fixture manifest remains unchanged,
and no CAM Assistant, Pendoleum, donor, live database, or live corpus path was
modified.

## Non-claims

This evidence does not establish truth, neutrality, human-pilot success, model
quality, external witnessing, signatures, encryption, democratic legitimacy,
or Gate 3/4 deliberation behavior.
