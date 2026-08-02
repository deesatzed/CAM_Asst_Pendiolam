# RISK_NOTES.md

## Risks

| Risk | Severity | Why It Matters | Mitigation |
|---|---|---|---|
| Reusing the permissive shape of the legacy packet could collapse typed Gate 2 objects | high | Fact, evidence, interpretation, value, action, and minority reports must remain mechanically distinct | Define a tagged Rust object enum and validate every reference/kind before projecting to the frozen packet schema |
| A receipt could accidentally include raw source text | high | Gate 2 requires provenance without leaking source bytes | Store raw bytes only under content IDs; serialize receipts from metadata and digests only; add a receipt redaction test |
| Published versions could be mutable through shared maps or stale drafts | high | Publication must be immutable and reconstructable | Content-address every object, reject duplicate `(packet_id, version)`, require parent version for successors, and verify all hashes on reconstruction |
| UI-only status could overstate a review or publication result | high | A status label is explicitly insufficient proof | Have the Rust verifier produce the receipt and make the UI render typed findings, blocked reasons, and integrity status from deterministic local data |
| Adding npm dependencies may require network during setup | medium | Gate 2 runtime/build must remain local-first | Pin dependencies in the workspace lockfile and keep runtime data local; record dependency installation as setup, not a cloud/runtime requirement |
| Frozen Gate 0 fixture digests could drift during implementation | high | Retuning fixtures would invalidate the evidence boundary | Never edit existing fixture bytes; run manifest verification before and after Gate 2 verification |
| Existing policy and ledger APIs may be too narrow for studio publication | medium | Duplicating them would create inconsistent authority rules | Add only generic canonical helper APIs or adapters; reuse `authorize`, `Ledger`, and `MerkleCheckpoint` directly |

## Safe Next Step

Write and execute the bounded Gate 2 implementation plan, starting with a
failing Rust test for source import and a tagged typed object graph. Preserve the
18-test baseline before changing any frozen fixture.
