---
kind: handoff
from: coder-tester
to: fact-checker
id: 20260808T180127Z-coder-tester-pg6h
timestamp: 2026-08-08T18:01:27Z
model: codex-gpt-5
immutable: true
references-to: []
topic: piyaz-sources-radar-exact-head-factcheck
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: fact-checker
  status: ready
  task:
  - Independently fact-check MR !1241 at exact head 4c6b3480feefab11e85c1ed5791f40813400dbf9 against the pinned Piyaz v0.4.13 repository and official documentation.
  - Return CLEAR or finding-specific BLOCK to governor-reviewer before Gate 2.
  must_read:
  - MR !1241
  - MR !1240
  - docs/sources.md Batch 18 on MR !1241
  - docs/radar.md external-context-graph entry on MR !1241
  - https://github.com/FrkAk/piyaz/tree/9a7a0fd363daa0a954ee05e9cc4be98c042b9fbd
  - https://docs.piyaz.ai/docs/using-piyaz/track-execution/
  - https://docs.piyaz.ai/docs/using-piyaz/review/
  - https://docs.piyaz.ai/docs/reference/mcp-overview/
  do_not_touch:
  - .claude/settings.local.json
  - MR !1241 implementation branch
  - Piyaz hosted MCP endpoint or plugin installation
---

# Exact-head fact-check request: Piyaz sources/radar classification

Review MR !1241 exact head
`4c6b3480feefab11e85c1ed5791f40813400dbf9` for source fidelity, attribution,
evidence limits, and maturity wording. The implementation adds only
`docs/sources.md` Batch 18 and one `docs/radar.md` item.

Please verify in particular:

- v0.4.13 maps to pinned commit
  `9a7a0fd363daa0a954ee05e9cc4be98c042b9fbd`;
- the PostgreSQL-backed graph, nine ref-first MCP tools, execution records,
  dependency/downstream behavior, and hosted/self-hosted licensing claims;
- the read-only review agent's plan/diff/CI/acceptance-criteria behavior,
  verdict values, five lenses, lack of status/write authority, and default
  human completion boundary;
- the optional `ask-each`/`auto-on-approve` divergence;
- that statements described as unverified or self-reported are not accidentally
  stronger than the primary sources support;
- that the radar question is non-binding and grants no external tool AGF gate
  authority.

The pinned tree and all three official documentation URLs resolved during
implementation. `git diff --check` and batch/entry uniqueness checks passed.
This handoff requests advisory factual evidence only; binding Gate 2 and merge
remain with governor-reviewer.
