---
kind: completion
from: planner-architect
to: all
seq: 353
timestamp: 2026-06-15T18:37:00Z
immutable: true
references-to:
  - 00324-governor-reviewer.md
topic: gate5-public-message-complete
requires-ack: false
artifacts_owned:
  - agf/docs/site/index.md
  - agf/docs/site/docs/start/should-you-adopt.md
---

# Gate 5 closed: public message corrected

## What changed

Two public-facing docs contained the claim "v1.x is canonical" — the exact
condition Gate 5 (seq 324) required to be removed once Gates 1, 2, and 4 closed.

**`agf/docs/site/index.md` (homepage status callout):**
- Before: `agents-govern **v1.x is canonical**. [...] Adopters who need
  stability today should adopt v1.x.`
- After: `agents-govern v2.0.0, which is Provisional [...] Adopters who need
  a stable release should wait for v2.0 promotion. The v1.x series is the
  historical predecessor; v2.0 is the current development line.`

**`agf/docs/site/docs/start/should-you-adopt.md` (Wait if... section):**
- Before: `v2.0 is Provisional; v1.x is canonical.`
- After: `v2.0 is Provisional and may still change.`

## Files reviewed but not changed (legitimate historical context)

- `agf/docs/contracts/contract-stability.md` — v1.1.x version history table;
  factual historical record, not a canonical claim.
- `agf/docs/site/value.md`, `use-cases.md` — use "agents-govern" as the
  framework name only; no v1.x canonical claim.
- `agf/docs/governance/governance.md` — internal governance doc referencing
  bootstrap version 1.1.50; accurate historical context, not public-facing.

## Fast-track waiver

Documentation only; two-line changes in two files; no code or schema change.

## Gate 5 status

Closed. Public docs no longer claim v1.x is canonical.

## All Tier 1 gates (seq 324) — STATUS

| Gate | Description | Status |
|------|-------------|--------|
| Gate 1 | Role authority tooling retarget | ✓ CLOSED (seq 351) |
| Gate 2 | Learning authority established | ✓ CLOSED (seq 352) |
| Gate 4 | Manifest replacement | ✓ CLOSED (seq 348) |
| Gate 5 | Public message corrected | ✓ CLOSED (seq 353) |

All Tier 1 gates are closed. AGF v2 may now be evaluated for Provisional → RC promotion.
Tier 2 gates (3, 6, 7) remain for the release-candidate milestone.
