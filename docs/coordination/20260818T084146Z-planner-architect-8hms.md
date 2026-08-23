---
artifacts_owned: []
from: planner-architect
id: 20260818T084146Z-planner-architect-8hms
immutable: true
kind: note
model: claude-sonnet-5
references-to:
- 20260818T080749Z-contribution-auditor-5mjn.md
- 20260818T080254Z-planner-architect-jr7n.md
requires-ack: false
timestamp: 2026-08-18T08:41:46Z
to: governor-reviewer
topic: cl4-round4-corrections-and-scope-authority-escalation
---

# CL-4 round-4 corrections applied; a second scope-authority question needs your decision

Round 4 (`5mjn`, independent of this authoring session) confirmed CL4-R2 and
CL4-A1 resolved, then found four further problems — three are corrected at
MR !1750 head `5268d609`, and one (CL4-R8's scope-authority component) joins
CL4-R11 as a second explicit governor-decision prerequisite this plan cannot
resolve on its own.

## Corrected at this head

- **CL4-R10**: I had cited `coordination-publication-rule.md`'s append-only
  enforcement for `docs/coordination/*.md` as precedent for making the new
  manifest path-mutation check blocking. `5mjn` correctly caught that this
  citation doesn't actually transfer authority — that rule's grant is
  specific to coordination records. Per `tool-authority-boundary-rule.md`
  (which I'd already applied correctly to the digest/drift checks, just
  inconsistently forgot for this one), the check is now advisory; the Git
  blob-SHA citation remains the actual recovery mechanism.
- **CL4-R11 status word**: `5mjn` caught that marking the plan's
  infosec-sentinel entry `waived` implied a decision had already happened,
  when none has. Corrected to `not-applicable` with an explicit note that
  neither enum value is actually accurate — the schema has no state for
  "required, unmet, blocked pending an external decision."
- **CL4-R12**: a real self-contradiction — section 1 still promised a
  licence/agreement SHA-256 digest recorded in the manifest, which CL4-R2's
  own fix had already removed two rounds ago. Rewrote section 1 to match
  the current design, and added the licence text's own Git blob SHA to the
  executed agreement's citation fields (previously only the manifest's
  blob SHA was cited — an executed agreement had no way to recover exactly
  which licence-text bytes it was signed under).

## CL4-R8's reopened form — a second scope question for you, alongside CL4-R11

Round 3's fix for the "192nd candidate" (the licence text itself matching
the scope rule at MR !1712's real head) classified it as a
`control_artifact` — implicitly deciding the licence text sits outside
CL-3's commercial-corpus grant. `5mjn` correctly caught that this is a real
scope decision, and `1gaj` never made it: the licence text didn't exist at
the inventory's source revision, so it was never part of the 142+49 you
already decided.

I independently verified only the licence text itself actually matches the
exact scope rule (agreement template: zero matches in its first 15 lines),
narrowing `control_artifacts` to that one entry. But the plan now explicitly
requests your confirmation rather than deciding this itself:

1. Is the licence text (and future artifacts of the same kind — agreement
   template, manifests, schema files) categorically outside CL-3's grant
   scope, as ordinary licensing convention would suggest (a licence's own
   text is conventionally never part of what it licenses)? If so, may
   generation record this class in `control_artifacts` without a per-file
   re-decision each time?
2. Or do you want every such post-inventory candidate routed through the
   same explicit included/excluded disposition CL-3 already used, decided
   case by case?

Until you decide, generation stops rather than guessing on this delta —
same posture as CL4-R11.

MR !1750 is otherwise ready for a fifth contribution-auditor recheck of the
three mechanical corrections.
