---
id: 20260912T112252Z-cargo-deny-advisory-db-clean-clone-gate1
status: approved
gate1_approved_by: governor-reviewer
gate1_approved_at: 2026-09-12T13:25:44Z
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
      Two consultations: `20260912T112815Z-contribution-auditor-34pp`
      (DENY-P1 root-cause overstatement, DENY-P2 borrowed waiver,
      DENY-A1 revert-framing) and its follow-up
      `20260912T115356Z-contribution-auditor-6kb2` (DENY-P2 and DENY-A1
      confirmed resolved; DENY-P1 residual — conditional language was
      still missing from "Alternatives" and the reproducer-cost claim
      assumed the disputed mechanism, both corrected in this revision;
      plus a named required post-approval correction to implementation
      comments that still assert the cause as fact). This revision
      addresses every point raised in both reviews except the underlying
      causal question itself, which is intentionally left open for the
      governor to accept as a stated validation limit — see "Problem".
---

# Plan: force a clean `cargo-deny` advisory-db clone in `rust:tooling:deny`

**Gate 1: APPROVED** per `20260912T132544Z-governor-reviewer-jhaq`
("gates approved. proceed", governor decision transcribed by Codex, no
AI-originated authority claimed). Approval accepts the disclosed
validation limit (the original failure cause and mitigation efficacy
are unproven) and the disclosed out-of-sequence implementation, and
authorizes prospective continuation from existing `!2723` head
`2c1a7989`, including the plan-required correction to the three
implementation comments before Gate 2 — not a claim that the mitigation's
cause has been demonstrated. Revisit if the original failure recurs or
evidence contradicts the mitigation's premise.

## Post-merge record recovery (2026-09-17)

This plan was approved before implementation MR !2723 merged, but its mixed
carrier MR !2725 remained open and the plan therefore never reached `main`.
MR !2723 subsequently merged at exact head
`a926e857a24cdd2c49c3a584fd1e32865cc19b6c` as merge commit
`eb55cbed580081eac1336f7f71acc0042d6c6a4a`. The required conditional-language
corrections described below are present on current `main`; implementation
handoff `20260912T133559Z-build-engineer-nq3h` records the reviewed result.

This plan-only carrier restores the already-approved Gate 1 artifact. It does
not retroactively change the sequencing disclosure, reapprove the merged
implementation, or claim stronger causal evidence. The exact approval and
review chain is restored independently by coordination MR !2947.

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

**Validation limit / choice put to the governor, corrected per `6kb2`:**
no bounded local reproducer of the torn-state precondition has been
built. The previous revision asserted that building one would require
simulating GitLab's cross-pipeline runner filesystem behavior and called
that disproportionate as though it were the only available method and a
proven impossibility — contribution-auditor correctly points out that
local `cargo-deny` source inspection or controlled local directory
states (e.g. deliberately creating a partial clone and observing whether
`cargo-deny` reproduces the same misparse) could inform the question
without needing to reproduce GitLab's runner lifecycle at all. This is
restated as a proposed effort tradeoff, not a demonstrated technical
impossibility: the governor is asked to accept either (a) the mitigation
as a reasonable, low-cost, non-weakening guard against this failure
shape regardless of unconfirmed precise cause, spending no further
effort on reproduction; or (b) hold Gate 1 and direct build-engineer to
attempt the cheaper local-inspection/controlled-directory approach
before continuing.

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
`GIT_DEPTH` precedent. The string-order assertion is a **configuration
guard** (confirms the line exists and runs first) — per `6kb2`, it is
not being described as a causal regression reproducer for the
torn-state hypothesis.

**Required correction not yet made (per `6kb2`, residual on `34pp`):**
the `.gitlab-ci.yml` comment above the `rm -rf` line, and the doc
comments in `ci_needs_implication.rs` and `gate_change_check.rs`, all
currently assert the prior-run torn-state cause as established fact —
the same overstatement this plan's "Problem" section now corrects. The
implementation freeze in the "Sequencing disclosure" section above means
this is **not** being fixed in this plan revision. It is named here as
required scope for the first commit made after a governor decision that
permits any further commit on `fix/rust-deny-advisory-db-cache`: reword
those three comments to match this plan's conditional language (observed
crash trace + unproven hypothesis), before that branch is presented as
Gate-2-ready.

## Alternatives considered

**Corrected per contribution-auditor review `20260912T115356Z-
contribution-auditor-6kb2` (DENY-P1, residual) — the previous revision
of this section still asserted the unproven torn-state cause as
established fact. Reworded to conditional language, consistent with
"Problem" above.**

- **Do nothing / retry the pipeline unchanged.** Rejected —
  `debugging-rule.md` forbids retrying a failure without a diagnosed
  root cause. If the hypothesized torn-state precondition is real, an
  unchanged retry would only pass on a runner that happens to land on a
  clean filesystem, which would not demonstrate the failure is fixed
  and could mask a recurrence on a future run; this is offered as a
  reason to prefer an explicit fix over an unchanged retry, not as
  proof of the mechanism itself.
- **Cache `$CARGO_HOME/advisory-dbs`.** Rejected — even setting aside
  which precise precondition is at fault, caching the exact path where
  the crash was observed risks reintroducing whatever state produced
  it, and adds a new persisted path whose staleness (an outdated
  advisory list) would be a worse failure mode for a security-advisory
  check than a slightly slower clean clone every run. This objection
  holds independent of whether the torn-state hypothesis is confirmed.

## Gate 2 evidence required

1. CI Lint / `glab ci lint` confirms the script parses.
2. Live pipeline proof that `rust:tooling:deny` passes on this branch
   (already green: MR !2723 pipeline, `merge_request_event` source,
   status `success`).
3. `cargo test -p agov --test ci_needs_implication
   rust_tooling_deny_clears_stale_advisory_db_before_check` passes
   locally.
4. Confirm no other job's script or cache configuration changed.
5. The named implementation-comment correction (above) is made and
   reviewed before this branch is presented as Gate-2-ready.

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
