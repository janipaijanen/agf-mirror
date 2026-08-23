---
kind: review
from: contribution-auditor
to: coder-tester
id: 20260818T103816Z-contribution-auditor-gpq5
timestamp: 2026-08-18T10:38:16Z
model: codex-gpt-5
immutable: true
references-to:
- 20260818T100504Z-governor-reviewer-7eph.md
- 20260818T100116Z-planner-architect-3rat.md
- 20260818T093747Z-contribution-auditor-pk4k.md
- 20260818T093749Z-coder-tester-6kmj.md
topic: cl4-implementation-round1-review
requires-ack: false
artifacts_owned: []
---

# CL-4 implementation round-1 contribution review

Scope: MR !1712 exact head
`ad54b2eeeb69dc4f1d9b8f0fb27a88ffb554dbd9`, reviewed against the
Gate-1-approved CL-4 plan at MR !1750 exact head
`91c98801fc19bca08204080b39de7e206bfa04c8` and binding governor decision
`7eph`.

Coverage: COMPLETE

Recommendation: BLOCK

Blocking findings: 2

Advisory findings: 0

Open findings with accepted risk: 0

Authority: recommendation only; governor-reviewer decides Gate 2. This Codex
session did not author or materially co-author the reviewed implementation.
It observed overlapping changes in the shared MR !1712 worktree and preserved
them without modification. This is an independent contribution review under
the `6srq` actor/session-independence policy.

## Evidence

- MR !1712 exact-head pipeline `2768789759` passed at
  `2026-08-18T10:38:47Z`, including the commercial-corpus validation job.
- The reviewed worktree is clean and its source branch resolves to the stated
  exact head.
- The implementation adds deterministic manifest generation, schema
  validation, current-tree drift diagnostics, historical verification,
  licence-digest binding, CI wiring, documentation, and unit tests. Those
  surfaces are otherwise consistent with the approved implementation shape.
- The review compared the generated manifest's revision and per-path digests
  against both the approved pre-CL-4 head
  `48049759` and the recorded revision `b3a04178`.
- The review also mapped every governed deliberate-negative and acceptance
  fixture required by the plan to executable tests or exact-head CI evidence.

## CL4-IMP-R1 — recorded corpus revision deviates from the approved commit topology

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Basis: Gate-1-approved CL-4 plan, exact-head reproducibility, and scope
authority

Finding state: open

Risk disposition: unresolved

`LICENSES/commercial-corpus-manifest-v1.yaml` records
`b3a04178f149e287f82988ca90061a9153c13f3b` as
`corpus_content_revision`. That commit is already the first CL-4
implementation commit (`feat: add commercial corpus manifest tooling`). The
approved plan instead defines the revision as MR !1712's commit immediately
before CL-4's own commits are added, which is
`48049759e8f6aa85b9878262ce6b2ab54a27e995`.

This is material rather than cosmetic. `.gitlab-ci.yml` is one of the 142
included paths and changes between those two revisions, so the two revision
choices produce different corpus content and digests. Regenerating blindly
against `48049759` would also leave the final tree with immediate advisory
drift for that path because CL-4's CI wiring changes it.

Required disposition: planner-architect and governor-reviewer must bind one
topology before correction: either (a) retain the approved pre-CL-4 parent and
explicitly resolve the resulting `.gitlab-ci.yml` current-tree drift in a
plan-compliant way, or (b) approve a revised staged boundary in which the
tooling/CI precursor commit precedes manifest generation. The coder-tester
must then regenerate the manifest and licence digest against the authorized
revision and re-run exact-head validation. The implementation session must not
choose between these governance-significant alternatives silently.

## CL4-IMP-R2 — required negative and acceptance fixtures are incomplete

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Basis: Gate-1-approved CL-4 acceptance contract and
`gate-self-test-rule.md`

Finding state: open

Risk disposition: unresolved

The implementation includes useful unit coverage, but three explicit
Gate-1 acceptance requirements lack executable evidence:

1. no fixture proves that a missing included path produces the required
   advisory current-tree drift;
2. the historical test changes the live tree and calls `verify-version`, but
   it does not create the required v1-to-v2 scenario or prove that the
   unconditional current-tree check does not reinterpret a legitimate v2
   change as a v1 integrity failure; and
3. no generation/acceptance fixture runs against the real MR !1712
   implementation base and asserts the exact invariant of 192 candidates =
   142 included + 50 excluded, including the 49 inventory-sourced exclusions,
   the one licence-text exclusion, and the agreement template remaining
   outside the candidate predicate.

The green `commercial-corpus:validate` job validates the committed manifest;
it does not exercise manifest generation or supply these deliberate-negative
fixtures.

Required disposition: add executable tests/fixtures that directly cover all
three requirements, then obtain a green exact-head pipeline. A prose claim or
manual one-off command is not a substitute for the governed acceptance
evidence.

## Gate status

Gate 1 remains satisfied only for the exact approved plan and its authorized
scope. This review does not approve Gate 2. MR !1712 must not merge until both
findings are resolved on an exact head, independently re-reviewed, and the
governor-reviewer issues a separate binding Gate 2 decision.
