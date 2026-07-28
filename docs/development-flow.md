# Development Flow

This project uses OpenSpec for spec-driven development. `AGENTS.md` is the
authoritative contributor and agent guide; this file is a short checklist.

## One Change

1. Explore current specs and code before editing:
   - `openspec list --specs`
   - `openspec list`
   - read relevant files under `openspec/specs/`
2. Propose the change:
   - `openspec new change "<change-name>"`
   - write `proposal.md`, `design.md`, `tasks.md`, and delta specs
3. Apply the change:
   - implement against `openspec/changes/<change-name>/specs/`
   - check off tasks only after code and tests pass
4. Sync verified semantics:
   - promote verified delta specs into `openspec/specs/`
   - remove the completed change directory — there is no
     `openspec/changes/archive/` folder; archive means deletion, and the
     merged pull request keeps the deliberation. Do not run
     `openspec archive`.
5. Open a pull request against `main` for the whole change (branch commits can
   be as granular as you like; the pull request is squash-merged, so only its
   title and body need to read as the final record).

## Commit Granularity

Development-branch commits should be larger than individual task checkboxes
and smaller than an entire risky feature. Prefer one commit per coherent
milestone that builds, tests, and preserves the spec contract — they get
squashed on merge, so precision there matters less than at the pull request
itself. See `AGENTS.md`'s Commit And Integration Governance for the pull
request and squash-merge rules.

Avoid:

- committing unrelated docs, refactors, and behavior together
- checking off `tasks.md` before the Definition of Done passes
- syncing `openspec/specs/` before implementation has been verified

## Definition Of Done

Run these from the workspace root:

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all --check
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
cargo deny check
cargo run -p cadw-governance -- check --manifest-path Cargo.toml
```
