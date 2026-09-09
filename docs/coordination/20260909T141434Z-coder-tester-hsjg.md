---
kind: handoff
from: coder-tester
to: planner-architect
id: 20260909T141434Z-coder-tester-hsjg
timestamp: 2026-09-09T14:14:34Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260909T135843Z-governor-reviewer-6sj1.md
  - 20260909T124606Z-governor-reviewer-sh3m.md
  - 20260909T132128Z-planner-architect-aars.md
topic: v211-remove-issue129-and-unblock-qbka
requires-ack: true
artifacts_owned:
  - docs/plans/20260909T021841Z-v211-release-scope.md
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - >-
      Reconcile the approved v2.11 scope plan with governor decision `6sj1`:
      remove issue #129 as an IN-scope workstream and freeze dependency, state
      that no semantic/linked-data assessment or implementation is pursued,
      and make qbka Slice A directly eligible for its separately governed
      planning sequence.
    - >-
      Treat this as a material scope/dependency amendment: obtain the required
      non-author exact-head review and return the revised plan to the governor
      for a binding exact-head decision. Do not infer implementation, merge,
      release, or tag authority from the withdrawal.
    - >-
      Preserve Claude's current CP-B ownership from `aars`; sequence or hand
      off the v2.11 amendment without silently abandoning MR !2610.
  must_read:
    - docs/coordination/20260909T135843Z-governor-reviewer-6sj1.md
    - docs/plans/20260909T021841Z-v211-release-scope.md
    - docs/coordination/20260908T185809Z-planner-architect-pe1k.md
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/2610
  do_not_touch:
    - linked-data or semantic-data assessment, implementation, namespace, schema, CLI, or public documentation
    - qbka implementation before its own approved Gate 1
    - release version, tag, package, OCI, or GitLab Release state
    - .claude/settings.local.json
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Reconcile v2.11 scope after issue #129 withdrawal

The current approved release plan still says #129 precedes and blocks qbka
Slice A. Decision `6sj1` explicitly withdraws that lane and says it must not
block qbka or other v2.11 work. The plan therefore needs a truthful
scope/dependency amendment; leaving the stale row in place would make the
release cut impossible even though the governor removed the work.

This handoff changes no plan or product file. It routes the amendment to the
active Claude planner lane and preserves every normal gate boundary.
