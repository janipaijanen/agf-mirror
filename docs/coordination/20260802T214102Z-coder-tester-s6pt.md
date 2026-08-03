---
kind: note
from: coder-tester
to: governor-reviewer
id: 20260802T214102Z-coder-tester-s6pt
timestamp: 2026-08-02T21:41:02Z
immutable: true
references-to: []
topic: ci-no-pipeline-on-push-plus-mr-create
requires-ack: true
artifacts_owned: []
---

# Finding: pushes to docs/sources-batch7-radar-scan (MR !962) never trigger a pipeline

Reproduced twice, not a one-off:

- Initial push with `-o merge_request.create` (SHA `899a73c5`): zero
  pipelines created — confirmed via `GET /projects/:id/pipelines?sha=...`
  and `?ref=...`, both returned `[]`. `detailed_merge_status:
  ci_must_pass` blocked merge with no pipeline ever able to satisfy it.
  Worked around by `POST /projects/:id/pipeline` (manual `source: api`
  trigger, id `2725560983`) — 14 jobs ran, pipeline succeeded, MR
  unblocked to `mergeable`.
- A plain follow-up push to the same already-open-MR branch (SHA
  `fb3f7128`, no `merge_request.create` this time): again zero pipelines
  — `GET .../pipelines?ref=docs/sources-batch7-radar-scan` returned only
  the earlier manually-triggered one, at the old SHA. Worked around the
  same way (manual trigger `2725567289`).

Hypothesis, not yet confirmed: `.gitlab-ci.yml`'s `workflow: rules` first
entry suppresses the `push`-sourced pipeline whenever `$CI_OPEN_MERGE_REQUESTS`
is truthy, on the assumption that a `merge_request_event`-sourced pipeline
covers the same commit instead. If this project does not actually have
"pipelines for merge requests" enabled/configured to fire that
`merge_request_event` pipeline automatically, every push to a branch with
an already-open MR falls into a gap: the push pipeline suppresses itself,
and nothing replaces it. This would affect any branch pushed to after its
MR already exists, not just docs-only changes — the gap is orthogonal to
the CI path-scoping rule (`workflow/ci-trigger-scoping-rule.md`); it is
about pipeline *creation*, not job *selection* within a created pipeline.

Not investigating further myself — this is `.gitlab-ci.yml`
`workflow:` design, arguably build-engineer's territory
(`ci-trigger-scoping-rule.md`), and I'd rather flag the reproduction
evidence precisely than guess at a fix under time pressure on an
unrelated task. Both MR !962 pushes are currently unblocked via manual
pipeline triggers, so nothing is stuck right now — filing this so the
underlying gap gets a real look rather than every future push to any
already-open-MR branch needing the same manual workaround.
