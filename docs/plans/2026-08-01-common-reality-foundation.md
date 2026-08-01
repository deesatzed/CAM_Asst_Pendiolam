# Common Reality Foundation Implementation Plan

> **For Codex:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build the first evidence-bound Common Reality constitutional core and
its frozen Campus AI Constitution evaluation surface.

**Architecture:** A Rust workspace owns strict typed objects, canonical bytes,
content identities, deterministic policy, and replayable ledger state. A later
React PWA presents Ground, Lens, Bridge, and the Disagreement Map without
allowing presentation or model output to become authority.

**Tech Stack:** Rust stable, serde, JSON Schema 2020-12 artifacts, SHA-256,
SQLite in the ledger phase, React/TypeScript PWA after the constitutional core.

---

### Task 1: Initialize the Rust workspace and test harness

**Files:**
- Create: `Cargo.toml`
- Create: `crates/common-reality-schema/Cargo.toml`
- Create: `crates/common-reality-schema/src/lib.rs`
- Create: `crates/common-reality-schema/tests/fixture_manifest.rs`
- Create: `.gitignore`

**Step 1: Write the failing test**

Create `fixture_manifest.rs` calling `fixture_manifest_path()` from the absent
crate and assert that the frozen manifest path exists.

**Step 2: Run test to verify it fails**

Run: `cargo test -p common-reality-schema --test fixture_manifest`

Expected: compilation failure because the workspace/crate does not exist.

**Step 3: Write minimal implementation**

Create the workspace and crate. Return the repository-relative manifest path;
do not yet parse or validate its content.

**Step 4: Run test to verify it passes**

Run: `cargo test -p common-reality-schema --test fixture_manifest`

Expected: PASS.

**Step 5: Commit**

```bash
git add Cargo.toml .gitignore crates/common-reality-schema
git commit -m "build: initialize constitutional schema workspace"
```

### Task 2: Freeze the Gate 0 fixture manifest

**Files:**
- Create: `fixtures/campus-ai-v1/manifest.json`
- Create: `fixtures/campus-ai-v1/packet.json`
- Create: `fixtures/campus-ai-v1/attacks/*.json`
- Modify: `crates/common-reality-schema/tests/fixture_manifest.rs`

**Step 1: Write failing tests**

Add tests requiring a manifest schema version, fixture entries with SHA-256
digests, the synthetic source corpus, expected invariants, allowed Lens fields,
model/vendor swap cases, all named attack classes, and hash equality with
fixture bytes.

**Step 2: Run test to verify it fails**

Run: `cargo test -p common-reality-schema --test fixture_manifest`

Expected: missing fixture/manifest or invalid manifest failure.

**Step 3: Write minimal implementation**

Add synthetic Campus AI packet, source corpus, expected-invariant, Lens-field,
model-swap, and adversarial JSON fixtures. Add a strict manifest parser and
SHA-256 verifier. Never use a live web source or model output in Gate 0.

**Step 4: Run tests to verify they pass**

Run: `cargo test -p common-reality-schema --test fixture_manifest`

Expected: PASS; each fixture digest matches and all required attacks exist.

**Step 5: Commit**

```bash
git add fixtures/campus-ai-v1 crates/common-reality-schema
git commit -m "test: freeze Campus AI Gate 0 fixtures"
```

### Task 3: Add strict Shared Reality Packet validation

**Files:**
- Create: `schemas/shared-reality-packet-v1.schema.json`
- Create: `crates/common-reality-schema/src/packet.rs`
- Create: `crates/common-reality-schema/tests/packet_validation.rs`
- Modify: `crates/common-reality-schema/src/lib.rs`

**Step 1: Write failing tests**

Test acceptance of the frozen packet and rejection of unknown nested fields,
dangling evidence references, missing counterevidence disclosure, and forbidden
Lens profile fields.

**Step 2: Run test to verify it fails**

Run: `cargo test -p common-reality-schema --test packet_validation`

Expected: compilation failure for absent `validate_packet` or failing fixture.

**Step 3: Write minimal implementation**

Define closed serde object types and deterministic cross-reference validation.
Keep claims, evidence, interpretations, values, action options, uncertainty,
and minority reports separate. Permit only declared presentation fields.

**Step 4: Run tests to verify they pass**

Run: `cargo test -p common-reality-schema --test packet_validation`

Expected: PASS.

**Step 5: Commit**

```bash
git add schemas crates/common-reality-schema
git commit -m "feat: validate strict Shared Reality Packets"
```

### Task 4: Canonical packet identity and immutable receipt

**Files:**
- Create: `crates/common-reality-canonical/Cargo.toml`
- Create: `crates/common-reality-canonical/src/lib.rs`
- Create: `crates/common-reality-canonical/tests/content_id.rs`

**Step 1: Write failing tests**

Require semantically identical packet values with different input key ordering
to receive the same SHA-256 identity and one-bit changes to receive a different
identity.

**Step 2: Run test to verify it fails**

Run: `cargo test -p common-reality-canonical --test content_id`

Expected: missing crate/function failure.

**Step 3: Write minimal implementation**

Implement deterministic canonical JSON serialization and content identity. Do
not introduce signatures, public witnessing, or private data storage yet.

**Step 4: Run tests to verify they pass**

Run: `cargo test -p common-reality-canonical --test content_id`

Expected: PASS.

**Step 5: Commit**

```bash
git add Cargo.toml crates/common-reality-canonical
git commit -m "feat: add canonical packet content identities"
```

### Task 5: Gate 1 append-only operation ledger and replay

**Files:**
- Create: `crates/common-reality-ledger/Cargo.toml`
- Create: `crates/common-reality-ledger/src/lib.rs`
- Create: `crates/common-reality-ledger/tests/replay.rs`

**Step 1: Write failing tests**

Require ordered event replay to reconstruct state and reject mutation,
deletion, reorder, stale writer, and packet-version mismatch.

**Step 2: Run test to verify it fails**

Run: `cargo test -p common-reality-ledger --test replay`

Expected: missing ledger failure.

**Step 3: Write minimal implementation**

Add content-addressed events, parent hash chaining, deterministic reducer
states, and a terminal truncation status that is explicitly not externally
witnessed.

**Step 4: Run tests to verify they pass**

Run: `cargo test -p common-reality-ledger --test replay`

Expected: PASS.

**Step 5: Commit**

```bash
git add Cargo.toml crates/common-reality-ledger
git commit -m "feat: add replayable packet operation ledger"
```

### Task 6: Build the first Ground and Lens PWA journey

**Files:**
- Create: `apps/web/package.json`
- Create: `apps/web/src/*`
- Create: `apps/web/tests/*`

**Step 1: Write failing tests**

Require the PWA to display the canonical packet, a declared-profile Lens,
omissions, profile fields used, and the Lens Referee verdict.

**Step 2: Run test to verify it fails**

Run: `npm test --workspace apps/web`

Expected: missing application/test failure.

**Step 3: Write minimal implementation**

Build a local-only UI over deterministic Rust/WASM validation. Never invoke a
model or infer a profile.

**Step 4: Run tests to verify they pass**

Run: `npm test --workspace apps/web`

Expected: PASS.

**Step 5: Commit**

```bash
git add apps/web
git commit -m "feat: add inspectable Ground and Lens journey"
```

### Task 7: Bridge, Disagreement Map, and reversible action proof

**Files:**
- Create: `crates/common-reality-deliberation/*`
- Create: `apps/web/src/features/bridge/*`
- Create: `evaluations/disagreement/*`

**Step 1: Write failing tests**

Require typed disputes, packet-version matching, purpose-bound grants,
minority preservation, traceability, and reversible action stop conditions.

**Step 2: Run test to verify it fails**

Run: `cargo test -p common-reality-deliberation`

Expected: missing deliberation core failure.

**Step 3: Write minimal implementation**

Implement deterministic aggregation returning consensus, majority, minority,
unresolved, uncertainty, missing evidence, and action-compatible overlap.

**Step 4: Run tests to verify they pass**

Run: `cargo test -p common-reality-deliberation`

Expected: PASS.

**Step 5: Commit**

```bash
git add crates/common-reality-deliberation apps/web evaluations
git commit -m "feat: preserve typed disagreement and reversible action"
```
