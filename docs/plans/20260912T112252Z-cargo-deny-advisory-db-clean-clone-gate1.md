---
id: 20260912T112252Z-cargo-deny-advisory-db-clean-clone-gate1
status: pending-gate1
topic: cargo-deny-advisory-db-clean-clone
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >-
      Ran `agov validate infosec`: OK, no findings. This is the
      validator's configured disclosure-screening evidence (credential,
      schema, and authority-surface leaks), not a semantic assessment of
      whether the cleanup actually fixes the clone/crash failure —
      flagged as a needed clarification by contribution-auditor review
      `20260912T112815Z-contribution-auditor-34pp`. `$CARGO_HOME` is a
      CI-runner-local ephemeral path already scoped to this job, not a
      cached or persisted directory (only `.cargo/registry`,
      `.cargo/bin`, and `tooling/rust/target` are cached per the existing
      cache configuration), so clearing it cannot remove anything another
      job or a later pipeline depends on.
  - role: contribution-auditor
    status: consulted
    reason: >-
      `20260912T112815Z-contribution-auditor-34pp` is the actual
      consultation (this plan's earlier "waived per gqmg precedent" was
      incorrect — a similar prior incident is classification evidence,
      not a governor waiver for this plan; corrected here per that
      review's DENY-P2). That review found two blocking findings, both
      addressed below: the root-cause claim is now stated as an unproven
      hypothesis (DENY-P1) rather than asserted fact, and the sequencing
      disclosure no longer frames revert-and-redo as the only alternative
      to retroactive approval (DENY-A1, advisory). Unresolved as of this
      revision: the underlying causal question in DENY-P1 (below) — this
      plan asks the governor to accept the validation limit rather than
      pay for further reproduction, not claiming it as resolved.
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

**Observed fact, corrected per contribution-auditor review `34pp`
(DENY-P1) — the previous revision of this section overstated this as an
established root cause; it is not.** Canonical pipeline `2842901098`'s
required `rust:tooling:deny` job (log for job `16461743517`) crashed:
after installing `cargo-deny` 0.19.9, it failed while parsing a Git
fatal diagnostic about an absent advisory-db directory as if it were a
`FETCH_HEAD` timestamp. That crash trace is directly observed.

**Unproven hypothesis, stated as such:** the working theory is that a
prior run left partial/torn state at `$CARGO_HOME/advisory-dbs` — a
path this project's CI cache configuration does not persist (only
`.cargo/registry`, `.cargo/bin`, and `tooling/rust/target` are cached)
— and that `cargo-deny` then tried to read an existing clone's
`FETCH_HEAD` and crashed on it. **This has not been demonstrated**: the
job log does not show a prior run's leftover state, does not explain
why a non-cached path would retain content across runs on GitLab's
shared runners, and `rm -rf` on a path that is genuinely absent is a
no-op — so if the real precondition differs (e.g. a partial clone left
mid-run by cargo-deny itself within the *same* invocation, rather than
carried over from an earlier one), this fix would not address it. The
one subsequent successful pipeline (`2842989228`) is evidence that
*that run* passed with the fix present; it is not a before/after causal
reproduction of the crash, and is not being upgraded to causal proof.

**Validation limit / choice put to the governor:** no bounded local
reproducer of the torn-state precondition has been built, and none is
proposed as a precondition for this Gate 1 decision — building one
would require simulating GitLab's ephemeral-runner filesystem behavior
across pipeline runs, which is disproportionate to a one-line mitigation
for a job that is otherwise passing. The governor is asked to accept
either: (a) the mitigation as a reasonable, low-cost, non-weakening
guard against this failure shape regardless of unconfirmed precise
cause, on the understanding the causal claim is unproven; or (b) hold
Gate 1 pending a bounded reproducer, in which case build-engineer will
scope one before continuing.

Same failure-*shape* (a required job's advertised failure looking like
a real finding but actually being a setup/evidence-availability defect)
as the `commercial-corpus:validate` shallow-history case fixed in
`20260911T094300Z-commercial-corpus-validate-shallow-history-fix` — that
comparison is offered as a classification aid, not as proof this case's
specific mechanism matches.

`always/gate-self-test-rule.md` does not apply, for the same reason
that precedent plan gave: this is not a gate-detection modification.
`cargo deny check` itself, its failure exit, and `deny.toml` are
byte-for-byte unchanged. This holds regardless of the open causal
question above — the change is additive precondition-clearing, not a
change to what `cargo-deny` accepts or rejects.

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
  undone — per contribution-auditor review `34pp` (DENY-A1), replaying
  identical commits after a revert does not un-happen the deviation, so
  revert-and-redo is not framed as the only path back to a clean record.
  Disclosed above; the governor may instead accept the deviation and
  authorize continuing from the exact reviewed existing candidate head
  without backdating the record. Revert-and-reimplement remains
  available if the governor judges it necessary regardless.
- **Root-cause uncertainty (DENY-P1, unresolved).** The precise
  precondition this fix addresses is an unproven hypothesis, not a
  demonstrated cause — see "Problem" above. Accepted as a stated
  validation limit unless the governor requires a bounded reproducer
  first.
- **Checkout/runtime cost.** Negligible — `rm -rf` on a small,
  non-cached, job-local directory before a clone that would happen
  anyway.

## Rollback

Single-line removal (`rm -rf "$CARGO_HOME/advisory-dbs"`); revert the
commit if a regression appears. No schema, data, or artifact migration.
