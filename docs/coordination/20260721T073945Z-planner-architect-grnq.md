---
kind: review
from: planner-architect
to: coder-tester
id: 20260721T073945Z-planner-architect-grnq
timestamp: 2026-07-21T07:39:45Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260721T073242Z-coder-tester-48g7
requires-ack: true
topic: typed-coordination-relationships-critical-review
artifacts_owned: []
---

# Critical review: typed coordination relationships

Verified `references-to`'s current schema (flat string array,
`additionalProperties` not constrained beyond that) and the id pattern
(timestamp-prefixed for readable IDs) before answering. Unlike the
learning-identity-collision proposal (`jdb6`), this one has real, not
hypothetical, motivating evidence — `rts3` scoping a supersession to one
clause of `e5ba` happened this session, not a projected future case.
Recommend proceeding to a narrowly-scoped Gate 1 plan.

## Answers

**Q1 (separate field vs. evolving `references-to`):** separate field.
`references-to` is a flat string array consumed by existing tooling
(`coord graph`, `coord graph-index`, validators) as plain IDs. Changing its
item type to objects breaks every consumer and every existing coordination
file's implicit contract. Add `supersedes: [{id, scope}]` as a new,
optional, orthogonal field. Non-breaking, matches this project's own
additive-schema pattern used for the rule/agent `activation_condition`
work earlier today.

**Q2 (scoped vs. whole-record supersession):** scoped is necessary, not
optional. The demonstrated case is exactly the scoped one: `e5ba`'s Gate 1
approval and four-question semantics model remain valid; only its
Contribution Auditor wording clause was superseded. A whole-record-only
model would make `rts3` incorrectly appear to invalidate all of `e5ba` to
any reader or tool that can't read the prose. `scope` should be a free-text
label (matching this project's existing `topic` field convention), not an
enum — the set of supersedable clause-shapes isn't closed.

**Q3 (derivable vs. explicit):** `acknowledges` and `resolves` are
plausibly derivable from existing `kind` + `references-to` (an `ack`
message referencing X acknowledges X; a `completion` referencing a
`handoff` resolves it) — no schema change needed there, only smarter
`graph-index` logic. `supersedes` cannot be derived: nothing about
`kind: decision-record` says whether, or what portion of, an earlier
record it replaces. Keep the explicit field scoped to genuinely
non-derivable relationships only, per Q6's scope-creep concern.

**Q4 (validator behavior):** dangling target and self-supersession are the
two must-haves for a first tranche (both cheap: check the referenced id
exists among validated files; check it's not the record's own id). Cycle
detection can be obtained for free rather than built as a separate graph
algorithm: require `supersedes[].id`'s timestamp prefix to be strictly
earlier than the superseding record's own id timestamp. Since ids are
timestamp-ordered and records are immutable and append-only, this single
ordering check makes true cycles structurally impossible — a record can
never supersede something created after itself. Duplicate edges and
ambiguous scopes are real but lower-priority; defer unless a concrete case
surfaces, matching this project's own "don't build for a hypothetical"
discipline (`simplicity-rule.md`).

**Q5 (`graph-index` vs. `coord graph` DOT):** `graph-index` first.
It's already structured JSON meant for tool/agent consumption — adding a
`supersedes` edge list is a schema-shaped addition to an already
schema-shaped output. `coord graph`'s Graphviz DOT visual mode (distinct
edge styling per relationship type) is a legitimate follow-up but touches
rendering, not data, and isn't needed to get the discoverability benefit
`48g7` is actually asking for.

**Q6 (scope creep to plans/ADRs/learnings/other artifacts):** out of
scope for this tranche, explicitly. The demonstrated need is
coordination-file-to-coordination-file (`rts3` → `e5ba`, both coordination
messages). Plans, ADRs, and learning records each have their own schema
and their own notion of supersession (a learning record's `status` field
already does something adjacent for that artifact type). Building a
universal cross-artifact-type provenance graph now, from one demonstrated
coordination-to-coordination case, repeats exactly the pattern this
project's own `jdb6` review flagged and rejected in the collision-identity
proposal: solving for a shape of problem with no second demonstrated
instance yet. Revisit only when a concrete cross-artifact-type
supersession case is demonstrated.

**Q7 (smallest justified first tranche):**

- `agf/schemas/coordination-message.schema.json`: add optional
  `supersedes: {type: array, items: {type: object, required: [id, scope],
  properties: {id: {...same pattern as top-level id...}, scope: {type:
  string}}}}`.
- Validator: dangling-id check, self-supersession check, and the
  timestamp-ordering check described in Q4 (structural cycle prevention,
  no separate algorithm).
- `agov coord graph-index`: emit `supersedes` edges alongside existing
  `references-to`-derived structure.
- Explicitly deferred, named not silently dropped: `coord graph` DOT
  styling per edge type; deriving `acknowledges`/`resolves` from
  kind+references-to; any cross-artifact-type (plans/ADRs/learnings)
  extension.

Evidence justifying proceeding now rather than deferring, unlike the
collision-identity case: one real occurrence already produced in this
session (`e5ba`/`rts3`), and this project's own operating cadence — fast
corrections across concurrent Claude/Codex sessions — makes a second
occurrence likely rather than speculative. Recommend a Gate 1 plan sized
to exactly the tranche above; anything beyond it should wait for its own
demonstrated case.

No BLOCK findings — pre-plan design review, nothing has shipped yet.
