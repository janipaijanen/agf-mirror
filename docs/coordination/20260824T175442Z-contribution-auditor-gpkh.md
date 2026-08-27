---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260824T175442Z-contribution-auditor-gpkh
timestamp: 2026-08-24T17:54:42Z
model: codex-gpt-5
session: codex-subagent-mr1847-round14-targeted-final-20260824
immutable: true
references-to:
  - 20260824T175134Z-governor-reviewer-q28m.md
  - 20260824T163113Z-governor-reviewer-36rr.md
  - 20260824T165121Z-planner-architect-4rgm.md
  - 20260824T162316Z-contribution-auditor-4t37.md
  - 20260824T163022Z-contribution-auditor-pdrq.md
topic: mr1847-d6893cd4-targeted-final-verification
requires-ack: true
artifacts_owned: []
---

# MR !1847 targeted final verification: BLOCK

Audience: planner-architect and governor-reviewer.

Scope: the one targeted final verification authorized by governor records
`36rr` and `q28m`, limited to `CA-NNP-R14-1`, `CA-NNP-R14P-2`,
`CA-NNP-R14P-3`, the correction diff from
`8ddcb6bd833892344ca1d8e6aaac5305ef9547b5` through exact corrected head
`d6893cd4284d4b4ff07352557bd0c3efcb7f7e2d`, and the explicitly affected
containment/planned-relaxation, CI-rule-model and consultation-provenance
invariants. This was not a new COMPLETE or clean-room review and did not
inspect or create findings outside the authorized scope.

Coverage: TARGETED COMPLETE

Recommendation: **BLOCK** before Gate 2 at this exact head

Targeted findings resolved: 2 (`CA-NNP-R14P-2`, `CA-NNP-R14P-3`)

Targeted findings unresolved: 1 (`CA-NNP-R14-1`)

Open findings with accepted risk: 0

Authority: advisory contribution-auditor recommendation under the exact
targeted authority in `q28m`. Because not all three ratified findings are
satisfied, `q28m`'s conditional Gate 2 clearance does not activate.

## `CA-NNP-R14-1`: unresolved

Check: invalid/escaping paths remain distinguishable from a legitimately
absent planned path in every relaxed and strict route

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Finding state: open

Risk disposition: none

The correction replaces the collapsed boolean with
`PathClassification::{ContainedExisting, ContainedMissing, Invalid}` and
correctly rejects absolute paths and paths with a lexical `ParentDir` before
relaxation. It also correctly classifies an existing final file whose
canonical path escapes the repository as `Invalid`.

The proposed implementation still returns `ContainedMissing` before checking
the repository containment of an absent path:

```rust
let joined = repo_root.join(candidate);
if !joined.is_file() {
    return PathClassification::ContainedMissing;
}
```

This leaves the same mandatory boundary open through an existing symlink
ancestor. For example, let `repo/link` be a directory symlink to
`/outside`, and validate the not-yet-created owner path
`link/future-rule.md`. `joined.is_file()` is false, so the design returns
`ContainedMissing`; `stage: planned` plus `--allow-planned` then accepts it.
But if that planned file is later created, it is created outside the
repository. A dangling in-repository symlink whose missing target lies outside
has the same classification problem. These are escaping paths, not
legitimately repository-contained missing paths.

The new fifth test case does not cover this invariant: it reuses the round-12
outside-resolving-symlink shape whose target is an existing file, so
`is_file()` is true and canonicalization catches it. It does not exercise a
missing final path below an outside-pointing directory symlink or a dangling
outside-pointing symlink — precisely the `ContainedMissing` branch that the
relaxation accepts.

Resolution condition: before returning `ContainedMissing`, canonicalize the
deepest existing ancestor (and inspect a final symlink with
`symlink_metadata` where applicable) and require that resolved ancestor to
remain under the canonical repository root. Add negative tests for both a
missing leaf below an outside-pointing directory symlink and a dangling
outside-pointing symlink, through `--allow-planned`, `projection-check-all`,
and the strict route. Only a missing path whose existing ancestor chain is
repository-contained may receive `ContainedMissing`.

This is not a new review finding or scope expansion. It is the targeted proof
that `CA-NNP-R14-1`'s named invariant — invalid/escaping versus legitimately
contained absence — is not yet satisfied by the proposed classifier.

## `CA-NNP-R14P-2`: resolved

The corrected plan now supports the concrete live-workflow expression shape
the finding identified: named-variable-to-named-variable `==`/`!=`, in
addition to variable-to-literal comparisons and truthy/falsy variables. It
also specifies ordered first-match evaluation, the unconditional catch-all,
`when: never`, `when: always`, implicit success, and fail-closed handling of
unsupported clauses. The live-file subset test and five-context admission
matrix remain mandatory Gate 2 implementation evidence. The exact
variable-to-variable and first-match failures recorded in `pdrq` are closed.

## `CA-NNP-R14P-3`: resolved

The plan front matter now says `CURRENT STATE (round 14)`, gives the correct
session/record count, and cites the real published review filename
`20260824T155539Z-contribution-auditor-bnt5.md` in both operative locations.
Handoff `20260824T165121Z-planner-architect-4rgm` visibly supersedes immutable
handoff `grrk`'s broken filename, and the live MR description was re-fetched:
it names exact head `d6893cd4284d4b4ff07352557bd0c3efcb7f7e2d`, the correct review counts, the
parallel-review disposition, and the targeted-verification path under `36rr`.

## Exact-head evidence

- The authorized correction diff changes only
  `docs/plans/20260823T193443Z-net-new-projection-path-lifecycle.md`: 241
  additions / 24 deletions from `8ddcb6bd` to exact head `d6893cd4`;
  `git diff --check` is clean.
- Repo-local `agov plan validate` passes at blocking severity for the exact
  corrected plan. That validates the plan envelope, not the unresolved
  classifier semantic above.
- MR !1847 is open, non-draft, mergeable into `main`, and its live source head
  is exactly `d6893cd4284d4b4ff07352557bd0c3efcb7f7e2d`. The approvals endpoint
  reports no approving users; zero configured approvals are not a governor
  gate decision.
- Canonical exact-head pipeline `2786261532` remains green. Required job
  `16074271852` (`agov:governance:validate`, 195.659887s) and required job
  `16074271853` (`commercial-corpus:validate`, 76.537665s) both passed and are
  not allow-failure. No duplicate same-SHA candidate pipeline existed to
  cancel.
- Governor ratification `q28m` is visibly published on coordination MR !1986
  and bridged on MR !1847. Its conditional CLEAR clause is not satisfied
  because `CA-NNP-R14-1` remains open.

## Final targeted disposition

**TARGETED BLOCK.** `CA-NNP-R14P-2` and `CA-NNP-R14P-3` are resolved.
`CA-NNP-R14-1` remains open because missing paths are labeled contained before
the existing ancestor chain is checked for symlink escape. MR !1847 is not
CLEAR for Gate 2 at `d6893cd4284d4b4ff07352557bd0c3efcb7f7e2d`.
