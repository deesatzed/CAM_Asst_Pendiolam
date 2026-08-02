# Common Reality

Common Reality is a local-first Rust and React system for building
evidence-bound Shared Reality Packets: versioned records that keep source
material, claims, supporting evidence, counterevidence, uncertainty,
interpretations, values, action options, and minority reports distinguishable
and reviewable.

The project is intentionally separate from CAM Assistant and Pendoleum. The
Campus AI material in this repository is synthetic evaluation data, not a live
institutional policy, a human study, or a claim about the state of the world.

## Product in one sentence

Common Reality helps a team build the shared record before asking people to
agree. It makes provenance, disagreement, uncertainty, authority, and change
visible enough to review, reconstruct, and deliberately act on.

## Purpose

Important decisions often fail before disagreement is even visible:

- a source is summarized without a stable digest;
- an interpretation is repeated as a fact;
- counterevidence disappears during editing;
- uncertainty is compressed into a confident sentence;
- a minority concern is represented as resolved because the final prose sounds
  consensual; or
- a later version cannot be reconstructed from the record that produced it.

Common Reality is designed to make those failure modes inspectable. The core
does not decide what is true or what a group must do. It makes the packet, the
provenance, the review findings, and the remaining uncertainty explicit so
people can decide with a shared, auditable starting point.

## Current verified status

Gate 2, Packet Studio, is complete and reproducible with synthetic local data.
The repository currently proves:

- Gate 0: a frozen Campus AI fixture manifest with SHA-256 digests and
  adversarial cases;
- Gate 1: strict closed-object packet validation, cross-reference checks,
  prohibited profile-field rejection, canonical identities, an append-only
  operation ledger, deterministic Merkle checkpoints, and exact-scope
  deny-by-default capability authorization; and
- Gate 2: bounded source import, typed packet authoring, fail-closed review,
  immutable publication, content-addressed manifests, local integrity receipts,
  deterministic diffs, byte-for-byte reconstruction, corruption detection, and
  a React/TypeScript inspection surface.

The executable Gate 2 evidence is recorded in
[docs/evidence/gate-2-packet-studio.md](docs/evidence/gate-2-packet-studio.md).
The deferred product contract for Gates 3 through 10 is recorded in
[docs/plans/2026-08-02-common-reality-later-scope-design.md](docs/plans/2026-08-02-common-reality-later-scope-design.md).

The current proof set passes:

~~~text
cargo fmt --all -- --check
cargo test --workspace
NPM_CONFIG_OFFLINE=true npm test --workspace apps/web
NPM_CONFIG_OFFLINE=true npm run build --workspace apps/web
./scripts/verify-gate2.sh
git diff --check
~~~

These checks establish a local, synthetic, evidence-bound slice. They do not
establish truth, neutrality, democratic legitimacy, human-pilot success,
external witnessing, encryption, model superiority, or completion of Gates
3 through 10.

## What it does

The Gate 2 flow is deliberately concrete:

~~~text
bounded source import
  -> source frozen with provenance and digest
  -> typed packet draft
  -> fail-closed review
  -> immutable published version and integrity receipt
  -> manifest/object reconstruction and integrity check
~~~

The workflow distinguishes at least these object classes:

- sources and source excerpts;
- claims and evidence links;
- counterevidence and uncertainty;
- interpretations and values;
- action options and minority reports; and
- packet versions, manifests, receipts, and deterministic diffs.

Unsupported claims cannot be published as supported. Material counterevidence
and uncertainty remain visible. A packet version is not treated as a mutable
document: its object graph, manifest, and receipt can be checked and rebuilt.

## Feature list

### Source and provenance

- Bounded import from an explicitly supplied local source path.
- Source freezing before packet authoring.
- Source metadata and SHA-256 digest capture.
- Explicit separation between source bytes, excerpts, claims, and
  interpretations.
- Deterministic canonical JSON for stable hashing and comparison.
- No implicit cloud upload or model-provider dependency in the local flow.
- Source bytes are not copied into integrity receipts.

### Typed Shared Reality Packets

- Closed-object packet schema with required typed sections.
- Stable identifiers and cross-reference validation.
- Claim support and counterevidence links.
- Explicit uncertainty records instead of confidence inflation.
- Values and action options kept distinct from descriptive claims.
- Minority reports preserved as first-class packet content.
- Version metadata and operation provenance.
- Rejection of prohibited profile fields at validation time.

### Review and authorization

- Fail-closed review before publication.
- Unsupported-claim and missing-material-counterevidence blockers.
- Review findings represented as structured reasons, not an opaque score.
- Deny-by-default capability authorization.
- Exact-scope grants with explicit operation and object boundaries.
- Human publication intent represented separately from source content.
- No imported prose can grant itself authority or change policy.

### Immutable publication and integrity

- Content-addressed packet objects.
- Immutable published versions.
- Deterministic manifests.
- Local integrity receipts containing hashes and process facts, not raw source
  bytes.
- Byte-for-byte reconstruction from a published manifest and object store.
- Missing-object and altered-object detection.
- Deterministic typed diffs between packet versions.
- Replayable append-only operation ledger and Merkle checkpoints from Gate 1.

### Packet Studio web surface

- Default landing page that states purpose, evidence boundary, current status,
  and later scope.
- Explicit Packet Studio entry at the hash route #studio.
- Source-frozen status and provenance visibility.
- Typed section inspection for claims, evidence, counterevidence, uncertainty,
  values, action options, and minority reports.
- Publication-blocked view with concrete review reasons.
- Local integrity receipt view.
- Reconstruction and deterministic-diff status.
- Accessible semantic structure and no implication that the UI is a truth
  oracle.

### Deferred product surface

The following are planned contracts, not Gate 2 features:

- compiled, profile-declared Lenses with omission manifests and Referee checks;
- participant-reviewed Bridges between a World Graph and a Self Graph;
- structured Disagreement Maps that preserve minority and unresolved states;
- consented human pilots with preregistered metrics and stop conditions;
- Thought Object memory with correction, forgetting, and explainable retrieval;
- optional typed AI-to-AI and MCP adapters with grants and state handles;
- Constellation group agents with charters, canaries, forks, merges, and
  lineage; and
- Chorus federation with nested maps, subgroup visibility, witness diversity,
  and poisoning defenses.

The sequencing and proof obligations for these items are in the deferred-scope
plan linked above. No later-gate behavior is implied by the Gate 2 UI.

## Why it is different

Common Reality is not just a document editor, a meeting-notes template, a
consensus dashboard, a model wrapper, or a hosted knowledge base. Its boundary
is structural: the representation, validators, authorization rules, and
evidence gates all work together to keep different kinds of statements
separate.

| Conventional default | Common Reality boundary |
| --- | --- |
| A document mixes facts, interpretation, values, and recommendations in prose. | Typed objects require those categories to remain distinguishable. |
| An edit history says what changed but not which semantic object changed. | Deterministic typed diffs expose object-level changes and status changes. |
| A checklist can be marked complete while a material caveat is missing. | Fail-closed review blocks publication when required support or counterevidence is absent. |
| A mutable database row is treated as the current truth. | Published versions are immutable, content-addressed, and reconstructable. |
| A fluent model response is treated as an answer or authority. | Models, if added later, sit behind validators, grants, and human authority boundaries. |
| Consensus prose tends to erase dissent. | Minority, unresolved, and contested objects survive as inspectable records. |
| A hosted workflow makes data movement the default. | Local-first storage makes deliberate export the boundary. |
| A receipt means a process ran. | A receipt proves integrity and process facts while explicitly refusing to prove truth. |

This is not a claim that every conventional tool is inferior. It is a claim
about which failure modes Common Reality chooses to make expensive: silent
category changes, unsupported publication, lost counterevidence, authority
confusion, and unreconstructable versions.

## Proposed theory, methods, and techniques

The following are proposed design contributions and engineering hypotheses.
They are not established scientific results, and the repository does not claim
novelty over prior literature without a comparative review.

### 1. Shared Reality Packet as a typed epistemic grammar

A packet treats a decision record as a graph of typed statements rather than a
single narrative. The useful unit is not only a sentence, but a statement with
provenance, support, counterevidence, uncertainty, interpretation, value, or
action semantics. This creates a machine-checkable grammar for asking what kind
of thing a statement is and what would be required to publish it.

### 2. Reconstruction as an epistemic integrity test

Reconstruction is more than a storage backup. If a published packet cannot be
rebuilt from its manifest and content-addressed objects, the system cannot
honestly claim to preserve the record that reviewers saw. Byte-level
reconstruction therefore acts as a precondition for later interpretation,
deliberation, and outcome analysis.

### 3. Counterevidence and uncertainty as mandatory visibility

The design treats counterevidence and material uncertainty as required
visibility conditions, not optional footnotes. A workflow may still publish a
contested or uncertain packet, but it cannot silently turn missing support into
support or make uncertainty disappear through fluent editing.

### 4. Integrity receipts with epistemic humility

An integrity receipt records hashes, manifests, version identity, and process
facts without asserting that the packet is true. This separates cryptographic
integrity from epistemic correctness and makes room for later human correction
without retroactively rewriting what was previously published.

### 5. Authority boundaries by construction

The longer-term architecture separates a World Graph, a Self Graph, and a
Bridge Graph. Shared-world facts, private or participant-owned perspective,
and explicit purpose-bound translation are different authority domains. The
boundary is intended to prevent a private profile from silently becoming a
group decision or a model inference from silently becoming a memory.

### 6. Evidence gates as a product method

Each gate names a capability, an executable proof condition, and a stop
condition. The method is to implement the smallest slice that can be falsified,
run adversarial cases, aggregate artifacts, and write product claims only after
the proof passes. This keeps roadmap language from becoming a substitute for
runtime behavior.

### 7. Presentation without authority transfer

Future Lenses are intended to change presentation for a declared purpose while
preserving factual atoms, counterevidence, uncertainty, citations, and
omissions. The proposed technique is to compile a Lens with an omission
manifest and a Referee that checks semantic invariants. A more persuasive
rendering must not gain authority merely by sounding clearer.

### 8. Disagreement as structured information

Future Disagreement Maps distinguish factual conflict, interpretation conflict,
value conflict, action conflict, missing evidence, consensus, majority,
minority, and unresolved states. The proposal is to preserve disagreement as
typed information that can guide evidence collection and reversible action,
instead of reducing it to a single approval percentage.

### 9. Reversible action as a constitutional output

Where action is eventually selected, the design prefers explicit options,
conditions, expiry, and rollback over an irreversible command emitted from
consensus prose. This is a safety technique for keeping uncertainty actionable
without pretending it has been eliminated.

## Why a finished product matters

A finished product would give teams a durable answer to questions that ordinary
notes make surprisingly difficult:

- What source did this claim come from?
- Which parts are observations, interpretations, values, or proposals?
- What counterevidence was considered, and what remains unknown?
- Who or what was authorized to change this packet?
- What changed between the version reviewed and the version acted on?
- Can an independent reviewer reconstruct the exact published record?
- Did a minority concern survive revision, or was it merely omitted?
- Which later action is reversible, time-bounded, or explicitly blocked?

The result would not be a universal arbiter. It would be a better substrate for
human judgment, audit, collaboration, and correction in settings where the
record itself is part of the safety case.

## Trial and breakthrough possibilities

The following are bounded hypotheses for future research or product trials:

1. Packet review may lower omission of material caveats compared with ordinary
   document review.
2. Typed diffs and receipts may reduce the time required for an independent
   reviewer to locate unsupported changes.
3. First-class minority objects may preserve risk information better than
   consensus minutes across multiple revisions.
4. Reconstructable packets may make policy iteration more reproducible across
   teams and time.
5. Local-first, deliberate export may reduce accidental data exposure in
   privacy-sensitive collaboration.
6. Separating claims, values, and action assumptions may reveal alternatives
   that a single consensus paragraph hides.
7. A compiled Lens plus Referee may keep factual atoms and uncertainty more
   stable across presentation changes or model swaps.
8. Structured disagreement may localize which questions need evidence, which
   need value negotiation, and which should remain unresolved.

These are not outcome claims. A credible trial would require consent where
people are involved, an appropriate privacy and ethics review, predefined
metrics, a comparator, adversarial cases, versioned evidence, and explicit
stopping rules. Agreement, enjoyment, or model fluency alone would not count
as success.

## Boundaries and non-goals

- No mandatory cloud service, hosted database, telemetry, model provider, or
  account is required for the local flow.
- Imported source text is untrusted data; it cannot change policy or authority.
- A local receipt proves integrity and process history, not truth or source
  correctness.
- The repository does not claim neutrality, objectivity, democratic legitimacy,
  sentience, human-pilot success, or state-of-the-art model quality.
- Gates 3 through 10 remain deferred until their named proof conditions pass.
- CAM Assistant, Pendoleum, donor repositories, live databases, personal vaults,
  and live corpora are outside this repository's implementation scope.

## Development

Read the controlling documents before changing the implementation:

1. [GOAL_Gate2.md](GOAL_Gate2.md)
2. [STANDARDS.md](STANDARDS.md)
3. [IMPLEMENT.md](IMPLEMENT.md)
4. [DECISIONS.md](DECISIONS.md)
5. [PROGRESS.md](PROGRESS.md)
6. [TASK_QUEUE.md](TASK_QUEUE.md)
7. [GOAL_FINISH_WIKI.md](GOAL_FINISH_WIKI.md)

Run the local proof set:

~~~bash
cargo fmt --all -- --check
cargo test --workspace
npm test --workspace apps/web
npm run build --workspace apps/web
./scripts/verify-gate2.sh
git diff --check
~~~

Evidence belongs in docs/evidence/. Progress and decision truth is kept in
[PROGRESS.md](PROGRESS.md) and [DECISIONS.md](DECISIONS.md). The default web
route is this product explanation; Packet Studio is available at the explicit
hash route #studio.

## License

The repository packages are marked Apache-2.0 and are not currently published
to a package registry.
