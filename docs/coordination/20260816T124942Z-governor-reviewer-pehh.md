---
kind: decision-record
from: governor-reviewer
to: all
id: 20260816T124942Z-governor-reviewer-pehh
timestamp: 2026-08-16T12:49:42Z
model: human
immutable: true
references-to:
  - 20260816T124150Z-planner-architect-cthj.md
  - 20260816T122659Z-coder-tester-be1q.md
  - 20260816T122021Z-planner-architect-cthy.md
  - 20260809T063108Z-independent-implementation-reviewer-12nr.md
  - 20260809T150324Z-contribution-auditor-sd8g.md
  - 20260809T153019Z-governor-reviewer-q86p.md
topic: contribution-auditor-authority-and-reviewer-reactivation-path
requires-ack: false
artifacts_owned: []
---

# Governor decision: approve `cthj` paths A, B2, and C1

## Transcription and authority

This is the binding record of the human governor's direct instruction in the
Codex session: “proceed as proposed. Communicate to Claude too.” Codex files
the decision mechanically; the decision originates from the human
governor-reviewer.

This record approves the immediate remediation in `cthj` A and selects paths
B2 and C1 with the boundaries below. It does not itself amend the manifest,
reactivate a suspended role, approve a future Gate 1 or Gate 2 candidate,
merge an MR, or authorize a release/tag/deployment.

## A — immediate remediation approved

1. Stop citing `20260816T002624Z-contribution-auditor-n2j3.md` or any other
   Codex-authored `from: contribution-auditor` record as valid
   contribution-auditor consultation evidence. Such content may be retained as
   historical/advisory material under its disclosed provenance, but its role
   label supplies no authority.
2. Hold plan MRs !806 and !807 at their current Gate 1 state. Neither may be
   activated using `n2j3` or another unauthorized-player record.
3. Do not revert merged MR !1610 solely because of this role-label defect.
   Its properly authorized coder-tester review and mechanical evidence remain
   distinct. If later review finds a concrete product defect, correct it
   through normal fix-forward governance.
4. MR !1578's plan frontmatter must not retain a false `consulted` claim.
   Planner-architect is authorized to prepare an honest metadata correction
   citing this decision and the invalid-evidence gap. The correction must use a
   fresh MR, pass Rust `agov plan validate` and exact-head CI, and return for
   normal review and the required binding gate/merge decision. This is drafting
   authority, not advance approval of the corrected wording or head.

MR !1617 remains held on `rmn4`/`be1q` findings and valid consultation; this
decision does not activate its Gate 1 plan.

## B2 — authorize a governed Codex contribution-auditor staffing proposal

Planner-architect is authorized to prepare a fresh Gate 1 plan and candidate
manifest change adding Codex as an additional foundry
`contribution-auditor` player, subject to all of these required boundaries:

- Codex's added scope is contribution audit of Gate 1 plans and other
  contribution/guidance artifacts already within
  `agf/agents/contribution-auditor.yaml`; it does not expand that role's
  jurisdiction.
- The reviewer must not be the author or from the authoring session. For a
  Claude-authored plan, Codex may supply the independent contribution audit;
  it remains advisory evidence and never Gate 1 approval.
- Contribution audit must not be relabeled full exact-head implementation
  diff/test/pipeline review. Coder-tester technical review and the separate
  independent-implementation-reviewer contract retain their own identities.
- Every review names the exact plan/artifact head and reviewer model/session
  provenance. A new head requires refreshed evidence unless the governing
  decision explicitly bounds a metadata-only continuity check.
- Codex gains no merge, release, tag, deployment, role-change, consultation-
  waiver, or mechanical blocking authority.
- Because this changes the staffing of the contribution-auditor role itself,
  the self-referential escalation boundary applies. The human governor must
  review the proposed role/manifest change; neither Claude nor Codex may
  self-supply the binding audit or gate decision for that change.

Until the governed manifest change passes Gate 1, implementation review,
Gate 2, and merge, Codex remains **unauthorized** to publish as
`contribution-auditor`. This decision authorizes the proposal path, not the
role change itself.

## C1 — authorize a fresh one-off review of the merged !1289 correction

Codex is authorized under its existing **coder-tester** role—not as
contribution-auditor and not as the suspended
independent-implementation-reviewer—to perform one bounded, read-only,
independent review of:

- MR !1289's merged corrective source head
  `6940d4deb7ce02b0244cddd2fb44f9cbc263962f`;
- the B1-B6 requirements in `12nr` and their correction/evidence trail;
- the current `main` role definition, manifest status, and adopter-facing
  projections at the exact base SHA observed by the review; and
- whether target advances since !1289 preserve the corrected contract.

The review must disclose that `sd8g` used an unauthorized role label, run the
reproducible Rust `agov`/projection/adopter-doc checks applicable to the
correction, name exact pipeline/history evidence, and return COMPLETE/CLEAR or
COMPLETE/BLOCK as advisory evidence. It may not edit the reviewed artifacts,
reactivate the role, or reinterpret `q86p` as prior reactivation authority.

If the review is CLEAR, planner-architect may prepare a separate Gate 1 plan
and manifest-status MR for reactivation. That fresh exact head still requires
normal consultation, validation, CI, review, Gate 1, Gate 2, and a binding
governor reactivation decision. If the review is BLOCK, route all findings
before any reactivation proposal proceeds.

The proposed retroactive independent-implementation-reviewer pass over !1610
is not authorized yet. It may be proposed after the role is validly
reactivated; any concrete defect found then is fix-forward work.

## Next actors

1. coder-tester: execute the bounded C1 review and publish its exact-evidence
   recommendation.
2. planner-architect: draft the A4 !1578 metadata correction and the B2 Gate 1
   staffing plan, without treating either as pre-approved.
3. governor-reviewer: decide the future exact Gate 1/Gate 2 heads and any role
   reactivation after the required evidence is visible.
