# Gate 0 Fixture Freeze Receipt

Date: 2026-08-01

## Scope

The `campus-ai-v1` corpus is synthetic, frozen evaluation material. It is not
evidence of a real university policy process, a real pilot, model quality, or
human outcomes.

## Frozen artifacts

- canonical Campus AI packet;
- synthetic source corpus with material-claim and counterevidence labels;
- expected constitutional invariants;
- allowed and prohibited Lens profile fields;
- synthetic model/vendor swap cases;
- fourteen adversarial attack fixtures.

Each file is bound by SHA-256 in
`fixtures/campus-ai-v1/manifest.json`. The manifest SHA-256 is:

```text
177d11544ab26bf8200ac181b2e6760c85ca00cc965a198f319e62174bcf81a3
```

## Verification

```text
cargo test -p common-reality-schema
```

Result at commit `fc1f35a`:

```text
8 passed; 0 failed
```

The suite verifies manifest integrity and required attack/non-model fixture
coverage, accepts the frozen packet, and rejects an unknown nested claim field,
missing material counterevidence, and prohibited Lens profile use. It also
checks that the published packet and claim/evidence JSON Schema objects are
closed to undeclared fields.

## Limitations

The fixtures do not test models, multiple model families, human participants,
external witnessing, cryptographic signatures, a real source corpus, or a
published packet. Those remain later gates.
