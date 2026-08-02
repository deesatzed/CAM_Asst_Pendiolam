# Common Reality Gate 2 Packet Studio Implementation Plan

> **For Codex:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Finish Gate 2 as a tested, local-first Packet Studio that imports bounded synthetic sources, edits typed evidence objects, reviews and publishes immutable packet versions, and reconstructs them from a content-addressed manifest.

**Architecture:** Add a `common-reality-studio` Rust crate as the authoritative Gate 2 domain. It owns a tagged typed object graph, fail-closed review reducer, deterministic object/manifest store, version diff, local receipt, and reconstruction checks. It projects valid drafts through the existing strict packet schema and reuses the existing canonical, ledger, Merkle, and policy crates. Add an offline-capable React/TypeScript PWA that renders the same typed states and failure reasons from local synthetic data; it is an inspection surface, not an authority.

**Tech Stack:** Rust 2021, serde/serde_json, SHA-256, existing Common Reality crates, React 18, TypeScript, Vite, Vitest, npm workspaces.

---

### Task 1: Add generic deterministic canonical bytes

**Files:**
- Modify: `crates/common-reality-canonical/src/lib.rs`
- Test: `crates/common-reality-canonical/tests/generic_value.rs`

**Step 1: Write the failing test**

Require equivalent generic JSON values to produce identical canonical bytes and
different values to produce different `sha256:` content IDs without requiring a
legacy Shared Reality Packet.

**Step 2: Run test to verify it fails**

Run: `cargo test -p common-reality-canonical --test generic_value`

Expected: compile failure because the generic helper is absent.

**Step 3: Write minimal implementation**

Expose deterministic recursive object-key ordering and generic SHA-256 helpers
while preserving the existing packet-validating `content_id` API.

**Step 4: Run test to verify it passes**

Run: `cargo test -p common-reality-canonical --test generic_value`

Expected: PASS.

**Step 5: Commit**

```bash
git add crates/common-reality-canonical
git commit -m "feat: expose generic canonical content identities"
```

### Task 2: Add the typed Packet Studio domain and source import

**Files:**
- Modify: `Cargo.toml`
- Create: `crates/common-reality-studio/Cargo.toml`
- Create: `crates/common-reality-studio/src/lib.rs`
- Create: `crates/common-reality-studio/tests/source_import.rs`
- Create: `crates/common-reality-studio/tests/typed_objects.rs`

**Step 1: Write the failing tests**

Test bounded local-file source import, stable digest/object storage, provenance
metadata, rejection of an over-limit source, and parsing the frozen fixture into
distinct tagged objects for claim, evidence, counterevidence, uncertainty,
interpretation, value, action, and minority report.

**Step 2: Run test to verify it fails**

Run: `cargo test -p common-reality-studio --test source_import`

Expected: compile failure because the crate and import API are absent.

**Step 3: Write minimal implementation**

Add a `TypedObject` enum, typed records with explicit IDs and references, a
bounded `ObjectStore`, source-file import by SHA-256 content ID, and a
`PacketStudioDraft::from_legacy_packet` adapter. Keep raw source bytes only in
the object store and keep source provenance in the typed metadata object.

**Step 4: Run focused tests**

Run: `cargo test -p common-reality-studio --test source_import --test typed_objects`

Expected: PASS.

**Step 5: Run the workspace baseline**

Run: `cargo test --workspace`

Expected: all prior tests plus the new focused tests pass.

**Step 6: Commit**

```bash
git add Cargo.toml Cargo.lock crates/common-reality-studio
git commit -m "feat: add typed Packet Studio source domain"
```

### Task 3: Implement fail-closed review and legacy packet projection

**Files:**
- Modify: `crates/common-reality-studio/src/lib.rs`
- Create: `crates/common-reality-studio/tests/review.rs`

**Step 1: Write the failing tests**

Require review to reject dangling references, an unsupported claim marked
supported, missing material counterevidence, missing uncertainty, incomplete
minority reports, and typed-kind substitutions. Require a valid frozen fixture
draft to project through `validate_packet`.

**Step 2: Run test to verify it fails**

Run: `cargo test -p common-reality-studio --test review`

Expected: FAIL because the review reducer and projection are absent.

**Step 3: Write minimal implementation**

Implement deterministic findings with stable codes and severities, a blocking
review result, typed reference checks, required counterevidence/uncertainty
visibility, and a legacy JSON projection validated by the existing schema crate.
Do not add truth scores, model decisions, or hidden profile fields.

**Step 4: Run focused and aggregate tests**

Run: `cargo test -p common-reality-studio --test review && cargo test --workspace`

Expected: PASS with all negative cases remaining asserted.

**Step 5: Commit**

```bash
git add crates/common-reality-studio
git commit -m "feat: enforce fail-closed Packet Studio review"
```

### Task 4: Publish immutable versions with manifest, diff, receipt, and reconstruction

**Files:**
- Modify: `crates/common-reality-studio/src/lib.rs`
- Create: `crates/common-reality-studio/tests/publication.rs`
- Create: `crates/common-reality-studio/tests/reconstruction.rs`

**Step 1: Write the failing tests**

Require exact authorization for publication, deterministic object IDs and
manifest digest, immutable publication, parent/version change sets, a receipt
without raw source bytes, byte-for-byte reconstruction, and failure when an
object is missing, altered, replaced, or attached to another packet/version.

**Step 2: Run test to verify it fails**

Run: `cargo test -p common-reality-studio --test publication --test reconstruction`

Expected: FAIL because publication and reconstruction APIs are absent.

**Step 3: Write minimal implementation**

Persist canonical typed objects plus a root draft in a bounded content-addressed
store. Build a deterministic sorted manifest, a local-only receipt, a ledger
checkpoint, and a `PacketDiff` containing additions, removals, and typed-field
changes but no truth score. Reject duplicate versions and verify every hash,
packet identity, version, and manifest digest during reconstruction.

**Step 4: Run focused and aggregate tests**

Run: `cargo test -p common-reality-studio --test publication --test reconstruction && cargo test --workspace`

Expected: PASS.

**Step 5: Commit**

```bash
git add crates/common-reality-studio
git commit -m "feat: publish and reconstruct immutable packet versions"
```

### Task 5: Add the executable Gate 2 verifier and evidence scaffold

**Files:**
- Create: `scripts/verify-gate2.sh`
- Create: `crates/common-reality-studio/examples/verify_gate2.rs`
- Create: `docs/evidence/gate-2-packet-studio.md`

**Step 1: Write the failing verifier test/example**

Make the example exercise the frozen synthetic packet end to end, including
negative review cases, publication, receipt redaction, diff, and destructive
reconstruction checks.

**Step 2: Run it to verify failure**

Run: `cargo run -p common-reality-studio --example verify_gate2`

Expected: compile or assertion failure while the verifier is incomplete.

**Step 3: Implement the smallest verifier path**

Add a strict shell wrapper that runs the example and emits a machine-readable
summary. Save command output and artifact paths in the evidence document; label
all Campus AI content synthetic and all receipts local integrity only.

**Step 4: Run the verifier**

Run: `./scripts/verify-gate2.sh`

Expected: PASS and evidence includes source/object IDs, review findings,
publication receipt, reconstruction checks, and limitations.

**Step 5: Commit**

```bash
git add scripts crates/common-reality-studio/examples docs/evidence/gate-2-packet-studio.md
git commit -m "test: add executable Gate 2 packet proof"
```

### Task 6: Build the local-first React/TypeScript Packet Studio surface

**Files:**
- Create: `package.json`
- Create: `apps/web/package.json`
- Create: `apps/web/tsconfig.json`
- Create: `apps/web/vite.config.ts`
- Create: `apps/web/index.html`
- Create: `apps/web/src/main.tsx`
- Create: `apps/web/src/App.tsx`
- Create: `apps/web/src/studioModel.ts`
- Create: `apps/web/src/styles.css`
- Create: `apps/web/tests/studioModel.test.ts`

**Step 1: Write the failing UI/model tests**

Require rendered local source-frozen status, typed section names, visible
counterevidence and uncertainty, blocked review reasons, receipt integrity
fields, reconstruction status, deterministic diff labels, and absence of
truth-score/neutrality language.

**Step 2: Run test to verify it fails**

Run: `npm test --workspace apps/web`

Expected: workspace/package/test failure because the app is absent.

**Step 3: Write minimal implementation**

Create a React app whose model is a local static representation of the same
synthetic packet and typed statuses. Render all required views, failure states,
and local-only disclaimers. Keep no network calls, credentials, telemetry, or
model client.

**Step 4: Run tests and build**

Run: `npm test --workspace apps/web && npm run build --workspace apps/web`

Expected: PASS with a deterministic `dist/` build.

**Step 5: Commit**

```bash
git add package.json package-lock.json apps/web
git commit -m "feat: add local-first Packet Studio web surface"
```

### Task 7: Aggregate verification, adversarial audit, evidence, and final commit

**Files:**
- Modify: `docs/evidence/gate-2-packet-studio.md`
- Modify: `PROGRESS.md`
- Modify: `DECISIONS.md`
- Modify: `REPO_MAP.md`
- Modify: `RISK_NOTES.md`

**Step 1: Run the full proof set**

Run:

```bash
cargo fmt --all -- --check
cargo test --workspace
npm test --workspace apps/web
npm run build --workspace apps/web
./scripts/verify-gate2.sh
git diff --check
```

Expected: every command exits 0 and the verifier reports no unresolved Gate 2
blocking finding.

**Step 2: Inspect generated evidence**

Confirm the saved evidence names every changed file, command, test count,
artifact path, limitation, and non-claim; confirm no raw source bytes appear in
receipts and no frozen fixture digest changed.

**Step 3: Commit**

```bash
git add .
git commit -m "docs: record Gate 2 completion evidence"
```

Do not push or deploy unless separately requested.
