---
id: learning-loop-foundations
status: approved
topic: learning-loop-foundations
gate1_approved_by: governor-reviewer
gate1_approved_at: "2026-07-21T06:36:12Z"
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: Public conceptual documentation and source-registry additions only.
  - role: contribution-auditor
    status: consulted
    reason: >
      Primary sources are required; the page must present them as conceptual
      foundations, not as empirical validation of AGF itself.
---

# Plan: establish the learning loop's known foundations

**Date:** 2026-07-21
**Status:** GATE 1 APPROVED — after reviewing the proposed source set and
scope, the human governor directed: "please proceed".

## Scope

- Add verified registry entries for Argyris and Schön's organizational
  learning, Deming's PDSA cycle, Leveson's CAST handbook, and ISO/IEC
  42001 continual improvement.
- Add a concise **Established foundations** section to **Learning loop and
  evidence**. Explain the specific relationship of each source to AGF and
  state that conceptual alignment does not empirically validate AGF.
- Refine the learning-loop diagram so evidence passes through system/causal
  analysis before a rule, documentation, or tooling proposal, and so governed
  approval precedes implementation and observed use.
- Update the figure explanation and changelog.

## Boundaries

- Do not claim AGF implements ISO/IEC 42001 or provides certification.
- Do not claim PDSA, double-loop learning, or CAST empirically validates AGF.
- Do not reproduce copyrighted source text; summarize and link.
- Do not turn the page into a literature survey or add weaker adjacent sources.
- Preserve MR !286's **Learning loop and evidence** label and existing URL.

## Verification

1. Validate this plan and the governor decision record.
2. Regenerate both committed learning-loop SVG copies from the DOT source.
3. Run strict MkDocs and `git diff --check`.
4. Visually inspect the rendered SVG for legibility.
5. Confirm all external links resolve to the verified publisher/owner pages.

## Downstream impact

Classification: **EDITORIAL**. No schema, validator, rule strength, command,
or adopter migration changes.
