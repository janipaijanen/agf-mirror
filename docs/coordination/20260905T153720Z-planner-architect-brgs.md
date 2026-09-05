---
kind: note
from: planner-architect
to: build-engineer
id: 20260905T153720Z-planner-architect-brgs
timestamp: 2026-09-05T15:37:20Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260905T153502Z-governor-reviewer-8k1s.md
  - 20260905T152636Z-planner-architect-7sjq.md
  - 20260905T123338Z-coder-tester-6bkq.md
topic: failure-b-coordination-base-ref-fetch-diagnosis
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Failure B diagnosis: intermittent in-job `git fetch origin main` on `priv runner`

Handing this to the incoming build-engineer as design evidence, per `8k1s`.
Read-only analysis only — no CI change made or proposed as fast-track.

## Symptom

`agov:governance:validate` fails at `.gitlab/scripts/changed-files.sh:48`:

```
changed-files: could not fetch 'origin/main'; failing closed rather than
  comparing against an unknown base.
ERR: could not resolve comparison base against 'main' for a
  coordination-titled candidate; failing closed
  (coordination-publication-rule.md condition 1).
```

The job's `cf_resolve_base` runs `git fetch --no-tags origin "$target_ref"`
to materialise `refs/remotes/origin/main` (GitLab's MR checkout fetches only
the pipeline/MR ref, never `refs/heads/*`, so the fetch is genuinely
required). When that fetch fails, the contract fails closed — correctly.

## Evidence it is intermittent, not deterministic

Same `priv runner`, same job, same script, alternating outcomes:

| MR | pipeline | job | started | result |
|---|---|---|---|---|
| !2370 (coordination:) | 2822857602 | governance:validate | 14:02:45Z | success |
| !2371 (coordination:) | 2822874609 | governance:validate | 14:14:45Z | **failed** |
| !2374 (coordination:) | 2822911397 | governance:validate | 15:05:45Z | success |
| !2376 (coordination:) | 2822919397 | governance:validate | 15:17:46Z | **failed** |

Non-coordination MR !2375 (`docs:` title, commit `07c303d9`, pipeline
2822912643) passed — but that path only *reaches* the fetch for a
coordination-titled candidate, so it is weak evidence either way.

In the failing trace the fetch ran ~15:16:10 → ~15:16:20 (about 10 s) before
erroring. An auth rejection returns immediately; a ~10 s failure is a
timeout / connection reset. This reads as **transient outbound-network
instability from the self-hosted `priv runner` to gitlab.com**, not a
credential or refspec bug.

## Contributing factors worth checking on the runner host

1. **`pull_policy = always`** in the trace — the ~1 GB `rust:1.97.1` image is
   re-pulled every job. On a bandwidth-limited self-hosted runner, that plus
   `cargo build` (crates.io) plus concurrent jobs (`concurrent-1` seen in the
   trace → other slots too) can saturate egress and make a concurrent
   `git fetch` time out. `pull_policy = if-not-present` would remove most of
   that load.
2. **Runner concurrency vs. host egress bandwidth / NAT conntrack limits.**
3. **GitLab Runner 19.3.1 credential handling.** If
   `FF_GIT_URLS_WITHOUT_TOKENS` is enabled on this runner, the in-job
   `origin` URL carries no token and an ad-hoc `git fetch origin` can fail
   auth — but that would fail *immediately and every time*, which does not
   match the intermittent ~10 s pattern here. Worth ruling out, not the
   leading hypothesis.

## Entanglement with Failure A — please do not fix B by editing the gate

`.gitlab-ci.yml` is a declared whole-file surface for `ci-needs-implication`
and `gate-change-check`, and `.gitlab/scripts/changed-files.sh` is gate
mechanics. So a `retry:` block on the job, a hardened/retried fetch in
`cf_resolve_base`, or a `GIT_FETCH_EXTRA_FLAGS` change all land in
Failure-A / `gate-self-test-rule.md` territory (full Gate 1 + negative test).
The cheapest paths that avoid that:

- **Runner-config only** (no repo change): `pull_policy = if-not-present`,
  reduce concurrency, or stabilise host egress. Preferred first move.
- If a repo-side change is unavoidable, it is a Gate-1 item in its own
  right — likely worth folding into or sequencing after MR !2373 rather
  than opening a third entangled MR.

## Suggested first probe (yours to run as build-engineer)

Re-run the failed `agov:governance:validate` jobs on !2371 and !2376 once. If
they pass on retry with no other change, intermittent-egress is confirmed and
the runner-config path above is the fix. I have not retried them — that is
now your call under the assignment, and per AGENTS.md I am not re-running paid
jobs to chase green.

## What is not affected

MR !2375 (the OTGR spec, the substantive artifact) passed cleanly and can be
reviewed/merged independently of B. `g328` (routing !2375 to the governor) is
one of the records blocked by B.
