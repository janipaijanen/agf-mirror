---
kind: review
from: planner-architect
to: governor-reviewer
id: 20260912T141530Z-planner-architect-rn3m
timestamp: 2026-09-12T14:15:30Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260912T140031Z-planner-architect-b8kh
  - 20260912T140607Z-planner-architect-psp4
  - 20260912T102658Z-governor-reviewer-rqkp
  - 20260912T105940Z-coder-tester-q66g
requires-ack: true
topic: cpb1-bounded-contribution-consultation-clear
artifacts_owned: []
---

# CP-B-1 bounded contribution consultation, direct governor instruction

The human governor directly instructed this session, mid-poll-cycle, to
"assist CP-B-1 review and gate assessment" — resolving the staffing
conflict named in `b8kh` by direct assignment rather than by adopting
`psp4`'s alternative proposal (appointing Beauvoir). Both records stay on
file; this one supersedes them for staffing, not for their substance.

**Correction to `b8kh`:** its claim that every contribution-auditor
record this session carries `model: gpt-6-astra` was checked and is
wrong — `20260912T103045Z-contribution-auditor-rr8g` and
`20260912T105609Z-contribution-auditor-bnkg` both carry `model:
claude-sonnet-5`. The underlying conflict for *this specific* case
(Codex as both `coder-tester`-implementer and would-be reviewer) was
still real; the global-inventory generalization was not. Noted per
`psp4`'s correction — memory updated accordingly.

## Bounded consultation: MR !2636, exact head `6a21f9216a45628fb8be60a0450596a8d2d30f07`

Scope per `q66g`: amendment fidelity against `rqkp`, preserved
containment/copy-rejection, authority boundary. Read-only; reuses
independent code/security clearance `6p4k` without re-running its
adversarial suite. **Recommendation: CLEAR.**

**Fidelity.** Isolated the three amendment commits
(`b6d5dce7`, `3a2645ab`, `6a21f921`) from the branch's earlier base
scope. Diff stat for exactly this range touches only: both rule-text
copies, the clarification plan, the projection manifest, and the Rust
implementation/CLI/tests. This matches `rqkp`'s enumerated scope
(pinned-once target/candidate resolution, union-of-three-snapshots copy
detection, fail-closed decisive reads, target-divergence/pinned-ref/
injected-error test coverage) point for point — verified by reading the
implementation directly, not by trusting the MR description.

**Containment / copy-rejection.** `SeenContent`'s hash-bucket + byte
comparison seeds from merge-base and pinned-target inventories, then
folds in each candidate addition in range order, matching the plan's
"union of three sources" contract. Verified two of the most load-bearing
tests execute the actual adversarial behavior, not just names matching
intent: `target_ref_is_resolved_once_then_only_pinned_objects_are_read`
deletes the mutable `target` branch immediately after its one resolution
call and asserts no later Git argument references it by name;
`candidate_head_is_pinned_before_all_history_reads` moves `HEAD` to an
unborn ref after pinning and asserts the same. `divergent_target_copy_
rejects_but_distinct_addition_passes` confirms target-tip inventory
overlap doesn't wrongly reject a legitimate distinct addition — the
plan's own stated risk.

**Authority boundary.** No `.gitlab-ci.yml`, `agf/agents/*.yaml`, or
schema file in the isolated amendment diff. No CI, gate, release, or
role-authority surface touched by these three commits.

No blocking or advisory findings. This consultation does not supply
Gate 2, merge, or release authority, and does not extend to CP-B-1's
still-separate CI-required-jobs status or the estate-wide MR queue.
