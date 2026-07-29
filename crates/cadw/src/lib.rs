//! Cadw: a thin, sans-I/O batch-fold core you compose.
//!
//! This crate is the curated public entrypoint. It re-exports the compose-level API of the
//! Cadw workspace so a consumer can depend on one crate:
//!
//! - the addressable identity — [`TargetId`];
//! - a target's lifecycle — [`State`];
//! - a single declared operation — [`Move`] (`Create`/`Close`/`Reopen`);
//! - the domain-supplied judgment port — [`Validator`];
//! - why a batch was rejected — [`Rejection`];
//! - the ledger itself — [`Ledger`] and its [`Ledger::fold_batch`].
//!
//! It carries no logic of its own: every item here is a re-export. Cadw's whole public surface
//! is compose-level, so the facade withholds nothing — there is no advanced kernel to reach
//! through [`cadw_contract`] directly.
//!
//! # The contract
//!
//! Cadw owns a *mechanism* and no *meaning*. [`Ledger::fold_batch`] applies a batch of `Create`/
//! `Close`/`Reopen` moves atomically — every move in the batch applies, or none do — and a
//! target no move mentions is unreachable by that fold. It decides no semantic identity and no
//! domain validity: that judgment is [`Validator`]'s, a port the domain implements with its own
//! closed, structured [`Validator::Rejection`]. See the crate-level docs of [`cadw_contract`]
//! for the full Core Contract and Non-Goals.
//!
//! # Composing a ledger
//!
//! A batch mixing `Create` and `Close` in one call, wired entirely through this entrypoint: a
//! new target is created and, in the same batch, an existing one is closed — both apply, or
//! neither would (run `cargo test --doc -p cadw` to see it execute):
//!
//! ```
//! use cadw::{Ledger, Move, State, TargetId, Validator};
//!
//! # #[derive(Debug)]
//! # struct NeverRejects;
//! # impl std::fmt::Display for NeverRejects {
//! #     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//! #         write!(f, "never rejects")
//! #     }
//! # }
//! # impl std::error::Error for NeverRejects {}
//! struct AlwaysValid;
//! impl Validator<String> for AlwaysValid {
//!     type Rejection = NeverRejects;
//!     fn validate(&self, _target: &TargetId, _outcome: &String) -> Result<(), NeverRejects> {
//!         Ok(())
//!     }
//! }
//!
//! let existing = TargetId::new("risk:vendor-sla-unclear");
//! let ledger = Ledger::new([existing.clone()]);
//!
//! let follow_up = TargetId::new("risk:rollback-plan-unclear");
//! let next = ledger
//!     .fold_batch(
//!         &[
//!             Move::Create {
//!                 target: follow_up.clone(),
//!             },
//!             Move::Close {
//!                 target: existing.clone(),
//!                 outcome: "vendor confirmed SLA in writing".to_string(),
//!             },
//!         ],
//!         &AlwaysValid,
//!     )
//!     .expect("a Create and a Close on different targets apply atomically");
//!
//! assert_eq!(next.state_of(&follow_up), Some(&State::Open));
//! assert!(matches!(next.state_of(&existing), Some(State::Closed(_))));
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

// A glob re-export makes "the facade withholds nothing" structurally true: the facade's
// surface *is* `cadw-contract`'s public surface, enforced by the compiler in both directions.
// A new public item in the core appears here automatically; none can be silently dropped or
// left behind. Matches `suunta`'s and `shaahid`'s own facades exactly.
pub use cadw_contract::*;
