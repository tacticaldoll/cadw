# Proposal

## Why

`architectural-governance` says more about what the governance gate catches than its accepted
boundaries observe. The dependency boundaries read only the normal dependency table, yet the
zero-dependency scenario fails on "any non-dev dependency", which includes a build-dependency, and
the independence scenario fails on "a dependency". The no-I/O boundaries see inline calls in the
library module tree, yet the scenario speaks of `cadw-contract`'s "source", which includes
`examples/`, `tests/`, and `build.rs`. The serde boundary sees derives and resolvable hand impls in
the library source, yet the scenario speaks of "any type in `cadw-contract`". The Purpose says the
gate enforces the Core Contract's "sans-I/O purity". The same overclaim appears in
`crates/cadw-governance/README.md` ("no … call anywhere in its source"), in `PROJECT.md` and
`AGENTS.md` ("enforces the boundaries this document claims", "enforces `PROJECT.md`'s Core Contract
boundaries"), and in `crates/cadw-contract/README.md` ("Zero non-dev dependencies, enforced by
`cadw-governance`").

## What Changes

- Four `architectural-governance` requirements keep their intent and gain one sentence naming what
  the gate observes and the review-governed remainder; their failing scenarios name only observed
  shapes: a normal dependency, an inline call in the library source, and a derive or impl written
  in the library source.
- The Purpose says the gate holds the observable part of the Core Contract boundaries.
- `crates/cadw-governance/README.md`, `crates/cadw-contract/README.md`, `PROJECT.md`'s "Governance
  with teeth" bullet, and `AGENTS.md`'s Definition Of Done prose narrow their enforcement claims the
  same way.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `architectural-governance`: four requirements narrow what their scenarios say the gate catches.

## Impact

- `openspec/specs/architectural-governance/spec.md` after sync, including its Purpose.
- `crates/cadw-governance/README.md`, `crates/cadw-contract/README.md`, `PROJECT.md`, `AGENTS.md`.
- No code or law change: the constitution, `AGENTS.cadw-law.md`, and `list --format json` stay as
  they are.
