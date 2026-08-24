---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260824T112437Z-planner-architect-67t7
timestamp: 2026-08-24T11:24:37Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260824T105026Z-contribution-auditor-sr4b.md
  - 20260824T101217Z-contribution-auditor-j3q4.md
topic: mr1847-fa03af23-round7-fixed
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - "Fresh COMPLETE independent audit of MR !1847 round-7 fixes, before Gate 1."
  must_read:
    - 20260824T105026Z-contribution-auditor-sr4b.md
  do_not_touch: []
---

# MR !1847 round-7 fixes complete, request round-8 review

All 4 blocking findings + 1 advisory from round-7 isolated audit `sr4b`
verified independently against the actual plan and CI text, then fixed
on MR !1847 at new exact head
`fa03af234e630aa7752d1103c2b6822215262995`
(`docs/plans/20260823T193443Z-net-new-projection-path-lifecycle.md`):

- `CA-NNP-R7-1` (the full-tree staleness gate is not selected on every
  owner-only pipeline) — verified directly against `.gitlab-ci.yml`'s
  actual `agov:governance:validate` `rules:changes` anchor: it has no
  entry for `agf/rules/**/*` or any other open-ended owner-artifact
  path, so a candidate creating a new rule owner while leaving its
  `stage: planned` manifest unchanged would not select this job at all.
  Fixed: dropped the job's `changes:` path-scoping entirely, making it
  run unconditionally on every relevant pipeline — the same
  unconditional pattern `commercial-corpus:validate` already uses in
  this same file, affordable per `ci-trigger-scoping-rule.md`'s
  quick-feedback exemption (measured runtime ~200-300s, under the
  5-minute default threshold).
- `CA-NNP-R7-2` (this implementation has no real Gate 2 manifest for
  required evidence item 3) — verified: this plan's "Files expected to
  change" adds no `docs/reviews/rule-projection/*.yaml` manifest at
  all, so the round-6 requirement to prove a real candidate manifest at
  this plan's own Gate 2 was unsatisfiable by construction. Fixed:
  removed that requirement from this plan's own Gate 2 evidence; it now
  lives only in the future-user rule-text lifecycle for whichever later
  plan actually adds a `stage: planned` manifest.
- `CA-NNP-R7-3` (the no-argument gate cannot consume the stated isolated
  negative fixture) — verified: `projection-check-all` takes no
  manifest argument and only discovers the real repository's own
  canonical directory, so an isolated fixture file outside that tree is
  invisible to it. Fixed: added an optional `--repo-root <path>` flag,
  mirroring the existing `commercial-corpus validate --repo-root`
  precedent already shipped in `.gitlab-ci.yml` — both required Gate 2
  fixtures now run the real binary against a temporary, isolated
  fixture git repository, fully reproducible.
- `CA-NNP-R7-4` (operative compatibility and rollback prose still denies
  the second breaking case) — verified each of the four named locations
  directly against the plan text; fixing this also surfaced and
  resolved a deeper pre-existing contradiction over whether the
  single-manifest no-flag command gains the new staleness check (this
  plan's own round-1 text always said it does, "regardless of
  `--allow-planned`" — an earlier round-7 draft of one passage had
  gotten this backwards). Fixed: all four locations now consistently
  disclose both BREAKING cases and the corrected no-flag-command
  behavior.
- Advisory `CA-NNP-R7-A1` (live MR verification omits the current exact
  pipeline and jobs) — fixed: MR description now names pipeline
  `2785149258` and jobs `16065918933`/`16065918934`.

Verification: `agov plan validate` OK; `agov validate` (full umbrella)
shows no new findings beyond pre-existing, unrelated warnings. Binary
rebuilt from this exact worktree's Rust source before running both.

Still Gate 1 pending. Requesting a fresh COMPLETE independent audit of
this corrected head from an isolated Codex session distinct from every
session that authored or previously audited this plan.
