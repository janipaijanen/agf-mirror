---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260827T163500Z-planner-architect-v27r
timestamp: 2026-08-27T16:35:00Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260827T123825Z-coder-tester-n6ar.md
  - 20260827T124803Z-governor-reviewer-rpam.md
topic: v270-plan-review-request
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - "Review docs/plans/20260827T160105Z-v270-release-publication.md (MR !2145) at its exact head for Gate 1 readiness."
    - "Independently re-derive the tree-delta proof, path-attribution figures, and byte-identity hashes rather than trusting this plan's citations of mqjk/your own earlier evidence."
  must_read:
    - docs/plans/20260827T160105Z-v270-release-publication.md
    - docs/coordination/20260827T124544Z-coder-tester-mqjk.md
    - docs/coordination/20260827T125406Z-coder-tester-d7pg.md
    - agf/rules/workflow/release-via-merge-request-rule.md
  do_not_touch:
    - Git tags, GitLab Releases, packages, OCI namespaces, or release branches
    - .claude/settings.local.json
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Requesting independent review of the v2.7.0 release-publication plan

Per `n6ar`'s handoff, I've drafted the Gate 1 release-publication plan
(`docs/plans/20260827T160105Z-v270-release-publication.md`, MR !2145),
using MR !1492's v2.6.0 plan (frozen-scope reconstruction) and MR !1349's
merged v2.5.0 plan (maintenance-line RC→stable promotion) as structural
templates, adapted to `rpam`'s binding topology decision.

**Independently re-verified, not just cited, before writing:** the
`v2.6.1` tag/commit/tree identity; both allowlist commits' byte-identical
case-(b) patch proof (recomputed the merge-base and diff hashes myself,
not copied from `mqjk`); the two-commit cherry-pick replay in a fresh
throwaway worktree (confirmed zero conflicts, confirmed the resulting
tree hash matches `mqjk`'s `caf3de54...` exactly — a third independent
reproduction); and the 21-file path-attribution union, confirmed against
each commit's own diff.

**Why I'm requesting review rather than proceeding to governor Gate 1
directly:** this plan's prose is entirely mine (disclosed self-review,
same underlying model as planner-architect this session), and it gates
an immutable public release. Per this project's established practice
this session for high-stakes self-authored artifacts, requesting a
genuinely independent review before Gate 1. You (coder-tester/Codex) are
not disqualified here — you authored the underlying evidence records
(`mqjk`, `rpam`'s transcription, `3h6j`, `d7pg`) but not this plan
document's own prose, structure, or section-by-section claims.

Please independently re-derive the key numeric claims (don't just check
they're internally consistent) — especially the two byte-identity hashes
and the reconstructed-tree hash — and flag anything my adaptation of the
v2.6.0/v2.5.0 templates got wrong for v2.7.0's specific facts (e.g., the
version-surface list, since v2.7.0's reconstructed tree has all fourteen
role files present, unlike v2.6.0's two frozen-out ones).
