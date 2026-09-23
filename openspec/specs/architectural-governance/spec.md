# architectural-governance Specification

## Purpose

Executable architectural governance (tianheng) for the Cadw workspace: enforces the observable
part of `PROJECT.md`'s Core Contract boundaries (`cadw-contract`'s zero-dependency and sans-I/O
purity, its no-serde rule, `cadw-governance`'s own independence), leaving the rest to review, and
guards against active-prose drift reintroducing this repository's discarded working names.

## Requirements

### Requirement: cadw-contract has zero non-dev dependencies

`cadw-contract` SHALL declare no dependencies beyond the Rust standard library. The governance
gate observes the normal dependency table; a build-dependency is review-governed.

#### Scenario: The governance gate fails if cadw-contract acquires a dependency

- **WHEN** `cadw-contract`'s manifest declares any normal dependency
- **THEN** `cargo run -p cadw-governance -- check` fails, naming the boundary and its reason

### Requirement: cadw-contract performs no I/O

`cadw-contract` SHALL call none of `std::io`, `std::fs`, `std::net`, or `std::process` anywhere
in its source. The governance gate observes inline calls into those paths in the library source;
macro-expanded I/O such as `println!`, a method called on an I/O value, a path taken as a value,
and the crate's examples, tests, and build script are review-governed.

#### Scenario: The governance gate fails if the kernel calls into I/O

- **WHEN** an inline call into `std::io`, `std::fs`, `std::net`, or `std::process` appears in
  `cadw-contract`'s library source
- **THEN** `cargo run -p cadw-governance -- check` fails, naming the boundary and its reason

### Requirement: cadw-contract acquires no serialization derive

`cadw-contract` SHALL not acquire `Serialize` or `Deserialize` anywhere in its source — this
crate is transient in-memory mechanism; serialization of a domain's `Outcome` is that domain's
own concern. The governance gate observes a derive or impl written in the library source; an impl
generated inside a macro, a hand impl whose self type the scan cannot resolve, and the crate's
examples and tests are review-governed.

#### Scenario: The governance gate fails if the kernel derives Serialize or Deserialize

- **WHEN** a type in `cadw-contract`'s library source has a written derive or impl of
  `Serialize` or `Deserialize`
- **THEN** `cargo run -p cadw-governance -- check` fails, naming the boundary and its reason

### Requirement: cadw-governance stays independent of the crate it judges

`cadw-governance` SHALL depend only on `tianheng`, never on `cadw-contract` or any other
workspace crate under its judgment. The governance gate observes the normal dependency table; a
dev or build dependency is review-governed.

#### Scenario: The governance gate fails if the gate itself depends on the judged crate

- **WHEN** `cadw-governance`'s manifest declares a normal dependency on `cadw-contract`
- **THEN** `cargo run -p cadw-governance -- check` fails, naming the boundary and its reason

### Requirement: Active prose never reintroduces discarded working-name vocabulary

`PROJECT.md`, `README.md`, `AGENTS.md`, `BACKLOG.md`, and `docs/domain-language.md` SHALL never
contain the discarded working names `"Motion"` or `"motion-contract"` — the identity this
repository grew out of before the brand settled on Cadw. `CHANGELOG.md` is deliberately exempt:
its released version entries legitimately narrate these discarded names as history of the rename
itself.

#### Scenario: The governance gate fails on a stale working-name phrase

- **WHEN** `PROJECT.md`, `README.md`, `AGENTS.md`, `BACKLOG.md`, or `docs/domain-language.md`
  contains the phrase `"Motion"` or `"motion-contract"`
- **THEN** `cargo run -p cadw-governance -- check` fails, naming the file, line, and phrase

#### Scenario: A missing governed prose file fails loudly

- **WHEN** any of `AGENTS.md`, `PROJECT.md`, `README.md`, `BACKLOG.md`, or
  `docs/domain-language.md` cannot be read at its path under the workspace root
- **THEN** `cargo run -p cadw-governance -- check` fails, naming the missing file, rather than
  silently skipping it and passing
