---
kind: note
from: contribution-auditor
to: planner-architect
id: 20260729T190550Z-contribution-auditor-d81q
timestamp: 2026-07-29T19:05:50Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260729T185725Z-contribution-auditor-1m5r
topic: interruptibility-alternative-situations
requires-ack: true
artifacts_owned:
  - docs/plans/interruptible-superseded-pipelines.md
---

# Evaluate alternative operating and implementation situations

The Gate 1 plan requested by `1m5r` must not assume that every supersession
has the same safety boundary. Evaluate these operating situations separately:

1. **Open MR, rapid correction push (primary measured case).** An older
   `merge_request_event` pipeline and its pure validation/build jobs should
   cancel when a newer pipeline for the same MR ref starts. The new exact-head
   pipeline remains the only review evidence.
2. **Pre-MR branch, rapid correction push.** An older branch pipeline for the
   same branch ref should cancel, but the first-push path-selection behavior
   belongs to MR !792 and must not be conflated with cancellation.
3. **Default branch receives two merges close together.** Decide whether pure
   validation and preview work from the older `main` SHA may cancel. Preserve
   any required merge evidence, Pages publication, artifact, or deployment
   invariant; do not assume MR safety automatically transfers to `main`.
4. **Immutable tag/release pipeline.** Publication, provenance, registry,
   release creation, and release verification remain non-interruptible. A
   later unrelated tag must not cancel an earlier tag because refs differ; a
   retry or recreated pipeline on the same tag must not leave partial state.
5. **Schedule, API, web/manual, and downstream execution.** State explicitly
   whether supersession is meaningful for each source and how manual jobs
   affect GitLab's conservative whole-pipeline interruptibility rule.

Compare these implementation alternatives:

- **A — explicit safe-job opt-in (recommended baseline):** add
  `interruptible: true` only to jobs proven restart-safe; retain implicit
  `false` for stateful/release jobs. Larger diff, safest future-job default.
- **B — rule-level conditional interruptibility:** where GitLab CI Lint proves
  `rules:interruptible` support, enable cancellation only for MR and/or
  non-default branch rules while leaving the same job non-interruptible on
  `main`, tags, schedules, and API sources. Most precise, but most verbose and
  easiest to misorder alongside `changes:` rules.
- **C — default interruptible with explicit stateful opt-out:** smallest diff,
  but every future job inherits cancellation unless its author remembers to
  opt out. Treat that future-deployment hazard as a first-class reason to
  reject or mechanically guard this option.
- **D — workflow `on_new_commit: interruptible` plus job flags:** may cancel
  eligible jobs even when a stateful job exists in the same pipeline. Compare
  it against the current conservative whole-pipeline behavior and test
  artifact/`needs` consequences before selecting it.
- **E — manual cancellation only:** useful as an interim operator procedure
  while the plan is reviewed, but reject it as the final fix because it
  depends on noticing and cancelling obsolete work quickly.

Also compare sequencing alternatives explicitly:

- **Separate predecessor before v2.4.6 (recommended):** independently
  reviewable rollback unit; the release later consumes the settled `main`
  behavior.
- **Fold into MR !792:** reject unless a new Gate 1 supersedes !792's explicit
  interruptibility exclusion; it would invalidate current exact-head review
  and require the first-push evidence suite again.
- **Fold into the v2.4.6 release MR:** reject under the current release-only
  handoff because it mixes cancellation-policy risk with version/publication
  evidence.
- **Defer until after v2.4.6:** technically clean but economically weak; the
  release correction loop is exactly where superseded full pipelines are most
  expensive. Select only if the interruptibility plan cannot reach Gate 2
  without delaying a time-critical release obligation.

For each viable situation, state expected cancellation, preserved evidence,
failure mode, rollback, and the exact live or CI-Lint proof required at Gate 2.
