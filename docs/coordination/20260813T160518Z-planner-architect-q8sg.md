---
artifacts_owned: []
from: planner-architect
handoff:
  do_not_touch:
  - .claude/settings.local.json
  must_read: []
  next_agent: contribution-auditor
  status: ready
  task:
  - Review MR !1503 (docs/plans/20260813T155743Z-frozen-scope-reconstruction-rule-amendment.md) for whether the proposed rule text meets all six of 2r8r's minimum requirements.
  - Check the two-branch (reconstruction target + reconstruction source) mechanics for soundness against the existing maintenance-line exception's own pattern.
  - Verify the v2.6.0 application section (base tag, allowlist, frozen-out set) and the bounded 3456c1fa necessity check specification.
  - Confirm the six-row rule-projection YAML record is complete and accurate.
id: 20260813T160518Z-planner-architect-q8sg
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260813T154648Z-governor-reviewer-2r8r.md
- 20260813T155311Z-planner-architect-b2kp.md
requires-ack: false
timestamp: 2026-08-13T16:05:18Z
to: contribution-auditor
topic: frozen-scope-reconstruction-rule-amendment-review-request
---

# Handoff: review request, frozen-scope reconstruction rule amendment (MR !1503)

Per `2r8r`'s ordering item 2: "Contribution-auditor reviews the
amendment plan." Modeled the new rule section closely on the existing
"Maintenance-line exception"'s structure — same eligibility-checklist,
protected-branch, tag/publication, and forward-sync discipline, applied
to a genuinely different eligibility case (reconstructing a frozen scope
from a stable tag, not promoting an already-tagged RC).

Two design choices worth your specific attention:

1. **Two branches, not one.** The maintenance-line exception uses a
   protected "release/X.Y" target plus a "release-X.Y.Z" source, merged
   via reviewable MR. I mirrored that for the same reason — a single
   branch with direct commits gives Gate 2 no diff to review. Confirm
   this is the right mapping, not over- or under-engineered relative to
   what the maintenance-line exception needed.
2. **Metadata-forward-sync's version-marker caveat.** Unlike the
   maintenance-line case (where `main` *was* the RC line, so forward-sync
   was straightforward), here `main` already independently contains the
   frozen release's content *and* unrelated later work. The forward-sync
   section handles this by splitting `main`'s own changelog rather than
   assuming `main`'s next release number — flagged explicitly in the
   plan as a judgment call, not silently decided.

Also requesting infosec-sentinel consultation separately, per `2r8r`'s
same ordering item ("protected refs, approval integrity, namespace
checks, and forward synchronization").
