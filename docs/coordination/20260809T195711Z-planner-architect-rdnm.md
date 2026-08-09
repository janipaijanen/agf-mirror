---
kind: note
from: planner-architect
to: coder-tester
id: 20260809T195711Z-planner-architect-rdnm
timestamp: 2026-08-09T19:57:11Z
model: GPT-5 Codex
immutable: true
references-to:
  - 20260809T194958Z-planner-architect-brrb.md
topic: v250-final-gate1-role-switch-bridge
requires-ack: false
artifacts_owned: []
---

# Visibility bridge: switch to contribution-auditor for MR !1349

There is intentionally nothing new in the coder-tester implementation queue.
The current release-critical obligation is merged handoff `brrb`, addressed to
`contribution-auditor`: Gate 1 review of MR !1349 at exact head
`53485310a3c4e567b8e7c45988f9b0f9fe17b548`.

The active manifest assigns contribution-auditor plan review to human Jani and
Claude. Claude should switch explicitly:

```bash
agov session resume --role contribution-auditor --base-branch main
agov handoff next contribution-auditor
```

Then claim `brrb`, perform its complete independent review, and publish
COMPLETE/CLEAR or COMPLETE/BLOCK. Do not implement the plan, create release
refs or protection settings, edit version/changelog files, or perform any tag,
package, OCI, GitLab Release, or deployment action. Gate 1 remains binding
Governor authority.

This note does not duplicate or replace the handoff; it only makes the existing
cross-role obligation visible to a Claude session polling as coder-tester.
