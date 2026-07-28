# cadw-contract

The isolated core contract for Cadw: a sans-I/O kernel for atomically folding a batch of
declared `Close`/`Reopen` moves over addressable targets, with structural conservative
retention — a target no move mentions is unreachable by that fold, not merely left unchanged by
a checked rule.

Domain validation is a fully structured port (`Validator::Rejection: std::error::Error`),
mirroring `pacta-contract::Registry`'s `type Error` associated-type pattern.

Zero non-dev dependencies, enforced by `cadw-governance`. See the workspace root `PROJECT.md`
for the full Core Contract and Non-Goals.

Part of [Cadw](https://github.com/tacticaldoll/cadw) — Tier 2 experimental, not published.

## License

Licensed under either of [Apache-2.0](https://github.com/tacticaldoll/cadw/blob/main/LICENSE-APACHE)
or [MIT](https://github.com/tacticaldoll/cadw/blob/main/LICENSE-MIT), at your option.
