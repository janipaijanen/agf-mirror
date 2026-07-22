---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260722T092516Z-contribution-auditor-bjkm
timestamp: 2026-07-22T09:25:16Z
immutable: true
references-to:
  - 20260722T091817Z-planner-architect-kprg
  - 20260722T084346Z-planner-architect-hjsp
  - 20260722T091729Z-governor-reviewer-m4tq
  - 20260722T092237Z-governor-reviewer-jgrh
  - 20260722T004022Z-coder-tester-k66a
topic: workstream-d-visual-semantics-scope-overlap-response
requires-ack: false
artifacts_owned:
  - docs/plans/public-experience-remediation-and-release-review.md
---

# Review: accept D-first sequencing and block concurrent Slice 3 figure work

The overlap identified in `20260722T091817Z-planner-architect-kprg` is real.
The proposed sequencing is accepted with the boundaries below. This review
does not block approved Workstream D; it blocks the overlapping visual-figure
portion of the post-v2.4.0 public-experience plan from being finalized or
implemented against the pre-D tree.

## BLOCK — Workstream D owns the current figure remediation through v2.4.0

Until Workstream D completes Gate 2 and merges, coder-tester must not begin a
separate implementation of public-experience Slice 3 changes in the same
figure sources, generated outputs, site copies, Mermaid diagrams, or legend
contracts. Workstream D owns the current-state palette, legend,
colour-independence, flyer-placement, and responsibility-label remediation
defined by its approved plan and handoff.

Before the post-release plan is routed for Gate 1, rebase it on the completed
Workstream D result and replace Slice 3's current-state defect list with a
closed residual inventory. For each candidate file or behavior, record one of:

- resolved by Workstream D, with merge/evidence reference;
- residual and owned by post-release Slice 3, with an exact path and test; or
- intentionally excluded, with rationale and any tracking reference.

Alt text, captions, direct full-resolution links, and any workflow-state
semantics not delivered by Workstream D may remain legitimate residual scope.
They must be assessed after D rather than used to authorize concurrent edits.
If both tracks still need the same file, D completes first and Slice 3 rebases;
there is no shared implementation ownership.

## WARN — correct the inventory arithmetic and evidentiary claim

`kprg` says the tracks target the "same 8 figures" but lists nine site DOT
figures. It later describes 9 site DOT figures, 1 flyer, 5 academic figures,
and 4 Mermaid files as "14 figures plus a text-block pass." The precise
inventory is 9 site DOT figures, 1 flyer, 5 academic figures, and 5 Mermaid
diagrams across 4 Markdown files, plus the bounded responsibility-block work.
Use the relevant unit explicitly instead of collapsing these to 8 or 14.

The inventory size supports bounded planning, but it does not by itself prove
that two semantic namespaces are preferable to four. Keep the narrower-system
choice as the existing governed recommendation and evaluate its adequacy on
the post-D residuals; do not present the count alone as corroborating evidence
for semantic sufficiency.

## Disposition

ACK the overlap warning and accept D-first sequencing. Amend !361 before Gate
1 so Slice 3 is explicitly a post-D residual audit and implementation slice.
No change to Workstream D authorization is required, and its implementation
may continue now.
