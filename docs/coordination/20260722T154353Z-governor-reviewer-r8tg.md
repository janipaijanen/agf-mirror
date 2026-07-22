---
kind: decision-record
from: governor-reviewer
to: all
id: 20260722T154353Z-governor-reviewer-r8tg
timestamp: 2026-07-22T15:43:53Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260722T151014Z-contribution-auditor-r3hj
  - 20260722T151632Z-contribution-auditor-r5mm
topic: activate-build-engineer-after-ci-cost-incident
requires-ack: false
artifacts_owned:
  - agf/agents/MANIFEST.yaml
  - AGENTS.md
---

# Decision: activate build-engineer for CI and release-build work

Codex-scribed record of the human governor's direct instruction in chat:
"bring the build/ci/release role".

## Decision

Activate the existing `build-engineer` role in foundry and assign it to Claude
for CI/build and release-build work.

Its active scope is the jurisdiction already defined in
`agf/agents/build-engineer.yaml`:

- `.gitlab-ci.yml` and CI scripts;
- build automation, artifacts, and caching;
- pipeline-source and path trigger rules;
- pipeline efficiency and runner-cost analysis; and
- release-build and packaging stages that produce artifacts.

This does **not** activate deployment authority. Release approval, immutable
tag decisions, deployment, and rollback remain outside build-engineer;
`deployment-engineer` remains skipped. The governor-reviewer retains every
binding Gate 1, Gate 2, risk-acceptance, and release-authorization decision.

## Evidence and supersession

MR !356 recommended against activation because it found no foundry incident
matching the role's origin evidence. That premise no longer holds: MR !398
commit `dd2e5e6` created duplicate branch and merge-request pipelines, repeated
substantive CI jobs, and consumed avoidable runner resources. Learning
`20260722T150853Z-0e14` and lesson proposal `r3hj` record that evidence.

The human governor has now made the binding judgment that the cost and ownership
gap justify activation. This decision supersedes MR !356's advisory
recommendation; the factual observations in that review remain useful history.

## First assignment and gate boundary

The first assignment is the urgent CI-deduplication work routed in `r5mm`:
consult on the Gate 1 plan, then implement only after governor Gate 1 approval.
Activation grants role jurisdiction; it does not waive either governance gate
or authorize disabling checks.

`agf/agents/MANIFEST.yaml` is updated as the authoritative role-status surface,
and `AGENTS.md` is updated so cold-starting agents receive the same state.
