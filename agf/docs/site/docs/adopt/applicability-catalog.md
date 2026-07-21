# Adopter applicability catalog

*Status: Released (v2.3 documentation, incomplete pilot). Audience: adopters
deciding what to activate, deactivate, or remove.*

!!! warning "Incomplete classification pilot"
    This page classifies **8 of 57 governed artifacts** (45 rules under
    `agf/rules/` — `agf/rules/README.md` is an index, not itself a rule, and
    is not counted — plus 12 agent role definitions under `agf/agents/`). It
    is a first representative slice, not a complete adopter-facing removal
    guide. See [Tracked completion](#tracked-completion) below for the
    remaining 49 artifacts and how that work is tracked.

## Why this page exists

[Should you adopt?](../start/should-you-adopt.md#what-ships-and-what-you-activate)
already draws the top-level distinction: AGF ships the complete governance
library, and a project activates a subset. That page answers *which subset
to start with*. This page answers a narrower, more concrete question per
artifact: **when does this specific rule or role's function become needed,
and what do you lose if you remove it?**

Those are two separable questions, not one:

- **Library membership** — what kind of artifact this is, and when it is
  typically introduced. Already answered by existing fields: an agent's
  `tier` (functional category) and `phase` (recommended growth-sequence
  stage — see [`framework.md` §7](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/main/agf/framework.md)),
  or a rule's `scope` (directory-based applicability domain).
- **Activation and removal consequence** — under what condition the
  artifact's function is actually needed in *your* project, and what
  guarantee you forfeit if you delete it. This is the gap this page closes:
  the new `activation_condition` field (and, for rules, `safe_to_remove_when`
  / `removal_forfeits`; for agents, `deactivation.local_deactivation` /
  `deactivation.definition_removal`).

A third question — *is this artifact currently active in your specific
project* — is neither answered here nor by any framework-shipped field. That
is project-local state and belongs in your own `MANIFEST.yaml` or
equivalent, not in framework metadata that's identical across every adopter.

## The eight classified artifacts

### Agent roles

| Role | Tier / Phase | Activation condition | If you stop using it locally | If you delete the definition |
|---|---|---|---|---|
| **Build Engineer** | Production, Phase 1 | Project has a CI pipeline | Coder-Tester resumes owning CI/build config — the default before Build Engineer is introduced | Any other project sharing this framework checkout loses the reference definition and must restore it from the framework's own release archive |
| **Contribution Auditor** | Cross-cutting, Phase 1 | First contribution (upstream or downstream) | Contribution review reverts to whatever ad hoc process existed before — no structural check remains against adversarial or unsound incoming contributions | Loses the framework's own reference implementation of adversarial-contribution screening |
| **InfoSec Sentinel** | Cross-cutting, Phase 1 | Project submits learning records | Externally-bound artifacts (learning records, upstream contributions) are no longer screened for information leaks before publication | Loses the framework's own reference implementation of the anonymization/leak-screening boundary |
| **Planner-Architect** | Production, Phase 1 | Always | Not meaningfully deactivatable while still using AGF's gate model — Gate 1 authorship has no other owner in the minimum system | Removes a load-bearing piece of the governed-change pipeline itself |
| **Coder-Tester** | Production, Phase 1 | Always | Not meaningfully deactivatable while still using AGF's gate model — Gate 2 implementation has no other owner in the minimum system | Removes a load-bearing piece of the governed-change pipeline itself |
| **Governor-Reviewer** | Governance, Phase 1 | Always | Not meaningfully deactivatable while still using AGF's gate model — gate approval authority has no other owner in the minimum system | Removes a load-bearing piece of the governed-change pipeline itself |

### Rules

| Rule | Scope | Activation condition | Safe to remove when | What removal forfeits |
|---|---|---|---|---|
| **Security hardening** (`always/security-hardening-rule.md`) | Always | Always | The project has no exposure to secrets, credentials, network-facing code, containers, or third-party dependencies — in practice, close to never for any real deployed project | The framework's structural guard against unpinned dependencies, unpinned image tags, unprotected CSRF/SSRF surfaces, and unlogged security-relevant changes. Removing the rule does not remove the underlying risk, only the check against it |
| **Accessibility** (`ui/accessibility-rule.md`) | UI | Project has a UI | The project has and will have no user-facing interface | WCAG AA conformance checking for any UI added later without re-adding this rule |

## A wording discrepancy, now resolved

Contribution Auditor's own definition (`contribution-auditor.yaml`)
previously said its trigger was "mandatory from first **external**
contribution," while `framework.md` said "mandatory from first
contribution, **upstream or downstream**." These were not obviously the
same condition — a downstream adopter's own first-ever contribution to
their own project is not necessarily "external" in the same sense an
upstream contribution to the framework itself is, and "external" already
has a distinct, different defined meaning elsewhere in this framework's own
vocabulary (`always/external-anonymization-rule.md`'s codebase-publication
boundary). Resolved by standardizing on `framework.md`'s "upstream or
downstream" wording, matching Contribution Auditor's own stated purpose of
screening contributions "before they enter the framework corpus or adopter
projects" — the two directions that phrase already names.

## Tracked completion

The remaining 49 governed artifacts (43 rules plus 6 agent roles:
fact-checker, capability-developer, documentation-writer,
deployment-engineer, academic-peer-reviewer, academic-visual-reviewer) are
not yet classified. Completing the classification is real editorial
judgment per rule and role — it belongs in a follow-up plan sized so a
reviewer can answer one yes/no question per batch, per this project's own
branch-sizing discipline, rather than one large flag-day change.

That follow-up is tracked, not merely implied:
[`docs/coordination/20260721T063606Z-coder-tester-pjms.md`](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/main/docs/coordination/20260721T063606Z-coder-tester-pjms.md).

## What this page does not do

- It does not add a `classification` field, a required schema property, or
  any change to which rules/roles are shipped or activated by default. All
  new fields are optional; every existing rule and agent file that doesn't
  use them stays schema-valid.
- It does not build automated tooling to check that these fields are
  populated correctly. `agov validate rules` does not exist yet — see the
  [tooling and token savings](tooling-and-tokens.md) page for what `agov`
  currently validates, and the tracked follow-up above for what's planned.
- It does not resolve project-local activation state. That is your own
  `MANIFEST.yaml`'s job, not this page's.

## See also

- [Should you adopt?](../start/should-you-adopt.md#what-ships-and-what-you-activate) —
  the top-level "what ships vs. what you activate" framing this page
  narrows to per-artifact detail.
- [Install the `agf/` directory](install-and-layouts.md#step-4-review-rule-loading-and-customization) —
  where rule review and deletion happens during commissioning.
- [Exit and decommission](exit-and-decommission.md) — the three exit modes
  (pause, downgrade, remove) this page's per-artifact removal consequences
  feed into.
