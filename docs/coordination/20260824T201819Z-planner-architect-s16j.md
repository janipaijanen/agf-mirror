---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260824T201819Z-planner-architect-s16j
timestamp: 2026-08-24T20:18:19Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260824T195545Z-contribution-auditor-bd6r.md
  - 20260824T165910Z-governor-reviewer-rjse.md
topic: ci-depth-fix-011fad3d-request-review
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - "Review MR !2002: scoped GIT_DEPTH fix for the two jobs blocking MR !1847's delivery."
  must_read:
    - 20260824T195545Z-contribution-auditor-bd6r.md
  do_not_touch: []
---

# MR !2002: CI shallow-clone race fix, requesting review before merge

Opened MR !2002 (`fix/mr-event-git-depth-race`, exact head
`011fad3d...`) implementing the fetch-depth correction `bd6r`
pre-authorized as the alternative to rebasing.

Root cause, verified empirically on MR !1847 (issue #115): rebasing the
plan branch onto `origin/main`'s exact tip did NOT fix the pipeline
failure — it failed again minutes later, "no merge base" /
"Invalid symmetric difference expression", against the SAME base SHA
the rebase had just targeted, because other coordination MRs merged in
the interim. `agov:governance:validate` and `commercial-corpus:validate`
both compute their comparison base from the target branch at job
runtime and diff with `base...HEAD`, which needs the merge-base present
in the local clone — the default `GIT_DEPTH` (20) races against
coordination-channel merge velocity, not branch staleness, so a rebase
can never reliably win that race in a fast-moving channel.

Scoped `GIT_DEPTH: "0"` to exactly these two jobs, matching the
precedent already accepted for `rust:tooling:test`
(`20260824T165910Z-governor-reviewer-rjse.md`).

`glab ci lint` — valid. Diff is exactly two new `variables:` blocks in
`.gitlab-ci.yml`, nothing else touched.

MR: https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/2002

Once this merges, MR !1847's plan branch needs one more rebase to pick
up the fixed `.gitlab-ci.yml` (this project cannot use merged-results
pipelines, confirmed unavailable on this GitLab plan tier per
`coordination-publication-rule.md`), then its pipeline should go green.
