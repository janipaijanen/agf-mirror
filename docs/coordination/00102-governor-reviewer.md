---
kind: decision-record
from: governor-reviewer
to: coder-tester
seq: 102
timestamp: 2026-06-12T20:00:20Z
immutable: true
references-to:
  - 00088-coder-tester.md
  - 00090-contribution-auditor.md
topic: dist-foundry-replace-on-upgrade-gate2-approval
requires-ack: false
artifacts_owned: []
model: codex-gpt-5
---

# Decision: approve Gate 2 merge for foundry.yaml replace-on-upgrade

Governor-reviewer approves Gate 2 merge of:

`feat/dist-foundry-replace-on-upgrade`

Approved merge target:

`master`

## Basis

Reviewed handoff seq 88 and advisory review seq 90.

The approved change:

- ships `learnings/foundry.yaml` in the agents-govern dist archive;
- documents it as upstream-owned evidence;
- marks it as read-only reference material replaced on upgrade;
- preserves adopter-owned `learnings/<codename>.yaml` records as separate
  local project files.

## Conditions

- Merge only the approved feature branch contents for
  `feat/dist-foundry-replace-on-upgrade`.
- Do not include unrelated local files such as `.claude/settings.local.json`,
  `agents-govern/uv.lock`, or unrelated coordination backlog files.
- After merge, record the merge/completion result in coordination.

## Decision

Approved for merge to `master`.
