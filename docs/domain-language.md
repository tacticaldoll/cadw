# Domain Language

The canonical vocabulary of Cadw's kernel. `PROJECT.md` states the contract these terms serve.

## Terms

- **Target** (`TargetId`): an opaque, domain-supplied identity for one addressable thing that can
  be `Open` or `Closed`. The kernel never interprets its content.
- **State**: `Open` or `Closed(Outcome)`. `Outcome` is domain-opaque.
- **Move**: a single declared operation — `Create` (bring a target into existence, `Open`, no
  payload), `Close` (with an outcome), or `Reopen` — addressing one target.
- **Validator**: the domain-supplied port judging whether a specific `Close`'s outcome is
  semantically acceptable.
- **Rejection**: why a batch was rejected — either a structural rule (`UnknownTarget`,
  `AlreadyExists`, `AlreadyClosed`, `NotClosed`, `DuplicateTargetInBatch`) or the domain's own
  (`Invalid(TargetId, Validator::Rejection)`).
- **Ledger**: the set of targets and their current states.
