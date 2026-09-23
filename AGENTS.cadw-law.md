# Cadw Tianheng Law Projection

This file is generated from `constitution()` in `crates/cadw-governance/src/main.rs`.
The Rust declaration is authoritative; do not edit the projection by hand.
Regenerate it with `BLESS=1 cargo test -p cadw-governance law_projection_is_fresh`.

# Constitution: cadw

## Static boundaries

### `cadw-contract` (crate)

> cadw-contract is the isolated core contract: a sans-I/O kernel with no time/lease/crash-recovery dimension. It needs no dependency at all, so it may depend on nothing.

- **rule**: restrict dependencies to (only: )
- **kind**: crate · **severity**: enforce

### `cadw` (crate)

> cadw is the curated public entrypoint: a pure re-export facade with no logic of its own. It must depend on cadw-contract only, never acquiring a dependency the core itself does not have.

- **rule**: restrict dependencies to (only: cadw-contract)
- **kind**: crate · **severity**: enforce

### `cadw-governance` (crate)

> the governance gate must stay independent of the workspace graph it judges: it may depend only on tianheng, never on cadw-contract or any other workspace crate under judgment.

- **rule**: restrict dependencies to (only: tianheng)
- **kind**: crate · **severity**: enforce

### `cadw-contract::crate` (module)

> the sans-I/O core contract performs no I/O: no code in cadw-contract may call into std::io/fs/net/process; a batch fold is a synchronous, in-memory operation, never a place I/O could hide.

- **rule**: inline symbol path confined to module (confined_prefix: std::io)
- **kind**: module · **severity**: enforce · **crate**: cadw-contract

### `cadw-contract::crate` (module)

> the sans-I/O core contract performs no I/O: no code in cadw-contract may call into std::io/fs/net/process; a batch fold is a synchronous, in-memory operation, never a place I/O could hide.

- **rule**: inline symbol path confined to module (confined_prefix: std::fs)
- **kind**: module · **severity**: enforce · **crate**: cadw-contract

### `cadw-contract::crate` (module)

> the sans-I/O core contract performs no I/O: no code in cadw-contract may call into std::io/fs/net/process; a batch fold is a synchronous, in-memory operation, never a place I/O could hide.

- **rule**: inline symbol path confined to module (confined_prefix: std::net)
- **kind**: module · **severity**: enforce · **crate**: cadw-contract

### `cadw-contract::crate` (module)

> the sans-I/O core contract performs no I/O: no code in cadw-contract may call into std::io/fs/net/process; a batch fold is a synchronous, in-memory operation, never a place I/O could hide.

- **rule**: inline symbol path confined to module (confined_prefix: std::process)
- **kind**: module · **severity**: enforce · **crate**: cadw-contract

## Forbidden-marker boundaries

### `cadw-contract::crate` (semantic)

> cadw-contract is transient in-memory mechanism, not a durable record type: it must not acquire Serialize/Deserialize anywhere. Serialization of a domain's Outcome is that domain's own concern, never this crate's.

- **rule**: must not acquire trait (forbidden: serde::Serialize, serde::Deserialize)
- **kind**: semantic · **severity**: enforce · **crate**: cadw-contract
