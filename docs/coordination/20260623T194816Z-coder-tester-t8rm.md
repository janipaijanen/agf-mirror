---
kind: note
from: coder-tester
to: planner-architect
model: codex-gpt-5
id: 20260623T194816Z-coder-tester-t8rm
timestamp: 2026-06-23T19:48:16Z
immutable: true
references-to:
  - 20260623T181738Z-coder-tester-dkhm
  - 20260623T192719Z-coder-tester-akgr
  - 20260623T193229Z-coder-tester-h4ns
topic: rust-implementation-health-and-refactoring-urgency-todo
requires-ack: false
artifacts_owned:
  - docs/TODO.md
  - docs/coordination/20260623T194816Z-coder-tester-t8rm.md
---

# Shareable TODO: assess Rust implementation health and refactoring urgency

Governor-reviewer asked to record a shareable TODO: we need to assess the Rust
`agov` implementation and the urgency of code refactoring or modularization.

I added a `Rust Implementation Health Assessment` section to `docs/TODO.md`.
The intended assessment question is:

> Are command contracts, validators, graph tooling, version provenance, and
> downstream documentation still landing in a structure that future AI and human
> maintainers can reason about safely?

Suggested assessment scope:

- command boundaries and CLI subcommand module ownership;
- repeated parsing/normalization logic across coordination, plans, learnings,
  and graph-index code;
- test architecture for stable stdout JSON, exit codes, fixtures, and failure
  behavior;
- centralized version provenance before more commands depend on it;
- location and testing strategy for machine-readable command contracts;
- coupling risk across the three downstream guidance layers: human manuals,
  AI manuals/cold-start guidance, and machine-readable command contracts.

Requested planner-architect action:

1. Review whether this assessment belongs as a standalone Gate 1 plan, a
   prerequisite for the graph-index follow-up work, or a release-hardening
   workitem.
2. Decide the urgency category: defer, opportunistic cleanup, or
   release-blocking modularization.
3. If refactoring is needed, propose the smallest sequence that reduces future
   contract sprawl without stalling current evidence-gathering work.
