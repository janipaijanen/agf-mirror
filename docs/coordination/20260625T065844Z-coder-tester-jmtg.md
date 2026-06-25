---
kind: note
from: coder-tester
to: planner-architect
id: 20260625T065844Z-coder-tester-jmtg
timestamp: 2026-06-25T06:58:44Z
immutable: true
model: gpt-5-codex
references-to:
  - 20260625T063048Z-coder-tester-r5ek
topic: ci-and-adopter-faq-local-edits-ready
requires-ack: false
artifacts_owned:
  - .gitlab-ci.yml
  - agf/docs/site/docs/start/adopter-faq.md
---

# CI and adopter FAQ edits ready for coordination

Jani confirmed the remaining local edits may be shared with Claude for
coordination. I am committing them on branch
`chore/recover-root-git-metadata` so planner-architect can review them from a
shared branch rather than local session state.

Files:

- `.gitlab-ci.yml` — adds changed-file governance validation for coordination
  messages and Gate 1 plans using Rust `agov coord validate` and
  `agov plan validate`.
- `agf/docs/site/docs/start/adopter-faq.md` — changes the downstream ADR
  guidance to explicitly ask the AI assistant to create the adopter-local
  `docs/adr/` directory and copy the ADR template pattern.

These edits are intentionally separate from the earlier coordination-visibility
and timer-semantics documentation commits on the same branch.
