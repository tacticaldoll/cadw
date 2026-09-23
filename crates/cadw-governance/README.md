# cadw-governance

Executable architectural governance for the Cadw workspace — the Tianheng constitution.

This crate is an internal gate, not a published library (`publish = false`). It runs
[Tianheng](https://github.com/tacticaldoll/tianheng) to keep the workspace's architecture from
drifting:

- `cadw-contract`'s zero-dependency boundary (no normal dependency);
- `cadw-contract`'s sans-I/O purity (no inline `std::io`/`fs`/`net`/`process` call in its library
  source);
- `cadw-contract`'s no-serialization rule (no `Serialize`/`Deserialize` derive or impl written in
  its library source; it is transient in-memory mechanism, never a durable record type —
  serializing a domain's `Outcome` is that domain's own concern);
- `cadw-governance`'s own independence from the crate it judges (its normal dependencies are
  `tianheng` alone);
- active-prose drift: this repository's discarded working names (`"Motion"`,
  `"motion-contract"`) must never reappear in `PROJECT.md`, `README.md`, `AGENTS.md`,
  `BACKLOG.md`, or `docs/domain-language.md`.

Shapes these boundaries do not observe — among them build and dev dependencies, macro-expanded
code, a method called on an I/O value, and the crate's examples, tests, and build script — stay
with review. The accepted boundaries and their coverage clauses are in `AGENTS.cadw-law.md` at the
repository root.

Run it from the workspace root:

```sh
cargo run -p cadw-governance -- check --manifest-path Cargo.toml
```

Part of [Cadw](https://github.com/tacticaldoll/cadw).

## License

Licensed under either of [Apache-2.0](https://github.com/tacticaldoll/cadw/blob/main/LICENSE-APACHE)
or [MIT](https://github.com/tacticaldoll/cadw/blob/main/LICENSE-MIT), at your option.
