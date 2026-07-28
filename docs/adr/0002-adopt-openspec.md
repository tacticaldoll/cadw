# ADR 0002: Adopt OpenSpec

## Status

Accepted

## Context

AI-assisted development benefits from a durable, tool-neutral source of truth.
Chat history and agent-specific command files are not reliable enough to define
the system contract.

## Decision

Use OpenSpec as the source of truth for requirements and change proposals.

Project behavior lives in `openspec/specs/`. Proposed behavior changes live in
`openspec/changes/` until they are implemented, verified, and synced — at which
point the change directory is removed. See `docs/adr/0003` for why this
repository deletes rather than archives a synced change.

## Consequences

- Feature work starts with a change proposal rather than ad hoc edits.
- Agents can share a common process through the OpenSpec CLI.
- Generated agent shims remain local to each clone and are not committed.
