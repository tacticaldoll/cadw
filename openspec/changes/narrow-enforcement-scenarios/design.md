# Design

## Context

The accepted boundary reasons in `AGENTS.cadw-law.md` state what each Tianheng boundary observes
and carry partial-coverage clauses. The reason-correction review probed them against Tianheng
0.6.1: a dev or build dependency of `cadw-governance` on `cadw-contract` and a dev dependency of
`cadw-contract` leave the gate clean, and so does an I/O call in `examples/`, `tests/`, or
`build.rs`. `BACKLOG.md` already records that `examples/` lies outside the no-I/O scan.

## Goals / Non-Goals

**Goals:**

- Every statement of what the gate enforces or fails on describes only a shape its boundaries
  observe.
- The unobserved remainder is named as review-governed rather than dropped.

**Non-Goals:**

- Changing any boundary, reason, runner check, or reaction test.
- Narrowing product intent. The SHALL statements (`SHALL declare no dependencies beyond the Rust
  standard library`, `SHALL call none of … anywhere in its source`, `SHALL not acquire … anywhere
  in its source`) stay as written, because judgment may be broader than its tooth.
- Renaming requirements. "cadw-contract has zero non-dev dependencies" states intent; its body
  gains the observation sentence instead.
- Editing released `CHANGELOG.md` entries.

## Decisions

- **Name the remainder in the requirement body, not in new scenarios.** One sentence per
  requirement says what the gate observes and what review holds. Alternative considered: a
  "not observed" scenario per bound. Rejected, since the scenarios would assert a non-reaction that
  no repository test pins.
- **Edit the Purpose at sync.** A delta spec cannot carry a Purpose, so sync rewrites the Purpose
  sentence directly; the change is stated in the proposal and in the tasks.
- **Use the reasons' wording.** "Normal dependencies", "inline call", and "library source" match
  the accepted reasons, so prose and law read the same.

## Risks / Trade-offs

- [A future Tianheng may observe more] -> The scenarios then still hold; the review-governed
  sentences can narrow in a later change.
