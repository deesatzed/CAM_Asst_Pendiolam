# GOAL_Gate2: Finish the Common Reality Packet Studio

Run this goal from the standalone Common Reality repository root:

```text
/goal GOAL_Gate2.md
```

This goal is an autonomous implementation contract for Gate 2 of
`GOAL_COMMON_REALITY.md`. It starts from the current Rust constitutional core
and builds the first inspectable, local-first Packet Studio for the synthetic
Campus AI Constitution pilot.

## OUTCOME

Complete a tested Packet Studio so an editor can create, inspect, version,
review, publish, and reconstruct one evidence-bound Shared Reality Packet
without network access, model access, private-personal data, or CAM/Pendoleum
runtime access.

The finished slice must make these transitions observable and deterministic:

```text
source import
  -> source frozen
  -> typed packet draft
  -> independent review
  -> published packet version
  -> manifest/object reconstruction
```

The Packet Studio is a local Rust service/core with a React/TypeScript PWA
surface. Models are not required and must not become authorities in this goal.

## PROOF OF DONE

All evidence must come from the current checkout and must be saved in
`docs/evidence/gate-2-packet-studio.md`.

1. `cargo test --workspace` exits 0 with all existing Gate 0 and Gate 1 tests
   still passing.
2. `npm test --workspace apps/web` exits 0 and covers the Packet Studio's
   user-visible states and failure states.
3. `npm run build --workspace apps/web` exits 0 with no required network,
   hosted service, model, or cloud credential.
4. `./scripts/verify-gate2.sh` exits 0 and proves, using only synthetic/local
   fixtures, that:

   - a source can be imported from a bounded local file, receives a stable
     digest, and retains provenance without exposing raw bytes in receipts;
   - claims, supporting evidence, counterevidence, uncertainty,
     interpretations, values, action options, and minority reports are edited
     as distinct typed objects;
   - every claim has a support status and required evidence references;
   - an unsupported claim cannot be published with status `supported`;
   - material counterevidence and uncertainty remain visible in the packet and
     in the UI;
   - fact, evidence, interpretation, value, and action objects cannot be
     silently substituted for one another;
   - packet versions are immutable after publication, and a new version
     records a deterministic change set and parent version;
   - review fails closed for invalid references, missing counterevidence,
     unsupported claims marked supported, missing uncertainty, or an incomplete
     minority report;
   - publication creates a local receipt bound to the packet content ID,
     manifest digest, object IDs, review result, and packet version;
   - the published packet can be reconstructed byte-for-byte from its manifest
     and content-addressed objects;
   - reconstruction fails closed when an object is missing, altered, replaced,
     or associated with the wrong packet/version;
   - packet diff output explains additions, removals, and changed typed fields
     without presenting a scalar truth score.

5. `git diff --check` exits 0.
6. The final report names every changed file, exact commands, test counts,
   artifact paths, limitations, and non-claims. A clean worktree is required
   before completion; do not push or deploy unless separately requested.

## REQUIRED DELIVERABLES

### Packet Studio core

Implement a bounded package (for example
`crates/common-reality-studio`) that owns:

- local source import and source digest/provenance records;
- packet draft construction from typed objects;
- support-status and cross-reference validation;
- counterevidence and uncertainty requirements;
- typed review decisions;
- immutable packet version publication;
- deterministic packet diff;
- content-addressed manifest/object persistence;
- reconstruction and integrity verification;
- local status-only receipts.

Use the existing schema, canonicalization, ledger, Merkle, and policy crates.
Extend them only through tested APIs; do not duplicate their logic.

### Packet Studio web surface

Create `apps/web` as a local-first React/TypeScript application with these
inspectable views:

- Source import and source-frozen state;
- Packet outline and version identity;
- Claim/evidence editor;
- Counterevidence and uncertainty editor;
- Interpretation/value/action/minority sections;
- Review findings and blocked-publication reasons;
- Published receipt and reconstruction check;
- Deterministic packet diff between two versions.

The UI must expose typed sections and status. It must never imply that a
published packet is eternal truth, objective neutrality, or democratic
legitimacy.

### Frozen fixture integration

Use the existing `fixtures/campus-ai-v1` material as synthetic evaluation data.
Do not alter its labels, attack cases, invariant list, profile policy, model
swap cases, or recorded digests after observing results. Add new fixtures only
with a documented reason, new digest, and focused failing test first.

## SCOPE

### May modify

- `GOAL_Gate2.md` only for contract corrections;
- `Cargo.toml`, `Cargo.lock`, and Rust crates under `crates/`;
- `schemas/` with versioned Gate 2 schemas or migration artifacts;
- `apps/web/` and its package/test configuration;
- `scripts/verify-gate2.sh`;
- synthetic fixtures under `fixtures/campus-ai-v1/` only when required by a
  test-first contract and with digests updated transparently;
- `docs/plans/` and `docs/evidence/` for the implementation plan and receipts;
- repository-local README/developer documentation needed to reproduce Gate 2.

### Read/reference

- `GOAL_COMMON_REALITY.md` Gate 2 and constitutional rules;
- `THIRD_BIRTH_UNIFIED_PLAN.md` Shared Reality Packet and Ground sections;
- `PENDOLEUM_TRANSFER_MAP.md` for transferable evidence-gated mechanisms;
- `docs/plans/2026-08-01-common-reality-foundation-design.md`;
- existing schemas, tests, fixtures, commits, and receipts in this repository.

### Must not modify

- `/Volumes/WS4TB/pendoleum/CAM_Assistant`;
- `/Volumes/WS4TB/pendoleum/CAM_Assistant/pendoleum`;
- any donor repository, CAM database, personal vault, live corpus, or external
  runtime;
- the frozen Gate 0 labels, thresholds, fixture semantics, or prior receipts
  merely to make a result pass.

## CONSTRAINTS

- Keep the app local-first and usable offline.
- Do not add a mandatory cloud service, hosted database, telemetry, provider,
  model, browser automation service, or account.
- Treat imported source text and all future model/tool output as untrusted data;
  source text never changes policy or authority.
- Raw source bytes may exist only in the explicitly bounded local object store;
  logs, receipts, UI status, and evidence reports contain digests and metadata,
  not unnecessary source text.
- Preserve the separation between claims, evidence, interpretations, values,
  action commitments, and minority reports in types, storage, UI, and diff.
- Use deny-by-default policy for publication/review actions and bind any grant
  to exact packet/version/state purpose.
- Do not claim signatures, encryption, external witnessing, Lens integrity,
  human pilot success, model quality, truth, neutrality, or SOTA unless those
  capabilities are actually implemented and independently proven. If Gate 1
  prerequisites are missing, implement only the minimum local contract needed
  and record the limitation.
- Use test-driven development: write one focused failing test, observe the
  intended failure, implement the smallest path, run focused and aggregate
  tests, then refactor.
- Preserve all existing tests and public semantics. Do not weaken assertions,
  delete negative fixtures, or replace deterministic validation with prose.

## ITERATION

1. Confirm repository path, branch, HEAD, clean/dirty state, tool versions, and
   the existing Gate 0/1 test baseline before editing.
2. Read this goal and the referenced source documents; write a bounded plan in
   `docs/plans/` with exact files and commands.
3. Implement in small ownership-safe batches, in this order:

   - source/object manifest and import digest;
   - packet studio domain/review reducer;
   - packet versioning, diff, publication, and reconstruction;
   - focused core tests and `verify-gate2.sh`;
   - React UI states and UI tests;
   - aggregate verification and saved evidence.

4. At each batch, record the observed red test, green result, changed files,
   and remaining limitation in the implementation evidence.
5. After each core change run the nearest focused Rust test and then
   `cargo test --workspace`. After each UI change run the nearest web test and
   build. Do not defer all verification to the end.
6. Before completion, run an adversarial review against every Gate 2 proof
   bullet, inspect the actual generated packet/manifest/object files, and
   confirm that no future Gate 3/4 behavior was smuggled in as completion.

## SAFETY / PROVENANCE

- The Campus AI material is synthetic and remains labeled synthetic.
- A receipt proves local integrity and process history, not truth or source
  correctness.
- Publication is a process state, not an eternal or universal truth claim.
- Every imported source retains origin, digest, source type, retrieval time,
  limitations, and packet/version linkage.
- Failed validation, rejected publication, missing objects, stale versions,
  and reconstruction failures remain visible evidence.
- No personal profile, hidden political attribute, emotional vulnerability, or
  predicted persuadability may be used by Packet Studio.
- No human, agent, or model is automatically committed to an action.

## STOP

Pause and report the exact evidence, files, and next safe action if:

- the current repository identity or source-of-truth documents conflict in a
  way that changes Gate 2 scope;
- a required product choice changes the constitutional object model or
  publication semantics;
- a credential, account, paid service, network source, live model, or external
  runtime is required;
- private, regulated, proprietary, or donor data would leave its boundary;
- frozen fixture integrity is compromised or a result would require retuning a
  label, threshold, or prior receipt;
- an operation would modify CAM Assistant, Pendoleum, a donor repository, or a
  live database/corpus;
- a destructive, irreversible, privileged, or production action is proposed;
- a required command cannot run and three materially different diagnostics or
  mitigations have failed;
- the same concrete implementation failure persists after three distinct
  evidence-based repairs;
- Gate 2 work would require silently weakening Gate 1 or claiming a later gate.

When stopped, leave the worktree recoverable, preserve failed output, update
the evidence report with `blocked` or `partial`, and do not claim completion.

## COMPLETE

Mark this goal complete only when every numbered `PROOF OF DONE` item passes
with current saved evidence, the Packet Studio can create and publish the
synthetic Campus AI packet locally, the published version reconstructs from
its manifest and objects, all Gate 0/1 regressions remain green, and the final
report clearly lists what Gate 3 and later still do not provide.

The following do not satisfy completion by themselves: a schema file, a mock
UI, a fixture-only parser, a status label, a proposed publication, a model
answer, a passing unit test without end-to-end reconstruction, or a successful
manual run without saved receipts.
