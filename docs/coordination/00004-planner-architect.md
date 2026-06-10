---
kind: note
from: planner-architect
to: all
seq: 4
timestamp: 2026-06-10T22:59:54Z
immutable: true
references-to:
  - 00003-planner-architect.md
topic: from-to-sender-identity-design
requires-ack: false
artifacts_owned: []
---

# Design note: sender identity in coordination messages

Recorded from live discussion during T1-1 Phase A review, 2026-06-10.
No decision reached; open design question for Phase B.

---

## The problem

The `from` and `to` fields in coordination message front matter need to
identify who sent a message and who it is addressed to. Three candidate
designs were discussed.

---

## Design 1: model name enum (`from: claude | human`)

Original schema. Identifies the LLM instance but not the capacity in
which it acted.

**Problem:** locks the protocol to a specific set of models. Adding a
second LLM requires a schema change. Does not say whether Claude sent
the message as planner-architect or coder-tester.

---

## Design 2: role identifier (`from: planner-architect`)

Proposed replacement. Identifies the capacity but not the model.

**Problem:** does not remove ambiguity when one LLM holds multiple
roles simultaneously. When Claude sends a message that touches both
planning and implementation concerns, the sender must pick one role
arbitrarily. The receiver cannot tell which reasoning mode produced
the message without reading the body.

Moves ambiguity rather than resolving it.

---

## Design 3: role@model (`from: planner-architect@claude-sonnet-4-6`)

Carries both role and model instance. Strictly more informative than
either design 1 or design 2 alone.

**Advantages:**
- Disambiguates overlapping roles: `planner-architect@claude-sonnet-4-6`
  and `coder-tester@claude-sonnet-4-6` are distinct senders.
- Accurate audit trail: historical messages permanently record which
  model version sent them, which is correct for an immutable log.

**Problem:** conflates three distinct concepts that may need to be
tracked independently:

| Concept | Definition | Stability |
|---------|------------|-----------|
| Role | Capacity in which the agent acts | Stable across model upgrades |
| Model | LLM instance that executed | Changes on upgrade |
| Session | Specific run that produced the message | Changes every invocation |

`role@model` collapses role and model. It omits session entirely.
Sender identity changes every time the underlying model upgrades, even
when the role and the governing human stay constant.

---

## Better design (not yet specified)

Separate `from` into stable role identity plus optional provenance
fields:

```yaml
from: planner-architect          # role — stable, always required
model: claude-sonnet-4-6         # LLM instance — optional, provenance
session: <session-id>            # run identity — optional, audit
```

`from` carries only what is stable. Model and session are provenance
metadata that matters for audit but not for operational routing.

**Open question:** is the coordination channel primarily an operational
tool (who owns what right now) or an audit trail (who did what with
which model)? The answer determines whether model and session fields
belong here or in a separate audit layer.

Discussion owner flagged: audit trail is a separate concern from the
coordination log. This note is recorded to preserve the reasoning for
Phase B schema design.

---

## Current schema state

`from` and `to` are currently `minLength: 1` strings with no enum —
open to any value. Existing messages use `claude` and `human`.
Convention update (role identifiers) is documented in
`docs/coordination/README.md` but not yet enforced by the validator.

Resolution deferred to Phase B (Elixir validator design).
