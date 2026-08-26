---
kind: decision-record
from: governor-reviewer
to: all
id: 20260814T040118Z-governor-reviewer-apkm
timestamp: 2026-08-14T04:01:18Z
model: human
immutable: true
references-to:
  - 20260813T154648Z-governor-reviewer-2r8r.md
  - 20260813T201833Z-coder-tester-adjp.md
topic: frozen-scope-forward-sync-target-race-exception
requires-ack: false
artifacts_owned:
  - docs/coordination/20260814T040118Z-governor-reviewer-apkm.md
---

# Decision: accept a narrow detect-and-remediate exception for metadata forward sync

The governor approves the recommendation presented after the round-eight poll:
for MR !1503's frozen-scope reconstruction mechanism, detect-and-remediate is
accepted as a narrow exception to decision `2r8r`'s literal fail-closed target-
drift requirement **only for the post-release metadata-forward-sync merge to
`main`**.

This decision accepts the disclosed residual target-side TOCTOU risk. GitLab's
merge API binds the source SHA but not the target SHA, so a proof-invalid merge
can land if `main` advances between the final target read and the merge call.
The approved control is therefore detection and recovery, not atomic
prevention, and must never be described as equivalent to compare-and-swap or
literal fail-closed behavior.

## Mandatory boundaries

The exception applies only when all of the following remain true:

1. The forward sync changes only the governed metadata surfaces enumerated by
   the eventually approved exact-head plan; it changes no code, tests, schema,
   rule, role, or release artifact.
2. Immediately before merge, the operator re-reads `main` and requires it to
   equal the recorded sync-target SHA, then merges only the recorded exact
   sync-source SHA without squash or rebase.
3. Immediately after merge, the operator checks the merge commit's actual
   first parent against the recorded sync-target SHA and independently proves
   that its actual first-parent diff contains only the authorized metadata
   changes.
4. Any mismatch is recorded as a proof-invalid merge. The operator opens a
   fresh corrective forward-sync MR from the new actual `main` tip and repeats
   the full proof, pipeline, and exact-head Gate 2 process. The already-
   published tag and reconstruction refs remain untouched.
5. Human observation of apparently idle merge activity is optional additional
   care, not the security property and not a claim of atomic serialization.
6. If the platform later supplies a usable target-bound merge, merge train, or
   equivalent serialization control, the stronger mechanism must be evaluated
   before reusing this exception.

This is an explicit, scoped exception to `2r8r`, not a finding that the plan's
existing mitigation satisfied `2r8r` as originally written. It creates no
precedent for ordinary feature, governance, release-construction, or other
forward-sync merges.

## What this decision does not approve

This record does not approve MR !1503 at Gate 1, resolve the round-eight
whole-result content-faithfulness blocker, substitute for refreshed
contribution-auditor or infosec-sentinel consultation, authorize implementation
or merge of the rule amendment, approve MR !1492, activate the conditional
`3456c1fa` prerequisite, create a release branch, or authorize tagging or
publication. Those gates and prerequisites remain separate.
