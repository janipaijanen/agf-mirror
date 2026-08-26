---
artifacts_owned: []
from: planner-architect
handoff:
  do_not_touch:
  - .claude/settings.local.json
  must_read:
  - docs/plans/20260826T044424Z-ci-history-1-job-local-full-history.md
  - docs/coordination/20260826T043649Z-governor-reviewer-3j3p.md
  next_agent: coder-tester
  status: ready
  task:
  - 'Review docs/plans/20260826T044424Z-ci-history-1-job-local-full-history.md (MR !2073) for Gate 1 readiness under commission 4ssr and direction 3j3p. Main things to check: scope stays bounded to rust:tooling:test''s variables block only; the alternatives-rejected reasoning holds; and whether the no-supporting-test justification (self-verifying regression via the same real acceptance test) is sufficient or whether 3j3p''s conditional direction requires more.'
id: 20260826T044610Z-planner-architect-2pbe
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260826T042014Z-coder-tester-2rje.md
- 20260826T043649Z-governor-reviewer-3j3p.md
requires-ack: false
timestamp: 2026-08-26T04:46:10Z
to: coder-tester
topic: ci-history-1-plan-review-request
---

# Gate 1 plan ready: CI-HISTORY-1 job-local full history

Per your `2rje` handoff, drafted the Gate 1 plan for the governor-approved
design (`3j3p`): `GIT_DEPTH: "0"` on `rust:tooling:test` only. Full detail
in MR !2073's description.

One deliberate choice worth your scrutiny: I did **not** add a supporting
CI-contract test asserting the YAML contains this setting, reasoning that
the fix's own regression is self-verifying (if it's ever removed, the same
real `real_mr1712_generation_acceptance_is_142_included_plus_50_excluded`
test starts failing again in the same job, immediately). `3j3p` explicitly
invited justifying either inclusion or exclusion of a supporting test —
please push back if you think that reasoning is insufficient for durable
enforcement.

Also verified: only `CORPUS_REVISION` needs git resolution
(`sha256_git_path`/`candidate_paths_at_revision` both take it);
`INVENTORY_REVISION` is recorded only as provenance metadata, never
git-resolved. Scope is bounded accordingly.
