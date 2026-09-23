# Cadw Tianheng Law Projection

This file is generated from `constitution()` in `crates/cadw-governance/src/main.rs`.
The Rust declaration is authoritative; do not edit the projection by hand.
Regenerate it with `BLESS=1 cargo test -p cadw-governance law_projection_is_fresh`.

# Constitution: cadw

## Static boundaries

### `cadw-contract` (crate)

> cadw-contract is the isolated core contract: a sans-I/O kernel with no time/lease/crash-recovery dimension needs no dependency at all, so its normal dependencies are none.

- **rule**: restrict dependencies to (only: )
- **kind**: crate · **severity**: enforce

### `cadw` (crate)

> cadw is the curated public entrypoint, a re-export facade over cadw-contract: its normal dependencies are cadw-contract alone, never one the core itself does not have. That the facade carries no logic of its own is held by review, not by this dependency boundary.

- **rule**: restrict dependencies to (only: cadw-contract)
- **kind**: crate · **severity**: enforce

### `cadw-governance` (crate)

> the governance gate must stay independent of the workspace graph it judges: its normal dependencies are tianheng alone, never cadw-contract or any other workspace crate under judgment.

- **rule**: restrict dependencies to (only: tianheng)
- **kind**: crate · **severity**: enforce

### `cadw-contract::crate` (module)

> the sans-I/O core contract performs no I/O: cadw-contract's library source makes no inline call into std::io/fs/net/process; a batch fold is a synchronous, in-memory operation. Coverage is partial by nature (macro-expanded I/O such as println!, a method called on an I/O value, and a path taken as a value are invisible to a source scan, and the crate's examples, tests, and build script lie outside it), so this tooth complements review rather than replacing it.

- **rule**: inline symbol path confined to module (confined_prefix: std::io)
- **kind**: module · **severity**: enforce · **crate**: cadw-contract

### `cadw-contract::crate` (module)

> the sans-I/O core contract performs no I/O: cadw-contract's library source makes no inline call into std::io/fs/net/process; a batch fold is a synchronous, in-memory operation. Coverage is partial by nature (macro-expanded I/O such as println!, a method called on an I/O value, and a path taken as a value are invisible to a source scan, and the crate's examples, tests, and build script lie outside it), so this tooth complements review rather than replacing it.

- **rule**: inline symbol path confined to module (confined_prefix: std::fs)
- **kind**: module · **severity**: enforce · **crate**: cadw-contract

### `cadw-contract::crate` (module)

> the sans-I/O core contract performs no I/O: cadw-contract's library source makes no inline call into std::io/fs/net/process; a batch fold is a synchronous, in-memory operation. Coverage is partial by nature (macro-expanded I/O such as println!, a method called on an I/O value, and a path taken as a value are invisible to a source scan, and the crate's examples, tests, and build script lie outside it), so this tooth complements review rather than replacing it.

- **rule**: inline symbol path confined to module (confined_prefix: std::net)
- **kind**: module · **severity**: enforce · **crate**: cadw-contract

### `cadw-contract::crate` (module)

> the sans-I/O core contract performs no I/O: cadw-contract's library source makes no inline call into std::io/fs/net/process; a batch fold is a synchronous, in-memory operation. Coverage is partial by nature (macro-expanded I/O such as println!, a method called on an I/O value, and a path taken as a value are invisible to a source scan, and the crate's examples, tests, and build script lie outside it), so this tooth complements review rather than replacing it.

- **rule**: inline symbol path confined to module (confined_prefix: std::process)
- **kind**: module · **severity**: enforce · **crate**: cadw-contract

## Forbidden-marker boundaries

### `cadw-contract::crate` (semantic)

> cadw-contract is transient in-memory mechanism, not a durable record type: its library source must not acquire Serialize/Deserialize, by derive or by impl. Serialization of a domain's Outcome is that domain's own concern, never this crate's. Coverage is partial by nature (an impl generated inside a macro is invisible to a source scan, and the crate's examples and tests lie outside it), so this tooth complements review rather than replacing it.

- **rule**: must not acquire trait (forbidden: serde::Serialize, serde::Deserialize)
- **kind**: semantic · **severity**: enforce · **crate**: cadw-contract
