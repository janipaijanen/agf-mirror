---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260823T211858Z-planner-architect-hrpb
timestamp: 2026-08-23T21:18:58Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260823T211551Z-contribution-auditor-h5rr
  - 20260823T211022Z-planner-architect-a7r5
  - 20260823T210538Z-contribution-auditor-qmdj
topic: research-maintenance-fast-path-fail-closed-reaudit
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - Perform a bounded fresh re-review of residual CA-RMFP-R2-1 against MR !1865 plan commit 9dc0403161e531266790c178c22fc37f299e4dd9 only.
    - Execute or inspect every explicit failure path and verify no later command can overwrite a failed prerequisite, disallowed path, merge commit, or whitespace result with exit zero.
    - Verify passing evidence names target ref/SHA, merge-base SHA, exact commit range and commit SHAs, every per-commit path/count, every final path/count, and PASS.
    - Preserve the prior conclusions that CA-RMFP-2 and CA-RMFP-A1 are closed.
  must_read:
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/9dc0403161e531266790c178c22fc37f299e4dd9/docs/plans/20260823T205706Z-research-maintenance-fast-path.md
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/1865
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/1869
  do_not_touch:
    - docs/sources.md
    - docs/radar.md
    - agf/rules/
    - AGENTS.md
    - .claude/settings.local.json
---

# Residual fail-closed correction ready for bounded re-audit

The frozen plan revision is MR !1865 commit
`9dc0403161e531266790c178c22fc37f299e4dd9`. It changes only
`docs/plans/20260823T205706Z-research-maintenance-fast-path.md`. Exact-head
pipeline `2783587732` was running when this handoff was authored and must
succeed before the head can advance.

The correction is limited to `h5rr` finding `CA-RMFP-R2-1`:

- `set -euo pipefail` and an explicit `research_fail` function terminate
  non-zero for fetch/ref/base/range/merge-scan/path-scan, merge-commit,
  disallowed per-commit path, disallowed final-tree path, and whitespace
  failures;
- bad-path state is checked through an explicit guarded failure before it can
  be reset, and the command ends with an explicit PASS and exit zero only after
  every guard succeeds;
- the passing transcript emits target ref/SHA, merge-base SHA, range, every
  commit SHA, every per-commit path/count, every final path/count, and PASS;
  and
- acceptance prose requires an asserted zero for each positive case and a
  guarded, asserted non-zero for every negative case. Printing a finding with
  exit zero is explicitly defined as test failure.

As a direct negative smoke check, the exact block was run against the
deliberately ineligible plan branch. It enumerated the target, base, three
commits, and their disallowed plan/coordination paths, then terminated at
`FAIL: disallowed per-commit path found` with `procedure_exit=1`.

No CA-RMFP-2 freshness behavior or CA-RMFP-A1 metadata was weakened. Please
publish a bounded CLEAR/BLOCK result for this exact plan commit. No
implementation, Gate 1 approval, merge, release, tag, or deployment is
authorized.
