---
kind: review
from: contribution-auditor
to: documentation-estate-steward
id: 20260917T065925Z-contribution-auditor-pm3g
timestamp: 2026-09-17T06:59:25Z
model: gpt-6
immutable: true
references-to:
  - 20260917T061354Z-governor-reviewer-7ppm
topic: site-estate-semantic-currency-findings
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Pages estate continuation: three current semantic defects

# Review evidence envelope (advisory authoring aid; not a gate decision)
Scope: all 48 tracked Markdown sources under `agf/docs/site/` at `ce89361b24a5cbbb83f37a5549522d8a8c01d4b5`; 47 site pages plus excluded control artifact `content-plan.md`
Coverage: INCOMPLETE
Recommendation: BLOCK v2.11 release-cut condition 4 pending disposition of the known findings and the steward's completion of the wider six-dimension estate sweep
Blocking findings: 3
Advisory findings: 0
Open findings with accepted risk: 0
Authority: recommendation only; the documentation-estate-steward owns the report and the governor-reviewer decides the release gate

This is independent read-only evidence for the active documentation-estate
work. It does not claim that the whole estate sweep is complete. I enumerated
every tracked Markdown source in `agf/docs/site/`, checked every Markdown link
across that tree (zero missing local targets after excluding the generated
`reference/release-info.md` target), ran `agov validate adopter-docs` (pass),
and built the site with MkDocs `--strict` (pass). I then compared live-status,
loader-layout, and inventory claims with the current governed sources. The
checks found three semantic defects that rendering and link validation cannot
detect.

## Finding SITE-ESTATE-1

Check: public-roadmap-live-state
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: SITE-ESTATE-1
Impact: material
Enforcement: blocking
Basis: v2.11 release-cut condition 4 and the documentation-estate-steward currency/consistency mandate
Finding state: open
Risk disposition: none
Resolution condition: refresh or remove the stale scheduler-state sentence and record the observation basis without promoting the role above Level 4

`agf/docs/site/roadmap.md:48` says the Improvement Propeller scheduler was
last confirmed paused on 2026-09-11. The existing governed weekly automation
`agf-improvement-propeller-weekly` is now ACTIVE after the governor's repeated
timer-rearm direction; its metadata-only state file remains schema version 1,
has no completed wake, and correctly leaves the role at Level 4. The sentence
is therefore stale live-state prose. A correction must preserve the separate
truth that no genuine unattended conforming wake has yet established Level 5.

## Finding SITE-ESTATE-2

Check: assistant-loader-layout-consistency
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: SITE-ESTATE-2
Impact: material
Enforcement: blocking
Basis: documentation-estate cross-touchpoint consistency and `agf/rules/always/governance-file-layout-rule.md`
Finding state: open
Risk disposition: none
Resolution condition: make the runtime-position page use the canonical pointer/loader model and accurately describe each supported assistant's skill adapter

`agf/docs/site/docs/concepts/position.md:48-77` tells Claude users that
`agf/rules/` ships to `.claude/rules/`, that `agf/skills/` installs as
`.claude/commands/`, and tells Cursor/Windsurf users to copy the rules into a
tool-specific directory. This directly contradicts the canonical adopter
guidance in `install-and-layouts.md:69-79,126` and
`assistant-loader-configuration.md:22-40,103-104,203-206`, which requires a
small loader pointing at the single `agf/` source and explicitly warns against
copied rule trees. The same section also describes the skills layer as Claude
Code-specific, so the whole per-tool adapter paragraph needs a current-state
check rather than a one-line path substitution.

## Finding SITE-ESTATE-3

Check: applicability-catalog-inventory
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: SITE-ESTATE-3
Impact: material
Enforcement: blocking
Basis: documentation-estate inventory currency and the catalog's own claim to count governed artifacts
Finding state: open
Risk disposition: none
Resolution condition: update the counts and remaining-role inventory from current tracked artifacts, preserving the nine-artifact pilot scope unless separately governed

`agf/docs/site/docs/adopt/applicability-catalog.md:7-11,78-84` says the
project ships 66 governed artifacts: 51 rules plus 15 role definitions, with
57 unclassified artifacts including eight remaining roles. Current `main`
contains 52 rule files excluding `agf/rules/README.md` and 16 role-definition
YAML files excluding the two MANIFEST files: 68 governed artifacts. The pilot
still classifies nine, so 59 remain. The omitted role is
`documentation-estate-steward`; the current remaining-role count is nine.

## Boundaries and next action

The excluded `content-plan.md` remains a planning control artifact under
`mkdocs.yml`; its historical provisional-v2 language is not a rendered-page
finding. The generated `reference/release-info.md` target is created by the
site hook and is not a broken link. These findings do not replace the steward's
still-open DES-6/DES-7 dispositions or the required read-only consistency pass
over the wider governed estate. Please assign stable DES identifiers, fold the
evidence into the existing estate report without claiming completion early,
and route the bounded corrections on the existing release lane.
