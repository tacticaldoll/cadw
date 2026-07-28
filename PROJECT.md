# Project Contract — Cadw

## Status

**Tier 2 — controlled prototype** (per a private family-level roadmap). Sans-I/O pure core.
Experimental. Not wired into any consumer. Not published to crates.io. Vacuum-unit-tested to
learn the shape; graduation to Tier 1 (a real bridge consumer, a public crates.io release) is a
separate, later decision this repository does not assume.

## Graduation

The contract freeze, adversarial testing, tianheng gate, non-toy consumer example, and
domain-language review are all done — `cadw-contract`'s shape is as proven as it can be in
isolation. What remains is not more work on the crate; it is a decision, and the decision has a
concrete trigger, not an open-ended "later":

**Trigger**: `ringi` — the project whose arbitrator-authors-a-whole-document tension is what this
crate grew out of — or another real consumer, actually decides to pursue structured move/
operation authorship for its own domain, replacing whole-document echo with discrete, validated
operations. That decision belongs to the consumer, not to Cadw: this repository does not propose
adoption to `ringi`, or to anyone. If and when such a decision is recorded in that project's own
governance, re-run a real assessment of whether `cadw-contract`'s shape actually fits — do not
assume adoption follows automatically from the trigger firing.

**Not a trigger**: growing `cadw-contract`'s generality — a new `Move` variant, a new `Rejection`
kind, a `cadw-conformance` crate for a hypothetical second implementation — in anticipation of a
consumer that has not actually materialized. `AGENTS.md`'s "don't design for hypothetical future
requirements" applies to this crate's own lifecycle milestone exactly as it applies to its code.

**An ungraduated Cadw is a legitimate permanent state.** If the trigger never fires, that is not a
stalled or failed project — it is a Tier 2 spike that answered its own question (is this
mechanism worth proving) and never found a real consumer, exactly what least-commitment is for.
No sunset clause, no forced timeline.

## Purpose

Cadw ("keep, retain, preserve" — Welsh) is a thin kernel for folding a batch of declared
`Close`/`Reopen` moves over addressable targets atomically: every move in a batch applies, or
none do. Conservative retention — a target no move mentions is untouched — is a structural
property of the data model, not a checked invariant: there is no code path through which an
unmentioned target could be silently dropped.

The kernel owns the fold/atomicity/conflict/state-transition mechanism only. It never judges
whether a specific `Outcome` is semantically valid — that is the domain-supplied `Validator` port,
whose `Rejection` associated type is a fully structured `std::error::Error`, mirroring
`pacta-contract::Registry`'s `type Error` pattern.

## Core Contract

The behavior that must be protected at all costs:

- **Atomic batch fold.** `Ledger::fold_batch` applies every move in a batch or none of them.
  Never a partial subset, regardless of how many moves precede the one that fails.
- **Structural conservative retention.** A target absent from a batch is unreachable by that
  batch's fold. This is enforced by the data model (no code path can touch an unaddressed
  target), not by a check that could be forgotten.
- **Domain validation is a port, never free-text.** `Validator::Rejection: std::error::Error` is
  the domain's own closed, structured type. No `String` ever crosses the boundary between the
  kernel's rejection vocabulary and the domain's own.
- **No time dimension.** No lease, no expiry, no crash-recovery concern. A batch fold is
  synchronous and in-memory, assumed to run entirely within an already-claimed unit of work a
  consumer has durably claimed through its own mechanism (e.g. `pacta`, composed outside this
  crate). Cadw does not compete with `pacta`'s scope — it has none of pacta's reasons to exist.
- **Governance with teeth.** `cadw-governance` (tianheng) enforces the boundaries this document
  claims, executably — see `crates/cadw-governance/README.md`.

## Terminology

- **Target** (`TargetId`): an opaque, domain-supplied identity for one addressable thing that can
  be `Open` or `Closed`. The kernel never interprets its content.
- **State**: `Open` or `Closed(Outcome)`. `Outcome` is domain-opaque.
- **Move**: a single declared operation — `Close` (with an outcome) or `Reopen` — addressing one
  target.
- **Validator**: the domain-supplied port judging whether a specific `Close`'s outcome is
  semantically acceptable.
- **Rejection**: why a batch was rejected — either a structural rule (`UnknownTarget`,
  `AlreadyClosed`, `NotClosed`, `DuplicateTargetInBatch`) or the domain's own
  (`Invalid(TargetId, Validator::Rejection)`).
- **Ledger**: the set of targets and their current states.

## Non-Goals

Cadw core is not:

- an event-sourcing framework
- a CRDT or general diff/patch engine
- a target-creation or target-discovery mechanism (targets are domain-supplied, already
  populated, before a batch runs)
- a place that knows what "reason" or "provenance" mean, or any other domain vocabulary — that
  belongs to whichever domain adopts this
- a decision about how many times a consumer invokes anything per unit of work, or how batches
  are assembled — entirely the consumer's strategy
- a durable or persistence mechanism of any kind

## Lineage

```
             tianheng  +  〔sans-I/O · OpenSpec · vocab-as-governance · least-commitment〕
                              │  (inherited discipline — provenance)
                              ▼
                      ●  Cadw (Tier 2)

   siblings: ▢ ▢ ▢  ← deliberately blank (sibling-blind)
   footnote: workspace shape and governance-first sequencing observed from the pacta reference
             implementation (dual MIT/Apache license, Cargo.toml conventions, associated-type
             Validator port mirroring `pacta-contract::Registry`'s `type Error`, and a
             tianheng-governed workspace built before feature work, not after); no code or crate
             dependency on pacta — core ⟂ core.
```

## First Project Change

This repository's first change, `initial-project-shape`, replaced this file's placeholders,
chose the two-crate workspace layout (`cadw-contract`, `cadw-governance`), ported the proven
mechanism and its vacuum tests, and built `cadw-governance` first per explicit direction rather
than as an afterthought.

## Change Prioritization

When comparing possible changes, prefer the one that protects the Core Contract earliest:

1. Correctness of the fold/atomicity/conflict/state-transition mechanism, and the governance
   that keeps it from drifting.
2. Specified feature completeness for concepts already declared in OpenSpec.
3. Operator and developer ergonomics (docs, examples).
4. Graduation decisions (a real consumer, a public release) — never pursued merely because a
   correctness or governance change makes them easier.
