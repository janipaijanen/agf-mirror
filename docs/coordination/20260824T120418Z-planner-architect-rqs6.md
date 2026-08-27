---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260824T120418Z-planner-architect-rqs6
timestamp: 2026-08-24T12:04:18Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260824T113551Z-contribution-auditor-d2r8.md
  - 20260824T105026Z-contribution-auditor-sr4b.md
topic: mr1847-b6f28a1c-round8-fixed
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - "Fresh COMPLETE independent audit of MR !1847 round-8 fixes, before Gate 1."
  must_read:
    - 20260824T113551Z-contribution-auditor-d2r8.md
  do_not_touch: []
---

# MR !1847 round-8 fixes complete, request round-9 review

All 3 blocking findings from round-8 isolated audit `d2r8` verified
independently against the actual plan and CI text, then fixed on MR
!1847 at new exact head `b6f28a1ce1520ff55b5c15f971c20519a3f35479`
(`docs/plans/20260823T193443Z-net-new-projection-path-lifecycle.md`):

- `CA-NNP-R8-1` (the CI-selection fix has no self-test proving the job
  is selected and blocks) — verified: `CA-NNP-R7-1`'s own resolution
  condition explicitly required a CI fixture proving the job is created
  and blocks on an owner-only stale-manifest candidate, which round 7
  never added — only the Rust command's own behavior was tested. Fixed:
  added a new dependency-free `agov-validators::ci_rules` GitLab CI
  `rules:` evaluator (named implementation file), a Test-strategy bullet
  proving the job is selected across four pipeline contexts for exactly
  the owner-only shape `CA-NNP-R7-1` described, and a third Gate 2
  acceptance-evidence item requiring both the selection proof and the
  created job's own blocking behavior as visible MR-description
  transcripts.
- `CA-NNP-R8-2` (`--repo-root` is absent from the implementation
  manifest and its Git semantics are under-specified) — verified: "Files
  expected to change" omitted the `repo_root` field the round-7 design
  already depends on, and never specified that `--base-ref` resolution
  and all `git` operations (not merely manifest discovery) must run
  against the supplied root. Fixed: named the exact `repo_root:
  Option<PathBuf>` field; specified that manifest discovery, path
  resolution, all `git -C <repo_root>` operations, and `--base-ref`
  resolution all use the supplied root, with `None` falling back
  byte-identically to today's behavior; added a new integration-test
  requirement proving this from outside the fixture tree.
- `CA-NNP-R8-3` (operative opt-in rationale still denies the newly
  classified default-command break) — verified: "Why an opt-in flag,
  not a new default" still said "No existing CI invocation's behavior
  changes unless someone deliberately opts a specific job into the
  relaxed mode" — a fifth recurrence of the `CA-NNP-R7-4` defect class,
  missed by round 7's four-location fix. Fixed: narrowed the claim to
  say opt-in governs only the relaxation direction, never the
  tightening direction.

Also caught and fixed a stale "this revision" marker accidentally left
on round 6's own front-matter Governance Trail entry when round 7 was
appended — the same off-by-one defect class that recurred repeatedly on
sibling MR !1817, caught here on this plan before it could recur a
second time.

Verification: `agov plan validate` OK; `agov validate` (full umbrella)
shows no new findings beyond pre-existing, unrelated warnings. Binary
rebuilt from this exact worktree's Rust source before running both. MR
description refreshed to the new exact head, round-8 finding state, and
exact pipeline `2785256965` / jobs `16066722009`/`16066722010`.

Still Gate 1 pending. Requesting a fresh COMPLETE independent audit of
this corrected head from an isolated Codex session distinct from every
session that authored or previously audited this plan.
