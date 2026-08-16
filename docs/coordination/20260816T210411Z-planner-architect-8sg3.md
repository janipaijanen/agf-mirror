---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260816T210411Z-planner-architect-8sg3
timestamp: 2026-08-16T21:04:11Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260816T204817Z-contribution-auditor-8276
  - 20260816T205045Z-coder-tester-bgqm
topic: v261-release-plan-path-decision-summary
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Select Path A, Path B, or a third option for v2.6.1's release
      construction base.
  must_read:
    - docs/plans/20260816T174141Z-v261-release-publication.md
  do_not_touch:
    - No branch, tag, or namespace object creation is authorized by this
      handoff. This is a decision request, not an implementation step.
---

# v2.6.1 release plan: converged, awaiting only your path selection

MR !1657's plan is now confirmed CLEAR on all technical and factual claims
by both contribution-auditor (`8276`) and coder-tester (`bgqm`) at exact
head `fb0b72363088edeea5096c950cc4161ac4e5aea9` (round 4). It took four
review rounds to get here — round 1's premise was factually wrong (main
was assumed to lack the coordination-reliability feature set; it doesn't),
and three further rounds fixed a rule-citation error and two metadata-
allowlist gaps. All of that is now resolved and independently
re-verified. I merged the three outstanding review MRs (!1663, !1665,
!1667) to `main` so the full audit trail is in one place.

**The one thing left is your decision.** One paragraph each, since you
weren't in the room for the back-and-forth:

- **Path A — release directly from current `main`.** `main` already has
  every functional commit v2.6.1 needs (the four coordination-
  reliability fixes, merged 2026-08-05, and the release-asset-contract
  fix, merged today). No exception or frozen-scope apparatus needed —
  standard release-via-merge-request workflow. The real cost: `main` also
  carries 987 other commits since the original v2.6.0 scope was frozen,
  none of which have been reviewed *as a release unit*.
- **Path B — repeat the frozen-scope reconstruction for v2.6.1.**
  Rebuilds from peeled v2.5.0 with the same deterministic-replay,
  namespace-check, and content-faithfulness proof matrix
  `reconstruct/v2.6` itself required across roughly a dozen review
  rounds, plus the one corrective commit. Preserves the original scope-
  discipline goal exactly, but is a large undertaking for what's actually
  a one-line stale-contract fix, and needs its own fresh application-
  specific authority (the frozen-scope rule's acceptance criteria 19/20
  are satisfied for the v2.6.0 application only, not a standing license
  to reuse).

Full trade-off detail, exact file lists, and all verification evidence
are in the plan itself. Once you pick, I'll expand the selected path into
one exact, complete, validation-ready construction and route it for real
infosec-sentinel consultation before requesting Gate 1 in earnest.
