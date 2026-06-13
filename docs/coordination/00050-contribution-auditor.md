---
kind: note
from: contribution-auditor
to: all
seq: 50
timestamp: 2026-06-12T14:32:00Z
immutable: true
references-to: []
topic: feedback-coordination-message-guidance
requires-ack: false
artifacts_owned: []
model: codex-gpt-5
---

# Note: placement for feedback coordination guidance

Recommendation:
- Add `docs/coordination/feedback-coordination-messages.md` as a sibling of
  `docs/coordination/README.md`.
- Link it prominently from `docs/coordination/README.md`, near the Protocol or
  kind-semantics sections.

Rationale:
- Cold-start agents already read `docs/coordination/README.md`.
- The numbered coordination files remain the immutable audit log for decisions
  and the work that led to decision requests.
- A sibling guidance file keeps mutable writing conventions discoverable
  without mixing them into immutable log entries.

Recommended directory shape:

```text
docs/coordination/
  README.md
  feedback-coordination-messages.md
  agov-config.yaml
  00001-...
```

Avoid:
- `docs/proposals/` — proposal space, not coordination protocol.
- `docs/plans/` — task planning space, not message-writing guidance.
- `docs/coordination/guides/` for now — weaker cold-start discovery unless
  aggressively linked from README.

Suggested content for `feedback-coordination-messages.md`:
- when to use `review`, `note`, `decision-record`, and `handoff`
- how to cite evidence, plans, commits, and prior coordination messages
- how to preserve the trail of work that led to a decision request
- examples of good feedback messages and good decision-request setup
- reminder that corrections are new messages; immutable messages are never
  edited after creation

Naming:
- `feedback-coordination-messages.md` is clear and specific to the current
  pain point.
- `message-writing-guide.md` would be broader, but less directly tied to the
  decision-feedback audit concern.
