# cadw-governance

Executable architectural governance for the Cadw workspace — the Tianheng constitution.

This crate is an internal gate, not a published library (`publish = false`). It runs
[Tianheng](https://github.com/tacticaldoll/tianheng) to keep the workspace's architecture from
drifting:

- `cadw-contract`'s zero-dependency boundary;
- `cadw-contract`'s sans-I/O purity (no `std::io`/`fs`/`net`/`process` call anywhere in its
  source);
- `cadw-contract`'s no-serialization rule (it is transient in-memory mechanism, never a durable
  record type — serializing a domain's `Outcome` is that domain's own concern);
- `cadw-governance`'s own independence from the crate it judges;
- active-prose drift: this repository's discarded working names (`"Motion"`,
  `"motion-contract"`) must never reappear in committed prose.

Run it from the workspace root:

```sh
cargo run -p cadw-governance -- check --manifest-path Cargo.toml
```

Part of [Cadw](https://github.com/tacticaldoll/cadw).

## License

Licensed under either of [Apache-2.0](https://github.com/tacticaldoll/cadw/blob/main/LICENSE-APACHE)
or [MIT](https://github.com/tacticaldoll/cadw/blob/main/LICENSE-MIT), at your option.
