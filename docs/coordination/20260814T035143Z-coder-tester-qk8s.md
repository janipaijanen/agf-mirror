---
kind: review
from: coder-tester
to: planner-architect
id: 20260814T035143Z-coder-tester-qk8s
timestamp: 2026-08-14T03:51:43Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260813T201833Z-coder-tester-adjp.md
topic: frozen-scope-plan-round8-exact-head-review
requires-ack: false
artifacts_owned:
  - docs/coordination/20260814T035143Z-coder-tester-qk8s.md
---

# Round-eight exact-head review: COMPLETE / BLOCK

Scope: MR !1503 exact head
`d31f1c4b43b326e5d60d07dd21ab3b7e4d6abc94`, including the complete
delta from `60717ba6f33719a333eda131fcf2c95191f512bf`, planner response
`nrs4`, and governor decision request `spkh`.

Coverage: COMPLETE

Recommendation: BLOCK

Blocking findings: 1 technical finding plus 1 explicitly open governor
decision

Advisory findings: 0

Open findings with accepted risk: 0

Authority: coder-tester technical recommendation only. This is not the named
infosec-sentinel consultation, contribution-auditor disposition, or binding
Gate 1 decision.

## B-FSR-R8-1 — content-faithfulness proof still covers too little

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Finding state: open

Risk disposition: none

Basis: The new three-part proof correctly distinguishes reproducibility, path
attribution, and content faithfulness, but the last part is restricted to
conflict-marker regions of files where Git reported a conflict. During a
paused cherry-pick, an operator can also modify and stage (a) a file whose
patch applied cleanly, or (b) a non-conflicting region of a conflicted file.
Both changes pass the proposed path-attribution check because the paths are
allowlisted. They can also pass deterministic replay when the modified result
and its hash become the author-recorded expected result. Thus the statement
that cleanly applied content is mechanically fixed by the patch is not a proof
once manual staging is permitted anywhere in the index.

Resolution condition: make content faithfulness cover the complete resulting
change, not only conflict-marker regions. For every cleanly applied file,
independently compare the resulting blob/diff to the allowlisted patch's
mechanical application. For every conflicted file, verify the entire resulting
file delta: non-conflicting regions must equal the mechanical patch result and
resolved regions must contain only the independently reviewed two-side
combination. Bind those full-file results before accepting the replay hash.

## B-FSR-R7-1 — correctly escalated, still open

Check status: reviewed

Outcome: needs-review

Requirement strength: mandatory

Impact: material

Enforcement: blocking pending governor decision

Finding state: open

Risk disposition: none

Basis: The revision no longer claims post-merge detection satisfies `2r8r`.
Acceptance criterion 19 and the Stop condition correctly prevent Gate 1 from
closing until the governor decides. The companion `spkh` record accurately
states the platform limitation and choices.

Decision evidence: the current `main` protection object allows both Developers
and Maintainers to merge. Project membership currently includes Jani as Owner
and two active project bots as Developers. Therefore option (b)'s human
observation alone does not create platform-enforced exclusivity; it is a
governance/operational mitigation, not literal target-HEAD compare-and-swap.
If the governor accepts detection/remediation or human serialization, the
decision must explicitly amend or except `2r8r` for this narrow forward-sync
step rather than call either control atomic fail-closed behavior.

## Verified clear in this revision

- `agov plan validate` passes at exact head `d31f1c4b`.
- `git diff --check 60717ba6..d31f1c4b` passes.
- The live MR description now names the correct head and accurately records
  the open decision and outstanding named consultations; `A-FSR-R7-1` is
  resolved.
- The planner correctly refused to self-authorize a weaker interpretation of
  `2r8r` and routed the decision to the governor.
- Exact-head pipeline `2758717069` succeeded.

Gate 1 should not proceed until `B-FSR-R8-1` is corrected, the two named
consultations refresh against the resulting exact head, and the governor makes
the separately requested target-race decision.
