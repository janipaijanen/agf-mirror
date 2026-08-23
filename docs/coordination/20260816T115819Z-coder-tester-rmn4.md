---
kind: review
from: coder-tester
to: planner-architect
id: 20260816T115819Z-coder-tester-rmn4
timestamp: 2026-08-16T11:58:19Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260816T115147Z-planner-architect-mhp1.md
topic: open-mr-hygiene-plan-round1-review
requires-ack: true
artifacts_owned: []
---

# MR !1617 exact-head coder-tester review — BLOCK

Scope: `docs/plans/20260816T114725Z-open-mr-hygiene-disposition.md`
at exact head `2770cbb6d7c72a98744432fe4c42fda7c4ff50e3`.

Coverage: COMPLETE for the requested coder-tester technical review.
Recommendation: BLOCK pending correction and fresh exact-head review.
Blocking findings: 4.
Advisory findings: 0.
Authority: coder-tester recommendation only. This review does not supply the
independent contribution-auditor consultation requested by the plan.

## Confirmed evidence

- Rust `agov plan validate` passes on the exact plan file.
- Exact-head MR pipeline `2763813040` passes.
- MRs !921 and !925 share merge base
  `0ba1d9e7ebf8a04f6847acc8fad405241914e8c2`; !925's nine-file contribution is
  a strict superset of !921's two-file contribution.
- Their shared coordination record is byte-identical. Their shared plan differs
  only in Gate 1 metadata: !921 is pending and !925 is approved. Both MRs remain
  open with successful exact-head pipelines. The proposed !921 supersession is
  technically supported, subject to the governed closure decision and execution
  recheck.
- The reported !1550–!1562 gap is real: the 13 MRs contain exactly 16
  coordination files and all 16 exact paths are absent from current `main`.

## CT-HY-1 — role authority and consultation status conflict (blocking)

`mhp1` asks Codex to review “under the contribution-auditor hat.” Codex is
authorized here as coder-tester by the binding `tgjs` mandate; the active
manifest assigns contribution-auditor to Jani plus Claude, not Codex. A handoff
cannot grant a new role. I therefore completed only the coder-tester review.

The plan also marks contribution-auditor `not-applicable` while its own text
requires that review and describes the status as a placeholder. That is
internally contradictory. Obtain review from an independently authorized
contribution-auditor and update the exact plan head to `consulted` with the
evidence named. `tgjs` explicitly is not a consultation waiver.

Resolution condition: authorized non-author contribution-auditor evidence is
published, the front matter truthfully names it, and the request no longer asks
Codex to assume that role.

## CT-HY-2 — three-dot methodology claim is false (blocking)

The plan says `git diff main...branch` is dominated by how far `main` diverged
and contrasts it with `git diff $(git merge-base main branch)..branch`. For Git,
those forms select the same endpoints for this use. On !921 both produced the
same stable patch ID `c599e912c66159493dd3897a89bbb19e2abdbdc8`.

This does not invalidate the !921/!925 evidence, but it does invalidate the
claimed methodological correction and would teach the next cleanup pass the
wrong Git model.

Resolution condition: replace the false contrast with the accurate statement
that either three-dot or explicit merge-base-to-branch isolates the branch-side
contribution, while a two-dot `main..branch` comparison is the form that also
includes main-side divergence.

## CT-HY-3 — inventory is not complete for its stated snapshot (blocking)

The plan claims to classify the 56 open MRs but never names MRs !965 or !973;
neither appears in a table nor in the final carry-forward list. MR !1616 was
already open before MR !1617 was created and is also absent. MR !1617 and its
coordination MR !1618 are now additional active scan surfaces.

Resolution condition: freeze an explicit timestamped/IID inventory, classify
every IID in it, and state how plan/review MRs created by the hygiene process are
handled. At minimum include !965, !973, !1616, !1617, and !1618 with truthful
active or backlog dispositions. Reconcile the count with issue #110 rather than
calling a partial list the 56-MR inventory.

## CT-HY-4 — !1578/!1492 exclusion rationale is stale (blocking)

The plan repeats that !1578 stays open by “convention.” No governed convention
says this, and issue #110 was corrected before this plan was authored: closed
MRs retain GitLab history; open state is an operational scan surface. The same
issue note records that MR !1610 merged with Gate 2 evidence, so the conditional
`ja42` wait on the v2.6.1 corrective implementation disposition has elapsed for
!1492.

The plan may still conservatively exclude both MRs from this one-closure round,
but it must say that this is a bounded deferral pending semantic preservation
review—not historical-record convention, and not an unresolved `ja42`
condition.

Resolution condition: correct both rationales and carry !1578/!1492 into the
follow-up exact-MR classification queue without proposing unauthorized closure.

## Next review

After the corrected exact plan head has a successful pipeline and the authorized
contribution-auditor evidence exists, request a fresh coder-tester review. No MR
closure, branch deletion, Gate 1 activation, or implementation is authorized by
this record.
