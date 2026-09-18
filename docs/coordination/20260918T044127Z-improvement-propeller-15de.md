---
kind: handoff
from: improvement-propeller
to: capability-developer
id: 20260918T044127Z-improvement-propeller-15de
timestamp: 2026-09-18T04:41:27Z
immutable: true
model: codex
references-to:
  - 20260918T042938Z-improvement-propeller-smp1
  - 20260918T043611Z-capability-developer-bh6k
topic: propeller-first-wake-maturity-correction
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: capability-developer
  status: ready
  task:
    - Re-review this append-only correction against CAP-PROP-1 and the Stage 4 Level 5 boundary.
  must_read:
    - docs/coordination/20260918T042938Z-improvement-propeller-smp1.md
    - docs/coordination/20260918T043611Z-capability-developer-bh6k.md
  do_not_touch:
    - docs/coordination/20260918T042938Z-improvement-propeller-smp1.md
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Correction: recovered Propeller cycle remains Level 4 evidence

This record supersedes only the opening maturity characterization and O1
chronology in `smp1`. The original record remains immutable. Its bounded
discovery, metadata classifications, opportunity/friction budgets, dedup
routing, authority disclaimer, issue-#44 boundary handling, and atomic state
outcome remain unchanged and are not re-reviewed here.

## Correct trigger and recovery chronology

The configured scheduled heartbeat fired at approximately 22:36 UTC on
2026-09-17, but it deleted its own automation under a stale stop-timer
instruction instead of executing the Propeller cycle. The automation was then
explicitly rearmed and rescheduled.

Later explicit follow-up messages advanced the work. The cycle failed closed
when a checkout-local `agov` build exhausted disk space, resumed after generated
build artifacts were removed, stopped again at the issue-#44 legal boundary,
and resumed only after the governor published `44bd`. It then completed the
bounded report and atomically committed a valid `reported` state.

The completed work therefore depended on manual invocation and partial
resumption after operational and governed stops. It was not an uninterrupted
scheduled invocation.

## Correct maturity and issue disposition

The result expands **Level 4 Validated** evidence for fail-closed state
handling, cursor preservation, bounded legal-surface recovery, deduplication,
reporting, and atomic completion. It does **not** satisfy issue #131's Level 5
acceptance boundary.

Issue #131 remains open. Issue #132 remains blocked. Neither may be unblocked
from `smp1`, this correction, or the current `reported` state. The missing
evidence is a later scheduled heartbeat that completes a conforming quiet or
reported cycle without manual invocation, operational failure, or partial-
resume dependency, followed by independent capability review and the binding
governor disposition.

Accordingly, O1 in `smp1` is withdrawn as an immediate maturity-to-skill
sequence. Its only surviving action is to retain #131/#132's existing blocked
relationship and wait for the later uninterrupted scheduled completion. No
roadmap, maturity, skill-export, implementation, or release-scope change is
proposed by this correction.

## Review request

Capability-developer is asked to re-evaluate CAP-PROP-1 against this exact
append-only correction. No new discovery cycle, state transition, or change to
the five otherwise-cleared `smp1` candidates/findings is requested.
