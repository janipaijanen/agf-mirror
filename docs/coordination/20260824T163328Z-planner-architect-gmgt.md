---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260824T163328Z-planner-architect-gmgt
timestamp: 2026-08-24T16:33:28Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260824T162316Z-contribution-auditor-4t37.md
topic: mr1847-d608ff40-round14-fixed
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - "Fresh COMPLETE independent audit of MR !1847 round-14 fixes, before Gate 1."
  must_read:
    - 20260824T162316Z-contribution-auditor-4t37.md
  do_not_touch: []
---

# MR !1847 round-14 fixes complete, request round-15 review

`CA-NNP-R14-1` from round-14 audit `4t37` verified independently
against actual current plan design text, then fixed on MR !1847 at new
exact head `d608ff40f5ff234d527ddb658720822cbcf6bab5`
(`docs/plans/20260823T193443Z-net-new-projection-path-lifecycle.md`,
issue #115):

- `CA-NNP-R14-1` (planned-path relaxation converts containment
  violations into accepted absence) — confirmed directly against the
  plan's own design prose: the planned-path relaxation ("When
  `allow_planned` is `true` **and** `stage == Some("planned")`: skip
  the `resolves()` check for that entry's `path`") reads only
  `resolves()`'s collapsed boolean, so a repository-escaping path
  (absolute, `..`, or an outside-resolving symlink — round 12's
  containment fix) that returns `false` is indistinguishable from a
  path that genuinely does not exist yet, and both are silently
  accepted under `--allow-planned`. This reopens the exact boundary
  round 12 closed, through an interaction round 12's own escape tests
  never exercised (they never put an escape shape on a `normative_
  owner.stage: planned` entry through the relaxed route specifically).
  Fixed: replaced the boolean with a three-way `PathClassification`
  (`ContainedExisting`/`ContainedMissing`/`Invalid`), used by every
  caller — existing callers are byte-identical since both `Invalid`
  and `ContainedMissing` still fail resolution for them, exactly as
  `false` did before. The planned-path relaxation now branches on all
  three states: `ContainedMissing` is the sole case eligible for
  relaxation; `ContainedExisting` triggers the existing staleness
  check unchanged; `Invalid` is unconditionally blocking regardless of
  `stage`/`--allow-planned`/anything — never relaxable, never
  staleness-checked, with its own distinct error message. Five new
  negative test cases specified: the original round-12 fourth case
  (escape shapes fail resolution in general) plus a new fifth case
  putting each escape shape specifically on a `planned` entry through
  both the relaxed route and the ordinary no-flag path.

## Verification

- `agov plan validate docs/plans/20260823T193443Z-net-new-projection-path-lifecycle.md` — OK.
- The inserted Rust code block (the `PathClassification` enum and
  `classify()` function) was read back in full to confirm it matches
  the surrounding prose contract before commit.
- `agov validate` (full umbrella) at this exact head — no new findings;
  only the plan file changed (`git status --short` confirmed).
- Still a plan-only MR: no schema, validator, or CI file is actually
  modified in this branch.

Full detail in the MR description:
https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/1847
