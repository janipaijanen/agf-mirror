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
it is decoration. Before this rule, this project marked a schema field
deprecated (`x-deprecated: true`) and never defined how long "deprecated"
would last — five weeks and several releases later, it was still simply
"kept," with no distinction between "on a tracked path to removal" and
"permanently grandfathered by inertia." Adopters and reviewers cannot
trust a deprecation notice if the project itself cannot say what it means.

## Two phases

### Phase 0 — pre-customer / no external dependents

<!-- CUSTOMIZE: state your project's actual customer/adopter status here -->

While a surface has zero external, production dependents, breaking
changes MAY ship without a deprecation cycle. This is not carelessness —
it is honest: a deprecation notice on a surface nobody depends on yet is
theater, and per this framework's own guidance (`framework.md`, "Feature
documentation lifecycle"), pre-customer products should show current
state only, not manufactured migration timelines that signal instability
to evaluators.

Requirements even in Phase 0:
- The change MUST be disclosed plainly in the CHANGELOG under a `Changed`
  (breaking) heading, not folded silently into `Added`.
- The commit or plan introducing the change MUST state, in one sentence,
  why no deprecation cycle was used (typically: "zero known dependents on
  this surface today").
- The change MUST NOT be described with vague or softening language
  ("improved," "simplified") that obscures that it broke something.

### Phase 1 — the surface has real adopters

<!-- CUSTOMIZE: define the trigger condition for your project — e.g.
     "first paying customer," "first non-founder external adopter,"
     "first production deployment report" -->

Once a surface has real external dependents, deprecate before removing.
Minimum support window: the **longer** of two independent floors — a
calendar-time floor and a release-count floor. Satisfying only one is not
enough: a fast-release-cadence project can clear a release-count floor in
days, and a slow-release-cadence project can take years to clear a
release-count floor while real time barely passes either way. Requiring
both protects against both failure modes.

<!-- CUSTOMIZE: pick your own numbers. Defaults below are modeled on
     Kubernetes' published API deprecation policy (see Prior art), scaled
     down for a project with a faster release cadence than Kubernetes. -->

| Tier | Minimum window |
|---|---|
| Default | 2 MINOR releases **or** 60 calendar days, whichever is longer |
| Security-sensitive surface | 3 MINOR releases **or** 90 calendar days, whichever is longer |

Requirements:
- The deprecation MUST be announced in a release's CHANGELOG the moment it
  ships, not discovered retroactively.
- Where the artifact type supports machine-readable deprecation markup
  (e.g. a schema `x-deprecated: true` vendor extension), use it, and pair
  it with an actual advisory warning at validation/parse time — markup
  with no runtime signal is decoration, not deprecation.
- Removal happens only in a MINOR or MAJOR release, never a PATCH release
  — PATCH releases are for compatible fixes only.
- Track deprecation start dates. A deprecation with no recorded start date
  cannot have its window measured, which defeats the purpose of having a
  window.

## What this rule does NOT do

It does not retroactively obligate cleanup of deprecations that predate
this rule (e.g., a schema field already marked deprecated with no removal
plan). It applies going forward. Whether to record a start date for a
pre-existing deprecation and apply the window from the date this rule is
adopted, or to grandfather it entirely, is a decision left to the adopting
project — not decided here.

## Prior art (why these particular numbers)

<!-- CUSTOMIZE: this section documents the research behind the default
     numbers above; keep or replace with your own research if you pick
     different thresholds. -->

Surveyed for precedent before choosing a hybrid time-or-release model:

| Project | Threshold shape | Approximate window |
|---|---|---|
| Kubernetes | Calendar time **or** release count, whichever longer, for Beta; GA has no fixed removal window at all | GA API versions may be deprecated but **must not be removed within a major version** (effectively indefinite); Beta API versions are deprecated no sooner than 9 months or 3 minor releases after introduction, and remain served for a further 9 months or 3 minor releases after deprecation (whichever is longer each time) (published policy, kubernetes.io/docs/reference/using-api/deprecation-policy/, verified via live fetch 2026-07-21) |
| Django | Release-count based | Deprecated in release N, removed no sooner than 2 feature releases later (docs.djangoproject.com/en/5.2/internals/release-process/#deprecation-policy, verified via live fetch 2026-07-21) |
| Python / CPython | Release-count based, PEP 387 | Minimum 2 minor releases of the same major version (or 1 minor release of an older major version) with a warning before removal; PEP 387 additionally notes a 5-year deprecation period is now the preferred practice (peps.python.org/pep-0387/, verified via live fetch 2026-07-21) |
| Node.js | Documented per-deprecation (DEP0XXX list), typically major-version-gated | No fixed universal number; removal timing is decided case by case, ranging from a couple of major versions to over a decade for the same API (nodejs.org/api/deprecations.html, verified via live fetch 2026-07-21) |
| Rust (standard library) | Near-permanent stability | `#[deprecated]` triggers a compiler lint, not a scheduled removal; consistent with Rust's general stability guarantees, though no single official policy document states stdlib items are never removed — treat as informed inference, not a cited number |
| GitHub REST/GraphQL API | Fixed calendar sunset date, announced in advance | Previous API version supported for **at least 24 months** after a new version's release (docs.github.com/rest/overview/api-versions, verified via live fetch 2026-07-21) |

Kubernetes' model is the most rigorous and the most precisely documented
of the group, which is why its shape — not necessarily its exact numbers
— is the basis for the table above. The exact numbers are scaled down
from Kubernetes' Beta-tier figures (not GA, which has no fixed window)
because most adopting projects have a faster release cadence and a
smaller blast radius than Kubernetes; adjust for your own project's
cadence and risk profile.

## Related rules

- `docs/implementation-honesty-rule.md` — a deprecation notice with no
  real removal plan behind it is itself a form of overclaiming.
- `always/verifiable-claims-rule.md` — "removed in a future major version"
  is not a citable claim unless a window is actually defined; this rule is
  what makes that claim citable going forward.
- `always/downstream-impact-rule.md` — a Phase 1 deprecation is at minimum
  ADDITIVE (new deprecation notice); the eventual removal is BREAKING and
  needs its own migration note at that time.
