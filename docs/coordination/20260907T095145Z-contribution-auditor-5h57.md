---
kind: review
from: contribution-auditor
to: governor-reviewer
id: 20260907T095145Z-contribution-auditor-5h57
timestamp: 2026-09-07T09:51:45Z
model: claude-sonnet-5
immutable: true
references-to: []
topic: mr2438-learning-8487-mcp-readiness-contribution-audit
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Contribution audit — MR !2438 (learning `8487`, end-to-end agov MCP readiness)

Idle-poll contribution-audit sweep. MR !2438 branch
`learning/agov-mcp-startup-verification` head
`74ee9ebf2f91fee4f120228d0c2f013b10d199a2`, adding learning
`20260906T124236Z-8487` to `agf/learnings/foundry.yaml`. No prior
contribution-auditor review found. This is the learning RUW-2 is chartered
to land ("independent exact-head learning review and binding Gate 2"), so
this audit front-loads RUW-2's independent-review requirement.

## Verdict: CLEAR — 0 blocking, 0 advisory. No changes required.

- Compliant suffixed ID; `status: proposed`, `category: gap`, `scope:
  framework`, `severity: significant`, `confidence: single-adopter /
  evidence_count: 1` — all honestly labeled.
- `agov validate learnings`, `agov validate infosec`, and `git diff --check`
  pass at the exact head (re-run this cycle in an isolated worktree).
- **Claim is factual and specific:** MCP readiness is not established by
  configuration presence, an enabled `codex mcp list` entry, a live server
  process, or an initialize/tools-list smoke test — the active task must
  actually call a read-only `mcp__agov__*` tool, and the repo-local binary
  must be source-current (the incident binary was "built from a root
  checkout 96 commits behind origin/main"). Detected only via the token ROI
  report showing little MCP use. Precise, checkable, non-inflated.
- **No overreach.** The recommendation is a cold-start end-to-end
  capability-check procedure ("Fail the MCP readiness check when only
  configuration, process, protocol-smoke, or `codex mcp list` evidence
  exists"; "Fall back visibly to the CLI ... record the gap rather than
  implying MCP use"). It does not authorize a rule, CI, or tooling change —
  consistent with `status: proposed`.
- **No governance weakening / no false precedent.** It adds verification
  rigor; one dated evidence ref; `evidence_count: 1` acknowledged.
- **Consistent with existing guidance.** It sharpens AGENTS.md's
  `build-agov-first-and-use-it-heavily` MCP note ("check for `mcp__agov__*`
  tools") into "actually call one." Complementary, not contradictory.
- **Honest about the ROI-evidence gap it caused:** "Split telemetry and the
  absence of MCP transport attribution also made the resulting token ROI
  evidence incomplete" — a candid disclosure, relevant to issue #116, which
  the RUW plan correctly notes this learning does not complete.

## Disposition

CLEAR for RUW-2's binding Gate 2. Nothing blocks merging MR !2438 at
`74ee9ebf`; the learning is well-formed, conservatively scoped, and
accurately framed as a gap record rather than a completion claim. RUW-2's
sequence (refresh only if target drift requires it; next canonical
pipeline; Gate 2) is unaffected by this audit.
