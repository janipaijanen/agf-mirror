# Deprecation Policy: When and How Long
#
# Framework principle: Implementation Honesty + Escalation over Improvisation
# See: framework.md — "Feature documentation lifecycle" (pre-customer vs.
# post-customer deprecation governance)

**Scope:** Any public contract this project ships to external consumers —
CLI flags, MCP tool fields, schema fields, HTTP/API parameters, config
keys — that the project wants to rename, restructure, or remove.

## Why this rule exists

A deprecation notice with no real removal plan behind it is not honesty,
it is decoration. Before this rule, this project marked the coordination-
message `seq` field deprecated (`x-deprecated: true`,
`agf/schemas/coordination-message.schema.json:43-48`) on 2026-06-18 and
never defined how long "deprecated" would last — five weeks and several
minor releases later, it was still simply "kept," with no distinction
between "on a tracked path to removal" and "permanently grandfathered by
inertia." The `agov-mcp` `dir` → `coordination_dir` rename
(`docs/plans/mcp-coordination-dir-contract-fix.md`) is what surfaced this
gap: rather than repeat the `seq` pattern by adding a second
never-actually-sunset alias, this rule was written so the next deprecation
this project makes has a real answer.

## Two phases

### Phase 0 — pre-customer / no external dependents (current state)

Per `agf/learnings/foundry.yaml`, `customer_state.paying_customers:
false`. This project is in Phase 0 for essentially every surface it ships
today, including `agov-mcp` (Level 4 Validated, not yet Level 5
Deployable). While a surface has zero external, production dependents,
breaking changes MAY ship without a deprecation cycle. This is not
carelessness — it is honest: a deprecation notice on a surface nobody
depends on yet is theater, and per `framework.md`'s own "Feature
documentation lifecycle" guidance, pre-customer products should show
current state only, not manufactured migration timelines that signal
instability to evaluators.

Requirements even in Phase 0:
- The change MUST be disclosed plainly in the CHANGELOG under a `Changed`
  (breaking) heading, not folded silently into `Added`.
- The commit or plan introducing the change MUST state, in one sentence,
  why no deprecation cycle was used (typically: "zero known dependents on
  this surface today").
- The change MUST NOT be described with vague or softening language
  ("improved," "simplified") that obscures that it broke something.

### Phase 1 — the surface has real adopters

This project's Phase 1 trigger: the first surface with an independently
confirmed external dependent — a documented external pilot, a merged
learning record submitted under an adopter's own codename citing use of
that specific surface, or the first paying/commercial engagement,
whichever comes first. (This mirrors the pilot-recruitment track already
proposed in `docs/reviews/
todo-and-sol-high-synthesis-prioritized-actions.md`'s P1-1 — the same
external evidence this project needs for its maturity claims is also what
flips a surface from Phase 0 to Phase 1 here.)

Once a surface has real external dependents, deprecate before removing.
Minimum support window: the **longer** of two independent floors — a
calendar-time floor and a release-count floor. Satisfying only one is not
enough: this project's release cadence is fast enough (multiple patch
releases in a single day have happened) that a release-count-only floor
could be cleared in under a week, which would not be a real migration
window.

| Tier | Minimum window |
|---|---|
| Default | 2 MINOR releases **or** 60 calendar days, whichever is longer |
| Security-sensitive surface (auth, coordination schema, gate/role authority) | 3 MINOR releases **or** 90 calendar days, whichever is longer |

Requirements:
- The deprecation MUST be announced in a release's CHANGELOG the moment it
  ships, not discovered retroactively.
- Where the artifact type supports machine-readable deprecation markup
  (schema `x-deprecated: true`, matching the existing `seq` pattern), use
  it, and pair it with an actual advisory warning at validation/parse
  time — markup with no runtime signal is decoration, not deprecation.
- Removal happens only in a MINOR or MAJOR release, never a PATCH release.
- Track deprecation start dates in the CHANGELOG entry and, where the
  artifact supports it, in the schema/code comment next to the deprecation
  marker itself.

## Open item: does this apply retroactively to `seq`?

Not decided here. `seq`'s deprecation (2026-06-18) predates this rule.
Per "What this rule does NOT do" below, this project must explicitly
choose one of: (a) backfill 2026-06-18 as its start date and apply the
Phase 1 window from there if a real `seq`-dependent adopter is ever
identified, or (b) grandfather it as an accepted permanent alias with no
active removal plan, documented as such. Flagging as an open governance
decision, not resolving it unilaterally in this rule.

## What this rule does NOT do

It does not retroactively obligate cleanup of deprecations that predate
this rule. It applies going forward.

## Prior art (why these particular numbers)

Surveyed for precedent before choosing a hybrid time-or-release model:

| Project | Threshold shape | Approximate window |
|---|---|---|
| Kubernetes | Calendar time **or** release count, whichever longer | GA: 12 months or 3 releases; Beta: 9 months or 3 releases (published policy, kubernetes.io/docs/reference/using-api/deprecation-policy/) |
| Django | Release-count based | Deprecated in release N, removed 2 feature releases later (~16 months at Django's release cadence; docs/internals/deprecation) |
| Python / CPython | Release-count based, PEP 387 | Minimum 2 releases with a warning before removal |
| Node.js | Documented per-deprecation (DEP0XXX list), typically major-version-gated | No fixed universal number; removal generally waits for the next major, often a year or more given LTS branch lengths |
| Rust (standard library) | Near-permanent stability | `#[deprecated]` triggers a compiler lint, not a scheduled removal; most deprecated stdlib items are never actually removed |
| GitHub REST/GraphQL API | Fixed calendar sunset date, announced in advance | Typically about 12 months notice per API version |

Kubernetes' model is the most rigorous and the most precisely documented
of the group, which is why its shape — not necessarily its exact numbers
— is the basis for the table above. The numbers are scaled down from
Kubernetes' because this project has a faster release cadence and a much
smaller current blast radius than Kubernetes.

## Related rules

- `docs/implementation-honesty-rule.md` — a deprecation notice with no
  real removal plan behind it is itself a form of overclaiming.
- `always/verifiable-claims-rule.md` — "removed in a future major version"
  is not a citable claim unless a window is actually defined; this rule is
  what makes that claim citable going forward.
- `always/downstream-impact-rule.md` — a Phase 1 deprecation is at minimum
  ADDITIVE (new deprecation notice); the eventual removal is BREAKING and
  needs its own migration note at that time.
- `docs/plans/mcp-coordination-dir-contract-fix.md` — the incident that
  motivated this rule; that plan's `dir` field is Phase 0 (hard rename, no
  deprecation cycle), correctly, under this rule.
