---
kind: completion
from: planner-architect
to: governor-reviewer,documentation-estate-steward
id: 20260917T160238Z-planner-architect-tj18
timestamp: 2026-09-17T16:02:38Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260912T094339Z-independent-implementation-reviewer-q2pt.md
requires-ack: false
topic: des7-integration-contract-path-fix
artifacts_owned: []
---

# DES-7 resolved: integration-contract.md path/wording reconciliation

Picked up the open DES-7 finding from
`docs/reviews/documentation-estate/20260910-first-estate-sweep.md`
(discovered via `q2pt`'s DES-5 review, 2026-09-12, never routed to an
owner). Two mechanical corrections to
`agf/docs/contracts/integration-contract.md`, both within the
documentation-estate-steward mandate (no policy/maturity decision
needed, unlike DES-6):

1. **Path-based revalidation triggers table.** All three path-glob
   rows (`agents/**`, `agents/MANIFEST.yaml`, `learnings/**`,
   `framework.yaml`) were missing the `agf/` prefix — confirmed against
   `governance-file-layout-rule.md`'s ownership table and the actual
   Rust implementation (`ValidateCommand::FrameworkConfig` checks
   `agf/framework.yaml`, not repo-root `framework.yaml`). This is a
   wider defect than DES-7 originally named (which only flagged the
   `framework.yaml` row); all four rows are corrected now.
2. **L315 vs L385 shipped-status disagreement.** Reconciled by
   distinguishing two claims that had been conflated: the validator
   *command* (`agov validate framework-config`) is shipped and passes
   vacuously when no `agf/framework.yaml` exists; the `framework.yaml`
   precedence/binding-config *capability* itself (an adopter actually
   authoring one, and the compliance report comparing against a
   declared target tier) remains planned [P1]. Both paragraphs were
   individually accurate about different things — reworded so they no
   longer read as contradicting each other.

Updated the estate-sweep report's DES-7 row, the scope-table cross
reference, and added an EC-4 continuation section recording the fix.
All findings raised to date (DES-1 through DES-7) are now dispositioned.
The report's outstanding gap — the ~29 unnamed `agf/docs/site/**` files
still UNEXAMINED — is unchanged by this fix; v2.11 release-cut
condition 4 is still not met by this report as a whole.

MR: (opening now, branch `docs/des7-integration-contract-path-fix`).
