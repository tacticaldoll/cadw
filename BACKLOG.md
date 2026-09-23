# Backlog & Deferred Decisions

## Origin

Cadw grew out of a design discussion while hardening a consumer's deliberation loop: its arbitrator
authors an entire successor document each turn (whole-document echo) rather than declaring discrete,
individually-validated operations. Two real bugs found while hardening that consumer that session
are direct, first-hand evidence that this class of "batch of things touching shared state" mechanism
is easy to get subtly wrong — a shared-scope claim collision (two unrelated coordinates sharing a
claim scope let one silently settle the other's pact), and domain validation settling outside a
claim's boundary (a structurally-valid-but-semantically-invalid response still marked "succeeded").
Two of `cadw-contract`'s vacuum tests
(`two_moves_in_one_batch_targeting_the_same_target_are_rejected`,
`a_batch_with_one_invalid_move_applies_none_of_them`) are direct translations of those two bugs.

This is a Tier 2 spike per a private family-level roadmap: an isolated, sans-I/O pure core, proven
in vacuum before any wiring or publishing decision — not an assumption that that consumer (or
anyone) will adopt it. See `PROJECT.md`'s `## Graduation` for the concrete, stated condition under
which that question gets reopened.

## Settled Decisions

- **Three-crate layout (core, facade, governance), not the reference implementation's six.** The
  reference implementation's workspace splits contract, executor, driver, memory, conformance, and
  facade crates because it has multiple backends implementing one trait, an execution-composition
  layer, and a curated facade distinct from its advanced core. `cadw-contract` has none of the
  multi-backend or execution-composition concerns: `Ledger` is one concrete type, not a trait with
  multiple implementations. Building those extra crates now would be governance surface for concerns
  that don't exist yet — see "A `cadw-conformance` crate" below for the condition under which that
  changes. A curated facade (`cadw`, `add-cadw-facade`) is a different thing from those
  execution-composition crates, and does earn its keep: sibling bricks prove the same
  core-plus-facade shape, and the originating consumer depends on the facade of every other brick it
  uses — `cadw-contract` was the sole, temporary exception until this facade existed.
- **`cadw-contract` has zero dependencies — stricter than the reference implementation's core
  contract** (which allows `serde`, `uuid`). `TargetId` wraps a plain `String`; nothing in this
  kernel is serialized or carries a UUID, so there is nothing to allow.
- **The no-serde rule covers the whole crate**, not a `kernel` submodule as in the reference
  implementation's equivalent check — `cadw-contract` has no kernel/durable-record split; the entire
  crate is "kernel," and serializing a domain's `Outcome` is that domain's own concern, never this
  crate's.
- **`Move` and `Rejection` are `#[non_exhaustive]`**: open for extension (a future variant can be
  added without breaking a downstream `_ =>` matcher), closed for modification (today's variants
  stay small and fixed rather than accreting optional fields into a growing DSL). This is the
  open/closed principle applied deliberately, not an accident of derive-macro habit.
- **`Reopen` carries no payload.** "Why a target was reopened" is an audit concern (a consumer
  would record it as its own event, separately), not a state-transition concern — the two are kept
  apart on purpose, so the state machine itself stays minimal.
- **`Validator::Rejection` is an associated type, not a generic parameter** — mirrors
  the reference implementation's `type Error: std::error::Error` pattern exactly, rather than
  inventing a new idiom for the same "domain supplies its own closed, structured error" shape.
- **Target creation and discovery are out of scope — superseded for creation
  (`add-target-creation-to-kernel`), discovery still stands.** Originally: "A `Ledger` is
  constructed already populated with its full set of `Open` targets by whoever assembles it —
  mirrors how a fresh [unit of work in the reference implementation] is *submitted*, a concern [the
  reference implementation] keeps separate from claim/settle authority." That reasoning held until a
  real consumer's shape existed to check it against: the originating consumer's shipped
  structured-move authorship — `AddRisk`/`AskQuestion` create a target *mid-batch*, not as a
  separate out-of-band submission step — 2 of its 5 `Move` variants, exercised live in its own
  dogfooding. `Move::Create { target }` now covers this (no payload, no id generation — the caller
  still supplies the `TargetId`; the kernel still never discovers or enumerates targets on its own).
  `Ledger::new`'s pre-population path is unchanged and remains the common case; `Create` is
  additive, not a replacement.
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
- **Graduation to Tier 1 is complete, not merely deferred anymore.** `PROJECT.md`'s stated trigger
  fired: the originating consumer adopted `cadw-contract` and proved the fit through real end-to-end
  dogfooding. `cadw-contract` `0.1.0` and the curated `cadw` facade are both published to crates.io,
  completing the release half — leaving `cadw-governance` unpublished, since it was never part of
  the trigger. See `PROJECT.md`'s `## Graduation` for the full record.
- **Workflow and archive convention match the reference implementation's and the originating
  consumer's exactly, from the first commit**: the
  four-step `explore -> propose -> apply -> sync` lifecycle (sync merges specs *and deletes* the
  change directory — nothing is archived into `openspec/changes/archive/`, never run
  `openspec archive`), and
  branch → PR → squash-merge for every change, including this repository's own first one. (An
  earlier direct-to-`main` commit and a later default-`openspec-archive` sync were both corrected
  by deleting and rebuilding the repository, rather than retrofitting history — see this
  repository's actual `git log` for the clean result.)
- **`architectural-governance/spec.md`'s active-prose Requirement understated what the gate
  checks.** `cadw-governance/src/main.rs`'s `ACTIVE_PROSE_FILES` has always covered four files
  (`AGENTS.md`, `PROJECT.md`, `README.md`, `BACKLOG.md`); the spec's Requirement text named only
  the first three. Corrected the spec to name all four — the code's behavior was already right,
  the spec simply never caught up to it.
- **Deferred: `docs/domain-language.md` is not yet under the active-prose gate.** The Terminology
  list moved there from `PROJECT.md` when the repository took the family skeleton, so the
  discarded-working-name check no longer scans it. Extending the gate is a requirement change and
  belongs in its own OpenSpec change to `architectural-governance`.
- **`CHANGELOG.md` is deliberately excluded from the active-prose stale-phrase gate.** Its
  `[0.1.0]` entry legitimately narrates the discarded working names this repository grew out of
  (see `cadw-governance/src/main.rs`'s `STALE_PHRASES` for the literal phrases — deliberately not
  repeated verbatim here, since this file is itself governed prose) as history of the rename
  itself — adding `CHANGELOG.md` to `ACTIVE_PROSE_FILES` would make the gate fail on the project's
  own release history. Recorded here so a future session does not "complete" the governed-file
  list by adding it and break CI discovering why the hard way.
- **Two of a sibling governance crate's safety-net tests were adopted verbatim in spirit.** That
  sibling pinned the identical `tianheng = "0.3.0"` (checked against both `Cargo.lock`s) and already
  tests `current_active_prose_satisfies_governance` (runs `check_active_prose` against the real
  workspace root, not just string fixtures) and `missing_active_prose_file_fails_loudly` (a root
  missing every governed file must fail loudly, not vacuously pass). `cadw-governance` had the
  identical `check_active_prose` logic — including the same unreadable-file branch — with neither
  test exercising it; `cargo test -p cadw-governance` alone could not have caught a mistake in
  either, only CI's separate `check` invocation could. Added both.
- **Not adopted: a sibling's generated `law_projection_is_fresh` / `AGENTS.*-law.md` mechanism.**
  `cadw`'s Constitution declares 3 boundaries total; that sibling's declares roughly a dozen across
  three dependency-kind variants per crate plus a semantic async-exposure reaction. Reading
  `constitution()` directly is still the fastest way to audit cadw's boundaries — a generated
  projection earns its keep once that stops being true, the same "not before it's needed"
  reasoning already applied to deferring a `cadw-conformance` crate below. Revisit if/when the
  boundary count grows enough to change that.
- **`docs/adr/` is dissolved; this repository does not keep a standalone ADR practice.**
  `PROJECT.md`'s own Lineage cites the reference implementation this repository's shape was observed
  from — and that implementation has no `docs/adr/` at all, recording every settled/deferred
  decision in its own `BACKLOG.md` (its `PROJECT.md` References line reads, word for word, "Deferred
  decisions: `BACKLOG.md`") plus `docs/blueprint.md` for architecture. Five of this family's other
  nine sibling repositories likewise carry no ADR folder; only three do, and none of those is cadw's
  stated reference. cadw's three ADRs had, in practice, already drifted into duplicating
  `BACKLOG.md`: ADR 0003's content ("sync means delete, not archive") was already substantively
  restated by this file's own "Workflow and archive convention match the reference implementation's
  and the originating consumer's exactly" entry above; ADR 0002's Decision ("use OpenSpec as the
  source of truth") was already stated more fully by `AGENTS.md`'s OpenSpec section — its Context is
  the one part worth preserving here: chat history and agent-specific command shims are not a
  reliable source of truth for AI-assisted development, which is why this project's actual behavior
  lives in `openspec/specs/` instead. ADR 0001 (the decision to keep ADRs at all) is superseded
  outright by this entry. All three files were removed; no other governed prose file referenced
  `docs/adr/` (`AGENTS.md`, `PROJECT.md`, `README.md`, and this file were grepped clean) — only
  `CHANGELOG.md` does, in its already-released `[0.1.0]` history, which stays untouched as the
  historical record it is.
- **Pre-release audit performed; no changes required beyond what is already recorded above.**
  Swept `cadw-contract`'s `fold_batch` logic by hand for correctness (duplicate-detection runs
  before any structural check; structural checks always read the pre-batch `self.targets`, never
  a partially-mutated intermediate state; the apply pass only ever produces a fresh `Ledger` via
  `Ok`, so no path can leak a partial mutation on `Err`) — found nothing beyond what the existing
  vacuum tests already prove. Ran `cargo clippy --workspace --all-targets -- -W dead_code -W
  unused` (clean) and `cargo clippy -- -W clippy::pedantic` (informational only, not part of the
  Definition of Done) to check for anything `-D warnings` might not surface. Two pedantic
  suggestions were considered and declined:
  - Merging `Move::Create`'s and `Move::Reopen`'s apply-loop match arms (both currently insert
    `State::Open`, so clippy's `match_same_arms` fires). Declined: `Move` is `#[non_exhaustive]`
    specifically to keep these as distinct, separately-named operations (see this file's own
    entry above on that point) — merging the arms would trade away that visible domain distinction
    for one fewer line, exactly the kind of premature-DRY this project's vocab-as-governance stance
    argues against.
  - `clippy::manual_string_new` on `"".into()` inside `lib.rs`'s test module. Declined as too
    trivial (test-only, purely stylistic) to be worth touching.
  Also checked `cargo update --dry-run` (nothing to update — already at latest compatible
  versions) and `cargo deny check --show-stats` (0 warnings across advisories/bans/licenses/
  sources). No documentation drift found: all three crate `README.md`s were re-read against
  `PROJECT.md`'s current `## Status`, and a repo-wide grep for stale `docs/adr`/`ADR` mentions
  turned up only this file's own explanatory prose and `CHANGELOG.md`'s untouched historical
  record. No crates.io release was cut: neither `cadw-contract` nor `cadw`'s published source
  changed since `0.1.0` — this session's prior changes touched only `BACKLOG.md`, `CHANGELOG.md`,
  `docs/adr/`, spec text, `cadw-governance`, CI, and Definition-of-Done docs, so there is no new
  code to publish. Unreleased work is recorded in OpenSpec changes, pull requests, and this file
  until a real code change to a published crate warrants cutting a new version; `CHANGELOG.md` is a
  release ledger with no `[Unreleased]` section.

## Deferred Work

- **A `cadw-conformance` crate.** Worth adding only once `Ledger` (or a domain's `Validator`)
  needs to be proven against more than one implementation — not before. Carrying one now, unused,
  would itself be governance surface with nothing to stay in sync with.
- ~~**Graduation** (real bridge consumer, public visibility, a crates.io release)~~ — moved to
  Settled Decisions above; no longer deferred.
