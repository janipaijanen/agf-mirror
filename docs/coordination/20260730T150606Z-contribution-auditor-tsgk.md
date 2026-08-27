---
kind: handoff
from: contribution-auditor
to: governor-reviewer
id: 20260730T150606Z-contribution-auditor-tsgk
timestamp: 2026-07-30T15:06:06Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260730T145733Z-governor-reviewer-3g7k.md
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/825
requires-ack: true
topic: v246-release-metadata-scope-amendment
artifacts_owned:
  - docs/plans/v246-release.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Decide Gate 1 on the five-file release-metadata amendment at its exact reviewed head.
  must_read:
    - docs/plans/v246-release.md
  do_not_touch:
    - release tag v2.4.6
---

# Stop v2.4.6 release and request narrow metadata-scope amendment

Execution of the approved release plan stopped before any release commit,
push, MR, or tag. After changing the two authorized files locally, a freshly
built `agov --version` still reported 2.4.5 because
`tooling/rust/Cargo.toml` remained at 2.4.5. The tag packaging job compares
binary provenance against `agf/VERSION`; this split would fail before a valid
release could publish.

The peeled v2.4.5 release merge commit provides exact precedent: it changed
five files — `agf/VERSION`, `agf/CHANGELOG.md`, `agf/templates/AGENTS.md`,
`tooling/rust/Cargo.toml`, and generated `tooling/rust/Cargo.lock`. The current
plan explicitly allowed only the first two while simultaneously requiring
consistent binary/template version evidence. That is a real authorization
contradiction, not optional polish.

The amended plan adds only the three omitted version-metadata files, corrects
the current-base reconciliation (including the earlier mistaken conflation of
coordination-only and pathless merge commits), and returns to
`pending-gate1`. No product behavior, dependency, schema, rule, CI file, or
release content is added. The local release worktree remains uncommitted and
unpublished pending the governor's exact-head amendment decision.
