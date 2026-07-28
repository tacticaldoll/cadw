# batch-fold-core Specification

## Purpose

A sans-I/O kernel for folding a batch of declared `Close`/`Reopen` moves over addressable targets
atomically — every move in a batch applies, or none do — with structural conservative retention:
a target no move mentions is unreachable by that fold, not merely left unchanged by a checked
rule. The kernel owns the fold/atomicity/conflict/state-transition mechanism only; it never
judges whether a specific `Outcome` is semantically valid — that is the domain-supplied
`Validator` port.

## Requirements

### Requirement: A batch of Moves applies atomically or not at all

`Ledger::fold_batch` SHALL apply every `Move` in a batch, or none of them, never a partial subset.
No move is ever silently dropped from a batch that is otherwise reported as fully applied.

#### Scenario: A batch with one invalid move applies none of them

- **WHEN** a batch contains one structurally or semantically invalid `Move` alongside otherwise
  valid ones
- **THEN** `fold_batch` returns a `Rejection` and the resulting `Ledger` reflects none of the
  batch's moves, not even the individually-valid ones

### Requirement: Two moves in one batch cannot target the same target

`fold_batch` SHALL reject a batch where more than one `Move` addresses the same `TargetId`,
before evaluating any other rule.

#### Scenario: A duplicate target within one batch is rejected

- **WHEN** a batch contains two `Move`s addressing the same `TargetId`
- **THEN** `fold_batch` returns `Rejection::DuplicateTargetInBatch` for that target, and no move
  in the batch applies

### Requirement: A target not mentioned in a batch is unchanged by construction

A `Target` absent from a batch SHALL be unreachable by that batch's fold — not merely left
unchanged by a checked rule, but structurally unaddressable.

#### Scenario: An untouched target survives a batch unchanged

- **WHEN** a batch of `Move`s does not mention a given `TargetId`
- **THEN** that target's state in the resulting `Ledger` is identical to its state before the
  fold

### Requirement: Close and Reopen follow a strict Open/Closed state machine

`Close` SHALL only succeed against an `Open` target; `Reopen` SHALL only succeed against a
`Closed` target. Each violation is a distinct, structured rejection.

#### Scenario: Closing an already-closed target is rejected

- **WHEN** `Close` addresses a target that is already `Closed`
- **THEN** `fold_batch` returns `Rejection::AlreadyClosed`

#### Scenario: Reopening an open target is rejected

- **WHEN** `Reopen` addresses a target that is `Open`
- **THEN** `fold_batch` returns `Rejection::NotClosed`

#### Scenario: A closed target can be reopened and closed again

- **WHEN** a `Closed` target is reopened by one batch and then closed again by a later batch
- **THEN** both operations succeed in sequence, and the target ends `Closed` with the later
  batch's `Outcome`

### Requirement: Domain validation is a fully structured port, never a free string

`Validator::Rejection` SHALL be a type implementing `std::error::Error`, supplied by the domain
adopting this crate — never a `String` on any path from the kernel's `Rejection` type through to
domain-supplied content.

#### Scenario: A structurally valid move can still be rejected by the domain's validator

- **WHEN** a `Move`'s target and state-transition are structurally legal, but the domain's
  `Validator::validate` returns an `Err`
- **THEN** `fold_batch` returns `Rejection::Invalid` wrapping the domain's own structured
  rejection value, and the batch does not apply

### Requirement: Unknown targets are rejected

`fold_batch` SHALL reject any `Move` addressing a `TargetId` the `Ledger` does not contain.

#### Scenario: A move addressing an unknown target is rejected

- **WHEN** a `Move` addresses a `TargetId` absent from the `Ledger`
- **THEN** `fold_batch` returns `Rejection::UnknownTarget`

### Requirement: An empty batch succeeds trivially

`fold_batch` called with no moves SHALL succeed, returning a `Ledger` identical to the input.

#### Scenario: An empty batch changes nothing

- **WHEN** `fold_batch` is called with an empty slice of moves
- **THEN** it returns `Ok` with a `Ledger` whose every target's state matches the input exactly
