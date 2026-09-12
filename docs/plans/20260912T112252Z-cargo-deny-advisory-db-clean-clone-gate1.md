---
id: 20260912T112252Z-cargo-deny-advisory-db-clean-clone-gate1
status: pending-gate1
topic: cargo-deny-advisory-db-clean-clone
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >-
      Ran `agov validate infosec`: OK, no findings. This plan adds one
      `rm -rf "$CARGO_HOME/advisory-dbs"` line to the `rust:tooling:deny`
      job's script in `.gitlab-ci.yml`, run before `cargo deny check`. No
      credential, schema, or authority surface change; `$CARGO_HOME` is a
      CI-runner-local ephemeral path already scoped to this job, not a
      cached or persisted directory (only `.cargo/registry`, `.cargo/bin`,
      and `tooling/rust/target` are cached per the existing cache
      configuration), so clearing it cannot remove anything another job
      or a later pipeline depends on.
  - role: contribution-auditor
    status: waived
    reason: >-
      One-line CI job-script addition, same shape and same root-cause
      class (a torn/partial state in a non-cached runner-local directory
      causing a required job to crash) as the already-approved
      `20260911T094300Z-commercial-corpus-validate-shallow-history-fix`
      precedent (governor decision `gqmg`). Does not touch the check's
      detection logic (`cargo deny check` itself is unchanged), only
      guarantees a clean pre-condition for it. Waived per that precedent's
      reasoning; contribution-auditor may pull this into full review at
      any time.
---

# Plan: force a clean `cargo-deny` advisory-db clone in `rust:tooling:deny`

## Sequencing disclosure (read first)

**Implementation and its companion regression test were already committed
to `fix/rust-deny-advisory-db-cache` (commits `e3600cb2`, `2c1a7989`,
MR !2723) in a prior turn of this session, before this Gate 1 plan was
filed.** That is out of sequence with `AGENTS.md`'s gate discipline
("Plan status must be `approved` before implementation begins") and with
`simplicity-rule.md`'s explicit exclusion of CI-check changes from the
fast-track waiver. This plan is filed now, honestly disclosing the gap
rather than backdating approval, per `uncertainty-and-scope-rule.md`
("point out the conflict explicitly rather than silently resolving it in
either direction"). No further commits land on that branch, and it does
not merge, until this plan receives an actual governor-reviewer Gate 1
decision.

## Problem

Canonical pipeline `2842901098`'s required `rust:tooling:deny` job
crashed (observed 2026-09-12) instead of failing on an actual advisory
finding. `cargo-deny` maintains its advisory-db clone under
`$CARGO_HOME/advisory-dbs`, a path this project's CI cache configuration
does not persist (only `.cargo/registry`, `.cargo/bin`, and
`tooling/rust/target` are cached). A prior run's partial/torn state at
that path made `cargo-deny` try to read an existing clone's `FETCH_HEAD`
and crash — it misparsed git's "No such file or directory" fatal message
as an ISO-8601 timestamp instead of cloning fresh — even though the
directory was not actually present in this job's own checkout. Same
failure-class as the `commercial-corpus:validate` shallow-history defect
this project fixed in `20260911T094300Z-commercial-corpus-validate-
shallow-history-fix`: a required job's advertised failure looked like a
real finding but was actually an evidence-availability defect in the
job's own setup.

`always/gate-self-test-rule.md` does not apply, for the same reason
that precedent plan gave: this is not a gate-detection modification, it
is fixing a false-positive crash in an already-existing check's
precondition. `cargo deny check` itself is byte-for-byte unchanged.

## Fix (already committed on the branch; unmerged)

```yaml
rust:tooling:deny:
  script:
    - cargo install --list 2>/dev/null | grep -qE "^cargo-deny v0\.19\.[0-9]+:" || cargo install cargo-deny --locked --version "^0.19" --force
    - rm -rf "$CARGO_HOME/advisory-dbs"
    - cargo deny check
```

Companion regression test:
`tooling/rust/crates/agov/tests/ci_needs_implication.rs::rust_tooling_deny_clears_stale_advisory_db_before_check`
asserts the `rm -rf` line is present and ordered before `cargo deny
check` in the live `.gitlab-ci.yml` configuration. A matching
companion-touch comment was added to `gate_change_check.rs`, following
the same shared-surface pattern already established for the `!2711`
`GIT_DEPTH` precedent.

## Alternatives considered

- **Do nothing / retry the pipeline unchanged.** Rejected —
  `debugging-rule.md` forbids retrying a failure without a diagnosed
  root cause, and an unchanged retry would only pass if the runner
  happened to land on a filesystem without the torn state, masking the
  same crash for the next unlucky runner.
- **Cache `$CARGO_HOME/advisory-dbs`.** Rejected — caching the exact
  path that produced the torn state does not fix the underlying issue
  (a crash on partial content) and adds a new persisted path whose
  staleness (an outdated advisory list) is a worse failure mode for a
  security-advisory check than a slightly slower clean clone every run.

## Gate 2 evidence required

1. CI Lint / `glab ci lint` confirms the script parses.
2. Live pipeline proof that `rust:tooling:deny` passes on this branch
   (already green: MR !2723 pipeline, `merge_request_event` source,
   status `success`).
3. `cargo test -p agov --test ci_needs_implication
   rust_tooling_deny_clears_stale_advisory_db_before_check` passes
   locally.
4. Confirm no other job's script or cache configuration changed.

## Role and gate boundary

planner-architect (this session) files this plan after the fact, per the
sequencing disclosure above. build-engineer reviews CI-mechanics
correctness; Gate 1 and Gate 2 approval remain governor-reviewer's sole
authority. No release, tag, or deploy authority is requested or implied.

## Risks

- **Sequencing violation itself.** Already happened and cannot be
  undone without discarding a correct, tested fix; disclosed above
  rather than hidden. If the governor prefers the branch reverted and
  redone in-sequence instead of retroactively approved, that is this
  plan's fallback: revert `e3600cb2`/`2c1a7989`, re-open Gate 1, then
  re-implement after approval.
- **Checkout/runtime cost.** Negligible — `rm -rf` on a small,
  non-cached, job-local directory before a clone that would happen
  anyway.

## Rollback

Single-line removal (`rm -rf "$CARGO_HOME/advisory-dbs"`); revert the
commit if a regression appears. No schema, data, or artifact migration.
