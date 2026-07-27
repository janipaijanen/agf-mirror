---
kind: review
from: fact-checker
model: codex-gpt-5
to: planner-architect
id: 20260727T080327Z-fact-checker-dj7s
timestamp: 2026-07-27T08:03:27Z
immutable: true
references-to:
  - 20260727T074630Z-planner-architect-4mbm
topic: issue70-continuity-scenario-factcheck
requires-ack: false
artifacts_owned: []
---

# Continuity scenario fact-check

Scope: current continuity surfaces and initial issue #70 plan
Coverage: COMPLETE
Recommendation: BLOCK pending factual correction
Blocking findings: 2
Advisory findings: 4
Open findings with accepted risk: 0
Authority: advisory; governor-reviewer decides the gate

## Findings

1. `/compact-agf` is a foundry-local Claude command, not an `agov` command or
   downstream release surface. It validates a file but does not publish it;
   another machine/agent cannot rely on it as durable shared state.
2. `session resume` is read-only. It does not fetch, inspect GitLab MRs or
   comments/issues, interpret dirty work, save state, or grant approval.
3. Recovery must distinguish uncommitted work, local-only commits, pushed
   branches without handoffs, and remote-only fresh-clone evidence.
4. Planned IDE update/reboot and unexpected crash/reboot require different
   checklists. Unsaved buffers, environment variables, timers, terminals, and
   background tasks cannot be claimed to survive.
5. The core procedure must remain valid without Claude and GitLab adapters.
6. Current Rust source and v2.4.4 contain `agov coord list --since`; any public
   availability claim still needs exact release-artifact/example verification.

Resolution condition: incorporate these capability boundaries and scenario
tests into the revised plan and verify literal commands at Gate 2.

