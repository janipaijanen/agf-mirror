---
id: 20260911T094300Z-commercial-corpus-validate-shallow-history-fix
status: pending-gate1
topic: coordination-ci-shallow-history-blocker
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >-
      Ran `agov validate infosec`: OK, no findings. This plan adds one
      `GIT_DEPTH: "0"` job variable to `.gitlab-ci.yml`, mirroring an
      already-approved precedent (`agov:governance:validate`,
      `rust:tooling:test`) on the same file. No credential, schema, or
      authority surface change. `commercial-corpus:validate` reads a
      commercial-corpus manifest and runs `agov commercial-corpus
      validate`; this plan changes only how much git history the job's
      checkout has available, never the validator's own logic or the
      manifest it reads.
  - role: contribution-auditor
    status: consulted
    reason: >-
      build-engineer (Codex, `gpt-6-astra`) diagnosed and reported this
      exact defect in `20260911T082740Z-build-engineer-5bbs`
      (`origin/codex/ci-shallow-history-report`, MR !2705), including the
      live failing pipeline, the exact `git diff` error, and an explicit
      instruction not to infer approval from the existing
      `rust:tooling:test` precedent. This plan is the requested bounded
      Gate 1 correction proposal `5bbs` asked build-engineer/planner-
      architect to prepare; re-verified independently against current
      `main` before drafting (see "Confirmation against current `main`").
---

# Plan: guarantee merge-base ancestry for `commercial-corpus:validate`

Formalizes `20260911T082740Z-build-engineer-5bbs`'s diagnosis into a Gate 1
plan, per its explicit request ("acknowledge ownership and prepare the
bounded Gate 1 correction proposal ... no CI implementation is approved by
this note"). **Drafting only. No `.gitlab-ci.yml` change is authorized until
this plan is Gate 1 approved.**

## Confirmation against current `main`

Re-read `.gitlab-ci.yml` at current `main` (`dae91b50`, 2026-09-11) before
drafting. The defect `5bbs` diagnosed is confirmed still present:

- `commercial-corpus:validate` (line 1180) runs
  `cargo run -p agov -- commercial-corpus validate --repo-root
  "$CI_PROJECT_DIR" --base-ref "$base_ref"` where `base_ref` defaults to
  `${CI_MERGE_REQUEST_DIFF_BASE_SHA:-HEAD^}` — this requires a real,
  resolvable merge base between the checkout and that ref.
- The job sets **no** `GIT_DEPTH` override, so it inherits this project's
  default shallow-clone depth (20, per the existing comment on
  `agov:governance:validate`, line 944, itself citing the GitLab project
  API).
- `agov:governance:validate` (line 938) already documents exactly this
  failure mode and already sets job-local `GIT_DEPTH: "0"` to avoid it
  (line 951): *"This job compares a range against the target branch, which
  needs a real merge base. The project default is 20 ... a branch more
  than ~20 commits past its base would have no merge base in a depth-20
  checkout."* Its comment explicitly says "see the note on
  `commercial-corpus:validate` below" — but no such note, and no
  `GIT_DEPTH` override, currently exists on that job. This reads as a
  dangling cross-reference to a sibling fix that was never applied.
- `rust:tooling:test` (line 1207) independently sets its own job-local
  `GIT_DEPTH: "0"` for an unrelated reason (issue #112 — a test reading
  historical repository state), confirming the precedent pattern this plan
  follows is already established practice in this file, not a novel one.
- `5bbs`'s live evidence: MR !2678 at exact head `0cb4f870`, pipeline
  `2840038842`, job `commercial-corpus:validate` (16441260784) failed with
  `no merge base` while the sibling `agov:governance:validate` job
  (16441260783) — which does set `GIT_DEPTH: "0"` — passed in the same
  pipeline. This is a direct, same-pipeline confirmation that the missing
  override is the differentiator.

## Fix

Add a job-local `GIT_DEPTH: "0"` variable to `commercial-corpus:validate`,
identical in shape to the existing `agov:governance:validate` override, with
a comment explaining why (mirroring that job's own comment style):

```yaml
commercial-corpus:validate:
  extends: .rust-base
  stage: test
  interruptible: true
  variables:
    # This job's --base-ref comparison needs a real merge base against
    # CI_MERGE_REQUEST_DIFF_BASE_SHA. The project default GIT_DEPTH (20)
    # truncates history before that merge base on any branch more than
    # ~20 commits past its target, producing a `no merge base` failure
    # unrelated to any actual commercial-corpus defect. Same fix as
    # agov:governance:validate above, for the same underlying reason.
    GIT_DEPTH: "0"
  script:
    ...
```

No change to the job's `script:` or `rules:`. No change to any other job.

## Alternatives considered

### A — `GIT_DEPTH: "0"` (selected)

Exactly mirrors two existing precedents in the same file. Simple,
consistent, and — per `ci-trigger-scoping-rule.md`'s own caution about
scoping mistakes — the safest default: a full clone can never produce a
false "no merge base" failure regardless of how long a branch grows.
Tradeoff: slightly larger checkout for this job on every pipeline run
(this job's `rules:` already runs it on every MR, branch push, and tag —
unconditional, matching `agov:governance:validate`'s own unconditional
rules). Accepted because the existing `agov:governance:validate` job
already pays this same cost today with no reported problem, and this job
runs the same `.rust-base` template (shared runner/cache characteristics).

### B — Targeted/bounded deepening (`git fetch --deepen N` or a
conditional `--unshallow` retry only on missing-merge-base)

`5bbs`'s note explicitly asked for this to be *compared*, not assumed
inferior. Rejected as the primary fix: it requires job-script logic to
detect the missing-merge-base condition and retry, which is strictly more
code than a one-line `GIT_DEPTH: "0"`, for a benefit (smaller checkout)
that the existing `agov:governance:validate` precedent shows is not needed
in practice on this project's runners. It also risks exactly the kind of
silent-pass regression `agov:governance:validate`'s own comment warns
about (a masked `git diff` failure) if the retry logic itself has a bug —
more moving parts than the fail-closed one-line fix. Retained as a future
option only if `GIT_DEPTH: "0"` is later measured to meaningfully slow
this job's pipeline stage.

### C — Do nothing, treat as a known limitation

Rejected: `5bbs` shows this actively blocks MR !2678's acceptance today
with a real failing job, not a hypothetical. `always/gate-self-test-rule.md`
does not apply here (this is not a gate modification, it is fixing a false
negative in an already-existing validation job's evidence availability),
but the underlying `always/downstream-impact-rule.md` PREVENTIVE
classification applies: this closes a gap already causing real blocked
work.

## Gate 2 evidence required

1. **CI Lint**: confirm `GIT_DEPTH: "0"` parses and appears correctly in
   the expanded configuration for `commercial-corpus:validate`, via
   `glab ci lint --dry-run --include-jobs` (same method as prior CI-config
   plans in this repo).
2. **Live pipeline proof on the actual blocked MR**: re-run (not retry —
   a fresh pipeline on the corrected `.gitlab-ci.yml`, per
   `always/debugging-rule.md`'s "diagnose before fixing" and this
   project's "do not repeatedly rerun unchanged paid work" convention)
   `commercial-corpus:validate` against MR !2678's exact ref/content and
   confirm it now resolves a merge base and passes.
3. **Regression check**: confirm `agov:governance:validate` and
   `rust:tooling:test` are unaffected (no shared-cache or checkout-order
   interaction from the same-file edit).
4. **Non-regression on a short branch**: confirm the job also passes
   normally on an ordinary MR well within the depth-20 window, showing the
   fix only changes checkout depth, never validator behavior.

## Role and gate boundary

planner-architect (this session) drafts and does not implement. Per
`5bbs`'s framing ("Claude build-engineer lane") and current role
assignment (`role-assignment-foundry` project memory: Claude plays
planner-architect + build-engineer; coder-tester is Codex), build-engineer
reviews CI-mechanics correctness once Gate 1 approves; implementation
itself is coder-tester's (Codex) lane unless the governor directs
otherwise, consistent with `interruptible-superseded-pipelines.md`'s and
`first-push-ci-changes-fallback.md`'s precedent for CI-config plans in this
project.

## Out of scope

- MR !2678's own coordination-history content — this plan only fixes the
  CI job's evidence-availability gap, not anything about what !2678 itself
  records.
- The separate first-push pipeline-duplication defect track referenced in
  `5bbs` (`references-to: 20260911T075938Z-coder-tester-g8sb`) — different
  mechanism, not conflated here, matching this project's established
  convention of keeping CI defects with different root causes in separate
  plans.
- Any other job's `GIT_DEPTH` setting.

## Risks

- **Checkout cost**: mitigated per Alternative A's reasoning above — this
  project already accepts the same cost for `agov:governance:validate` on
  every pipeline.
- **Scope drift into "fix all shallow-history-dependent jobs"**: explicitly
  rejected. Only the one job `5bbs` diagnosed and evidenced with a live
  failure gets the fix; a broader audit of every job's history assumptions
  is a separate future task, not folded in here.

## Rollback

Single-line variable removal; revert the commit if a regression appears.
No schema, data, or artifact migration.
