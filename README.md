# Cadw

**Tier 1 — graduated and published to crates.io.** `ringi` is the real bridge consumer whose
adoption completed graduation; see `PROJECT.md` for the Core Contract and full Graduation record,
and `BACKLOG.md` for every settled decision and its reason.

Cadw ("keep, retain, preserve" — Welsh) is a sans-I/O kernel for atomically folding a batch of
individually-validated `Create`/`Close`/`Reopen` moves over addressable targets, with conservative
retention as a structural property of the data model rather than a checked invariant.

Born from a design discussion while hardening `ringi`'s deliberation loop: an arbitrator authoring
an entire successor document each turn (whole-document echo) kept giving two real bugs new places
to hide — an immutable field silently drifting, and a domain-rejected response still settling as
"succeeded." Two of this crate's vacuum tests are direct translations of those two bugs.

`ringi` adopted the kernel for its own structured move/operation authorship, proved the fit
through real end-to-end dogfooding, and both `cadw-contract` and the `cadw` facade are now
published — the open question this spike started with is answered.

## Workspace

- `crates/cadw-contract` — the kernel: `TargetId`, `State`, `Move`, `Validator`, `Rejection`,
  `Ledger::fold_batch`.
- `crates/cadw` — the curated public entrypoint: a pure re-export facade, matching
  `pacta`/`suunta`/`shaahid`'s own facade convention. This is the recommended crate to depend on.
- `crates/cadw-governance` — executable architectural governance (tianheng), built first per
  explicit direction. Run it with:

  ```bash
  cargo run -p cadw-governance -- check --manifest-path Cargo.toml
  ```

## Definition of Done

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all --check
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
cargo deny check
cargo run -p cadw-governance -- check --manifest-path Cargo.toml
cargo run --example dissent_resolution -p cadw-contract
```

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT), at your option.
