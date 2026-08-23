---
id: agov-version-drift-warning
status: pending-gate1
topic: agov-version-drift-warning
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >-
      Requesting Codex's independent review of this plan and its
      implementation before Gate 2. Claude (planner-architect) authors
      both the plan and the implementation here, so the contribution-
      auditor review must come from a session that authored neither —
      per `6srq`, this cannot be self-satisfied.
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      Reads two already-repo-local plaintext version strings
      (`env!("CARGO_PKG_VERSION")`, compiled into the binary; `agf/
      VERSION`, already tracked in version control) and prints one
      advisory line. No credential, network call, schema change, or
      security-surface change.
---

# Plan: advisory warning when the running `agov` binary's version drifts
# from the commissioned `agf/VERSION`

Implements issue #114 (filed 2026-08-23). Human governor requested
immediate implementation, targeted at the next release.

## Problem (verified by code search before writing this plan)

`agov` never compares its own build version
(`env!("CARGO_PKG_VERSION")`) against the `agf/VERSION` of the tree it
runs against. Confirmed: the only existing version-comparison logic is
(a) `framework_config.rs`'s `contract_version` (in `framework.yaml`) vs
the same tree's `VERSION` — a same-tree, two-declaration check — and (b)
`provenance.rs`'s distribution-provenance-record vs packaged-binary
self-report check — a release-integrity check for building/publishing a
release archive, not a check that fires when an already-installed binary
is run against an arbitrary commissioned tree. Neither compares "the
binary currently executing" against "the tree it is pointed at."

Two independent risk directions, per issue #114:

1. **agov older than the commissioned tree** — the binary cannot know
   about schema fields, subcommands, or rules added after its own build;
   worst case it reports `OK` while silently not checking something the
   current schema requires.
2. **agov newer than the commissioned tree** — lower risk (over-strict,
   not silently permissive) but can still produce a confusing false
   failure against an unmigrated tree.

## Decision requested

Add one advisory (non-blocking) version-drift check, run at the two
places that already read repo state on every invocation: `agov validate`
(the umbrella command, also the one CI's `agov:governance:validate` job
runs) and `agov session resume` (the literal first command
`docs/reference/agent-cold-start-habits.md` instructs every agent to run
at cold start, before any other repo-state check). Advisory, never
blocking — no existing rule grants this check blocking authority
(`tool-authority-boundary-rule.md`), and a hard block would break
legitimate cases (staged rollouts, deliberate compatibility testing).

## Design

New function in `agov-core` (shared by both call sites):

```rust
/// Resolve the commissioned agf/ tree's own VERSION file, if present.
/// Returns None if agf/VERSION does not exist (e.g. this is agov's own
/// upstream repo, or a non-AGF-adopted tree) — absence is not a defect.
pub fn commissioned_version() -> Option<String>
```

New function in the `agov` binary crate (needs `env!("CARGO_PKG_VERSION")`,
which is only available at the crate being compiled, hence not in
`agov-core`):

```rust
/// Compare the running binary's own build version against the
/// commissioned agf/VERSION. Returns Some(warning line) on mismatch,
/// None if agf/VERSION is absent or matches — nothing to report either
/// way.
fn version_drift_warning() -> Option<String>
```

Call sites:

- `run_all_validators` (the `agov validate` umbrella, `main.rs`): render
  the warning via the existing `OutputCounter::validate_warn` method
  (same shape every other advisory validate finding already uses —
  `WARN {label}: {location}: {message}`), never affecting the exit code,
  same as `render_readme_stats`'s existing advisory-only pattern.
- `session_resume` (`main.rs`): render the warning as one more line in
  the existing warnings list surfaced at cold start, alongside the
  existing "working tree has uncommitted changes" style warnings.

Both call sites use the same `agov-core` primitive so the comparison
logic exists in exactly one place.

## What this plan does NOT do

- No blocking behavior, no new exit-code path, no new CI gate.
- No check of the `agf/schemas/*.json` versions, subcommand
  compatibility matrix, or any deeper feature-level compatibility check
  — this is a coarse, cheap, string-equality version-string comparison
  only, matching the existing `contract_version` check's own precedent
  in `framework_config.rs`.
- No change to `adoption.framework_version` in learning records (issue
  #112's territory — a related but distinct axis: that issue is about
  metadata *describing* the tree drifting from the tree; this plan is
  about the *running tool* drifting from the tree).
- Does not touch `agf/skills/upgrade/SKILL.md` or any other skill/doc —
  purely a tooling-side advisory check.

## Files expected to change

- `tooling/rust/crates/agov-core/src/lib.rs` — add `commissioned_version()`.
- `tooling/rust/crates/agov/src/main.rs` — add `version_drift_warning()`;
  wire into `run_all_validators` and `session_resume`.
- `tooling/rust/crates/agov/tests/validate.rs` — new test(s): mismatch
  produces the WARN line and exit code 0; match produces no WARN line;
  missing `agf/VERSION` produces no WARN line and no error (existing
  `temp_agf_repo()` fixture already omits `agf/VERSION`, so existing
  tests continue to pass unchanged — this is itself the missing-VERSION
  negative case).
- `tooling/rust/crates/agov/tests/session_resume.rs` — same three cases
  for the `session resume` cold-start path.
- `CHANGELOG.md` / `agf/CHANGELOG.md` — one `### Added` entry under the
  next release heading.

No migration, no schema change, no new dependency.

## Test strategy (TDD — tests written before implementation)

1. RED: add a mismatch-case test asserting the WARN line appears and
   exit code is 0; confirm it fails against current code (no such
   warning exists yet).
2. RED: add a match-case test asserting no WARN line appears (still
   passes trivially pre-implementation, since nothing prints a WARN line
   at all yet — the meaningful new coverage is the mismatch case).
3. GREEN: implement `commissioned_version()` and `version_drift_warning()`,
   wire into both call sites; both new tests pass.
4. Confirm no existing test in `validate.rs` or `session_resume.rs`
   regresses (the shared `temp_agf_repo()` fixture has no `agf/VERSION`
   file, exercising the missing-file no-op path for every pre-existing
   test that uses it).
5. `cargo test --workspace` full pass; `cargo clippy` clean.

## Rollback

Purely additive stdout warning text with no exit-code or schema effect;
reverting is a one-commit revert with no data or state implications.
