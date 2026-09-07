---
kind: decision-record
from: governor-reviewer
to: all
id: 20260907T134221Z-governor-reviewer-p1j2
timestamp: 2026-09-07T13:42:21Z
model: human
immutable: true
references-to:
  - 20260907T133638Z-contribution-auditor-rt3p.md
  - 20260907T131738Z-contribution-auditor-qdpm.md
  - 20260907T125729Z-contribution-auditor-j34a.md
  - 20260907T130612Z-coder-tester-tkhs.md
  - 20260907T133205Z-coder-tester-kq5q.md
topic: v210-ruw1-mr2500-gate2-approved
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 2 approved — v2.10 RUW-1 (MR !2500)

The governor-reviewer approves Gate 2 for MR !2500
("docs: complete v2.10 universal polling release evidence") at exact head
`ad6006a6f7c4bddbb3673949024dddb6808b3657`. Issued as a binding in-session
directive (2026-09-07, "gate approved. proceed."), transcribed to
coordination.

## Basis

The independent contribution-audit chain covered every head and returned
CLEAR:

- `j34a` @ `79de28a4` — CLEAR, 2 advisory (RUW1-A1 changelog BREAKING/
  migration axis; RUW1-A2 projection-YAML enumerated the wrong Claude
  pointer);
- `qdpm` @ `12b492aa` — RUW1-A1 and RUW1-A2 resolved; 1 new minor advisory
  RUW1-A3 (`.claude/CLAUDE.md` lost its `not_applicable` disposition);
- `rt3p` @ `ad6006a6` — RUW1-A3 resolved with no regression; **0 blocking,
  0 advisory**.

The four-file change stays inside the RUW-1 allowlist (`README.md`,
`CHANGELOG.md`, `agf/CHANGELOG.md`, the universal-polling projection YAML).
Strict `agov plan projection-check`, `readme_stats` (48→50),
`assistant_loader`, `adopter_docs`, and `git diff --check` pass at the exact
head.

## Merge condition and executor

This approval authorizes merging MR !2500 **only after its `ad6006a6`
canonical merge-request pipeline (`2826680261`) is green** — the one
outstanding condition `rt3p` named, and required by
`only_allow_merge_if_pipeline_succeeds`. The merge is executed by the
RUW-1 lane owner (Codex coder-tester) or the governor once that pipeline
passes; the merged MR plus green pipeline is the authorization, no further
confirmation needed.

The branch's merge base is behind `main`; the merge must be a three-way
(`--no-ff`) merge so the main-side coordination records are preserved (MR
API diff confirms four modified files, no deletions).

## Scope

Authorizes merging the RUW-1 quality-closure unit only. It does not approve
RUW-3 (Propeller Stage 3), RUW-4 (#120 child Gate 1), the release-metadata
MR, a tag, or the release. Each remaining RUW package takes its own
exact-head Gate 2.
