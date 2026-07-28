# Backlog & Deferred Decisions

## Origin

Cadw grew out of a design discussion while hardening `ringi`'s deliberation loop: `ringi`'s
arbitrator authors an entire successor document each turn (whole-document echo) rather than
declaring discrete, individually-validated operations. Two real bugs found while hardening
`ringi` that session are direct, first-hand evidence that this class of "batch of things touching
shared state" mechanism is easy to get subtly wrong — a shared-scope claim collision (two
unrelated coordinates sharing a claim scope let one silently settle the other's pact), and domain
validation settling outside a claim's boundary (a structurally-valid-but-semantically-invalid
response still marked "succeeded"). Two of `cadw-contract`'s vacuum tests
(`two_moves_in_one_batch_targeting_the_same_target_are_rejected`,
`a_batch_with_one_invalid_move_applies_none_of_them`) are direct translations of those two bugs.

This is a Tier 2 spike per a private family-level roadmap: an isolated, sans-I/O pure core, proven
in vacuum before any wiring or publishing decision — not an assumption that `ringi` (or anyone)
will adopt it. See `PROJECT.md`'s `## Graduation` for the concrete, stated condition under which
that question gets reopened.

## Settled Decisions

- **Two-crate layout, not pacta's six.** `pacta`'s workspace splits `pacta-contract`/
  `pacta-executor`/`pacta-driver`/`pacta-memory`/`pacta-conformance`/the `pacta` facade because it
  has multiple backends implementing one trait, an execution-composition layer, and a curated
  facade distinct from its advanced core. `cadw-contract` has none of that: `Ledger` is one
  concrete type, not a trait with multiple implementations, and there is no separate
  execution/composition concern to seam off. Building those crates now would be governance
  surface for concerns that don't exist yet — see "A `cadw-conformance` crate" below for the
  condition under which that changes.
- **`cadw-contract` has zero dependencies — stricter than `pacta-contract`'s** (which allows
  `serde`, `uuid`). `TargetId` wraps a plain `String`; nothing in this kernel is serialized or
  carries a UUID, so there is nothing to allow.
- **The no-serde rule covers the whole crate**, not a `kernel` submodule as in
  `pacta-governance`'s equivalent check — `cadw-contract` has no kernel/durable-record split; the
  entire crate is "kernel," and serializing a domain's `Outcome` is that domain's own concern,
  never this crate's.
- **`Move` and `Rejection` are `#[non_exhaustive]`**: open for extension (a future variant can be
  added without breaking a downstream `_ =>` matcher), closed for modification (today's variants
  stay small and fixed rather than accreting optional fields into a growing DSL). This is the
  open/closed principle applied deliberately, not an accident of derive-macro habit.
- **`Reopen` carries no payload.** "Why a target was reopened" is an audit concern (a consumer
  would record it as its own event, separately), not a state-transition concern — the two are kept
  apart on purpose, so the state machine itself stays minimal.
- **`Validator::Rejection` is an associated type, not a generic parameter** — mirrors
  `pacta-contract::Registry`'s `type Error: std::error::Error` pattern exactly, rather than
  inventing a new idiom for the same "domain supplies its own closed, structured error" shape.
- **Target creation and discovery are out of scope.** A `Ledger` is constructed already populated
  with its full set of `Open` targets by whoever assembles it — mirrors how a fresh `pacta::Pact`
  is *submitted*, a concern `pacta` keeps separate from claim/settle authority.
- **The atomicity test's guarantee is narrower than first stated, and stronger in a different
  way.** `fold_batch` takes `&self` and only ever returns a freshly built `Ledger` via `Ok`, so no
  code path can leak a partially-mutated ledger on `Err` regardless of internal pass ordering —
  that part of atomicity is guaranteed by the ownership shape itself, not by getting the algorithm
  right. `a_batch_with_one_invalid_move_applies_none_of_them`'s real, confirmed value is catching
  a different mistake: silently skipping an invalid move and applying the rest anyway. Verified by
  deliberately introducing exactly that mutation and watching the test fail before trusting it.
- **`examples/` sits outside `cadw-governance`'s no-I/O boundary scan.** `ModuleBoundary` scans
  only `crate::`'s own module tree starting at `src/lib.rs`; an example is a separate compilation
  target. Confirmed empirically (not assumed) — `examples/dissent_resolution.rs` uses `println!`
  and the governance gate stays clean.
- **Workflow and archive convention match `ringi`/`pacta` exactly, from the first commit**: the
  four-step `explore -> propose -> apply -> sync` lifecycle (sync merges specs *and deletes* the
  change directory — no `openspec/changes/archive/` folder, never run `openspec archive`), and
  branch → PR → squash-merge for every change, including this repository's own first one. (An
  earlier direct-to-`main` commit and a later default-`openspec-archive` sync were both corrected
  by deleting and rebuilding the repository, rather than retrofitting history — see this
  repository's actual `git log` for the clean result.)

## Deferred Work

- **A `cadw-conformance` crate.** Worth adding only once `Ledger` (or a domain's `Validator`)
  needs to be proven against more than one implementation — not before. Carrying one now, unused,
  would itself be governance surface with nothing to stay in sync with.
- **Graduation** (real bridge consumer, public visibility, a crates.io release): fully specified
  in `PROJECT.md`'s `## Graduation` section. Not restated here to avoid two documents describing
  the same condition and drifting apart.
