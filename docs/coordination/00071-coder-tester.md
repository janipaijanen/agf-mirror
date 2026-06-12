---
kind: handoff
from: coder-tester
to: governor-reviewer
seq: 71
timestamp: 2026-06-12T19:34:00Z
immutable: true
references-to:
  - 00058-coder-tester.md
  - 00060-contribution-auditor.md
  - 00056-governor-reviewer.md
topic: downstream-impact-rule-gate2
requires-ack: true
artifacts_owned: []
model: claude-sonnet-4-6
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Gate 2 review of feat/downstream-impact-rule"
    - "Merge branch to master with --no-ff if approved"
  must_read:
    - .claude/rules/always/downstream-impact-rule.md
    - spec/rules-template/always/downstream-impact-rule.md
    - docs/plans/downstream-impact-rule.md
    - docs/coordination/00060-contribution-auditor.md
  do_not_touch:
    - agents-govern/src/agents_govern/validate_agents.py
    - .claude/settings.local.json
    - agents-govern/uv.lock
---

# Gate 2 Handoff: downstream-impact-rule (re-filed for visibility)

Re-filing seq 58 Gate 2 handoff. The original (seq 58) was committed only on
`feat/downstream-impact-rule` and not visible in the master working tree.
This copy is filed here so governor-reviewer's polling loop can pick it up.

## Status

- Gate 1 approved: seq 56 (governor-reviewer)
- Implementation: branch `feat/downstream-impact-rule`, 3 commits ahead of master
- CA review: seq 60 — no blocking findings, Gate 2 recommended

## What was implemented

Two files per the approved plan (seq 56):

`.claude/rules/always/downstream-impact-rule.md` — foundry working rule.
Trigger paths, four-type classification table (BREAKING/ADDITIVE/EDITORIAL/
PREVENTIVE with IEEE 14764 equivalents), three-circle impact statement format,
EDITORIAL fast-track, STOP condition.

`spec/rules-template/always/downstream-impact-rule.md` — distributable template.
Same content plus CUSTOMIZE block scoping rule to framework-distributing projects.

## Gate 2 checklist

- [ ] Implementation matches approved plan (seq 56)
- [ ] Classification table: BREAKING / ADDITIVE / EDITORIAL / PREVENTIVE present
- [ ] Three-circle impact statement format present
- [ ] BREAKING STOP condition present
- [ ] EDITORIAL fast-track documented
- [ ] CUSTOMIZE block in template version
- [ ] CA cleared (seq 60, no blocking findings)
- [ ] Merge to master with `--no-ff`

## Non-blocking note from CA (seq 60)

Both files reference `framework.md` in the header without path qualification.
The actual file is `agents-govern/framework.md`. Not a blocker; editorial
cleanup deferred.
