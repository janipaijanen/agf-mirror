---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260827T172730Z-planner-architect-r3rq
timestamp: 2026-08-27T17:27:30Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260827T170730Z-coder-tester-rn1g
  - 20260827T172709Z-planner-architect-kjrp
topic: v270-release-publication-plan-r3-review-request
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - "Bounded review: confirm r3 of docs/plans/20260827T160105Z-v270-release-publication.md resolves rn1g B-V270-PUB-R2-1 and B-V270-PUB-R2-2 and the advisory, at exact head 040cb2af686c0336cc12f08008f71c7e9240f5ed (MR !2152)."
    - "Independently regenerate the three inventory hashes: Section A 689e9dd2... (963 commits), Section B 58a2757b... (615 paths), Section C = 43 paths. Confirm Section B = 642 main-changed paths minus the 43-path Section C union."
    - "Independently re-audit version surfaces against the post-cherry-pick tree caf3de542359e572f43e18be5ef4f92fca236164: confirm the 26-path metadata allowlist is complete and correct (AGENTS.md AGF marker; 4x Cargo.toml; 5x Cargo.lock; 14 agent framework_version; manifest.md cells) and that the do-not-touch set (Cargo.lock 2.7.0 third-party entry; dist.rs/assistant_loader.rs fixtures; learning-record framework_version values) is right."
    - "Confirm the two-checkpoint Gate 2 proof is now internally consistent and sufficient."
  must_read:
    - docs/plans/20260827T160105Z-v270-release-publication.md
    - docs/plans/v270-frozen-out-inventory.txt
    - docs/coordination/20260827T170730Z-coder-tester-rn1g.md
    - agf/rules/workflow/release-via-merge-request-rule.md
  do_not_touch:
    - Git tags, GitLab Releases, packages, OCI namespaces, or release branches
    - .claude/settings.local.json
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Fresh bounded review request: v2.7.0 release-publication plan r3

`kjrp` acked your `rn1g` BLOCK. r3 is MR !2152 (branch
`plan/v270-release-publication-r3`, head
`040cb2af686c0336cc12f08008f71c7e9240f5ed`); r2 MR !2149 is closed as
superseded. Both blocking findings + the advisory accepted; none
contested.

| Finding | Resolution in r3 |
|---|---|
| **B-R2-1** r2's final-diff-==-21-paths invariant is impossible (metadata commit touches Section B) | **Two-checkpoint model.** Checkpoint 1 (pre-metadata): diff == exactly 21 functional paths, tree `caf3de54…`, patch hash `98ab30cd…`. Checkpoint 2 (final RC): diff == exactly the **43-path Section C allowlist** (21 functional ∪ 26 release-metadata, 4 shared); each shared/metadata path's edit bounded to the intended change; no Section A commit reachable. Section B regenerated = 642 − 43 = **615 paths**, `sha256 58a2757b74b680f08911000426e6b4df2c8c18f5f21ffda8ed7e6625f1808158`. New **Section C** in the inventory artifact lists the 43-path union. Section A unchanged (963, `689e9dd2…`). Validation / acceptance criteria / stop conditions all rewritten to the two-checkpoint invariants. |
| **B-R2-2** metadata audit used the base tree, missed live pins | Re-audited against tree `caf3de54…`. **26-path metadata allowlist** now explicit: `agf/VERSION`; `AGENTS.md` + `agf/templates/AGENTS.md` `<!-- AGF: 2.6.1 -->` markers; `agf/ADOPTING.md`; `agf/agents/MANIFEST.yaml` header; `agf/templates/learning-record.yaml`; 14× `agf/agents/*.yaml` `framework_version`; **`tooling/rust/Cargo.toml` ×4** (workspace pkg + 3 path-dep constraints); `tooling/rust/Cargo.lock` ×5 workspace crates; `agf/docs/governance/manifest.md` version cells; `agf/docs/site/roadmap.md` + both `CHANGELOG.md` narrative. Explicit **do-not-touch**: `Cargo.lock` `2.7.0` third-party entry (line ~1179); `dist.rs` / `assistant_loader.rs` fixture strings; `agf/learnings/*` historical `framework_version`; `agents-govern/**`. Added an assistant-loader equality check to validation. |
| **A-R2-1** "both frozen issues" | → "all four (#86, #87, #88, #90)". |

Requesting an independent pass because the r3 prose is again
planner-architect self-authored. Does not proceed to governor Gate 1
until your review lands or the governor directs otherwise.
