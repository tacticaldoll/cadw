# Project Contract — Cadw

## Vision

Cadw ("keep, retain, preserve" — Welsh) is a thin kernel for folding a batch of declared
`Create`/`Close`/`Reopen` moves over addressable targets atomically: every move in a batch
applies, or none do. Conservative retention — a target no move mentions is untouched — is a
structural property of the data model, not a checked invariant: there is no code path through
which an unmentioned target could be silently dropped.

## Product Positioning

The kernel owns the fold/atomicity/conflict/state-transition mechanism only. It never judges
whether a specific `Outcome` is semantically valid — that is the domain-supplied `Validator` port,
whose `Rejection` associated type is a fully structured `std::error::Error`, mirroring the
associated `type Error` pattern of the reference implementation Cadw's workspace shape was
observed from (see `## Lineage`).

## Status

**Tier 1 — graduated** (per a private family-level roadmap). Sans-I/O pure core, published to
crates.io as `cadw-contract` and the curated `cadw` facade. A real bridge consumer's adoption
completed graduation — see `## Graduation` below for the full record.

## Graduation

The contract freeze, adversarial testing, tianheng gate, non-toy consumer example, and
domain-language review are all done — `cadw-contract`'s shape is as proven as it can be in
isolation. What remains is not more work on the crate; it is a decision, and the decision has a
concrete trigger, not an open-ended "later":

**Trigger**: the consumer whose arbitrator-authors-a-whole-document tension is what this crate grew
out of — or another real consumer — actually decides to pursue structured move/operation authorship
for its own domain, replacing whole-document echo with discrete, validated operations. That decision
belongs to the consumer, not to Cadw: this repository does not propose adoption to that consumer, or
to anyone. If and when such a decision is recorded in that project's own governance, re-run a real
assessment of whether `cadw-contract`'s shape actually fits — do not assume adoption follows
automatically from the trigger firing.

**Not a trigger**: growing `cadw-contract`'s generality — a new `Move` variant, a new `Rejection`
kind, a `cadw-conformance` crate for a hypothetical second implementation — in anticipation of a
consumer that has not actually materialized. `AGENTS.md`'s "don't design for hypothetical future
requirements" applies to this crate's own lifecycle milestone exactly as it applies to its code.

**An ungraduated Cadw is a legitimate permanent state.** If the trigger never fires, that is not a
stalled or failed project — it is a Tier 2 spike that answered its own question (is this
mechanism worth proving) and never found a real consumer, exactly what least-commitment is for.
No sunset clause, no forced timeline.

**The trigger fired (this change).** The originating consumer recorded, in its own backlog, a
settled decision to pursue structured move/operation authorship, then shipped it: a `Move` enum
(`ResolveDissent`, `AddRisk`, `CloseRisk`, `AskQuestion`, `AnswerQuestion`) applied via
`Revision::apply_moves`, tested and dogfooded end-to-end — built independently, with no code or
crate dependency on this repository, exactly as `core ⟂ core` (Lineage, below) says a sibling
should. Re-running the fit assessment (not assuming adoption) found one real, structural gap:
`AddRisk`/`AskQuestion` create a target mid-batch, which the kernel could not previously express.
This change (`add-target-creation-to-kernel`) is that assessment's concrete outcome — the kernel
absorbing a scope correction learned from the originating consumer's real, working implementation,
still with zero consumers and still unpublished. Whether that consumer (or anyone) actually adopts
the corrected kernel remains a separate, later, un-forced decision this change does not itself make.

**Graduation completed (`graduate-to-tier-1-and-publish`, `complete-release-metadata`,
`add-cadw-facade`).** The originating consumer adopted `cadw-contract` (initially as a temporary git
dependency, pending this publish) and proved the fit through real end-to-end dogfooding — a batch
mixing `Create` and `Close` in one turn, the exact shape `add-target-creation-to-kernel` exists for.
Per this section's own stated condition, Tier 1 required a real bridge consumer *and* a public
crates.io release *together*: the consumer half was real first; `cadw-contract` `0.1.0` and the
curated `cadw` facade (matching the family's established core-plus-facade shape) are now both
published, completing the release half. `cadw-governance` remains unpublished — it was never part of
the trigger, and nothing outside this workspace needs it. The consumer's own dependency flip from
`cadw-contract` to the published `cadw` facade is that consumer's decision, tracked in its own
repository.

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
  consumer has durably claimed through its own mechanism (for example a separate claim library,
  composed outside this crate). Cadw does not compete with such a library's scope — it has none
  of its reasons to exist.
- **Governance with teeth.** `cadw-governance` (tianheng) enforces the observable part of the
  boundaries this document claims, executably; the rest is held by review — see
  `crates/cadw-governance/README.md`.

## Non-Goals

Cadw core is not:

- an event-sourcing framework
- a CRDT or general diff/patch engine
- a target-*discovery* mechanism (the kernel never enumerates or searches for targets — a
  consumer that wants to know what targets exist maintains that itself; `Create` only ever brings
  *one named target* — supplied by the caller — into existence, it does not generate or discover
  ids)
- a place that stores any content for an `Open` target, or that knows what "reason" or
  "provenance" mean, or any other domain vocabulary — `Create` carries no payload, and a target's
  descriptive content (if any) stays entirely the domain's concern, correlated by the same
  `TargetId`
- a decision about how many times a consumer invokes anything per unit of work, or how batches
  are assembled — entirely the consumer's strategy
- a durable or persistence mechanism of any kind

## Lineage

```
             tianheng  +  〔sans-I/O · OpenSpec · vocab-as-governance · least-commitment〕
                              │  (inherited discipline — provenance)
                              ▼
                      ●  Cadw (Tier 1)

   siblings: ▢ ▢ ▢  ← deliberately blank (sibling-blind)
   footnote: workspace shape and governance-first sequencing observed from a reference
             implementation (dual MIT/Apache license, Cargo.toml conventions, associated-type
             Validator port mirroring its `type Error`, and a tianheng-governed workspace built
             before feature work, not after); no code or crate dependency on it — core ⟂ core.
```

## First Project Change

This repository's first change, `initial-project-shape`, replaced this file's placeholders,
chose the two-crate workspace layout (`cadw-contract`, `cadw-governance`), ported the proven
mechanism and its vacuum tests, and built `cadw-governance` first per explicit direction rather
than as an afterthought.

A third crate, `cadw` (the curated facade, `pub use cadw_contract::*;`), was added at
graduation (`add-cadw-facade`) to match the family's established two-crate-plus-facade
shape — see `BACKLOG.md`'s "Two-crate layout" entry for why this does not reopen that decision's
original reasoning.

## Change Prioritization

When comparing possible changes, prefer the one that protects the Core Contract earliest:

1. Correctness of the fold/atomicity/conflict/state-transition mechanism, and the governance
   that keeps it from drifting.
2. Specified feature completeness for concepts already declared in OpenSpec.
3. Operator and developer ergonomics (docs, examples).
4. Graduation decisions (a real consumer, a public release) — never pursued merely because a
   correctness or governance change makes them easier.

## References

- `AGENTS.cadw-law.md` — the generated projection of the accepted constitution.
- `docs/domain-language.md` — the canonical vocabulary (Target, State, Move, Validator, Rejection,
  Ledger).
- `BACKLOG.md` — the origin, every settled decision and its reason, and deferred work.
- `crates/cadw-governance/README.md` — the boundaries the governance gate enforces.
- `openspec/specs/` — the shipped specification.
