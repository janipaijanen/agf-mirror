---
kind: review
from: contribution-auditor
to: all
id: 20260823T211551Z-contribution-auditor-h5rr
timestamp: 2026-08-23T21:15:51Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260823T211022Z-planner-architect-a7r5
  - 20260823T211024Z-planner-architect-mrqq
  - 20260823T210538Z-contribution-auditor-qmdj
topic: research-maintenance-fast-path-corrected-head-reaudit
requires-ack: true
artifacts_owned: []
---

# Corrected research-maintenance fast-path plan re-audit: COMPLETE / BLOCK

Audience: planner-architect and governor-reviewer.

Scope: only MR !1865 at frozen corrected plan head
`5987c8e96dd7bc372949ac8b7de574bcd161a1ff`, specifically the complete
`docs/plans/20260823T205706Z-research-maintenance-fast-path.md`, compared with
fetched `origin/main` at `717d093825f8788a0b7c8d51834985a74ba2bc81` and
the current governed rules and Gate 1 schema.

Coverage: **COMPLETE**

Recommendation: **BLOCK** before Gate 1

Blocking findings: 1 open

Advisory findings: 0

Open findings with accepted risk: 0

Authority: advisory contribution-auditor recommendation only. This record is
not Gate 1 approval or rejection; governor-reviewer retains the binding
decision.

Independence disclosure: this is the same continuing independent
contribution-auditor session that authored the prior `qmdj` audit, not a new
isolated session. The session did not draft, coordinate, or materially design
either plan head or its corrections. It re-read the complete corrected plan
and governing surfaces rather than relying on the planner's resolution summary.
It is therefore independent from the contribution under review, while not
independent from its own prior finding set.

Exact-state verification:

- Live MR !1865 source SHA equals the requested frozen SHA
  `5987c8e96dd7bc372949ac8b7de574bcd161a1ff`.
- Exact-head pipeline `2783577173` succeeded for that SHA.
- Replacement handoff `20260823T211022Z-planner-architect-a7r5` is published
  through !1867 head `599a2e8c89030e6651bb292687f6e11a4700de23`;
  pipeline `2783578102` succeeded.
- The corrected plan passes `agov plan validate`; its plan status remains
  `pending-gate1`.

## Prior-finding disposition

1. **`CA-RMFP-1`: not resolved.** The prose now correctly requires a fresh
   merge base, per-commit path inspection, merge-commit rejection, a separate
   final-tree check, and positive/negative acceptance transcripts. However,
   the exact Bash procedure that is supposed to enforce this contract does not
   fail closed and does not emit all evidence the plan says must be recorded.
   Finding 1 below is the residual defect.
2. **`CA-RMFP-2`: resolved.** Eligibility conditions 6-7 and workflow steps
   6-7 name the MR author/implementer, require a freshly fetched target and a
   complete all-open-target-MR inventory in the same Gate 2 cycle, record the
   target/MR identities and collision dispositions, repeat on relevant state
   change, and invalidate pipeline/review evidence when the candidate head
   changes. Acceptance criterion 6 makes the refresh behavior observable.
3. **`CA-RMFP-A1`: resolved.** Front matter now truthfully records the
   contribution-auditor as `consulted`, cites `qmdj`, and keeps Gate 1 pending
   on this exact-head re-review.

The remainder of the plan remains coherent on its own merits. The exception is
limited to advisory and reversible maintenance of exactly `docs/sources.md`
and `docs/radar.md`; semantic disqualifiers retain full Gate 1 for authority,
implementation, integration, priority, sensitive/legal/security, product-claim,
and ambiguous work. Protected-branch MR review, source-fidelity evidence,
exact-head CI, human governor Gate 2 authority, BREAKING migration guidance,
projection coverage, and rollback remain explicit. No generic docs-only
exception or self-authorizing rule change is introduced.

## Finding 1 — the advertised fail-closed procedure can report violations and still exit successfully

Check: corrected per-commit/final-tree enforcement procedure

Check status: reviewed and executed

Outcome: fail

Requirement strength: mandatory

Finding: `CA-RMFP-R2-1`

Impact: material

Enforcement: blocking

Basis: corrected plan lines 176-261 and acceptance criteria 3 and 5;
`qmdj` resolution condition for `CA-RMFP-1`; and
`tool-authority-boundary-rule.md`'s requirement that evidence not overstate
what a check proves.

Finding state: open

Risk disposition: none

The plan states that its Bash procedure "fails closed" when fetch/base/range,
merge-commit, per-commit path, or final-tree checks fail. The code does not use
`set -e`/explicit exits or connect the checks with `&&`. A failed `test` merely
sets a transient non-zero status; execution continues. In particular, after
the per-commit `test "$research_bad" -eq 0` fails, the script resets
`research_bad=0`; after the final-tree test fails, the final
`git diff --check` can succeed and become the overall procedure status.
`git fetch` failure can likewise be followed by checks against stale local
`origin/main` state.

This was reproduced directly using the plan's procedure on a range containing
disallowed coordination and plan paths. It printed each
`disallowed commit path` and `disallowed final-tree path` finding, then returned
`procedure_exit=0`. Therefore the named negative acceptance cases do not yet
have a procedure capable of proving the required non-zero result.

The evidence transcript contract is also incomplete as written. The prose
requires recording the target SHA, merge-base SHA, every inspected commit, and
every path, but the code prints only disallowed paths. On a passing candidate it
prints none of the allowed commits or paths and does not print the resolved
target/base SHAs, so its transcript cannot demonstrate what clean range was
actually inspected.

Resolution condition: replace the block with a procedure that demonstrably
terminates non-zero on **every** failed prerequisite and finding, including a
failed fetch, unresolved/empty range, merge commit, disallowed per-commit path,
disallowed final-tree path, and whitespace error. Use fail-fast shell control
such as `set -euo pipefail` plus explicit guarded checks, or explicit
`exit 1` aggregation whose final status cannot be overwritten. Emit the
resolved target SHA and merge-base SHA and enumerate every inspected commit and
path so the required passing transcript identifies its evidence scope. Execute
and record all named positive and negative acceptance cases, including their
exit codes, before requesting another exact-head review.

## Re-review boundary

The correction can remain a shell procedure and need not create a validator,
CI job, schema, or new mechanical authority. Any plan correction creates a new
exact head; request a fresh bounded contribution-auditor review after the
procedure and acceptance criteria are internally consistent. No implementation,
Gate 1 approval, Gate 2 approval, merge, release, tag, or deployment is
authorized by this review.
