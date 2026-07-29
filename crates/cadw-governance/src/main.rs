//! Executable architectural governance for the Cadw workspace.

#![forbid(unsafe_code)]

use std::{
    env, fs,
    path::{Path, PathBuf},
    process::ExitCode,
};

use tianheng::prelude::*;

const CONTRACT_REASON: &str = "cadw-contract is the isolated core contract: a sans-I/O kernel with no time/lease/crash-recovery dimension. It needs no dependency at all, so it may depend on nothing.";
const FACADE_REASON: &str = "cadw is the curated public entrypoint: a pure re-export facade with no logic of its own. It must depend on cadw-contract only, never acquiring a dependency the core itself does not have.";
const GOVERNANCE_REASON: &str = "the governance gate must stay independent of the workspace graph it judges: it may depend only on tianheng, never on cadw-contract or any other workspace crate under judgment.";
const CORE_NO_IO_REASON: &str = "the sans-I/O core contract performs no I/O: no code in cadw-contract may call into std::io/fs/net/process; a batch fold is a synchronous, in-memory operation, never a place I/O could hide.";
const NO_SERDE_REASON: &str = "cadw-contract is transient in-memory mechanism, not a durable record type: it must not acquire Serialize/Deserialize anywhere. Serialization of a domain's Outcome is that domain's own concern, never this crate's.";

const ACTIVE_PROSE_FILES: &[&str] = &["AGENTS.md", "PROJECT.md", "README.md", "BACKLOG.md"];

const STALE_PHRASES: &[StalePhrase] = &[
    StalePhrase {
        phrase: "motion-contract",
        reason: "the discarded working-name crate identity this repository grew out of before the brand settled on Cadw.",
    },
    StalePhrase {
        phrase: "Motion",
        reason: "the discarded working-name concept identity this repository grew out of before the brand settled on Cadw.",
    },
];

#[derive(Debug, Clone, Copy)]
struct StalePhrase {
    phrase: &'static str,
    reason: &'static str,
}

#[derive(Debug, PartialEq, Eq)]
struct ProseViolation {
    path: String,
    line: usize,
    phrase: &'static str,
    reason: &'static str,
}

fn constitution() -> Constitution {
    Constitution::new("cadw")
        .boundary(
            CrateBoundary::crate_("cadw-contract")
                .restrict_dependencies_to(Vec::<&str>::new())
                .because(CONTRACT_REASON),
        )
        .boundary(
            CrateBoundary::crate_("cadw")
                .restrict_dependencies_to(["cadw-contract"])
                .because(FACADE_REASON),
        )
        .boundary(
            CrateBoundary::crate_("cadw-governance")
                .restrict_dependencies_to(["tianheng"])
                .because(GOVERNANCE_REASON),
        )
        .boundary(
            ModuleBoundary::in_crate("cadw-contract")
                .module("crate")
                .must_not_call_inline("std::io")
                .because(CORE_NO_IO_REASON),
        )
        .boundary(
            ModuleBoundary::in_crate("cadw-contract")
                .module("crate")
                .must_not_call_inline("std::fs")
                .because(CORE_NO_IO_REASON),
        )
        .boundary(
            ModuleBoundary::in_crate("cadw-contract")
                .module("crate")
                .must_not_call_inline("std::net")
                .because(CORE_NO_IO_REASON),
        )
        .boundary(
            ModuleBoundary::in_crate("cadw-contract")
                .module("crate")
                .must_not_call_inline("std::process")
                .because(CORE_NO_IO_REASON),
        )
        .forbidden_marker_boundary(
            ForbiddenMarkerBoundary::in_crate("cadw-contract")
                .module("crate")
                .must_not_acquire("serde::Serialize")
                .and_not_acquire("serde::Deserialize")
                .because(NO_SERDE_REASON),
        )
}

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<_>>();

    if should_check_prose(&args) {
        let manifest = manifest_path_from_args(&args);
        let root = manifest
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from("."));

        if let Err(violations) = check_active_prose(&root) {
            eprintln!("cadw prose governance failed: stale working-name vocabulary reintroduced");
            for violation in violations {
                eprintln!(
                    "{}:{}: `{}` - {}",
                    violation.path, violation.line, violation.phrase, violation.reason
                );
            }
            return ExitCode::from(1);
        }
    }

    tianheng::run(&constitution(), args)
}

fn should_check_prose(args: &[String]) -> bool {
    args.iter().skip(1).any(|arg| arg == "check")
}

fn manifest_path_from_args(args: &[String]) -> PathBuf {
    for index in 0..args.len() {
        if args[index] == "--manifest-path"
            && let Some(path) = args.get(index + 1)
        {
            return PathBuf::from(path);
        }

        if let Some(path) = args[index].strip_prefix("--manifest-path=") {
            return PathBuf::from(path);
        }
    }

    PathBuf::from("Cargo.toml")
}

fn check_active_prose(root: &Path) -> Result<(), Vec<ProseViolation>> {
    let mut violations = Vec::new();

    for relative in ACTIVE_PROSE_FILES {
        let path = root.join(relative);
        let Ok(content) = fs::read_to_string(&path) else {
            violations.push(ProseViolation {
                path: String::from(*relative),
                line: 0,
                phrase: "<unreadable>",
                reason: "a governed active-prose file must be present and readable",
            });
            continue;
        };

        violations.extend(check_prose_content(relative, &content));
    }

    if violations.is_empty() {
        Ok(())
    } else {
        Err(violations)
    }
}

fn check_prose_content(path: &str, content: &str) -> Vec<ProseViolation> {
    let mut violations = Vec::new();

    for (index, line) in content.lines().enumerate() {
        for rule in STALE_PHRASES {
            if line.contains(rule.phrase) {
                violations.push(ProseViolation {
                    path: path.to_owned(),
                    line: index + 1,
                    phrase: rule.phrase,
                    reason: rule.reason,
                });
            }
        }
    }

    violations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_prose_passes() {
        let content = "# Cadw\n\nA sans-I/O kernel for folding batches of moves.\n";
        assert!(check_prose_content("README.md", content).is_empty());
    }

    #[test]
    fn a_stale_working_name_is_caught() {
        let content = "# Cadw\n\nformerly known as Motion, now settled.\n";
        let violations = check_prose_content("README.md", content);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].phrase, "Motion");
        assert_eq!(violations[0].line, 3);
    }

    #[test]
    fn a_stale_crate_name_is_caught() {
        let content = "See the motion-contract prototype for background.\n";
        let violations = check_prose_content("PROJECT.md", content);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].phrase, "motion-contract");
    }
}
