# ADR 0003: Sync Means Delete, Not Archive

## Status

Accepted

## Context

The `rust-openspec-starter` template this repository was created from defaults to a five-step
lifecycle — `explore -> propose -> apply -> sync -> archive` — where the archive step moves a
completed change to `openspec/changes/archive/YYYY-MM-DD-<name>/` and keeps it there
indefinitely. `ringi` and `pacta` instead use a four-step lifecycle where sync merges verified
delta specs into `openspec/specs/` *and* deletes the change directory in the same step; neither
keeps a populated archive folder, and neither runs `openspec archive`.

See `BACKLOG.md`'s Settled Decisions for the fuller story, including the false start this
repository went through before adopting this convention cleanly from its first change.

## Decision

Adopt `ringi`/`pacta`'s convention: sync means merge-then-delete. There is no
`openspec/changes/archive/` folder in this repository. A completed change's deliberation record
is its squash-merged pull request (title, body, and the commits it contained) plus whatever of
its reasoning was durable enough to belong in `PROJECT.md` or `BACKLOG.md` — not a lingering
change directory.

## Consequences

- `AGENTS.md` and `docs/development-flow.md` state the four-step lifecycle, not the template's
  five-step one; `openspec archive` is never run.
- Reasoning worth keeping past a single change's lifetime must be written into `PROJECT.md`'s
  Core Contract/Graduation or `BACKLOG.md`'s Settled Decisions/Deferred Work *during* that
  change, not left implicitly in an archived folder for someone to find later. This ADR, and
  ADR 0002's now-corrected reference to it, are themselves an instance of that discipline.
- A future session opened fresh in this repository, with no memory of any past conversation, can
  reconstruct why this repository's workflow differs from its own template's default by reading
  this file and `BACKLOG.md` — it does not need to guess or ask.
