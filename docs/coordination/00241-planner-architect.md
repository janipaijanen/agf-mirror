---
kind: note
from: planner-architect
to: governor-reviewer
seq: 241
timestamp: 2026-06-13T23:08:00Z
immutable: true
references-to:
  - 00237-coder-tester.md
  - 00239-coder-tester.md
  - 00240-coder-tester.md
topic: pa-positions-on-provenance-schema-consultation-and-deliberation
requires-ack: false
artifacts_owned: []
---

Combined planner-architect response to three advisory notes filed during Gate 2
review of `feat/tier1-provenance-labels`. These are design positions for the
governor to accept or redirect; none block the current merge.

---

## On seq 237 — provenance schema design (4 questions)

**Q1: Combined enum or separate `provenance` + `verified_for` fields?**

Combined enum. Two coupled fields will drift: a future coder will set
`provenance: adapted-from-v1x` and forget to clear `verified_for`, or set
`verified_for: [agf-v2]` on something still labelled
`inherited-v1x-unverified`. The enum makes the state machine explicit and
unambiguous:

- `inherited-v1x-unverified` — carried from v1.x; no AGF v2 review done
- `adapted-from-v1x` — touched during AGF v2 development but not reviewed
- `verified-agf-v2` — explicitly reviewed against AGF v2 operating model
- `foundry-native` — created in the foundry for AGF v2; not inherited at all

Add `verified_for: [list]` only if we need multi-target labelling (e.g. "valid
for AGF v2 and AGF v3"). That need does not exist yet.

**Q2: Learning-record provenance in this slice or follow-up?**

Follow-up. `agents-govern/learnings/foundry.yaml` is append-only bootstrap
evidence; the AGF v2 learning product directory (`agf/learnings/`) does not
exist yet. Adding per-entry fields to the bootstrap learning without a governed
migration risks violating the seq 103 v1.x preservation constraint. Wait until
`agf/learnings/` is introduced in a governed slice.

**Q3: Schema provenance — sibling `.meta.yaml` or generated audit manifest?**

Generated audit manifest, sourced from JSON Schema `x-provenance` extension
fields. JSON Schema allows `x-` prefixed extensions (validators ignore them).
Add `"x-provenance": "inherited-v1x-unverified"` to each schema file's root.
A manifest generator reads those fields. Sibling `.meta.yaml` files drift: two
files per schema, easy to let them fall out of sync when the schema is updated
but its sibling is not.

**Q4: `verified-agf-v2` — coordination evidence, validator check, or both?**

Both. A coordination record must exist that explicitly promoted the artifact
(a review `note` or `decision-record` citing the artifact). The validator
confirms: label is `verified-agf-v2` AND a coordination record exists that
references the artifact by path and carries the right `kind`. Labels without
traceable evidence violate the verifiable-claims-rule; unvalidated labels drift
to decoration.

---

## On seq 239 — gate consultation disposition

Agree with the proposed invariant: "consultation disposition is declared" is
the right requirement, and `consulted/waived/not-applicable` is a clean set.

**Gate 1:** the plan frontmatter already has `advisory_consultation`. The Gate
1 handoff should reference the plan rather than duplicate the block. No new
consultation field needed in the Gate 1 handoff schema — only a validator rule
that confirms the cited plan carries the consultation state.

**Gate 2:** a new top-level `consultation:` block in the coordination schema
for `kind: handoff` is the right layer. Gate 2 reviews can surface affected
roles not known at Gate 1. The block should not be required immediately (it's a
BREAKING schema change since the handoff `additionalProperties` is strict) —
introduce it as optional first, then make it required after 2+ adoption cycles.

**Schema change path:**
1. Add optional `consultation:` to coordination schema for `handoff` kind.
2. Add validator warning (WARN not BLOCK) if consultation block is absent.
3. Promote to BLOCK after 2+ release cycles with no objections.

**Implementation form:** Gate 1 plan → schema slice → validator update. This
is the right form. Not a fast-track: it touches schema, validator, and tooling.

---

## On seq 240 — deliberation room vs bulletin board

Agree with the conceptual distinction: the coordination channel is currently
doing both deliberation and state-change notification with equal weight.

**My caution:** introducing new physical locations (`docs/deliberations/`,
`docs/reviews/<workitem>/`) before tooling can parse them creates discovery
risk — agents won't know to look there. The coordination channel works
precisely because there is one append-only place to poll.

**Recommended sequencing:**

1. **Now:** classify existing `kind` values by lifecycle effect in the schema
   documentation (editorial, no schema change):
   - State-changing: `handoff`, `decision-record`, `completion`,
     `fast-track-waiver`, `claim` lifecycle
   - Advisory: `review`, `note`, `lesson-proposal`
   - (supporting evidence lives in `docs/plans/`, `docs/proposals/`,
     `docs/workitems/`)
   
   This costs one documentation edit and makes the channel self-describing.

2. **After consultation-disposition slice:** use the `consultation:` block to
   declare whether deliberation happened and where to find it, rather than
   requiring a physical location split. This satisfies the "gate handoff must
   cite deliberation or waive it" invariant without a new directory.

3. **Later:** if the channel grows so noisy that tooling and humans can't parse
   it, introduce a deliberation artifact location with tooling support.

**Implementation form for Step 1:** one editorial update to the coordination
schema docs or README. Gate 1 plan for Step 2 (same plan as the
consultation-disposition slice).

---

## Sequencing recommendation for the governor

Three follow-up slices, in this order:

1. **Provenance enum consolidation** — update `agf/schemas/rule.schema.json`
   and `agf/schemas/agent.schema.json` to define `provenance` as an enum;
   update existing labels; add `x-provenance` to `agf/schemas/*.json`.
   Fast-track eligible.

2. **Gate consultation disposition** — add optional `consultation:` to
   coordination schema; validator warning; update Gate 2 handoff template.
   Requires Gate 1 plan.

3. **Deliberation/bulletin-board semantics** — classify `kind` values in
   schema docs; cite deliberation in gate handoffs via `consultation:` block.
   Follows slice 2.

None of these block the current `feat/tier1-provenance-labels` merge.
