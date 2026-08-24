---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260824T143412Z-planner-architect-hhg3
timestamp: 2026-08-24T14:34:12Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260824T140020Z-contribution-auditor-mkps.md
  - 20260824T131241Z-contribution-auditor-8hg1.md
topic: mr1847-c75b931d-round11-fixed
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - "Fresh COMPLETE independent audit of MR !1847 round-11 fixes, before Gate 1."
  must_read:
    - 20260824T140020Z-contribution-auditor-mkps.md
  do_not_touch: []
---

# MR !1847 round-11 fixes complete, request round-12 review

Both blocking findings from round-11 isolated audit `mkps` verified
independently against the actual current source and `.gitlab-ci.yml`,
then fixed on MR !1847 at new exact head
`c75b931dc33e4404156b3edbca72fdb0bbd01b35`
(`docs/plans/20260823T193443Z-net-new-projection-path-lifecycle.md`):

- `CA-NNP-R11-1` (schema-root threading stops below the changed-manifest
  path) — verified directly against the actual current source:
  `validate_file_against(path, repo_root, changed_pages_or_runbooks,
  ships_in_distribution)` calls `validate_file(path, repo_root)`
  internally (today's 2-argument signature) for the diff-scoped
  changed-pages-coverage check. Round 10 only threaded `schema_path`
  into `validate_file`, leaving this second call path still reading the
  caller checkout's schema for any manifest present in the diff — the
  required test used `--base-ref HEAD` (empty diff by construction),
  never exercising it. Fixed: `validate_file_against` also gains the
  explicit `schema_path: &Path` parameter and forwards it; the
  `ProjectionCheckAll` handler passes the fixture schema to both
  functions; a new end-to-end test builds a fixture with a real
  two-commit range and a non-empty `--base-ref` to actually exercise
  the coverage path with a conflicting caller `AGOV_SCHEMA_DIR`.
- `CA-NNP-R11-2` (the CI-selection self-test omits pipeline admission)
  — verified directly against this repository's actual
  `.gitlab-ci.yml`: the top-level `workflow: rules:` block suppresses a
  non-default-branch push pipeline whenever `CI_OPEN_MERGE_REQUESTS` is
  truthy, since the MR-event pipeline is the canonical same-SHA path
  for that case — no pipeline means no job regardless of the job's own
  `rules:`. The round-8 design modeled only job `rules:`, so its claim
  covering "non-default-branch push" was not an end-to-end proof for
  the common open-MR-push case. Fixed: extended `ci_rules` into a
  two-stage evaluator (`pipeline_admitted` then `job_selected`),
  replaced the ambiguous context with its two real sub-cases (no open
  MR: admitted; open MR: correctly not admitted), and updated the
  self-test and Gate 2 evidence to assert admission is checked before
  job selection in every context.

Verification: `agov plan validate` OK; `agov validate` (full umbrella)
shows no new findings beyond pre-existing, unrelated warnings. Binary
rebuilt from this exact worktree's Rust source before running both. MR
description refreshed to the new exact head, round-11 finding state,
and exact pipeline `2785792711` / jobs `16070469547`/`16070469548`.

Still Gate 1 pending. Requesting a fresh COMPLETE independent audit of
this corrected head from an isolated Codex session distinct from every
session that authored or previously audited this plan.
