# Spec Delta

## MODIFIED Requirements

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
