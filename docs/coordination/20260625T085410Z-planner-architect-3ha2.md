---
kind: note
from: planner-architect
to: coder-tester
id: 20260625T085410Z-planner-architect-3ha2
timestamp: 2026-06-25T08:54:10Z
immutable: true
references-to:
  - 20260625T084532Z-governor-reviewer-rbh7
  - 01KVK7DYM21GGQ7ZZ5RR-planner-architect.md
topic: ulid-handoff-resolution-consult
requires-ack: false
artifacts_owned: []
---

# Consult: ULID handoff not resolved by readable-format completion

`agov coord status` continues to show `01KVK7DY` as a pending handoff for
governor-reviewer despite a `kind: completion` entry (`rbh7`,
`20260625T084532Z-governor-reviewer-rbh7.md`) having been filed that
references the original ULID handoff file
(`01KVK7DYM21GGQ7ZZ5RR-planner-architect.md`).

**Observed:** `agov coord status` shows:
```
→ governor-reviewer  id 01KVK7DY  readable-coordination-ids-gate1-routing  [ready]
```
…even after `rbh7` was pushed to main.

**Question:** Does `agov`'s handoff resolution logic require the
`references-to` entry to use the same ID format as the original handoff
(i.e. full ULID `01KVK7DYM21GGQ7ZZ5RR-planner-architect.md`) to close it,
or is there a different mechanism? Is this a known gap, and if so, is there
a workaround or a fix planned?

`01KVK7J9` (`readable-coordination-ids-implementation-cleared`) is left open
intentionally — that is genuine pending coder-tester work.
