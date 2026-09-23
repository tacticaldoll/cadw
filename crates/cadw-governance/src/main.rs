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

// CHANGELOG.md is deliberately absent: its released version entries legitimately narrate the
// discarded working names ("Motion", "motion-contract") as history of the rename itself, so
// governing it here would fail the gate on the project's own release history.
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

    const LAW_PROJECTION_PREAMBLE: &str = "\
# Cadw Tianheng Law Projection

This file is generated from `constitution()` in `crates/cadw-governance/src/main.rs`.
The Rust declaration is authoritative; do not edit the projection by hand.
Regenerate it with `BLESS=1 cargo test -p cadw-governance law_projection_is_fresh`.

";

    fn workspace_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
    }

    #[test]
    fn current_workspace_satisfies_constitution() {
        GovernanceTest::for_constitution(constitution())
            .with_manifest_dir(workspace_root())
            .assert_clean();
    }

    #[test]
    fn every_workspace_crate_is_covered() {
        GovernanceTest::for_constitution(constitution())
            .with_manifest_dir(workspace_root())
            .assert_all_workspace_members_covered();
    }

    #[test]
    fn law_projection_is_fresh() {
        GovernanceTest::for_constitution(constitution())
            .with_manifest_dir(workspace_root())
            .assert_projection_fresh_with_preamble("AGENTS.cadw-law.md", LAW_PROJECTION_PREAMBLE);
    }

    // Violating witnesses: one per accepted boundary. Each scratch workspace is the clean
    // baseline below plus exactly one planted leak, so the asserted violation can only come from
    // that leak; `scratch_baseline_is_clean` proves the baseline itself passes.

    #[test]
    fn scratch_baseline_is_clean() {
        let workspace = TempWorkspace::baseline("cadw-governance-baseline-clean");

        let outcome = check_constitution(&constitution(), &workspace.manifest());
        assert!(
            matches!(outcome, Outcome::Clean(_)),
            "the leak-free scratch baseline must pass: {outcome:?}"
        );
    }

    #[test]
    fn unapproved_contract_dependency_is_rejected() {
        let workspace = TempWorkspace::baseline("cadw-governance-contract-dependency");
        workspace.write_package("cadw-contract", EXTRA_DEPENDENCY, CLEAN_CONTRACT_SOURCE);

        assert_single_enforced(&workspace, "cadw-contract", DEPENDENCY_RULE, "extra");
    }

    #[test]
    fn unapproved_facade_dependency_is_rejected() {
        let workspace = TempWorkspace::baseline("cadw-governance-facade-dependency");
        workspace.write_package(
            "cadw",
            &format!("{FACADE_DEPENDENCY}{}", EXTRA_DEPENDENCY_LINE),
            FACADE_SOURCE,
        );

        assert_single_enforced(&workspace, "cadw", DEPENDENCY_RULE, "extra");
    }

    #[test]
    fn governance_dependency_on_judged_crate_is_rejected() {
        let workspace = TempWorkspace::baseline("cadw-governance-governance-dependency");
        workspace.write_package(
            "cadw-governance",
            &format!("{GOVERNANCE_DEPENDENCY}cadw-contract = {{ path = \"../cadw-contract\" }}\n"),
            "",
        );

        assert_single_enforced(
            &workspace,
            "cadw-governance",
            DEPENDENCY_RULE,
            "cadw-contract",
        );
    }

    #[test]
    fn contract_std_io_call_is_rejected() {
        assert_contract_leak_fires(
            "cadw-governance-contract-io",
            "pub fn leak() -> std::io::Stdout {\n    std::io::stdout()\n}\n",
            "std::io",
            "std::io::stdout in crate",
        );
    }

    #[test]
    fn contract_std_fs_call_is_rejected() {
        assert_contract_leak_fires(
            "cadw-governance-contract-fs",
            "pub fn leak() -> bool {\n    std::fs::metadata(\"x\").is_ok()\n}\n",
            "std::fs",
            "std::fs::metadata in crate",
        );
    }

    #[test]
    fn contract_std_net_call_is_rejected() {
        assert_contract_leak_fires(
            "cadw-governance-contract-net",
            "pub fn leak() -> bool {\n    std::net::TcpStream::connect(\"127.0.0.1:1\").is_ok()\n}\n",
            "std::net",
            "std::net::TcpStream::connect in crate",
        );
    }

    #[test]
    fn contract_std_process_call_is_rejected() {
        assert_contract_leak_fires(
            "cadw-governance-contract-process",
            "pub fn leak() -> u32 {\n    std::process::id()\n}\n",
            "std::process",
            "std::process::id in crate",
        );
    }

    #[test]
    fn contract_serde_derive_is_rejected() {
        let workspace = TempWorkspace::baseline("cadw-governance-contract-serde");
        workspace.write_package(
            "cadw-contract",
            "",
            "#[derive(serde::Serialize)]\npub struct Leak;\n",
        );

        let violation = assert_single_enforced(
            &workspace,
            "crate",
            "must not acquire trait",
            "derive serde::Serialize on crate::Leak",
        );
        assert_governs_contract(&violation);
    }

    const DEPENDENCY_RULE: &str = "restrict dependencies to";
    const INLINE_RULE: &str = "inline symbol path confined to module";
    const CLEAN_CONTRACT_SOURCE: &str =
        "pub fn fold(values: &[u32]) -> u32 {\n    values.iter().sum()\n}\n";
    const FACADE_SOURCE: &str = "pub use cadw_contract::*;\n";
    const FACADE_DEPENDENCY: &str =
        "[dependencies]\ncadw-contract = { path = \"../cadw-contract\" }\n";
    const GOVERNANCE_DEPENDENCY: &str = "[dependencies]\ntianheng = { path = \"../tianheng\" }\n";
    const EXTRA_DEPENDENCY: &str = "[dependencies]\nextra = { path = \"../extra\" }\n";
    const EXTRA_DEPENDENCY_LINE: &str = "extra = { path = \"../extra\" }\n";

    fn assert_contract_leak_fires(name: &str, source: &str, prefix: &str, finding: &str) {
        let workspace = TempWorkspace::baseline(name);
        workspace.write_package("cadw-contract", "", source);

        let violation = assert_single_enforced(&workspace, prefix, INLINE_RULE, finding);
        assert_governs_contract(&violation);
    }

    /// Assert the planted leak produces exactly one violation, enforced and unbaselined, with the
    /// expected target, rule, and finding; return it for further checks.
    fn assert_single_enforced(
        workspace: &TempWorkspace,
        target: &str,
        rule: &str,
        finding: &str,
    ) -> Violation {
        let outcome = check_constitution(&constitution(), &workspace.manifest());
        let Outcome::Violations(report) = outcome else {
            panic!("expected a `{rule}` violation on `{target}`, got {outcome:?}");
        };
        let [violation] = report.violations.as_slice() else {
            panic!(
                "expected exactly one violation, got {:?}",
                report.violations
            );
        };
        assert_eq!(violation.target(), target);
        assert_eq!(violation.rule, rule);
        assert_eq!(violation.finding, finding);
        assert_eq!(violation.severity, Severity::Enforce);
        assert!(!violation.baselined, "a planted leak must not be baselined");
        violation.clone()
    }

    fn assert_governs_contract(violation: &Violation) {
        assert!(
            violation
                .fact()
                .fields()
                .any(|(name, value)| name == "governing_package" && value == "cadw-contract"),
            "expected the violation to come from a cadw-contract boundary: {violation:?}"
        );
    }

    struct TempWorkspace {
        path: PathBuf,
    }

    impl TempWorkspace {
        /// The leak-free baseline: the three governed crates in their allowed shape, plus stub
        /// `tianheng` and `extra` packages so a planted dependency leak adds only one manifest
        /// line.
        fn baseline(name: &str) -> Self {
            let path = env::temp_dir().join(format!("{name}-{}", std::process::id()));
            if path.exists() {
                fs::remove_dir_all(&path).expect("stale temporary workspace should be removable");
            }
            fs::create_dir_all(&path).expect("temporary workspace should be creatable");
            let workspace = Self { path };
            fs::write(
                workspace.manifest(),
                "[workspace]\nresolver = \"2\"\nmembers = [\"cadw\", \"cadw-contract\", \"cadw-governance\", \"tianheng\", \"extra\"]\n",
            )
            .expect("workspace manifest should be writable");
            workspace.write_package("cadw-contract", "", CLEAN_CONTRACT_SOURCE);
            workspace.write_package("cadw", FACADE_DEPENDENCY, FACADE_SOURCE);
            workspace.write_package("cadw-governance", GOVERNANCE_DEPENDENCY, "");
            workspace.write_package("tianheng", "", "");
            workspace.write_package("extra", "", "");
            workspace
        }

        fn manifest(&self) -> PathBuf {
            self.path.join("Cargo.toml")
        }

        fn write_package(&self, name: &str, dependencies: &str, source: &str) {
            let package = self.path.join(name);
            fs::create_dir_all(package.join("src")).expect("package source dir should be writable");
            fs::write(
                package.join("Cargo.toml"),
                format!(
                    "[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n{dependencies}"
                ),
            )
            .expect("package manifest should be writable");
            fs::write(package.join("src/lib.rs"), source)
                .expect("package source should be writable");
        }
    }

    impl Drop for TempWorkspace {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

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

    #[test]
    fn current_active_prose_satisfies_governance() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");

        assert_eq!(check_active_prose(&root), Ok(()));
    }

    #[test]
    fn missing_active_prose_file_fails_loudly() {
        // A root with none of the canonical governed prose files must fail the gate, not pass
        // vacuously by skipping every unreadable file.
        let root = env::temp_dir().join(format!(
            "cadw-governance-missing-prose-{}",
            std::process::id()
        ));
        if root.exists() {
            fs::remove_dir_all(&root).expect("stale temporary directory should be removable");
        }
        fs::create_dir_all(&root).expect("temporary directory should be creatable");

        let Err(violations) = check_active_prose(&root) else {
            panic!("a root missing every governed prose file must fail the gate");
        };
        assert!(
            violations
                .iter()
                .any(|violation| violation.phrase == "<unreadable>"),
            "expected an unreadable-file violation naming a governed file: {violations:?}"
        );

        fs::remove_dir_all(&root).expect("temporary directory should be removable");
    }
}
