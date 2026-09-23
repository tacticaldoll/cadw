//! A sans-I/O kernel for folding a batch of declared `Create`/`Close`/`Reopen` [`Move`]s over
//! addressable [`Target`](TargetId)s atomically — every move in a batch applies, or none do —
//! with structural conservative retention: a target no move mentions is unreachable by that fold,
//! not merely left unchanged by a checked rule.
//!
//! The kernel owns the fold/atomicity/conflict/state-transition mechanism. It never judges
//! whether a specific `Outcome` is semantically valid — that is the domain-supplied [`Validator`]
//! port, whose [`Validator::Rejection`] is a fully structured `std::error::Error`, following
//! an associated `type Error` pattern.
//!
//! Most consumers should depend on the [`cadw`](https://crates.io/crates/cadw) facade instead;
//! depend on `cadw-contract` directly only to implement a `Validator` without the facade's
//! curated re-export. See the workspace root `PROJECT.md` for the full Core Contract, Non-Goals,
//! and current graduation status.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::collections::{HashMap, HashSet};
use std::fmt;

/// An opaque, domain-supplied identity for one addressable target. The kernel never interprets
/// its content — it is a key, not a claim about meaning.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TargetId(String);

impl TargetId {
    /// Construct a target identity from any string-like value.
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl fmt::Display for TargetId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A target's lifecycle state. `Outcome` is domain-opaque — the kernel stores it but never
/// inspects it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State<Outcome> {
    /// Not yet closed — a live, unresolved target.
    Open,
    /// Closed with a domain-supplied outcome. Reopenable.
    Closed(Outcome),
}

/// A single declared operation on one target.
///
/// `#[non_exhaustive]`: the variant set may grow in a later version without breaking a
/// downstream `_ =>` matcher. It stays deliberately small today — extension happens by adding a
/// variant later, not by widening these two with ever more optional fields into a growing DSL.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum Move<Outcome> {
    /// Bring a new target into existence, `Open`, with no prior state. Carries no payload: the
    /// kernel never stores anything for an `Open` target (only `Closed` carries an `Outcome`),
    /// so it has nothing to accept here either — a target's descriptive content, if any, is the
    /// domain's own concern, correlated by the same `TargetId`.
    Create {
        /// The target this move addresses.
        target: TargetId,
    },
    /// Close an `Open` target with a domain-supplied outcome.
    Close {
        /// The target this move addresses.
        target: TargetId,
        /// The domain-opaque outcome to close it with.
        outcome: Outcome,
    },
    /// Reopen a `Closed` target. Carries no payload: "why this was reopened" is an audit
    /// concern for the domain to record separately, not a state-transition concern.
    Reopen {
        /// The target this move addresses.
        target: TargetId,
    },
}

impl<Outcome> Move<Outcome> {
    /// The target this move addresses, regardless of variant.
    pub fn target(&self) -> &TargetId {
        match self {
            Move::Create { target } => target,
            Move::Close { target, .. } => target,
            Move::Reopen { target } => target,
        }
    }
}

/// The domain-supplied port that judges whether a specific `Close`'s outcome is semantically
/// valid. The kernel calls this and nothing else touches domain semantics.
pub trait Validator<Outcome> {
    /// The domain's own closed, structured rejection type — never a free string.
    type Rejection: std::error::Error;

    /// Judge whether `outcome` is an acceptable way to close `target`.
    fn validate(&self, target: &TargetId, outcome: &Outcome) -> Result<(), Self::Rejection>;
}

/// Why a batch was rejected. `#[non_exhaustive]` for the same open/closed-principle reason as
/// [`Move`].
#[non_exhaustive]
#[derive(Debug)]
pub enum Rejection<VR: std::error::Error> {
    /// A move addressed a target the ledger does not contain.
    UnknownTarget(TargetId),
    /// A `Create` addressed a target that already exists in the ledger, `Open` or `Closed`.
    AlreadyExists(TargetId),
    /// A `Close` addressed a target that was already `Closed`.
    AlreadyClosed(TargetId),
    /// A `Reopen` addressed a target that was not `Closed`.
    NotClosed(TargetId),
    /// More than one move in the batch addressed the same target.
    DuplicateTargetInBatch(TargetId),
    /// The domain's own [`Validator`] rejected a structurally legal `Close`.
    Invalid(TargetId, VR),
}

impl<VR: std::error::Error> fmt::Display for Rejection<VR> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rejection::UnknownTarget(t) => write!(f, "unknown target: {t}"),
            Rejection::AlreadyExists(t) => write!(f, "target already exists: {t}"),
            Rejection::AlreadyClosed(t) => write!(f, "target already closed: {t}"),
            Rejection::NotClosed(t) => write!(f, "target not closed, cannot reopen: {t}"),
            Rejection::DuplicateTargetInBatch(t) => {
                write!(f, "target addressed more than once in one batch: {t}")
            }
            Rejection::Invalid(t, e) => write!(f, "move for target {t} rejected: {e}"),
        }
    }
}

impl<VR: std::error::Error + 'static> std::error::Error for Rejection<VR> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Rejection::Invalid(_, e) => Some(e),
            _ => None,
        }
    }
}

/// The set of targets and their current states. Construction (which targets exist) is the
/// domain's concern — this type only folds batches of moves over an already-populated set.
#[derive(Debug, Clone)]
pub struct Ledger<Outcome> {
    targets: HashMap<TargetId, State<Outcome>>,
}

impl<Outcome> Ledger<Outcome> {
    /// Construct a ledger with every given target `Open`.
    pub fn new(targets: impl IntoIterator<Item = TargetId>) -> Self {
        Self {
            targets: targets.into_iter().map(|t| (t, State::Open)).collect(),
        }
    }

    /// The current state of one target, if the ledger contains it.
    pub fn state_of(&self, target: &TargetId) -> Option<&State<Outcome>> {
        self.targets.get(target)
    }

    /// Fold a batch of moves atomically: every move applies, or none do.
    ///
    /// Order of checks: (1) no two moves in the batch address the same target; (2) a `Create`'s
    /// target must not already exist, and a `Close`/`Reopen`'s target must exist and its current
    /// state must permit the requested transition; (3) every `Close` passes the domain's
    /// [`Validator`]. Only if all three pass for the whole batch is a new `Ledger` produced with
    /// every move applied.
    pub fn fold_batch<V>(
        &self,
        moves: &[Move<Outcome>],
        validator: &V,
    ) -> Result<Ledger<Outcome>, Rejection<V::Rejection>>
    where
        V: Validator<Outcome>,
        Outcome: Clone,
    {
        let mut seen: HashSet<&TargetId> = HashSet::new();
        for mv in moves {
            if !seen.insert(mv.target()) {
                return Err(Rejection::DuplicateTargetInBatch(mv.target().clone()));
            }
        }

        for mv in moves {
            let target = mv.target();
            match mv {
                Move::Create { .. } => {
                    if self.targets.contains_key(target) {
                        return Err(Rejection::AlreadyExists(target.clone()));
                    }
                }
                Move::Close { outcome, .. } => {
                    match self
                        .targets
                        .get(target)
                        .ok_or_else(|| Rejection::UnknownTarget(target.clone()))?
                    {
                        State::Open => {
                            validator
                                .validate(target, outcome)
                                .map_err(|e| Rejection::Invalid(target.clone(), e))?;
                        }
                        State::Closed(_) => {
                            return Err(Rejection::AlreadyClosed(target.clone()));
                        }
                    }
                }
                Move::Reopen { .. } => {
                    match self
                        .targets
                        .get(target)
                        .ok_or_else(|| Rejection::UnknownTarget(target.clone()))?
                    {
                        State::Closed(_) => {}
                        State::Open => {
                            return Err(Rejection::NotClosed(target.clone()));
                        }
                    }
                }
            }
        }

        let mut new_targets = self.targets.clone();
        for mv in moves {
            match mv {
                Move::Create { target } => {
                    new_targets.insert(target.clone(), State::Open);
                }
                Move::Close { target, outcome } => {
                    new_targets.insert(target.clone(), State::Closed(outcome.clone()));
                }
                Move::Reopen { target } => {
                    new_targets.insert(target.clone(), State::Open);
                }
            }
        }

        Ok(Ledger {
            targets: new_targets,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn target(id: &str) -> TargetId {
        TargetId::new(id)
    }

    struct AlwaysValid;

    #[derive(Debug)]
    struct NeverRejects;

    impl fmt::Display for NeverRejects {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "never rejects")
        }
    }

    impl std::error::Error for NeverRejects {}

    impl Validator<String> for AlwaysValid {
        type Rejection = NeverRejects;

        fn validate(&self, _target: &TargetId, _outcome: &String) -> Result<(), NeverRejects> {
            Ok(())
        }
    }

    struct RejectEmpty;

    #[derive(Debug)]
    struct EmptyOutcome;

    impl fmt::Display for EmptyOutcome {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "outcome must not be empty")
        }
    }

    impl std::error::Error for EmptyOutcome {}

    impl Validator<String> for RejectEmpty {
        type Rejection = EmptyOutcome;

        fn validate(&self, _target: &TargetId, outcome: &String) -> Result<(), EmptyOutcome> {
            if outcome.is_empty() {
                Err(EmptyOutcome)
            } else {
                Ok(())
            }
        }
    }

    #[test]
    fn a_fresh_target_can_be_closed() {
        let ledger = Ledger::new([target("a")]);
        let moves = [Move::Close {
            target: target("a"),
            outcome: "done".to_string(),
        }];
        let next = ledger.fold_batch(&moves, &AlwaysValid).unwrap();
        assert_eq!(
            next.state_of(&target("a")),
            Some(&State::Closed("done".to_string()))
        );
    }

    #[test]
    fn an_already_closed_target_cannot_be_closed_again() {
        let ledger = Ledger::new([target("a")]);
        let closed = ledger
            .fold_batch(
                &[Move::Close {
                    target: target("a"),
                    outcome: "x".into(),
                }],
                &AlwaysValid,
            )
            .unwrap();
        let err = closed
            .fold_batch(
                &[Move::Close {
                    target: target("a"),
                    outcome: "y".into(),
                }],
                &AlwaysValid,
            )
            .unwrap_err();
        assert!(matches!(err, Rejection::AlreadyClosed(t) if t == target("a")));
    }

    #[test]
    fn an_open_target_cannot_be_reopened() {
        let ledger = Ledger::new([target("a")]);
        let err = ledger
            .fold_batch(
                &[Move::Reopen {
                    target: target("a"),
                }],
                &AlwaysValid,
            )
            .unwrap_err();
        assert!(matches!(err, Rejection::NotClosed(t) if t == target("a")));
    }

    #[test]
    fn a_closed_target_can_be_reopened_then_reclosed() {
        let ledger = Ledger::new([target("a")]);
        let closed = ledger
            .fold_batch(
                &[Move::Close {
                    target: target("a"),
                    outcome: "x".into(),
                }],
                &AlwaysValid,
            )
            .unwrap();
        let reopened = closed
            .fold_batch(
                &[Move::Reopen {
                    target: target("a"),
                }],
                &AlwaysValid,
            )
            .unwrap();
        assert_eq!(reopened.state_of(&target("a")), Some(&State::Open));
        let reclosed = reopened
            .fold_batch(
                &[Move::Close {
                    target: target("a"),
                    outcome: "y".into(),
                }],
                &AlwaysValid,
            )
            .unwrap();
        assert_eq!(
            reclosed.state_of(&target("a")),
            Some(&State::Closed("y".to_string()))
        );
    }

    #[test]
    fn two_moves_in_one_batch_targeting_the_same_target_are_rejected() {
        let ledger = Ledger::new([target("a")]);
        let moves = [
            Move::Close {
                target: target("a"),
                outcome: "x".into(),
            },
            Move::Reopen {
                target: target("a"),
            },
        ];
        let err = ledger.fold_batch(&moves, &AlwaysValid).unwrap_err();
        assert!(matches!(err, Rejection::DuplicateTargetInBatch(t) if t == target("a")));
    }

    #[test]
    fn a_batch_with_one_invalid_move_applies_none_of_them() {
        let ledger = Ledger::new([target("a"), target("b")]);
        let moves = [
            Move::Close {
                target: target("a"),
                outcome: "valid".into(),
            },
            Move::Close {
                target: target("b"),
                outcome: "".into(),
            },
        ];
        let err = ledger.fold_batch(&moves, &RejectEmpty).unwrap_err();
        assert!(matches!(err, Rejection::Invalid(t, _) if t == target("b")));
        assert_eq!(ledger.state_of(&target("a")), Some(&State::Open));
        assert_eq!(ledger.state_of(&target("b")), Some(&State::Open));
    }

    #[test]
    fn a_target_not_mentioned_in_the_batch_is_unchanged() {
        let ledger = Ledger::new([target("a"), target("b")]);
        let moves = [Move::Close {
            target: target("a"),
            outcome: "x".into(),
        }];
        let next = ledger.fold_batch(&moves, &AlwaysValid).unwrap();
        assert_eq!(next.state_of(&target("b")), Some(&State::Open));
    }

    #[test]
    fn unknown_target_is_rejected() {
        let ledger = Ledger::new([target("a")]);
        let err = ledger
            .fold_batch(
                &[Move::Close {
                    target: target("z"),
                    outcome: "x".into(),
                }],
                &AlwaysValid,
            )
            .unwrap_err();
        assert!(matches!(err, Rejection::UnknownTarget(t) if t == target("z")));
    }

    #[test]
    fn the_validator_can_reject_a_structurally_valid_move() {
        let ledger = Ledger::new([target("a")]);
        let err = ledger
            .fold_batch(
                &[Move::Close {
                    target: target("a"),
                    outcome: "".into(),
                }],
                &RejectEmpty,
            )
            .unwrap_err();
        assert!(matches!(err, Rejection::Invalid(t, _) if t == target("a")));
    }

    #[test]
    fn an_empty_batch_changes_nothing() {
        let ledger = Ledger::new([target("a"), target("b")]);
        let next = ledger.fold_batch(&[], &AlwaysValid).unwrap();
        assert_eq!(next.state_of(&target("a")), Some(&State::Open));
        assert_eq!(next.state_of(&target("b")), Some(&State::Open));
    }

    #[test]
    fn creating_a_fresh_target_succeeds() {
        let ledger: Ledger<String> = Ledger::new([]);
        let next = ledger
            .fold_batch(
                &[Move::Create {
                    target: target("a"),
                }],
                &AlwaysValid,
            )
            .unwrap();
        assert_eq!(next.state_of(&target("a")), Some(&State::Open));
    }

    #[test]
    fn creating_an_already_open_target_is_rejected() {
        let ledger: Ledger<String> = Ledger::new([target("a")]);
        let err = ledger
            .fold_batch(
                &[Move::Create {
                    target: target("a"),
                }],
                &AlwaysValid,
            )
            .unwrap_err();
        assert!(matches!(err, Rejection::AlreadyExists(t) if t == target("a")));
    }

    #[test]
    fn creating_an_already_closed_target_is_rejected() {
        let ledger = Ledger::new([target("a")]);
        let closed = ledger
            .fold_batch(
                &[Move::Close {
                    target: target("a"),
                    outcome: "x".into(),
                }],
                &AlwaysValid,
            )
            .unwrap();
        let err = closed
            .fold_batch(
                &[Move::Create {
                    target: target("a"),
                }],
                &AlwaysValid,
            )
            .unwrap_err();
        assert!(matches!(err, Rejection::AlreadyExists(t) if t == target("a")));
    }

    #[test]
    fn a_created_target_can_be_closed_in_a_later_batch() {
        let ledger: Ledger<String> = Ledger::new([]);
        let created = ledger
            .fold_batch(
                &[Move::Create {
                    target: target("a"),
                }],
                &AlwaysValid,
            )
            .unwrap();
        let closed = created
            .fold_batch(
                &[Move::Close {
                    target: target("a"),
                    outcome: "done".into(),
                }],
                &AlwaysValid,
            )
            .unwrap();
        assert_eq!(
            closed.state_of(&target("a")),
            Some(&State::Closed("done".to_string()))
        );
    }

    #[test]
    fn a_batch_mixing_create_and_close_on_different_targets_applies_atomically() {
        let ledger = Ledger::new([target("existing")]);
        let next = ledger
            .fold_batch(
                &[
                    Move::Create {
                        target: target("new"),
                    },
                    Move::Close {
                        target: target("existing"),
                        outcome: "done".into(),
                    },
                ],
                &AlwaysValid,
            )
            .unwrap();
        assert_eq!(next.state_of(&target("new")), Some(&State::Open));
        assert_eq!(
            next.state_of(&target("existing")),
            Some(&State::Closed("done".to_string()))
        );
    }

    #[test]
    fn creating_and_closing_the_same_target_in_one_batch_is_rejected() {
        let ledger: Ledger<String> = Ledger::new([]);
        let err = ledger
            .fold_batch(
                &[
                    Move::Create {
                        target: target("a"),
                    },
                    Move::Close {
                        target: target("a"),
                        outcome: "done".into(),
                    },
                ],
                &AlwaysValid,
            )
            .unwrap_err();
        assert!(matches!(err, Rejection::DuplicateTargetInBatch(t) if t == target("a")));
    }

    #[test]
    fn a_reopen_only_batch_touches_nothing_else() {
        let ledger = Ledger::new([target("a"), target("b")]);
        let closed = ledger
            .fold_batch(
                &[
                    Move::Close {
                        target: target("a"),
                        outcome: "x".into(),
                    },
                    Move::Close {
                        target: target("b"),
                        outcome: "y".into(),
                    },
                ],
                &AlwaysValid,
            )
            .unwrap();

        let reopened = closed
            .fold_batch(
                &[Move::Reopen {
                    target: target("a"),
                }],
                &AlwaysValid,
            )
            .unwrap();

        assert_eq!(reopened.state_of(&target("a")), Some(&State::Open));
        assert_eq!(
            reopened.state_of(&target("b")),
            Some(&State::Closed("y".to_string()))
        );
    }

    #[test]
    fn a_large_mixed_batch_applies_every_move_exactly_once() {
        let targets: Vec<TargetId> = (0..50).map(|i| target(&format!("t{i}"))).collect();
        let ledger = Ledger::new(targets.iter().cloned());

        // Close the first half.
        let close_moves: Vec<Move<String>> = targets[..25]
            .iter()
            .map(|t| Move::Close {
                target: t.clone(),
                outcome: format!("closed-{t}"),
            })
            .collect();
        let after_close = ledger.fold_batch(&close_moves, &AlwaysValid).unwrap();

        for t in &targets[..25] {
            assert_eq!(
                after_close.state_of(t),
                Some(&State::Closed(format!("closed-{t}")))
            );
        }
        for t in &targets[25..] {
            assert_eq!(after_close.state_of(t), Some(&State::Open));
        }

        // Reopen the first quarter, close the second quarter for the first time, in one batch —
        // exercises both variants together at scale, none targeting the same id twice.
        let mixed_moves: Vec<Move<String>> = targets[..12]
            .iter()
            .map(|t| Move::Reopen { target: t.clone() })
            .chain(targets[25..].iter().map(|t| Move::Close {
                target: t.clone(),
                outcome: format!("closed-{t}"),
            }))
            .collect();
        let after_mixed = after_close.fold_batch(&mixed_moves, &AlwaysValid).unwrap();

        for t in &targets[..12] {
            assert_eq!(after_mixed.state_of(t), Some(&State::Open));
        }
        for t in &targets[12..25] {
            assert_eq!(
                after_mixed.state_of(t),
                Some(&State::Closed(format!("closed-{t}")))
            );
        }
        for t in &targets[25..] {
            assert_eq!(
                after_mixed.state_of(t),
                Some(&State::Closed(format!("closed-{t}")))
            );
        }
    }
}
