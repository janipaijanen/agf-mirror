---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20260726T231147Z-governor-reviewer-mqej
timestamp: 2026-07-26T23:11:47Z
model: human-governor
immutable: true
references-to:
  - 20260726T225814Z-coder-tester-hmh4
topic: mr654-gate1-blocked
requires-ack: false
artifacts_owned: []
---

# Gate 1 blocked: MR !654

The human governor-reviewer instructed the project to proceed with the
supportable dispositions from the live governing session. Gate 1 is BLOCKED
for MR !654 at head `8cfe3bfaad31c82346da48fb3e3ff13067490eef`.

The plan's inventory is not reliable enough to authorize canonicalization. The
live audit found 44 Claude rule files and 47 AGF rule files: 42 shared names,
2 Claude-only, 5 AGF-only, and a 49-file union. After normalizing provenance
wrappers, 26 shared files are equivalent and 16 diverge. This conflicts with
the plan's `44 shared / 39 trivial / 8 divergent` account. The plan also omits
the divergent `workflow/development-workflow-rule.md`, understates adopter
impact from canonical `agf/rules/` changes, proposes carrying foundry/legacy
coordination language into adopter rules without adequate disposition, and
lacks sufficient consultation and Windows/Git symlink portability evidence.

Do not implement or merge the plan. A revision must correct the complete
inventory, classify adopter impact honestly, disposition foundry-only
language, and supply the missing portability and advisory evidence before a
new Gate 1 request.
