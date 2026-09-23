//! A non-toy consumer example: models a real consumer's `Resolution` shape (`reason: String`,
//! `provenance: Vec<EventRef>` — substituted here with `Vec<String>`, since the id
//! *representation* is not what this example tests) as a [`Validator`] implementation, proving
//! the port composes with a realistic, multi-field domain outcome and a closed rejection enum —
//! not just the vacuum test suite's trivial `String` outcome and single-check validator.
//!
//! Does not depend on that consumer. Run with:
//! `cargo run --example dissent_resolution -p cadw-contract`.

use cadw_contract::{Ledger, Move, TargetId, Validator};

/// Mirrors the consumer's `Resolution` shape: both fields are required non-empty before a
/// dissent may close.
#[derive(Debug, Clone, PartialEq)]
struct Resolution {
    reason: String,
    provenance: Vec<String>,
}

/// The domain's own closed, structured rejection type — never a free string.
#[derive(Debug)]
enum DissentRejection {
    EmptyReason,
    EmptyProvenance,
}

impl std::fmt::Display for DissentRejection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DissentRejection::EmptyReason => write!(f, "a resolution requires a non-empty reason"),
            DissentRejection::EmptyProvenance => {
                write!(f, "a resolution requires at least one provenance reference")
            }
        }
    }
}

impl std::error::Error for DissentRejection {}

struct DissentValidator;

impl Validator<Resolution> for DissentValidator {
    type Rejection = DissentRejection;

    fn validate(&self, _target: &TargetId, outcome: &Resolution) -> Result<(), DissentRejection> {
        if outcome.reason.is_empty() {
            return Err(DissentRejection::EmptyReason);
        }
        if outcome.provenance.is_empty() {
            return Err(DissentRejection::EmptyProvenance);
        }
        Ok(())
    }
}

fn main() {
    let too_slow = TargetId::new("dissent:too-slow");
    let missing_rollback = TargetId::new("dissent:missing-rollback");
    let unclear_owner = TargetId::new("dissent:unclear-owner");

    let ledger = Ledger::new([
        too_slow.clone(),
        missing_rollback.clone(),
        unclear_owner.clone(),
    ]);

    // One individually-valid resolution, one with a missing reason.
    let batch = [
        Move::Close {
            target: too_slow.clone(),
            outcome: Resolution {
                reason: "load test confirms the latency budget is met".into(),
                provenance: vec!["event:42".into()],
            },
        },
        Move::Close {
            target: missing_rollback.clone(),
            outcome: Resolution {
                reason: String::new(),
                provenance: vec![],
            },
        },
    ];

    let result = ledger.fold_batch(&batch, &DissentValidator);
    let err = result.expect_err("a batch containing an empty-reason resolution must be rejected");
    println!("batch rejected as expected: {err}");

    // Neither move applied — not even "too-slow", whose own resolution was individually valid.
    assert_eq!(
        ledger.state_of(&too_slow),
        Some(&cadw_contract::State::Open)
    );
    assert_eq!(
        ledger.state_of(&missing_rollback),
        Some(&cadw_contract::State::Open)
    );
    // "unclear-owner" was never mentioned by the batch at all — conservative retention holding
    // under a realistic multi-field Outcome, not just a plain String.
    assert_eq!(
        ledger.state_of(&unclear_owner),
        Some(&cadw_contract::State::Open)
    );

    // Retry with a valid resolution for the previously-rejected dissent — this time the batch
    // applies, and the untouched dissent is still exactly where it was.
    let retry = [Move::Close {
        target: missing_rollback.clone(),
        outcome: Resolution {
            reason: "rollback script added and tested in staging".into(),
            provenance: vec!["event:57".into()],
        },
    }];
    let next = ledger
        .fold_batch(&retry, &DissentValidator)
        .expect("a resolution with both fields populated must be accepted");

    assert!(matches!(
        next.state_of(&missing_rollback),
        Some(cadw_contract::State::Closed(_))
    ));
    assert_eq!(
        next.state_of(&unclear_owner),
        Some(&cadw_contract::State::Open)
    );

    println!("realistic dissent-resolution batch composed correctly with the Validator port");
}
