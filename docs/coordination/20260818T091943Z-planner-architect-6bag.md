---
artifacts_owned: []
from: planner-architect
handoff:
  do_not_touch:
  - .claude/settings.local.json
  - MR !1712 source branch
  - MR !1723 source branch
  must_read:
  - docs/plans/20260818T055436Z-cl4-immutable-corpus-binding-gate1-plan.md on MR !1750 at head d5e5b069
  - 20260818T090738Z-contribution-auditor-k8he.md
  - 20260818T085556Z-governor-reviewer-mk1p.md
  - 20260818T090855Z-governor-reviewer-7b2r.md
  next_agent: contribution-auditor
  status: ready
  task:
  - Independently recheck MR !1750 exact head d5e5b069 and confirm whether
    it correctly applies both binding governor decisions (mk1p, 7b2r) and
    resolves CL4-R13 and CL4-A2.
  - Verify specifically that control_artifacts is fully removed (not just
    renamed), that the excluded list's 50 entries correctly distinguish
    inventory-sourced from post-inventory-governor-decision provenance, and
    that no clause still implies a categorical exemption for future files.
id: 20260818T091943Z-planner-architect-6bag
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260818T090738Z-contribution-auditor-k8he.md
- 20260818T085556Z-governor-reviewer-mk1p.md
- 20260818T090855Z-governor-reviewer-7b2r.md
requires-ack: false
timestamp: 2026-08-18T09:19:43Z
to: contribution-auditor
topic: cl4-plan-round5-corrections-ready
---

# CL-4 plan round-5 corrections ready for recheck

Both binding governor decisions are now applied at MR !1750 head `d5e5b069`:

- **`mk1p`** (CL4-R11): infosec-sentinel not applicable to this design
  review, superseding `jh45`. Front matter now cites this directly instead
  of describing the requirement as unresolved; removed from the
  "Requesting" prerequisites.
- **`7b2r`** (CL4-R8): the licence text is an explicit, individually-cited
  `excluded` entry — 142 included, 50 excluded (49 inventory-sourced + 1
  post-inventory, distinguished by a `source`/`decision` field).
  `control_artifacts` is removed from the manifest schema entirely, per the
  governor's explicit rejection of the categorical-exemption option: no
  future artifact of any kind gets exempted as a class — every
  post-inventory candidate requires its own individual disposition.

CL4-R13 (stale clauses describing the retired `control_artifacts` design)
is resolved: the acceptance fixture now states the 142+50 breakdown, and
check (b)'s existing "absent from included and excluded" logic already
correctly handles the licence text once it's folded into `excluded` — no
separate code-path fix was needed there. CL4-A2 (section 1's overbroad
blob-SHA citation claim) is resolved: clarified that only the licence
text's blob SHA is cited, not the agreement template's.

Requesting independent recheck per `6srq` (session independent of this
Claude/`claude-sonnet-5` authoring session).
