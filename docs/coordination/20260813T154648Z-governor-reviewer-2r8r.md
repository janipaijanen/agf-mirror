---
kind: decision-record
from: governor-reviewer
to: all
id: 20260813T154648Z-governor-reviewer-2r8r
timestamp: 2026-08-13T15:46:48Z
immutable: true
model: human
references-to:
  - 20260813T152130Z-governor-reviewer-jtbq.md
  - 20260813T152050Z-contribution-auditor-mm4k.md
  - 20260813T044747Z-governor-reviewer-8hgm.md
topic: v260-prerequisite-and-release-lineage-decision
requires-ack: false
artifacts_owned: []
---

# Decision: approve the proposed governed path for v2.6.0

The governor approves the recommendation presented after the round-3 review.
This record supersedes `jtbq`'s unresolved-decision state, but it does not yet
approve MR !1492 at Gate 1 or authorize a release branch, version bump, merge,
tag, publication, or issue closure.

## 1. Sole prerequisite exception: `3456c1fa`

`3456c1fa` is conditionally accepted as the sole named prerequisite exception
to the frozen v2.6.0 contents. Before the exception is consumed, perform one
bounded release-path check without that commit. If the original mandatory
`aarch64-unknown-linux-musl` path still fails or exceeds the available runner
capacity, that result conclusively satisfies the necessity condition and the
exception becomes effective. If the without-it path succeeds reliably, stop
and return to the governor rather than including `3456c1fa` automatically.

The exception is specific to v2.6.0 and this exact commit. It creates no
general precedent that delivery-adjacent CI, documentation, plan, or artifact-
matrix changes may enter a frozen release without an explicit governor record.
Do not partially cherry-pick or recreate the commit under this authority.

## 2. Non-`main` release construction: rule amendment required

Do not use a one-off waiver for the proposed lineage from the immutable
`v2.5.0` tag. Before MR !1492 may receive Gate 1 approval, add a separately
planned, reviewed, and Gate-2-approved narrow amendment to
`agf/rules/workflow/release-via-merge-request-rule.md` that authorizes this
class of frozen-scope reconstruction.

The amendment must require at least:

- the exact immutable stable base and an enumerated cherry-pick allowlist;
- protected, retained source and target refs;
- proof of the exact tree delta and absence of all frozen-out content;
- ordinary exact-head Gate 2, tag-pipeline, namespace, and downloaded-artifact
  verification;
- an explicit, reviewable forward-synchronization record connecting the
  released lineage back to `main`; and
- fail-closed behavior on base, target, head, tree, or allowlist drift.

The rule amendment is governance infrastructure on `main`; it is not part of
the v2.6.0 artifact contents. The frozen release remains exactly issues #76,
#84, #85, and #89, plus `3456c1fa` only if the bounded necessity check above
makes its conditional exception effective.

## Next ownership and ordering

1. Planner-architect authors the narrow rule-amendment Gate 1 plan and revises
   MR !1492 so its release sequence depends on the governed amendment.
2. Contribution-auditor reviews the amendment plan; infosec-sentinel is
   consulted on protected refs, approval integrity, namespace checks, and
   forward synchronization.
3. The bounded without-`3456c1fa` release-path check is specified and executed
   under the appropriate approved plan before the v2.6 release branch is cut.
4. Only after the rule amendment is merged and the prerequisite condition is
   resolved may MR !1492 return for a new exact-head Gate 1 decision.

No OpenClaw work is authorized by this decision.
