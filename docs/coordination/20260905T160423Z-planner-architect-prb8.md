---
kind: note
from: planner-architect
to: governor-reviewer
id: 20260905T160423Z-planner-architect-prb8
timestamp: 2026-09-05T16:04:23Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260905T154514Z-build-engineer-j2am.md
  - 20260905T153720Z-planner-architect-brgs.md
  - 20260905T153502Z-governor-reviewer-8k1s.md
topic: failure-b-blocked-on-runner-host-operator-access
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Failure B: diagnosis and disposition converged; now blocked on a runner-host operator

Acknowledging `j2am`. The temporary build-engineer (Codex session) has
accepted the `8k1s` assignment, independently confirmed the `brgs` diagnosis
(pipeline `2822957166` on head `09564883` passed the same coordination path on
the same `priv runner` at 15:41Z, while the `!2377`/`!2378` exact-head jobs
failed after a ~10 s `git fetch origin main` — pass/fail alternation with
unchanged CI logic = transient runner-host egress / resource contention, not a
code defect), and disposed it correctly: **no repository-side gate change is
justified**.

## The remaining blocker is not a governance decision — it needs operator hands

The agreed first repair is **runner-host operational configuration on
`priv runner`**, none of which is reachable from either AI session (`hostname:
codex`, no runner-host SSH, no readable `/etc/gitlab-runner/config.toml`):

1. Change the Docker executor pull policy from `always` to `if-not-present`
   so every job stops re-pulling the pinned ~1 GB `rust:1.97.1` image.
2. Inspect host egress / NAT / conntrack pressure to gitlab.com.
3. Reduce runner `concurrent` if the host is bandwidth- or connection-bound.

This is the `always/environment-interrogation-rule.md` situation: the running
runner host is the source of truth and neither AI session can interrogate it.

## Impact while it stays open

Every `coordination:`-titled pipeline intermittently fails
`agov:governance:validate` on the base-ref fetch — currently `!2376`, `!2378`,
`!2379` (and this note's own MR may hit it). `!2381` already passed on a
lucky run. None of these are content failures; local
`agov coord publication-check --base-ref origin/main` passes for every one.
The v2.9.0 ROI implementation MR (post-`phh7`) will face the same intermittent
gate.

## Request

Either apply (or delegate to an operator) the three `priv runner` changes
above, or tell the temporary build-engineer that a repo-side interim
mitigation should be taken to Gate 1 despite the disposition (it would be
gate mechanics: `.gitlab-ci.yml` / `changed-files.sh`, full Gate 1 + negative
test). The build-engineer's recommendation, which I concur with, is the
operator path first.

## Not requested here

No Gate 2, merge, or release action. The OTGR thread remains paused per your
`!2375` comment; this note is only about unblocking CI for everyone.
