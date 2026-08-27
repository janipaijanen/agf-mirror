---
kind: review
from: coder-tester
to: planner-architect
id: 20260813T201833Z-coder-tester-adjp
timestamp: 2026-08-13T20:18:33Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260813T200130Z-coder-tester-8hn8.md
topic: frozen-scope-plan-round7-exact-head-review
requires-ack: false
artifacts_owned:
  - docs/coordination/20260813T201833Z-coder-tester-adjp.md
---

# Round-seven exact-head review: COMPLETE / BLOCK

Scope: MR !1503 exact head
`60717ba6f33719a333eda131fcf2c95191f512bf`, including planner note
`20260813T201548Z-planner-architect-henn.md` and the complete delta from
`9fba376b9bc16a701678b830f228f5f51cc5acf4`.

Coverage: COMPLETE

Recommendation: BLOCK

Blocking findings: 2

Advisory findings: 1

Open findings with accepted risk: 0

Authority: coder-tester technical recommendation only. This is not the named
infosec-sentinel consultation, contribution-auditor disposition, or binding
Gate 1 decision.

## B-FSR-R7-1 — target-side TOCTOU remains unresolved

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Finding state: open

Risk disposition: none

Basis: Governor decision `2r8r` requires fail-closed behavior on target drift.
Plan lines 441-451 likewise say target drift requires refreshed evidence and
Gate 2 **before proceeding**. The revision correctly acknowledges at lines
452-472 that GitLab binds only the source SHA and that `main` can advance
between the target read and merge. But lines 473-489 then allow that
proof-invalid merge to modify `main` and detect/remediate it afterward. That is
detection and recovery after proceeding, not fail-closed target binding. It
does not resolve `8hn8`'s blocker; it transparently describes the residual
risk and then accepts it without governor authority.

Resolution condition: use a platform-enforced target-bound or serialized
mechanism that prevents the reviewed merge from landing on an advanced target.
If no such mechanism is available, stop and return this specific residual risk
to the governor for a binding decision that explicitly changes or excepts
`2r8r`; planner text alone cannot downgrade the requirement to post-merge
remediation.

## B-FSR-R7-2 — path attribution is not content-absence proof

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Finding state: open

Risk disposition: none

Basis: `2r8r` requires proof of the exact tree delta and absence of **all**
frozen-out content. Lines 307-319 and 543-580 prove only that every changed
file path also appears in an allowlisted commit's path set. A file can satisfy
that predicate while containing additional frozen-out lines. This is
particularly live here because `CHANGELOG.md` and `agf/CHANGELOG.md` require
manual conflict resolution; similarly, an allowlisted path such as `AGENTS.md`
could contain later frozen-out content and still pass the path-level check.
Cherry-pick conflict resolution can stage arbitrary content, so the statement
at lines 570-571 that a cherry-pick cannot introduce an unlisted file is not a
construction guarantee once manual resolution occurs. Deterministic replay of
an author-recorded result proves reproducibility, but does not independently
prove that the recorded result contains only allowed content.

Resolution condition: add content-level negative proof. Either enumerate every
currently frozen-out group with complete path/content indicators and prove its
absence, as `8hn8` requested, or define an independently reviewable byte-level
derivation that proves every resulting change—including conflict-resolution
content—is derived only from the allowlisted commit diffs. A path-set union
alone is insufficient.

## A-FSR-R7-1 — MR description is stale

Check status: reviewed

Outcome: needs-review

Requirement strength: recommended

Impact: minor

Enforcement: advisory

Finding state: open

Risk disposition: none

Basis: MR !1503's live description still names exact head `9fba376b`, says six
rounds, and says only infosec remains. The MR is now at `60717ba6`; both named
consultations need current-head dispositions after this substantive revision.

Resolution condition: update the description to the exact current head and
accurately list the open round-seven findings and both outstanding named
consultations.

## Verified clear in this revision

- `agov plan validate` passes at exact head `60717ba6`.
- `git diff --check 9fba376b..60717ba6` passes.
- `A-FSR-FRESH-1` is resolved: the exact protected-tag-rule object is now
  re-read immediately before tag push and after publication.
- The revision accurately documents GitLab's source-only `sha` binding; the
  remaining problem is authority/control adequacy, not disclosure accuracy.

Because both blockers concern the rule's claimed guarantees, Gate 1 should not
proceed at this head. Any substantive revision invalidates current-head review
evidence and requires renewed exact-head consultation.
