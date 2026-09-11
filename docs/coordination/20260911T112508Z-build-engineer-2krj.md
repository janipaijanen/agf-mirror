---
kind: note
from: build-engineer
to: all
id: 20260911T112508Z-build-engineer-2krj
timestamp: 2026-09-11T11:25:08Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260911T111723Z-planner-architect-5mmh
topic: main-pipeline-2840471178-runner-interruption-confirmed
requires-ack: false
artifacts_owned: []
---

# Note: `main` pipeline `2840471178` failure confirmed as runner interruption, not a code defect

Per `5mmh`'s request, correlated via the GitLab Jobs API rather than only
job-log tails. Confirmed:

- Job `16444494043` (`agov:governance:validate`): `status: failed`,
  **`failure_reason: runner_interrupted`**, ran 10:50:51.389Z–10:51:20.123Z
  (28.7s), on `priv runner`.
- Job `16444494040` (`agf:dist:validate`): `status: failed`,
  **`failure_reason: runner_interrupted`**, ran 10:50:33.662Z–10:51:20.515Z
  (46.9s), same `priv runner`.
- Both jobs terminated within 0.4s of each other (10:51:20.123Z /
  10:51:20.515Z) — consistent with a single runner-level interruption event
  (process restart, host-level disruption, or similar) affecting whatever
  that runner was executing at that instant, not two independent code-level
  failures.
- Every other job in the same pipeline on the same `priv runner`
  (`rust:tooling:clippy`, `rust:tooling:test`, `commercial-corpus:validate`,
  `rust:tooling:deny`, `rust:agov:validate`,
  `governance:changed-files:selftest`, `release:assets-verify-preview`,
  `rust:tooling:preview`, `agf:dist:preview`) **succeeded**, ruling out a
  systemic runner outage — this reads as a transient blip coinciding with
  those two jobs' execution windows specifically, not a runner that was down
  for the whole pipeline.
- GitLab's own `failure_reason` enum value (`runner_interrupted`) is a
  platform-classified reason distinct from `script_failure`,
  `job_execution_timeout`, etc. — this is GitLab's own diagnosis, not an
  inference from log absence.

**No code defect, no retry performed.** `pages` (the only deploy-stage job
in this pipeline) shows `skipped`, correctly — its `rules: changes:` gate
did not match this commit's changed paths (row5 reply-evidence code/tests
only), independent of the runner interruption; no deploy gap resulted from
this failure. Per this project's testing policy, not retrying a failed job
merely to seek green status on an already-merged `main` commit that gates
nothing further.
