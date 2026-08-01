# Changelog

All notable changes to this project are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres
to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1] - 2026-08-01

Governance, documentation, and CI hardening. No change to `cadw-contract` or `cadw`'s published
source — this release exists to record the accumulated process work below, not new library
behavior.

### Changed

- `openspec/specs/architectural-governance/spec.md`'s active-prose Requirement corrected to name
  all four files `cadw-governance` has always checked (`AGENTS.md`, `PROJECT.md`, `README.md`,
  `BACKLOG.md`), not three — the spec had not caught up to the code.
- `docs/adr/` removed; this repository no longer keeps a standalone ADR practice. `PROJECT.md`'s
  own Lineage names `pacta` — which keeps no ADR folder — as this repository's reference
  implementation, and cadw's three ADRs had drifted into duplicating content `BACKLOG.md` and
  `AGENTS.md` already carried. All non-duplicated reasoning was folded into new `BACKLOG.md`
  Settled Decisions.
- `batch-fold-core/spec.md`'s "Domain validation is a fully structured port, never a free
  string" Requirement gains a Scenario stating that its realistic multi-field composition proof
  must be continuously demonstrated, not merely compiled.

### Added

- `cadw-governance`: a code comment recording why `CHANGELOG.md` is deliberately excluded from
  the active-prose gate, and two safety-net tests adopted from sibling `shaahid-governance`
  (`current_active_prose_satisfies_governance`, `missing_active_prose_file_fails_loudly`).
- The Definition of Done (`AGENTS.md`, `docs/development-flow.md`, `README.md`) and CI's `dod` job
  now run `cargo run --example dissent_resolution -p cadw-contract`. Previously
  `cargo build`/`cargo test --workspace` only compiled this non-toy example; nothing executed its
  assertions, so a regression that kept it compiling but broke its behavior would have passed
  every existing gate silently.

### Chore

- Pre-release audit: hand-swept `fold_batch` for correctness, ran clippy beyond the Definition of
  Done's default lint level (`dead_code`/`unused`, `clippy::pedantic`), checked
  `cargo update --dry-run` and `cargo deny --show-stats`, and re-read every crate `README.md`
  against `PROJECT.md`'s current status. No bugs, dead code, or documentation drift found; two
  pedantic clippy suggestions were considered and declined (recorded in `BACKLOG.md`).

## [0.1.0] - 2026-07-29

First release: `cadw-contract`'s sans-I/O batch-fold kernel and the `cadw` curated facade, both
published together, completing the Tier 1 graduation trigger `ringi` fired by actually adopting
the kernel for its own residual-ledger validation.

### Added

- Workspace shape: a two-crate layout (`cadw-contract`, `cadw-governance`) built governance-first
  per explicit direction, adopting `ringi`/`pacta`'s exact `explore -> propose -> apply -> sync`
  workflow and commit governance (Conventional Commits, branch -> PR -> squash-merge, no AI
  attribution) from the very first change.
- **`cadw-contract`**: the isolated, sans-I/O batch-fold kernel — `TargetId`, `State`, `Move`
  (`Create`/`Close`/`Reopen`), `Validator`, `Rejection`, and `Ledger::fold_batch`, atomically
  applying a batch of moves over addressable targets with structural conservative retention (a
  target no move mentions is unreachable by that fold, not merely left unchanged by a checked
  rule). Ported from the `motion-contract` throwaway prototype, with adversarial tests added (a
  large mixed batch, an empty batch, a reopen-only batch).
- **`cadw-governance`**: executable architectural governance (`tianheng`), built first per
  explicit direction — a Constitution enforcing `cadw-contract`'s zero-dependency and no-I/O
  boundaries, a no-serde forbidden-marker rule, `cadw-governance`'s own independence from the
  crate it judges, and an active-prose gate against reintroducing this repository's discarded
  working names ("Motion", "motion-contract"). Every boundary was deliberately violated once and
  confirmed to fail the gate before being trusted.
- A non-toy consumer example (`crates/cadw-contract/examples/dissent_resolution.rs`) modeling
  `ringi::revision::Resolution`'s real shape (a `reason` and event `provenance`, both required
  non-empty) as a `Validator` implementation, proving the port composes with a realistic
  multi-field outcome and a closed, multi-variant rejection enum, not just the vacuum suite's
  trivial `String` outcome.
- `Move::Create { target }` and `Rejection::AlreadyExists`: `ringi` adopted structured move/
  operation authorship for its own domain (`PROJECT.md`'s stated Graduation trigger, actually
  firing) and its `AddRisk`/`AskQuestion` moves create a target mid-batch — a real, structural gap
  `Ledger::fold_batch` had no way to express. `Create` carries no payload, matching the kernel's
  existing content-blindness; `Ledger::new`'s pre-population path is unchanged, `Create` is
  additive.
- **`cadw`**, the curated public entrypoint: a pure `pub use cadw_contract::*;` re-export,
  matching `pacta`/`suunta`/`shaahid`'s own facade convention exactly, with a runnable
  "Composing a ledger" doctest and a `cadw-governance` boundary restricting its own dependencies
  to `cadw-contract` only.
- `deny.toml`, mirroring `pacta`'s, verified clean against cadw's actual dependency graph before
  adopting it.
- CI (`.github/workflows/ci.yml`): Definition of Done, `cargo-deny`, and `cadw-governance` checks,
  on every push and pull request — deliberately simpler than `pacta`'s (no feature-matrix job, no
  MSRV job, until either is actually needed).
- `docs/adr/0003-sync-means-delete-not-archive.md`, the formal, dated record `docs/adr/0001` says
  a decision like this deserves.
- `BACKLOG.md`, the durable record of why: an Origin section (the `ringi` design tension and the
  two real bugs that motivated this spike), Settled Decisions (every scoping and design call made
  across this repository's changes, with its reason), and Deferred Work.

### Changed

- The target-creation/discovery Non-Goal narrowed to discovery only, once `Move::Create` shipped:
  `Create` still takes a caller-supplied id, never generates or enumerates one.
- `PROJECT.md`'s Graduation section, once stated only as "a separate, later decision," now names
  the concrete trigger (a real consumer deciding to pursue structured move/operation authorship
  for its own domain), then records that trigger firing and the crates.io publish completing it.

### Fixed

- `docs/adr/0002-adopt-openspec.md`, a starter-template stub never touched since this repository
  was created, still described the lifecycle as ending in "implemented, verified, synced, and
  archived" — directly contradicting `AGENTS.md`'s actual convention (sync merges specs and
  deletes the change directory; no archive folder; never run `openspec archive`). The active-prose
  stale-phrase gate does not catch this class of drift (it only scans for discarded working-name
  vocabulary), so it sat wrong, undetected, since the first change.
- `README.md`'s opening paragraph called `Move`s "operations," while `PROJECT.md` and every doc
  comment consistently say "moves" — fixed to the canonical term.

### Documentation

- `README.md` corrected: the opening line asserted "Tier 2 experimental spike — not published to
  crates.io, not wired into any consumer," stale since publishing; now states the actual Tier 1,
  published, `ringi`-adopted status, and `## Workspace` now lists `crates/cadw` alongside
  `cadw-contract`/`cadw-governance`.

[0.1.1]: https://github.com/tacticaldoll/cadw/releases/tag/v0.1.1
[0.1.0]: https://github.com/tacticaldoll/cadw/releases/tag/v0.1.0
