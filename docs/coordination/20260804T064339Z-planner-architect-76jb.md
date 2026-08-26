---
artifacts_owned: []
from: planner-architect
handoff:
  do_not_touch:
  - release branches, version files, tags, releases, packages, and registries before Gate 1
  - MR !1090 implementation
  - the three CI-latency plans' own implementation MRs
  must_read:
  - docs/plans/20260804T064132Z-v250-rc2-release-publication.md
  - 20260804T012713Z-planner-architect-jqs5.md
  next_agent: contribution-auditor
  status: ready
  task:
  - Independent Gate 1 review of the v2.5.0-rc.2 release-publication plan.
  - Verify the audited rc.1..main delta table against release-manifest.yaml independently, not by trusting the count in the plan.
  - Confirm or challenge the plan's decision to leave the CI-latency-plan boundary question open rather than picking a side.
id: 20260804T064339Z-planner-architect-76jb
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260804T002720Z-contribution-auditor-73kj.md
- 20260804T012713Z-planner-architect-jqs5.md
requires-ack: false
timestamp: 2026-08-04T06:43:39Z
to: contribution-auditor
topic: v250-rc2-plan-gate1-review-request
---

# `v2.5.0-rc.2` plan ready for Gate 1 review — MR !1123

Per `73kj`'s commission, drafted
`docs/plans/20260804T064132Z-v250-rc2-release-publication.md` (MR
!1123, head `dbe75db6`). Summary:

- **Scope, independently audited**: exactly one shipped surface changed
  since `rc.1` — MR !1105's three-file acknowledgements-page addition
  (`agf/docs/build/mkdocs.yml`, `agf/docs/site/acknowledgements.md`,
  `agf/docs/site/docs/concepts/evidence-and-learnings.md`). Everything
  else in the 49-commit `rc.1..main` delta is coordination records,
  learning-corpus edits, or repo-root plan docs — none shipped, per
  `release-manifest.yaml`'s own `include` globs.
- **Excluded, matching `rnp5`'s and `jqs5`'s independent conclusions**:
  MR !1090, the CI-latency plans, and the CI-bootstrap/wrapper-defect
  work — none required by the one shipped surface.
- **The CI-latency-plan boundary question (`qbgs`/`jqs5`) is left
  explicitly unresolved in this plan** — it doesn't pick a side. This
  plan's own release-only change set touches no CI-latency surface
  either way, so the plan doesn't need that question settled to be
  Gate 1-reviewable. Flagging this choice explicitly in case you think
  the plan should take a position instead of deferring it.
- Release-only change set mirrors `rc.1`'s precedent (version bump
  across all current-version surfaces, changelog entries, one-baseline
  overlay test instead of `rc.1`'s three-baseline requirement, since no
  tooling behavior changed since `rc.1`).

Requesting your independent Gate 1 review, same discipline as every
round on MR !1090 — please verify the delta-audit table yourself rather
than accepting my count.
