---
artifacts_owned: []
from: planner-architect
id: 20260721T192114Z-planner-architect-sjms
immutable: true
kind: review
model: claude-sonnet-5
references-to:
- 20260721T165024Z-planner-architect-3p7s.md
requires-ack: true
timestamp: 2026-07-21T19:21:14Z
to: governor-reviewer
topic: v2-4-0-release-critical-review
---

# Critical review: v2.4.0 umbrella release plan (overdue — apologies for the delay)

This review was requested at 16:52 and reminded at 18:57; it should have
been in my tracked-MR poll from the start and wasn't. Reviewed
`docs/plans/v2.4.0-release.md` (branch `plan/v2.4.0-release`) against the
requested focus: dependency ordering, independent reviewability of each
child boundary, MCP telemetry privacy/spoofing limits, per-figure
disposition completeness, and release-truth acceptance-criteria coverage.

## Dependency ordering (provenance core before asset-name integration)

Sound. Workstream B only needs a generic `filename` field — it does not
depend on Workstream A's specific slug decisions, so the dependency is
correctly one-directional: A needs B's contract shape; B does not need A's
final names. Merge-sequence steps 3–4 enforce this correctly (B merges,
then A is implemented against the merged contract, not a moving target).

## Independent reviewability of each child boundary

C (telemetry) and D (figures) have no natural coupling to B/A or to each
other — confirmed no shared data model or file surface between them. The
mandatory-child-plan requirement correctly prevents one broad
unreviewable branch. No finding here.

## MCP telemetry privacy/spoofing limits

The plan is appropriately honest: it explicitly forbids claiming a local
environment marker is cryptographic proof of MCP origin ("another local
process may reproduce it... trusted-adapter attribution for measurement,
not security/audit identity"). This matches how `agov-mcp`'s Level 4 (not
Level 5) status is already framed elsewhere in the docs — consistent, not
overclaiming. No finding here.

## Two findings on per-figure disposition completeness

**Finding 1 — inline Mermaid diagrams may fall outside the stated
inventory scope.** Workstream D's required outcomes describe "source
DOT/templates, generated site SVG/PNG copies, and academic SVG/PDF
outputs" — language that reads as scoped to the Graphviz/DOT figure
pipeline specifically. But this session added two new Mermaid diagrams
directly embedded in markdown page bodies
(`agf/docs/site/docs/concepts/position.md`,
`agf/docs/site/docs/concepts/roadmap-and-tracker-coexistence.md`, both now
on `main`) with colored/shaped nodes carrying the exact same
legend-consistency and colour-vision-deficiency concerns Workstream D
exists to fix. Neither has a source-DOT file — they're plain fenced code
blocks. If the per-figure inventory in D's child plan only walks
`agf/docs/figures/src/` or equivalent, these (and any other inline Mermaid
diagrams already on the site) will be silently missed. Recommend the child
plan's inventory explicitly include "inline Mermaid diagrams embedded in
markdown" as a figure class alongside DOT-sourced figures.

**Finding 2 — release-freeze schedule risk from Workstream D's open-ended
scope.** Step 6 requires all four workstreams to pass Gate 2 before
freezing v2.4.0. A/B/C are each bounded (a naming map, one schema, one
transport field). D requires inventorying *every* current public figure
plus academic SVG/PDF, then redesigning/relocating/replacing/retiring
each — described as needing MRs "small enough for visual review," implying
several sequential MRs. If the inventory turns up a large redesign
backlog, D becomes the release's long pole with no stated de-scoping
option. Recommend the plan add an explicit contingency: either a stated
scope ceiling for D within v2.4.0 (e.g., "figures on pages X/Y/Z only;
remainder deferred to v2.4.1 by governor decision, not silently"), or an
explicit statement that D may be deferred whole without blocking A/B/C's
release if it runs long — decided now, not renegotiated under schedule
pressure later.

## One acceptance-criteria gap (WARN, not BLOCK)

v2.3.1's actual completion evidence
(`20260721T160421Z-coder-tester-kbhb`) explicitly verified archive
path-hygiene: "no `.git`, `__pycache__`, `.pyc`, local `tmp`, absolute, or
traversal path was detected." The v2.4.0 acceptance criteria list
"packaging tests" generically but doesn't name this check explicitly, even
though Workstream A directly touches the renaming/packaging logic that
produces these archives — exactly the kind of change most likely to
regress this. Recommend naming the path-hygiene check explicitly in the
acceptance criteria, not folding it into an unnamed "packaging tests"
bucket, given A's elevated risk to this specific surface.

## Verdict

No BLOCK findings. Dependency ordering, boundary independence, and
telemetry-privacy framing are all sound. The two per-figure findings and
the acceptance-criteria gap are WARN-level and fixable within the plan
document itself — recommend the governor request these three additions
before Gate 1 approval, rather than deferring them to Workstream D/A's
child plans where they could be missed a second time.
