# cadw

The curated entrypoint to Cadw: a thin, sans-I/O batch-fold core you compose.

`cadw` is a pure re-export facade — it carries no logic of its own. It re-exports the
compose-level surface you need to fold a batch of moves end to end: the addressable
identity (`TargetId`), a target's lifecycle (`State`), a single declared operation
(`Move`'s `Create`/`Close`/`Reopen`), the domain-supplied judgment port (`Validator`),
why a batch was rejected (`Rejection`), and the ledger itself (`Ledger`,
`Ledger::fold_batch`). This is the recommended crate to depend on.

Cadw owns one mechanism — atomic batch-fold over addressable targets, with structural
conservative retention — and outsources every semantic judgment to the domain: given a
batch of moves and a `Validator`, `Ledger::fold_batch` applies every move or none, and
makes no judgment of its own about what a `Close`'s outcome means.

Cadw's whole public surface is compose-level, so this facade withholds nothing; there is
no advanced kernel to reach for through
[`cadw-contract`](https://crates.io/crates/cadw-contract) directly.

Part of [Cadw](https://github.com/tacticaldoll/cadw).

## License

Licensed under either of [Apache-2.0](https://github.com/tacticaldoll/cadw/blob/main/LICENSE-APACHE)
or [MIT](https://github.com/tacticaldoll/cadw/blob/main/LICENSE-MIT), at your option.
