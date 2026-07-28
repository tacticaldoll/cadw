# architectural-governance Specification

## Purpose

Executable architectural governance (tianheng) for the Cadw workspace: enforces `PROJECT.md`'s
Core Contract boundaries (`cadw-contract`'s zero-dependency and sans-I/O purity, its no-serde
rule, `cadw-governance`'s own independence) and guards against active-prose drift reintroducing
this repository's discarded working names.

## Requirements

### Requirement: cadw-contract has zero non-dev dependencies

`cadw-contract` SHALL declare no dependencies beyond the Rust standard library.

#### Scenario: The governance gate fails if cadw-contract acquires a dependency

- **WHEN** `cadw-contract`'s manifest declares any non-dev dependency
- **THEN** `cargo run -p cadw-governance -- check` fails, naming the boundary and its reason

### Requirement: cadw-contract performs no I/O

`cadw-contract` SHALL call none of `std::io`, `std::fs`, `std::net`, or `std::process` anywhere
in its source.

#### Scenario: The governance gate fails if the kernel calls into I/O

- **WHEN** any inline call into `std::io`, `std::fs`, `std::net`, or `std::process` appears in
  `cadw-contract`'s source
- **THEN** `cargo run -p cadw-governance -- check` fails, naming the boundary and its reason

### Requirement: cadw-contract acquires no serialization derive

`cadw-contract` SHALL not acquire `Serialize` or `Deserialize` anywhere in its source — this
crate is transient in-memory mechanism; serialization of a domain's `Outcome` is that domain's
own concern.

#### Scenario: The governance gate fails if the kernel derives Serialize or Deserialize

- **WHEN** any type in `cadw-contract` derives or implements `Serialize` or `Deserialize`
- **THEN** `cargo run -p cadw-governance -- check` fails, naming the boundary and its reason

### Requirement: cadw-governance stays independent of the crate it judges

`cadw-governance` SHALL depend only on `tianheng`, never on `cadw-contract` or any other
workspace crate under its judgment.

#### Scenario: The governance gate fails if the gate itself depends on the judged crate

- **WHEN** `cadw-governance`'s manifest declares a dependency on `cadw-contract`
- **THEN** `cargo run -p cadw-governance -- check` fails, naming the boundary and its reason

### Requirement: Active prose never reintroduces discarded working-name vocabulary

`PROJECT.md`, `README.md`, and `AGENTS.md` SHALL never contain the discarded working names
`"Motion"` or `"motion-contract"` — the identity this repository grew out of before the brand
settled on Cadw.

#### Scenario: The governance gate fails on a stale working-name phrase

- **WHEN** `PROJECT.md`, `README.md`, or `AGENTS.md` contains the phrase `"Motion"` or
  `"motion-contract"`
- **THEN** `cargo run -p cadw-governance -- check` fails, naming the file, line, and phrase
