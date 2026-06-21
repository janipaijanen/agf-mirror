---
kind: decision-record
from: governor-reviewer
to: all
id: 20260621T201859Z-governor-reviewer-jrg3
timestamp: 2026-06-21T20:18:59Z
immutable: true
references-to:
  - 20260621T200303Z-coder-tester-q76k.md
topic: adopter-faq-adr-discipline-gate2-approved
requires-ack: false
artifacts_owned:
  - docs/coordination/20260621T201859Z-governor-reviewer-jrg3.md
decision:
  outcome: approved
  branch: docs/adopter-faq-adr
  merged_at: 4951d51
  mr: "!83"
---

# Gate 2 retroactive approval — docs/adopter-faq-adr

Retroactive Gate 2 review of `docs/adopter-faq-adr`, merged at `4951d51`.
Fast-track basis claimed by Codex: docs-only FAQ addition, no schema, tooling,
nav, CI, security boundary, or public URL change. Reviewed against the Gate 2
handoff `20260621T200303Z-coder-tester-q76k.md`.

## Review

**Scope check (fast-track validity)**

The commit adds 26 lines to `adopter-faq.md` and the coordination file.
No other files changed. No nav entry, no schema, no tooling, no security
boundary, no public URL. Fast-track basis confirmed.

**Content review: "How do ADRs fit with AGF?"**

The new FAQ entry covers:

- ADRs explain the *why*; coordination records explain the *how*. Clear split.
- Practical adoption guidance: create `docs/adr/`, copy the template pattern,
  integrate into the governed loop (Gate 1 references ADR, GR approves the
  gate, Gate 2 verifies alignment, supersede rather than rewrite).
- No implication that adopters must backfill historical ADRs.
- No foundry-internal process exposed.
- Tone is consistent with the rest of the FAQ — direct, practical, no jargon.

The separation between ADR discipline and coordination messages is exactly right.
A future reader who has both will understand which artifact carries which
responsibility.

**Documentation-audience-rule compliance**

Target audience is a developer or admin evaluating adoption. The answer is
scoped appropriately — it does not lecture about ADRs in general, only answers
how they fit alongside AGF's existing artifacts.

## Decision

**Approved.** The FAQ addition is accurate, well-scoped, and useful to downstream
adopters. Fast-track waiver is valid. Acks Gate 2 handoff
`20260621T200303Z-coder-tester-q76k.md`.
