# AGENTS.md

Meta-guideline for any AI coding agent working in this repository. Read this
first.

## This Project Uses OpenSpec

The source of truth lives in `openspec/`, which is version-controlled and
agent-agnostic.

- `openspec/specs/` - the living specification of what the system currently is.
- `openspec/changes/` - active change proposals as delta specs.

Per-agent command files such as `.codex/`, `.claude/`, and editor-specific shims
are per-clone generated files and are not committed. After cloning, generate
your own with:

```bash
openspec init --tools codex
# or: openspec init --tools claude,cursor,github-copilot
```

## Workflow

Follow this lifecycle:

```text
explore -> propose -> apply -> sync
```

1. **Explore**: think and investigate only. Do not write feature code outside of
   a change.
2. **Propose**: create a change with `proposal.md`, `design.md`, `tasks.md`, and
   delta specs.
3. **Apply**: implement tasks one at a time, checking each off in `tasks.md`
   only after verification.
4. **Sync**: merge verified delta specs into `openspec/specs/` (agent-driven),
   then remove the completed change directory. There is **no**
   `openspec/changes/archive/` **folder** — archive means deletion; git history
   (and the merged pull request) keeps the deliberation. Do not run
   `openspec archive`.

## OpenSpec CLI

If your agent has no OpenSpec slash commands, use the CLI:

```bash
openspec list [--json] [--specs]
openspec new change "<name>"
openspec status --change "<name>" --json
openspec instructions <artifact> --change "<name>"
```

## Rules

- Before implementing anything, read the relevant files in `openspec/specs/` and
  the active change's artifacts.
- Do not write feature code without an active change proposal that contains
  tasks.
- Keep changes minimal and scoped to the task being implemented.
- Treat `openspec/specs/` as the truth. Reflect requirement changes there via
  the sync step, not by editing code silently.
- Keep project-specific contract, terms, and priorities in `PROJECT.md`.

## Language

- Write OpenSpec artifacts, ADRs, code comments, and commit messages in English.
- Converse with users in the language they use.

## Commit And Integration Governance

### Branch Commits

- Use Conventional Commits: `type(scope): summary`.
- Write the subject in English, lowercase imperative mood, at no more than 72
  characters.
- Use the body to record motivation, important decisions, constraints, and
  verification when that context exists.
- Do not append pull request or issue numbers to the subject or body.
- Development branches may contain multiple coherent commits because the pull
  request is squash-merged.

### Pull Requests

- Branch from `main` and open every change directly against `main`.
- Make the pull request title the intended squash commit subject.
- Give every pull request a non-empty body that explains why the change is
  needed, what changed, consequential decisions or tradeoffs, and
  verification.
- Rebase the branch onto the current `main` before final verification.
- Do not introduce a release integration branch between a change and `main`.

### Squash Merges

- Squash-merge every verified pull request into `main`.
- Make the squash commit subject exactly the approved pull request title.
- Give every squash commit a non-empty body distilled from the approved pull
  request body.
- Do not append a pull request number, issue number, or URL to the squash
  subject or body.
- Every content-changing commit on `main` must come from a squash-merged pull
  request.
- Keep `main` releasable after every merge.

### Attribution

- Do not include AI, agent, model, tool, automation, or generation attribution
  in commits, pull requests, tags, changelogs, or release notes.
- A `Co-authored-by` trailer is allowed only for a real human contributor.

## Definition Of Done

Run these from the workspace root before checking off a task, syncing specs, or
merging a change:

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

The last command executes `cadw-contract`'s non-toy consumer example, proving
`batch-fold-core/spec.md`'s realistic multi-field `Validator` scenario continues
to hold — `cargo build`/`cargo test` alone only compile it, never run its
assertions.

The `cadw-governance` line is the executable architectural governance gate (tianheng). It
enforces `PROJECT.md`'s Core Contract boundaries — do not treat it as optional
or as a slower duplicate of clippy. `cargo deny check` enforces `deny.toml`'s
license/advisory/bans/sources policy over the resolved dependency graph. CI
(`.github/workflows/ci.yml`) runs all of this on every push and pull request.

If a command cannot run in the current environment, report that explicitly.
