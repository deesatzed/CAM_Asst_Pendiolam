# Common Reality Foundation Design

## Status

Approved on 2026-08-01. This is a new standalone repository, intentionally
separate from CAM Assistant and Pendoleum.

## Purpose

Build the constitutional core and first human-facing demonstration of Common
Reality: several people can deliberate from one evidence-bound packet, see
permitted presentation differences without semantic divergence, submit typed
disagreement, preserve minority concerns, and select a reversible next action.

The first domain is the Campus AI Constitution question. The product is not a
truth oracle, persuasion system, social feed, sentient collective, or general
personal companion.

## Chosen Boundary

Common Reality is its own Rust workspace plus React PWA. CAM Assistant is not
modified by this work and may later become a private Self Graph host through a
typed, purpose-bound adapter. Pendoleum remains a read-only research-method
source; only its validated methodological ideas are transferred.

This avoids coupling a civic shared-evidence system to personal-vault state and
keeps the three-graph constitution enforceable:

```text
Self Graph     private and optional to the first pilot
World Graph    canonical, shared, evidence-bound packet
Bridge Graph   explicit, purpose-bound response to one packet
```

## Architecture

The repository begins with a deterministic Rust constitutional core and a
local-first React PWA that consumes it through typed interfaces.

```text
Frozen fixtures
  -> schema validation and canonical serialization
  -> content identity, signatures, policy and append-only ledger
  -> packet state replay
  -> Ground packet studio
  -> Lens compiler and referee
  -> Bridge disagreement packets
  -> Disagreement Map and reversible action record
```

Models may propose text or structured candidate objects. They never publish a
packet, alter claim status, authorize a grant, aggregate a group, choose an
action, or write authoritative history. Deterministic validators and policies
decide those transitions.

## Gate 0 First

Before product behavior, freeze a synthetic Campus AI Constitution corpus,
material claims and counterevidence labels, permitted presentation fields,
expected invariants, model/vendor swap cases, and adversarial fixtures. A
manifest records SHA-256 digests of every fixture.

The initial adversarial suite covers unsupported additions, material omission,
uncertainty reduction, interpretation-as-fact, value-as-evidence, forbidden
profile use, stakeholder omission, source laundering, stale packets, minority
deletion, prompt injection, truncation, and false receipts.

The supplied packet schema is refined before adoption: every nested object is
closed, evidence objects are required where claims reference them, identifiers
and references are validated, and the complete named object family receives
versioned schemas rather than a single permissive packet document.

## Failure Semantics

The core fails closed. It refuses publication or consequential aggregation for
invalid schema, unknown fields, dangling references, mismatched packet version,
prohibited profile use, unsigned or stale authority, missing material coverage,
minority loss, receipt mismatch, replay mismatch, or unresolved integrity
failure. A failure receipt records status and identifiers without private source
content.

## Testing and Evidence

Every production behavior follows red-green-refactor. Gate 0 uses frozen,
synthetic fixtures only; it cannot substantiate a real pilot or model-quality
claim. Gate 1 tests identical-object identity, one-bit divergence, deterministic
replay, mutation/deletion/reorder/stale-writer detection, deny-by-default
authorization, and private/public separation. Later tests distinguish fixture,
integration, model, human-pilot, and external-witness evidence.

## Initial Repository Shape

```text
crates/
  common-reality-schema/       typed objects and strict validation
  common-reality-canonical/    canonical JSON and content IDs
  common-reality-ledger/       event log, replay, Merkle checkpoints
  common-reality-policy/       deny-by-default grants and checks
apps/web/                      React PWA: Ground, Lens, Bridge, Map
schemas/                       versioned JSON Schema artifacts
fixtures/campus-ai-v1/         frozen synthetic corpus and attacks
evaluations/lens-integrity/    adversarial and cross-Lens evaluation
docs/evidence/                 saved commands and receipts
```

## Explicit Non-Claims

This foundation does not claim truth, political neutrality, democratic
legitimacy, sentience, emergent communication, pilot success, or superiority
over ordinary deliberation. It establishes the conditions under which those
questions can later be tested honestly.
