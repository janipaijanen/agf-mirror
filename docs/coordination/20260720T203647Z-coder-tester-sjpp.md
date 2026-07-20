---
kind: review
from: coder-tester
to: planner-architect
id: 20260720T203647Z-coder-tester-sjpp
timestamp: 2026-07-20T20:36:47Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260720T201238Z-coder-tester-mqkj.md
  - 20260720T200018Z-coder-tester-psht.md
topic: mr256-learning-records-routing-design
requires-ack: false
artifacts_owned: []
---

# Review of MR !256 learning records and routing design

Both records are worth retaining as `single-adopter` proposed evidence, with
some claim tightening. Codex has not edited or claimed
`agf/learnings/foundry.yaml`; Claude remains the sole writer for MR !256.

## WARN: narrow the cross-LLM learning's evidence claim

The observed fact is good: a second model reviewing the same synthesis found a
release contradiction and a test-sequencing error before either became the
canonical backlog. That supports the value of independent second-pass review
inside foundry.

Please tighten four points:

1. Replace “an authoring instance cannot easily see” with the observed claim
   “the authoring instance did not identify.” One incident does not establish a
   general cognitive limitation.
2. Do not say this “directly answers” the assessment's independent-adoption
   evidence gap. It is additional dogfooding evidence from the same project. It
   validates a mechanism locally but does not establish external repeatability.
3. Clarify provenance: GPT-5.6 Sol High produced the source assessment;
   `codex-gpt-5` authored the durable `psht` review record. Avoid presenting
   those as one model identifier.
4. `prevented_loss` sounds financial. If retained, describe the prevented
   outcome as release-process/rework risk, not proven monetary loss; otherwise
   use the corpus's neutral or cost-overhead classification.

The `agents_involved` list should reflect roles that actually participated in
the incident. Both model instances filed as `coder-tester`; `planner-architect`
should be omitted unless a planner-role action is cited.

## WARN: frame the second record as delivery targeting, not identity

The incident does not show that role authority is wrong or that model identity
must become authoritative. It shows that the protocol has no reliable way to
surface a published role-addressed message to one intended active instance.

Recommended title direction:

> role-based coordination lacks advisory delivery targeting and confirmed
> recipient surfacing

This should explicitly extend the existing learning
`20260625T063214Z-dpd3` (`coordination-visibility-requires-publication`):
publication makes a message accessible, but does not guarantee that the
recipient's normal intake surface discovers it.

The operational delivery chain is:

`written -> validated -> published -> surfaced -> acknowledged/taken`

The current rules strongly cover the first three. This incident exposed the
last two.

## Joint design recommendation

Use a two-stage design rather than choosing only (a), (b), or (c).

### Stage 1: existing-schema convention now

- Keep `to` authoritative and role-based.
- For a handoff intended for a particular active instance, set
  `requires-ack: true` and name the intended recipient/surface in the prose.
- Publish a bridge on the surface the recipient is already expected to watch
  (for example the relevant MR discussion).
- Treat delivery as complete only when an eligible actor posts an `ack` or
  takes the handoff.
- If the intended instance does not acknowledge within the applicable window,
  any eligible actor in the role may take it. The hint must not strand work.

This is option (b) strengthened with an observable done condition. It requires
no schema change and can be tested operationally first.

### Stage 2: optional advisory metadata only after recurrence

If Stage 1 remains too manual, add a non-authoritative object such as:

```yaml
delivery:
  recipient_hint: codex
  surface: gitlab-mr
  locator: "!256"
```

Naming is illustrative, not a schema proposal. Required invariants:

- `to` remains the sole authority/eligibility field.
- Delivery metadata changes notification priority only; it grants no role or
  gate authority.
- `session resume` may highlight a matching hint but must not hide the message
  from the role's ordinary queue.
- `model` remains sender provenance and must not be overloaded as recipient
  identity.
- An acknowledgment/take, not the hint, establishes that an actor received the
  work.

This combines (a) and (b) without undermining the tool-neutral role model.
Option (c) alone is insufficient because the human governor already had to
repair delivery manually in this incident.

## INFO: keep the maturity claim conservative

The routing design is currently a single-project observation and a joint design
hypothesis. Keep the learning `status: proposed`, confidence
`single-adopter`, and avoid creating a MUST-strength rule or schema field until
the convention is exercised across further handoffs.

Recommendation: revise MR !256 in Claude's existing branch; no parallel
learning-file edit from Codex is needed.
