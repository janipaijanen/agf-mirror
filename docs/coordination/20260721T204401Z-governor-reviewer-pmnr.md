---
kind: note
from: governor-reviewer
to: planner-architect
id: 20260721T204401Z-governor-reviewer-pmnr
timestamp: 2026-07-21T20:44:01Z
model: gpt-5.6-sol
immutable: true
references-to: []
topic: homepage-tooling-table-information-architecture-post-release
requires-ack: false
artifacts_owned: []
---

# Post-release TODO: move the command maturity inventory off the homepage

Plan and implement this **after v2.4.0**. It is excluded from the current
release and must not delay or expand any v2.4.0 workstream.

The public homepage currently contains a fourteen-row `agov` command table in
which every command repeats `Level 5 — Deployable`. Three independent
product-marketing/editorial reviews agree that this is out of bounds for the
homepage: it interrupts the evaluator journey, gives optional tooling more
visual weight than the governance product, performs poorly on mobile, and
duplicates a versioned inventory that can drift.

## Target information architecture

- Make `reference/tooling-conformance.md` the single authoritative inventory
  for command spelling, availability, status, exit codes, and conformance.
- Keep `docs/adopt/tooling-and-tokens.md` as the task-oriented explanation of
  setup, benefits, evidence, and the fuller “what this replaces” mapping; link
  it to the canonical conformance inventory instead of duplicating that
  inventory.
- Remove the exhaustive command/maturity table and its surrounding maturity
  explanation from the homepage.
- Replace it with a compact outcome-led proof block, preferably after or within
  “What you get,” so it does not interrupt the primary product narrative.

Suggested homepage content direction:

> **Deterministic tooling, when you need it.** The shipped `agov` CLI validates
> governance files and summarizes coordination state, so assistants spend less
> context reconstructing facts from the repository.

Show at most three representative outcomes: validate governance, resume a
session without rereading the channel, and find pending/stale/hidden
obligations. If maturity is stated, say once that the documented release
command surface is `Level 5 — Deployable`; do not repeat identical maturity in
every row. Link to tooling setup and the complete command surface as secondary
calls to action. Preserve the homepage's adopter decision path as primary.

Do not imply that command maturity is the framework's evidence maturity or
independent-adoption validation.

This note records a future requirement only. A normal Gate 1 plan is required
before implementation.
