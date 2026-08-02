# DECISIONS.md

## 2026-08-01: Gate 2 owns a tagged typed object graph

The Packet Studio will not use a single untyped JSON bag as its authoring model.
Claims, evidence, counterevidence, uncertainty, interpretations, values,
actions, and minority reports are represented by distinct tagged Rust objects
with explicit references. A legacy packet projection is used only to preserve
the existing Gate 0 schema contract.

## 2026-08-01: Local content-addressed persistence is sufficient for Gate 2

Gate 2 will use a bounded in-memory/local object store and deterministic
manifest/receipt APIs rather than introducing SQLite, hosted storage, or a
network service. This proves the required integrity and reconstruction semantics
without expanding the authority boundary.

## 2026-08-01: UI is an inspectable local projection, not a second authority

The React/TypeScript surface renders deterministic local synthetic data and
failure states. It does not publish packets, call models, infer truth, or use
network credentials; the Rust proof path remains authoritative for review and
reconstruction claims.
