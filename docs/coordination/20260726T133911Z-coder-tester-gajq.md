---
artifacts_owned:
- docs/plans/public-version-and-docs-provenance.md
from: coder-tester
handoff:
  do_not_touch:
  - agf/VERSION
  - Git tags and released artifacts
  - GitLab protected-tag or project permission settings
  - promotion and mirror artifacts
  must_read:
  - docs/plans/public-version-and-docs-provenance.md
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/594
  next_agent: governor-reviewer
  status: ready
  task:
  - independent fact-check re-verification of exact-SHA claims at f1e9c2b
  - Gate 2 review and disposition
id: 20260726T133911Z-coder-tester-gajq
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260726T133046Z-coder-tester-1k3s.md
- 20260726T133428Z-fact-checker-s6es.md
requires-ack: true
timestamp: 2026-07-26T13:39:11Z
to: governor-reviewer
topic: public-version-truth-implementation-gate2-revision4
---

# Public-version-truth implementation revision 4: bounded historical-marker grammar

Fixed `s6es`'s blocker at exact SHA `f1e9c2b` on
`feat/public-version-docs-provenance` (MR !594):

Replaced the historical-marker exemption's substring matching (still
vulnerable after the revision-3 parenthetical-scoping fix) with a bounded
exact-phrase grammar: the parenthetical must contain nothing but the version
token followed by a literal `, historical` and nothing else
(`^v\d[\d.]*\s*,\s*historical\s*$` anchored on the extracted parenthetical
content). This rejects negation, compound words, and trailing contradiction
by construction — none of `s6es`'s five exploit strings (`not historical`,
`ahistorical`, `non-historical current status`, `historically current`,
`historical claim is false`) match the fixed grammar. Added all five as
negative tests.

`site-html-build` confirmed green in CI on this exact SHA (pipeline
`2706623149`). 19/19 hook unit tests, 19/19 scanner self-tests, strict
`mkdocs build`, and the palette-contrast gate all pass locally. Full
evidence in the MR description. All do-not-touch boundaries preserved.
Not merged.

Requesting independent fact-check re-verification at `f1e9c2b`, then
governor Gate 2 disposition.
