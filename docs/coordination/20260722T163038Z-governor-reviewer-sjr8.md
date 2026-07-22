---
kind: handoff
from: governor-reviewer
to: coder-tester
id: 20260722T163038Z-governor-reviewer-sjr8
timestamp: 2026-07-22T16:30:38Z
model: human-governor
immutable: true
references-to:
  - 20260722T163028Z-governor-reviewer-3ktr
  - 20260722T153006Z-planner-architect-p5rk
topic: dossier-authoritative-references-implementation
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - After MR !328 and the approved !401 plan are merged, branch from current main and implement the plan test-first.
    - Preserve MR !328 wording and add link-only, exact-build repository references without reading or copying referenced artifact bodies.
    - Route an exact tested head for Gate 2 with rendered link, anchor, inventory, and no-copied-content evidence.
  must_read:
    - docs/plans/dossier-authoritative-references.md
    - docs/coordination/20260722T163028Z-governor-reviewer-3ktr.md
    - MR !328
    - MR !401
  do_not_touch:
    - v2.4.0 release scope, version, tag, or schedule
    - referenced repository artifact bodies
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Implementation handoff: dossier authoritative reference layer

Claude, acting as `coder-tester`, may implement the approved !401 plan after
MR !328 and the plan itself are merged to `main`. Begin from that resulting
current `main`; do not reuse a pre-!328 implementation base.

The implementation is link-only for repository artifacts. Preserve the full
existing public-documentation corpus, add the direct Evaluate-to-dossier route,
generate exact-build links with an explicitly unpinned fallback, and keep the
copied Evaluate self-link inside the dossier valid. Do not read, copy,
summarize, or embed the referenced repository artifact bodies.

This is out-of-release work. It is not part of v2.4.0 and must not delay or
change that release. Route the implementation independently for Gate 2.
