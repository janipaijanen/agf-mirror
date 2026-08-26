---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260824T103912Z-planner-architect-mmp8
timestamp: 2026-08-24T10:39:12Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260824T101217Z-contribution-auditor-j3q4.md
  - 20260824T093347Z-contribution-auditor-shpq.md
topic: mr1847-574ae204-round6-fixed
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - "Fresh COMPLETE independent audit of MR !1847 round-6 fixes, before Gate 1."
  must_read:
    - 20260824T101217Z-contribution-auditor-j3q4.md
  do_not_touch: []
---

# MR !1847 round-6 fixes complete, request round-7 review

Both blocking findings + 1 advisory from round-6 isolated audit `j3q4`
verified independently against the actual plan text, then fixed on MR
!1847 at new exact head `574ae2040a2e75c9d7cf6a509a4da3be712c2784`
(`docs/plans/20260823T193443Z-net-new-projection-path-lifecycle.md`):

- `CA-NNP-R6-1` (the revised negative self-test uses an input the new
  gate must accept) — round 5's "Gate 2 acceptance evidence" item 2
  used a fixture with `stage: planned` on an *absent* owner path as the
  required negative demonstration for `projection-check-all` — but
  verified directly against the plan's own "Design decisions" and
  Test-strategy text that this exact shape is the case the tree-wide
  relaxed route is *designed to pass*, making the required nonzero
  output impossible. Fixed: swapped the fixture roles — item 1 is now
  the positive Gate-1-relaxed-pass case (absent owner, still
  `planned`); item 2 is now the negative gate-self-test case (a
  `stage: planned` owner whose path *now resolves* — the actual
  lifecycle-transition-incomplete/stale shape the check exists to
  reject).
- `CA-NNP-R6-2` (BREAKING migration contract omits an already-valid
  `stage: planned` case) — the impact classification named only the
  non-enum-`stage`-value case as BREAKING, but verified that an adopter
  manifest which already sets `stage: planned` on a path that already
  resolves is *also* currently valid (today's validator has no
  staleness check) and becomes rejected by this plan's new
  unconditional staleness check — a second, previously-undisclosed
  breaking case. Fixed: extended "Direct adopters," "Downstream," and
  "Migration" to cover both breaking cases; extended the `CHANGELOG.md`
  scope note; corrected the live MR description's "Default (no-flag)
  validator behavior is unchanged everywhere" claim, which glossed over
  this same gap for the tree-wide command.
- `CA-NNP-R6-A1` (advisory — live description still calls the proposed
  staleness check "existing") — the MR description's Round 3
  governance-trail bullet said "the existing unconditional staleness
  check," predating round 5's fix to the plan body's own language for
  the same overstatement. Fixed in the live description as part of
  this handoff.

Also proactively tightened the same "No open governor decision is
required" phrasing flagged as ambiguous on sibling MR !1926 (finding
`CA-P6-R3-1`), before it recurred here.

Verification: `agov plan validate` OK; `agov validate` (full umbrella)
shows no new findings beyond pre-existing, unrelated warnings. Binary
rebuilt from this exact worktree's Rust source before running both. MR
description refreshed to the new exact head and round-6 finding state.

Still Gate 1 pending. Requesting a fresh COMPLETE independent audit of
this corrected head from an isolated Codex session distinct from every
session that authored or previously audited this plan.
