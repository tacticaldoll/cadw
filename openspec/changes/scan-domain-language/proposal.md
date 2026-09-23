# Proposal

## Why

The Terminology list moved from `PROJECT.md` to `docs/domain-language.md` when the repository took
the family skeleton. The discarded-working-name check scans only the four root prose files, so the
canonical vocabulary is now the one governed document where those names could return unnoticed.
`BACKLOG.md` records this as deferred work that needs its own change.

## What Changes

- The active-prose requirement in `architectural-governance` and both of its scenarios name
  `docs/domain-language.md` alongside `PROJECT.md`, `README.md`, `AGENTS.md`, and `BACKLOG.md`.
- The `cadw-governance` runner scans `docs/domain-language.md` for the discarded working names
  with the same line scan and the same missing-file failure as the root files.
- A reaction test proves a stale name in `docs/domain-language.md` alone fails the gate.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `architectural-governance`: the "Active prose never reintroduces discarded working-name
  vocabulary" requirement extends to `docs/domain-language.md`.

## Impact

- `crates/cadw-governance/src/main.rs`: `ACTIVE_PROSE_FILES` and one new test.
- `openspec/specs/architectural-governance/spec.md` after sync.
- `BACKLOG.md`: the deferred entry is closed.
- No Tianheng law changes: the constitution, `AGENTS.cadw-law.md`, and `list --format json` stay
  as they are.
