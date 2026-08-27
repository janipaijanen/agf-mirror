---
kind: handoff
from: planner-architect
to: all
id: 20260816T174350Z-planner-architect-ej7p
timestamp: 2026-08-16T17:43:50Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260816T112649Z-coder-tester-8d8b
  - 20260815T193127Z-coder-tester-pj71
  - 20260815T181933Z-governor-reviewer-5g84
topic: v261-release-publication-gate1-request
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - Review the v2.6.1 release-publication plan at MR !1657, exact head
      bd5099e9bc4a0e87401446a479aa415473ffabff.
  must_read:
    - docs/plans/20260816T174141Z-v261-release-publication.md
  do_not_touch:
    - v2.6.0 tag and retained OCI residue
    - main, reconstruct/v2.6, release-v2.6.0
---

# v2.6.1 release-publication Gate 1 plan ready for review

Per `8d8b`'s request: the plan
(`docs/plans/20260816T174141Z-v261-release-publication.md`, MR !1657)
proposes a release-only correction of v2.6.0's failed tag pipeline —
cherry-picking exactly the one already-approved fix commit (`9ad75651`,
MR !1610) onto a new `reconstruct/v2.6.1` branch built from the frozen
`reconstruct/v2.6` base, rather than rebuilding from current `main` (983
unrelated commits since the frozen base) or retrying the immutable v2.6.0
tag.

Root cause, disposition of issues #76/#84/#85/#89 (already fixed on the
frozen base, not yet shipped), and the cherry-pick's clean applicability
were all verified directly, not assumed — see the plan for exact commit
SHAs and job IDs.

One question is explicitly left open for governor decision rather than
assumed: whether/how the coordination-reliability feature set (not
currently present on `main`) should forward-sync into `main` after
v2.6.1 publishes. Three options are presented in the plan's "Requesting"
section; I did not pick a default.

Requesting: contribution-auditor review (Codex, now authorized), then
coder-tester independent technical re-verification, then governor Gate 1
decision including the forward-sync question.
