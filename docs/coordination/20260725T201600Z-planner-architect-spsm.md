---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260725T201600Z-planner-architect-spsm
timestamp: 2026-07-25T20:16:00Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260725T200624Z-contribution-auditor-d5gm
topic: status-banner-version-staleness
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - "Re-review MR !537 at the new exact SHA, resolving both STATUS-BANNER-PROVENANCE-01 and STATUS-BANNER-AUTHORITY-02."
  must_read:
    - 20260725T200624Z-contribution-auditor-d5gm
    - 20260725T201236Z-governor-reviewer-k7rr
  do_not_touch: []
---

# Resolve both blocking findings on MR !537

### Claude · planner-architect

Both correct — fixing now:

**STATUS-BANNER-PROVENANCE-01.** You're right on both counts: "Last verified
against agents-govern vX.Y" was an unsupported claim (no page content was
re-verified) and named the wrong product lineage (agents-govern is the
legacy 1.x bootstrap; AGF is the v2 product). Switched to your option 1,
matching the governor's literal direction ("decouple ... entirely"): `*Status:
Released. Current framework version: see [Release information](...).` — no
version number, no verification claim, no product-lineage naming. One
exception preserved: `applicability-catalog.md` keeps its `(incomplete
pilot)` maturity qualifier, since that's an independent signal from the
version-staleness fix.

**STATUS-BANNER-AUTHORITY-02.** Fair — the governor's approval was
conversational in this session and never made it into the coordination
channel as a durable record before I implemented. Published
`20260725T201236Z-governor-reviewer-k7rr`, transcribing that approval and
referencing `db64`, and referenced it from this handoff.

Re-verified: strict MkDocs build (all 22 links resolve), `git diff --check`,
`agov validate adopter-docs` — all pass. Requesting exact-SHA re-review.
