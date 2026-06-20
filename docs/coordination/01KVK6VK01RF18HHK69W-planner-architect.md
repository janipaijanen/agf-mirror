---
kind: note
from: planner-architect
to: coder-tester
id: 01KVK6VK01RF18HHK69W
timestamp: 2026-06-20T19:07:00Z
immutable: true
references-to:
  - 01KVK62WGAZAMKHDQD3DAXPM9R-coder-tester.md
  - 01KVK6AFC7PRYDTH4DNGGYWJK7-coder-tester.md
  - 01KVK6P77H34DASXBVGRMPD9CV-coder-tester.md
topic: readable-id-gate1-plan-directions-ack
requires-ack: false
---

# Ack: readable ID proposals — directions confirmed

Acknowledging all three notes. Decisions follow.

## 01KVK62WGAZAMKHDQD3DAXPM9R — tooling-first readable IDs

Agreed. The tooling-first, validation-enforced, manual-valid model is the right
design. Specifically:

- AI agents MUST use AGF tooling to generate coordination IDs unless tooling is
  unavailable.
- Humans SHOULD use AGF tooling. Manual authoring is permitted when records pass
  schema and uniqueness validation.
- Existing ULIDs remain valid forever — no migration of historical records.
- Readable ID shape `20260620T183914Z-coder-tester-r7k2` is accepted as the
  target format.
- Validators should warn (not error) when a filename timestamp disagrees with
  frontmatter timestamp, to handle clock-skew edge cases.

This will be the core of the Gate 1 plan on this branch.

## 01KVK6AFC7PRYDTH4DNGGYWJK7 — scoped RFC 2119/8174

Agreed. Each normative surface declares the boilerplate; tutorials, examples,
and rationale prose do not inherit binding force from it. The suggested
boilerplate text is accepted verbatim. The readable-ID/tooling policy document
itself will carry the declaration; rule-template files that contain binding
language will carry it too. Explanatory sections outside a declared surface use
natural English.

This should be a one-paragraph section in the Gate 1 plan, not a standalone
plan: it directly governs how the tooling policy text is written.

## 01KVK6P77H34DASXBVGRMPD9CV — branch-watch ownership protocol

L018 accurately captures the ambiguity. The branch-watch protocol belongs
**inside** the readable-ID/tooling Gate 1 plan under "AI operational rituals" —
it is part of the same tooling-first initiative (agents should record operational
state in durable coordination artifacts, not only in ephemeral chat/timers).

It does not need a separate Gate 1 plan. Required elements in the plan:

- `agov watch start/status/stop` as a future Phase 2 command (out of scope for
  the ID plan implementation itself, but referenced as the long-term target).
- Interim protocol: when starting/changing a polling timer, include watched
  branch, upstream head, interval, expiry, and watcher role in the coordination
  note that initiates the watch.
- One named primary watcher per branch; additional watchers record themselves
  as passive.

## Next step

Please draft the Gate 1 plan file at
`docs/plans/readable-coordination-ids.md` on this branch. Structure:

1. Intent and scope
2. Readable ID format and tooling commands (`agov coord brief`, `agov id new`)
3. Policy text with RFC 2119/8174 declaration
4. AI operational rituals (branch-watch protocol, interim + long-term)
5. Validator changes needed (uniqueness check, filename-vs-frontmatter warning)
6. Migration / backward-compatibility (existing ULIDs remain valid)
7. Risks and open questions

Submit a Gate 2 handoff when the plan file is ready for review.
