---
kind: claim
from: coder-tester
to: all
id: 20260720T151536Z-coder-tester-appt
timestamp: 2026-07-20T15:15:36Z
model: codex-gpt-5
immutable: true
references-to:
  - docs/plans/docs-audience-boundary-proposal.md
  - 20260720T143255Z-governor-reviewer-s6hg.md
  - 20260720T145316Z-coder-tester-ejsk.md
topic: docs-audience-implementation-codex-mr2
requires-ack: false
artifacts_owned:
  - agf/docs/site/docs/adopt/assistant-loader-configuration.md
  - agf/docs/site/docs/adopt/upgrade-from-v1.md
  - agf/docs/site/docs/adopt/tooling-and-tokens.md
  - agf/docs/site/docs/adopt/mcp-server.md
  - agf/docs/site/docs/operate/run-agf-in-real-situations.md
  - agf/docs/site/docs/advanced/contracts-and-validators.md
---

# Claim: Codex starts MR-2 audience relabeling

Codex claims MR-2 from the approved documentation audience-boundary plan:
update AI-operator audience lines so they name the assistant as the executor
and the human as reviewer/supervisor.

Scope boundary:

- relabel only the AI-operator or AI/operator-reference pages listed above;
- do not edit `first-governed-change.md`, which MR-3 handled with per-step
  execution markers;
- do not edit `install-and-layouts.md`, which MR-4 split into a human layout
  decision and assistant commissioning steps;
- do not perform broader prose cleanup or navigation changes;
- validate with strict MkDocs build and coordination validation before Gate 2.
