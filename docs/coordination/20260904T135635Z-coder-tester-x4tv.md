---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260904T135635Z-coder-tester-x4tv
timestamp: 2026-09-04T13:56:35Z
immutable: true
model: claude-sonnet-5
references-to:
  - 20260904T113400Z-coder-tester-vq9r.md
topic: route-all-jobs-to-dedicated-runner-gate2
requires-ack: true
artifacts_owned:
  - .gitlab-ci.yml
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Gate 2 review of a single-file .gitlab-ci.yml change adding a default: tags: block."
    - Confirm the BREAKING classification and accepted single-point-of-failure trade-off (governor already directed this live in session).
    - Merge to main via the MR if accepted; its own MR pipeline is the first live test of whether the dedicated runner can execute jobs, since shared-runner fallback is removed by this change.
  must_read:
    - "gitlab-ci.yml (the new default/tags block, just above Shared templates)"
    - "docs/coordination/20260904T113400Z-coder-tester-vq9r.md (prior diagnosis - quota block, zero jobs ever run on the dedicated runner)"
  do_not_touch:
    - .agf/gates.yaml
    - refs/tags/v2.8.0
    - release branches, tags, published artifacts
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Route every CI job to the dedicated project runner

## Diagnosis (recap of `vq9r`, extended live in session)

Dedicated project runner `#55908035` ("priv runner", tags `podman`/`linux`/
`amd64`) is online, authenticated, `run_untagged: true`, not paused or
locked — and had **executed zero jobs**. The failed `main` jobs
(`agov:governance:validate` etc.) showed `runner: none`, `ci_quota_exceeded`.

Root cause, confirmed live with the governor: `.gitlab-ci.yml` sets no
`tags:` on any job, so every job is eligible for **both** the shared-runner
fleet and the dedicated runner (both have `run_untagged: true`). GitLab has
no "prefer this runner" mechanism between two equally-eligible runners for
an untagged job — it is a race, and the always-on shared fleet was winning
it every time, so jobs kept hitting the exhausted shared-runner quota while
the dedicated runner sat idle. Removing the runner's own tags (considered
and rejected in-session) would not have helped: `run_untagged` already
governs untagged-job eligibility independently of the runner's tag list.

## Decision

Governor direction, live in session, 2026-09-04: **keep shared runners
enabled** (`shared_runners_enabled: true`, unchanged) **and** the dedicated
runner; route **all** jobs to the dedicated runner via job-side `tags:`,
after being shown the alternative (tag only the currently-blocked jobs) and
choosing full routing.

## Change

Added one `default:` block to `.gitlab-ci.yml`, immediately before the
"Shared templates" section:

```yaml
default:
  tags:
    - podman
    - linux
    - amd64
```

No job in the file previously set `tags:`, so this is the complete change —
no per-job edits needed. `default:` applies to every job that does not
define its own `tags:` key; a job may still override it locally if a
future need arises.

## Verification

`POST /projects/:id/ci/lint` (`include_jobs: true`) against this exact file
content: `valid: true`, zero errors, zero warnings. All **32** jobs report
`tag_list: ["podman", "linux", "amd64"]` in the linted result, including
`site-html-build`, `agf:dist:preview`, `rust:tooling:preview`,
`rust:tooling:release`, and every other job in the pipeline — confirmed by
inspecting the full linted job list, not a sample.

## Classification (`downstream-impact-rule.md`)

**BREAKING.** This is not merely additive: it removes shared-runner
eligibility for every job in the pipeline, a real observable change to the
project's CI reliability contract.

```
Impact:
  Foundry:          Yes — every pipeline now depends on #55908035 alone.
                     If that runner goes offline, every job stays pending
                     indefinitely; there is no shared-runner fallback while
                     this block exists.
  Direct adopters:   N/A — this project's own .gitlab-ci.yml is not shipped
                     framework content (agf/ is unaffected); adopters are
                     not exposed to this file.
  Downstream:        None — no adopter-produced artifact shape or agent
                     behaviour toward adopters' users changes.
  Migration:         If the dedicated runner becomes unavailable: delete the
                     default: tags: block (or just its tags: key) in a
                     fast-track commit. That single edit fully restores the
                     prior untagged-race behaviour; no other file needs to
                     change.
  Evidence:          This record; vq9r (prior diagnosis); the CI lint
                     verification above.
```

## Self-test note

This is not a governance gate under `always/gate-self-test-rule.md` (no
`.agf/gates.yaml`, validator, or CI-blocking-check entry changed — this is
runner *routing*, not a new check), so that rule's negative-test requirement
does not apply. The functional test is this MR's own pipeline: it can only
go green by actually running on the dedicated runner, since shared-runner
eligibility is removed for it too.

## Risk accepted, stated plainly

If `#55908035` cannot execute jobs for a reason unrelated to routing (its
own executor/privileged/Docker config — outside this session's access; see
`vq9r`), this MR's pipeline will hang pending with no fallback, which is
itself the diagnostic signal that the runner has an execution-layer problem,
not a routing one. If that happens: revert this commit to restore shared
routing while the runner's own configuration is fixed separately.
