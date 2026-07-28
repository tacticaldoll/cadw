# Cadw

**Tier 2 experimental spike — not published to crates.io, not wired into any consumer.** See
`PROJECT.md` for the Core Contract and `BACKLOG.md` for settled decisions and their reasons.

Cadw ("keep, retain, preserve" — Welsh) is a sans-I/O kernel for atomically folding a batch of
individually-validated `Close`/`Reopen` moves over addressable targets, with conservative
retention as a structural property of the data model rather than a checked invariant.

Born from a design discussion while hardening `ringi`'s deliberation loop: an arbitrator authoring
an entire successor document each turn (whole-document echo) kept giving two real bugs new places
to hide — an immutable field silently drifting, and a domain-rejected response still settling as
"succeeded." Two of this crate's vacuum tests are direct translations of those two bugs.

Whether this graduates into a real, adopted family member (in the style of `pacta`/`suunta`/
`shaahid`) is an open question this spike exists to help answer — not an assumption it starts
from.

## Workspace

- `crates/cadw-contract` — the kernel: `TargetId`, `State`, `Move`, `Validator`, `Rejection`,
  `Ledger::fold_batch`.
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
```

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT), at your option.
