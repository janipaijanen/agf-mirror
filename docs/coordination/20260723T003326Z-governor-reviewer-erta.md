---
kind: handoff
from: governor-reviewer
to: coder-tester
id: 20260723T003326Z-governor-reviewer-erta
timestamp: 2026-07-23T00:33:26Z
model: human-jani
immutable: true
references-to:
  - 20260723T003222Z-governor-reviewer-31mq.md
  - 20260723T002811Z-governor-reviewer-5stq.md
topic: v240-workstream-d-class5-start
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Acknowledge and claim Workstream D Class 5 from current main merge commit c7eb0cc.
    - Implement the approved restrained responsibility-labelled text-block standardization across the public MkDocs site.
    - Publish a Gate 2 handoff with exact-SHA pipeline, grep/count, rendered-site, readability, and scope evidence.
  must_read:
    - docs/plans/v2.4.0-workstream-d-figure-system.md
    - agf/docs/site/reference/palette.md
    - docs/coordination/20260723T003222Z-governor-reviewer-31mq.md
    - docs/coordination/20260723T002811Z-governor-reviewer-5stq.md
  do_not_touch:
    - agf/framework.md
    - agf/docs/agent-communication-0.26.0.md
    - docs/academic/
    - .gitlab-ci.yml
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Release handoff: begin Workstream D Class 5

MR !429 merged to `main` as
`c7eb0ccab06c4ac81c06b42323ba448e4de0da3e`. Class 4 is complete for this
release. Class 5 is now the next release-critical Workstream D slice and takes
priority over taxonomy, learning, operating-efficiency, and build-system
assessment work whenever actionable.

## Approved scope

Extend the existing public-site responsibility-label pattern using the five
approved exact labels:

- Human decision
- AI action
- Automated check
- Shared concept
- Reference

Reconcile `AI-operator instructions` to `AI action`. Add `Automated check` only
where responsibility changes or ambiguity is likely. Keep the treatment
restrained; do not blanket-label ordinary prose merely to increase coverage.

## Acceptance evidence

- Start from exact current `origin/main` merge commit `c7eb0cc`.
- Inventory existing label variants before editing and list every touched file.
- Grep confirms the five canonical labels are consistent and no stale
  `AI-operator instructions` wording remains in the approved public-site scope.
- A count and manual spot check demonstrate labels remain low-volume and
  responsibility-relevant rather than wrapping ordinary prose.
- Strict MkDocs build and relevant validators pass.
- Rendered pages are checked at desktop and narrow width for layout and
  non-native-English readability; this text-block slice must not claim the
  deferred Class 4 diagram accessibility work is fixed.
- Gate 2 handoff enumerates every pipeline for the exact implementation SHA,
  with ID, source, and status, including any failure or duplicate graph.

## CI-cost boundary

Create the skipped seed and MR boundary before the first real implementation
push. Batch the complete Class 5 slice locally and use one ordinary acceptance
push. Do not fragment the class into per-page paid pipelines, and do not use
`ci.skip` for implementation acceptance evidence.

After Class 5 clears and merges, return to the v2.4.0 release checklist. The
post-release mobile/screen-reader obligation from `5stq` remains separate and
must not expand this release slice.
