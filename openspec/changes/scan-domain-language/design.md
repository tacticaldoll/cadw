# Design

## Context

`cadw-governance check` runs two kinds of check: Tianheng boundaries from the constitution, and a
runner-side active-prose scan. The scan reads each path in `ACTIVE_PROSE_FILES`, relative to the
workspace root, and fails on any line containing a phrase from `STALE_PHRASES`. A file that
cannot be read is itself a violation, so a missing governed file fails loudly.

## Goals / Non-Goals

**Goals:**

- Put `docs/domain-language.md` under the same scan and missing-file failure as the root files.

**Non-Goals:**

- Scanning every file under `docs/`. Only the file that took over `PROJECT.md`'s vocabulary
  inherits its governance.
- Expressing the check as Tianheng law. It stays a runner check; the constitution is unchanged.
- Scanning `CHANGELOG.md`, which stays exempt.

## Decisions

- **Add one entry to `ACTIVE_PROSE_FILES`.** The scan already joins each entry to the workspace
  root, so a nested relative path works without new code. Alternative considered: a separate
  list for `docs/` files. Rejected, since it would duplicate the scan for one file.
- **Prove the extension with a dedicated reaction test.** The test writes every governed file
  clean except `docs/domain-language.md` and asserts exactly one violation naming that file, so
  the violation can only come from the new entry.

## Risks / Trade-offs

- [The vocabulary file may legitimately need to mention a discarded name] -> It does not today;
  if it ever must, that is a requirement change to reopen, as `CHANGELOG.md`'s exemption was.
