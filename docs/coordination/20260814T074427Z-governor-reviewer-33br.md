---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20260814T074427Z-governor-reviewer-33br
timestamp: 2026-08-14T07:44:27Z
model: human
immutable: true
references-to:
  - 20260814T073332Z-coder-tester-q3hs.md
  - 20260814T073333Z-coder-tester-sj5s.md
  - 20260814T070936Z-coder-tester-amjp.md
  - 20260814T040118Z-governor-reviewer-apkm.md
  - 20260813T154648Z-governor-reviewer-2r8r.md
topic: frozen-scope-reconstruction-gate1-approved
requires-ack: false
artifacts_owned:
  - docs/plans/20260813T155743Z-frozen-scope-reconstruction-rule-amendment.md
---

# Binding decision: MR !1503 Gate 1 approved with scoped residual and consultation waiver

This record transcribes Jani's explicit in-session decision, “approved.
proceed as proposed,” after receiving the decision-ready poll and recommendation
in `20260814T073332Z-coder-tester-q3hs.md`.

## Exact approved artifact

- MR: !1503
- Exact head: `838028991362a921ba1f5aff0f890574101689d6`
- Exact-head pipeline: `2759765452` — SUCCESS
- Plan: `docs/plans/20260813T155743Z-frozen-scope-reconstruction-rule-amendment.md`

## Decision 1: criterion 20 residual accepted

The governor accepts, for this frozen-scope reconstruction rule and its v2.6.0
application only, the release-construction target-race residual described at
the approved head. GitLab does not atomically bind the target HEAD during the
merge. A proof-invalid merge can therefore land on the dedicated protected
reconstruction target before the mismatch is detected.

This acceptance is conditioned on every boundary already stated by the plan:

- immediately before merge, both target and source heads equal the exact SHAs
  named by the binding governor record;
- the merge API binds the exact reviewed source SHA, with no squash or rebase;
- immediately after merge, the first parent equals the reviewed target, the
  second parent equals the reviewed source, and the result tree OID equals the
  exact reviewed final source HEAD's post-metadata tree OID;
- any mismatch stops before tag, release, package, OCI, or other namespace
  creation; the isolated target is marked proof-invalid and abandoned; and a
  fresh protected target plus the complete proof and review cycle is required;
- this is described as fail-closed at the publication boundary, never as
  atomic target binding or literal prevention of target drift; and
- any future application must reassess whether GitLab then provides a stronger
  target-bound or serialized mechanism before reusing this residual.

This is a separate scoped exception from `apkm`; it does not broaden `apkm`,
authorize detect-and-remediate for ordinary feature/governance merges, or set
a general release-construction precedent outside the conjunctive rule.

## Decision 2: infosec-sentinel semantic consultation explicitly waived

The configured `infosec-sentinel` is the CI actor
`agov-validate-infosec`. Its governed blocking authority is limited to its
configured mechanical checks; it cannot author the semantic protected-ref,
approval-integrity, namespace, and merge-race review requested for this plan.
The exact-head pipeline succeeded, but that success is mechanical evidence,
not a semantic role disposition.

The governor therefore explicitly waives the named infosec-sentinel semantic
consultation for this MR and exact head. The extensive coder-tester security
reviews are retained as advisory evidence and are not reattributed to
infosec-sentinel. This waiver is plan-specific and does not expand Codex's role
authority or establish that future security consultations may be silently
substituted.

## Decision 3: Gate 1 approved

Gate 1 is approved for MR !1503 at exact head
`838028991362a921ba1f5aff0f890574101689d6`, incorporating the two decisions
above. The planner-architect may make only the mechanical reflection edits
needed to record this decision in the plan: set `status: approved`, add
`gate1_approved_by: governor-reviewer` and this record's timestamp, change the
infosec-sentinel consultation status to `waived` with this rationale, and mark
acceptance criterion 20 satisfied by this decision. Those edits must cite this
record and pass exact-head validation. Any other substantive plan change
requires renewed review and a fresh governor decision.

This approves the rule-amendment plan. It does not merge MR !1503, approve its
future implementation at Gate 2, approve MR !1492, create a release branch,
publish v2.6.0, alter the frozen release sequence, or resume OpenClaw #104.
