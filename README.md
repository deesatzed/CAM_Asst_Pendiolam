# Common Reality

Common Reality is a local-first Rust/React system for building evidence-bound
Shared Reality Packets: versioned records that keep source material, claims,
supporting evidence, counterevidence, uncertainty, interpretations, values,
action options, and minority reports distinguishable and reviewable.

The project is intentionally separate from CAM Assistant and Pendoleum. The
Campus AI material in this repository is synthetic evaluation data, not a live
institutional policy, a human study, or a claim about the state of the world.

## Why this exists

Important decisions often fail before disagreement is even visible. A source is
summarized without a stable digest, an interpretation is repeated as a fact,
counterevidence disappears during editing, uncertainty is compressed into a
confident sentence, or a later version cannot be reconstructed from the record
that produced it.

Common Reality is designed to make those failure modes inspectable. Its core
does not decide what is true or what a group must do. It makes the packet, the
provenance, the review findings, and the remaining uncertainty explicit so
people can decide with a shared, auditable starting point.

## Why someone would want the finished product

A finished Packet Studio would be useful when the cost of a lost caveat or an
untraceable edit is higher than the cost of careful preparation. For example:

- A policy team could compare two versions of a proposal and see which claims,
  evidence links, counterarguments, and action assumptions changed.
- A research or safety review group could require every material claim to show
  its support status, counterevidence, uncertainty, and source limitations
  before publication.
- A cross-functional team could preserve a minority concern instead of treating
  consensus language as proof that the concern was resolved.
- A privacy-sensitive organization could keep source bytes and packet work on a
  local machine while sharing only bounded manifests, digests, and deliberate
  exports.
- An engineer or auditor could reconstruct a published packet from its
  content-addressed objects and determine whether an object was missing,
  altered, or attached to the wrong version.

The value is not an authority score, a persuasion engine, or a promise of
neutrality. The value is a durable workflow in which people can tell what was
observed, what was inferred, what is valued, what is proposed, and what remains
unknown.

## Current status

The repository is being built gate-by-gate. The verified foundation currently
includes:

- a frozen, synthetic Campus AI fixture manifest with SHA-256 digests and
  adversarial cases;
- strict closed-object Shared Reality Packet validation with cross-reference
  checks and prohibited profile-field rejection;
- deterministic canonical packet identities and generic canonical JSON helpers;
- a replayable append-only packet operation ledger with deterministic Merkle
  checkpoints; and
- deny-by-default, exact-scope capability authorization.

Gate 2 is specified in [`GOAL_Gate2.md`](GOAL_Gate2.md) and is still an active
implementation objective. It will add the Packet Studio core, immutable
publication and reconstruction proof, a local React/TypeScript surface, and an
executable evidence receipt. The current plan and repository audit are in
[`docs/plans/2026-08-01-common-reality-gate2-packet-studio.md`](docs/plans/2026-08-01-common-reality-gate2-packet-studio.md),
[`REPO_MAP.md`](REPO_MAP.md), and [`RISK_NOTES.md`](RISK_NOTES.md).

## What a finished Gate 2 slice will prove

The intended local flow is:

```text
bounded source import
  -> source frozen with provenance and digest
  -> typed packet draft
  -> fail-closed review
  -> immutable published version and receipt
  -> manifest/object reconstruction and integrity check
```

The proof standard is deliberately stronger than a mock screen or a passing
fixture parser. Gate 2 must demonstrate, with synthetic local data, that
unsupported claims cannot be published as supported, material counterevidence
and uncertainty remain visible, typed objects cannot be silently substituted,
version diffs are deterministic, receipts do not contain raw source bytes, and
altered or missing objects fail reconstruction.

## Trial and breakthrough possibilities

These are hypotheses for bounded future trials, not results claimed by this
repository. Each would need consent, an appropriate privacy/ethics review,
predefined metrics, a comparator, and explicit stop conditions.

1. **Lower omission of material caveats.** Compare ordinary document review
   with Packet Studio review on matched synthetic and domain-approved cases.
   Measure recall of required counterevidence and uncertainty disclosures, not
   agreement with a preferred conclusion.
2. **Faster independent audit.** Give reviewers a packet and a version diff in
   randomized order. Measure time to identify a changed claim, missing source,
   or unsupported status, while preserving accuracy and reviewer confidence.
3. **More durable minority preservation.** Compare free-form minutes with
   typed minority reports across revisions. Measure whether the original risk,
   evidence links, and unresolved status survive editing without inflating or
   suppressing the minority position.
4. **Reproducible policy iteration.** Ask separate teams to reconstruct and
   review the same published packet from its manifest and objects. Measure
   byte-level reconstruction, change-set agreement, and the rate of unexplained
   drift.
5. **Safer local collaboration.** In a privacy-reviewed pilot, compare local
   object storage plus deliberate digest/manifest sharing with a conventional
   hosted workflow. Measure data exposure, recovery from a corrupted object,
   and the ability to revoke or stop a publication before an external action.
6. **Better accessibility-policy tradeoff discovery.** Use the synthetic Campus
   AI case as a rehearsal, then only a consented domain-specific study, to test
   whether separating claims, values, and action assumptions reveals workable
   alternatives that a single consensus paragraph hides.

None of these trials would establish truth, neutrality, democratic legitimacy,
model superiority, or universal benefit. A positive result would support only
the tested workflow, population, and packet versions.

## Boundaries and non-goals

- No mandatory cloud service, hosted database, telemetry, model provider, or
  account is required for the local flow.
- Imported source text is untrusted data; it cannot change policy or authority.
- A local receipt proves integrity and process history, not truth or source
  correctness.
- The repository does not yet claim human-pilot success, external witnessing,
  signatures, encryption, Lens integrity, or Gate 3/4 deliberation behavior.
- CAM Assistant, Pendoleum, donor repositories, live databases, personal
  vaults, and live corpora are outside this repository's implementation scope.

## Development

Read [`GOAL_Gate2.md`](GOAL_Gate2.md) before implementing. The current Rust
baseline can be checked with:

```bash
cargo test --workspace
```

When Gate 2 is implemented, the complete proof set will also include:

```bash
npm test --workspace apps/web
npm run build --workspace apps/web
./scripts/verify-gate2.sh
git diff --check
```

Evidence belongs in `docs/evidence/`, and progress/decision truth is kept in
[`PROGRESS.md`](PROGRESS.md) and [`DECISIONS.md`](DECISIONS.md).

## License

The repository packages are marked `Apache-2.0` and are not currently
published to a package registry.
