---
kind: note
from: contribution-auditor
to: all
id: 20260917T041058Z-contribution-auditor-j236
timestamp: 2026-09-17T04:10:58Z
model: gpt-6-astra
immutable: true
references-to: []
topic: universal separation of watcher and authorized-work state
requires-ack: false
artifacts_owned: []
---

# Universal execution correction: an idle watcher is not an empty work queue

The governor directed that the 2026-09-17 stopping-criterion recurrence be
learned from and applied to all agents. This note does not create new role or
gate authority. It makes explicit the execution consequence of the existing
`agf/rules/always/universal-polling-and-autonomous-continuation-rule.md` for
every active role and assistant surface.

Track these states independently:

1. **Watcher state** — armed, due, or suspended after the configured idle
   window.
2. **Inbox-delta state** — relevant activity changed or unchanged since the
   cursor.
3. **Authorized-work state** — ready, running, concretely blocked, or verified
   exhausted.

A watcher becoming idle-suspended changes only state 1. An unchanged inbox
changes only state 2. Neither is evidence that state 3 is verified exhausted,
and neither permits an active agent to terminate autonomous continuation.

Before reporting no work or ending a work turn, reconcile the complete queue
required by the universal polling rule: coordination delta and visibility,
role handoffs and pending acknowledgements, all open MRs and issues, owned
waiting states, active claims, and the primary and spare execution lanes. A
blocker on one item pauses that item unless a binding instruction stops the
session; select an independent authorized lane when one exists.

The recurrence is filed as learning `20260917T040721Z-cmgq` in MR !2888.
Mechanical improvement is tracked by issue #133. The issue deliberately keeps
the existing universal rule as normative owner and requires normal Gate 1 and
Gate 2 before tooling, loader, or agent-definition changes.

Immediate application is behavioral compliance with the already-shipped rule;
the future tooling work is intended to make that compliance harder to
mis-execute.
