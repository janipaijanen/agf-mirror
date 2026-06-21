---
kind: decision-record
from: governor-reviewer
to: all
id: 20260621T210956Z-governor-reviewer-gkmp
timestamp: 2026-06-21T21:09:56Z
immutable: true
references-to:
  - 20260621T205912Z-coder-tester-5hr8.md
topic: release-status-wording-gate2-approved
requires-ack: false
artifacts_owned:
  - docs/coordination/20260621T210956Z-governor-reviewer-gkmp.md
decision:
  outcome: approved
  branch: docs/release-status-wording
  mr: "!89"
---

# Gate 2 approval — docs/release-status-wording

Branch `docs/release-status-wording`. Reviewing against `5hr8` handoff
`20260621T205912Z-coder-tester-5hr8.md`.

Pipeline: `mkdocs build --strict` pass (per handoff). `coord validate`: pass
(advisory only — missing `model` field, non-blocking). `git diff --check`: pass.
Stale broad-provisional `rg` scan: no matches.

## Fast-track basis

Docs-only wording change. 19 public pages + README + CHANGELOG. No nav
restructure, no schema, CI, or security boundary change. All content
changes are wording only — "Provisional (v2.0 documentation)" →
"Released (v2.1 documentation)". Fast-track waiver is sound.

## Checklist verification

**README no longer says AGF v2.0 is provisional or structurally unstable.**
Confirmed. The wait/skip bullet now reads "you need every rule and extension
point to be backed by multi-project evidence today." The prior "AGF v2.0 is
provisional and structure may still change" is removed. Correct.

**Public page banners say "Released (v2.1 documentation)".**
Confirmed across all 17 changed doc pages: downstream-impact, exit-and-decommission,
install-and-layouts, tooling-and-tokens, contracts-and-validators,
customizing-the-framework, evidence-and-learnings, roles-and-gates,
docs/index, day-to-day, review-and-release, adopter-faq,
first-governed-change, should-you-adopt, index (landing), disclaimer,
use-cases. Consistent.

**Landing page points to release information for exact version and commit.**
Confirmed. New text: "The exact version and source commit for this build are
recorded in Release information." Correct — removes the dead-end "wait for
v2.0 promotion" instruction and redirects to authoritative release-info page.

**Evidence-specific Provisional wording remains for rule/evidence maturity.**
Confirmed by the stale-scan (no residual broad-provisional hits) combined
with the fact that no `agf/rules/` files were touched. Individual rule
`Provisional` headers survive intact; only page-level release-status banners
changed. The distinction is preserved.

**CHANGELOG records the wording change.**
Confirmed. `agf/CHANGELOG.md` 2.1.1 entry: "Public README and documentation
pages now describe v2.1 as the released documentation line while keeping
evidence-limited rule status visible where it applies."

## Additional observation

`exit-and-decommission.md`: "adopt provisionally" → "adopt in stages" is a
correct secondary cleanup. The word "provisionally" previously overloaded
two senses (release stability + trial adoption); "in stages" is unambiguous.

## Decision

**Approved.** All five checklist items verified. Content accurately reflects
current release status. The wording now correctly separates release stability
(v2.1 is released) from evidence maturity (individual rules may still be
Provisional). Acks Gate 2 handoff `20260621T205912Z-coder-tester-5hr8.md`.
