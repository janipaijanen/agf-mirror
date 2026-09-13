---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20260913T083739Z-governor-reviewer-s4km
timestamp: 2026-09-13T08:37:39Z
immutable: true
references-to:
  - 20260913T082918Z-coder-tester-jkhg
requires-ack: true
topic: work-item-steward-existing-trigger-integration
artifacts_owned: []
---

# Stewardship uses the existing trigger mechanism

Recorded by Codex as a transcription of the governor's current conversation.
After the proposed author/submission, periodic polling, closure/handoff and
bounded backlog triggers were explained, Jani said: “good idea to attach it
to the existing trigger mechanism”.

Carry this accepted design constraint into the jkhg stewardship plan: attach
the proposed work-item checks to the existing polling mechanism, with one
named sweep owner, a persistent item/head/body cursor, deduplicated findings
and a bounded historical-work budget. Do not create a parallel timer/service
merely to instantiate the role. The plan must make the actual wake path and
inactivity behavior explicit; a role definition alone is not a running monitor.

The preceding discussion also requested proper GitLab templates stored in
Git. Include repository MR and issue templates and a check that GitLab makes
them available. Author/submission and completion/handoff checks complement
the periodic scan; classify mandatory versus advisory checks according to
existing authority rather than inventing an extra merge gate.

This accepts trigger integration for the proposed design. It does not assert
that the role is already active or that an unwritten implementation plan has
passed Gate 1. Continue the existing jkhg planning handoff with these constraints.
