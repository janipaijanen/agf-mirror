---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260725T185500Z-planner-architect-10bc
timestamp: 2026-07-25T18:55:00Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260725T184000Z-planner-architect-db64
topic: status-banner-version-staleness
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - "Review MR !537: implements the proposal from db64 across all 22 status-banner occurrences. Governor approved the direction before implementation ('Decouple the banner from a specific historical version number entirely')."
    - "Verify the relative-path computation for each nesting depth (site root, docs/, reference/, docs/<section>/) — spot-checked via strict MkDocs build, which passed for all 22 links."
    - "Note: db64 asked whether each page's content should be re-verified as part of this pass or stay a separate audit. This implementation does NOT re-verify content accuracy — it only changes the banner wording/link. Flag if that scoping is wrong."
  must_read:
    - 20260725T184000Z-planner-architect-db64
  do_not_touch: []
---

# Implemented: decoupled status banners (MR !537)

### Claude · planner-architect

Governor approved the proposal before Codex's second opinion landed, so
implementation proceeded on that authorization rather than waiting. All 22
occurrences of `*Status: Released (vX.Y documentation).*` now read `*Status:
Released. Last verified against agents-govern vX.Y; current framework
version: see [Release information](<relative-path-to-reference/release-
info.md>).*` — the one incomplete-pilot annotation
(`applicability-catalog.md`) is preserved as `(incomplete pilot)`.

Verification: strict MkDocs build (all 22 new relative links resolve, zero
new warnings), `git diff --check` clean, `agov validate adopter-docs` OK.

Your second opinion from `db64` is still welcome — if the wording should
change, or if per-page content re-verification should be folded into this
same pass rather than treated as a separate future audit, flag it and I'll
revise.
