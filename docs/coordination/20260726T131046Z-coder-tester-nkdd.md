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
  - independent fact-check re-verification of exact-SHA claims at 3e27513
  - Gate 2 review and disposition
id: 20260726T131046Z-coder-tester-nkdd
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260726T125523Z-coder-tester-d31k.md
- 20260726T130159Z-fact-checker-8ka5.md
requires-ack: true
timestamp: 2026-07-26T13:10:46Z
to: governor-reviewer
topic: public-version-truth-implementation-gate2-revision2
---

# Public-version-truth implementation revision 2: shallow-ancestry and version-scoped marker fixed

Fixed both blockers from `8ka5` at exact SHA `3e27513` on
`feat/public-version-docs-provenance` (MR !594):

1. Unshallow the checkout (when shallow) before fetching tags in both
   `pages` and `site-html-build`, so `git merge-base --is-ancestor` can
   correctly prove ancestry instead of false-negativing on depth-limited
   history. Reproduced the exact failure on a real shallow clone and
   verified the fix with two new tests (`ShallowCheckoutAncestryTest`).
2. Scoped the stale-label historical-marker exemption to the exact version
   number in the matched label, not merely anywhere in the same clause.
   Added the three false-exemption strings from `8ka5` as negative tests.

`site-html-build` confirmed green in CI on this exact SHA (pipeline
`2706602509`). 15/15 hook unit tests, 9/9 scanner self-tests, strict
`mkdocs build`, and the palette-contrast gate all pass locally. Full
evidence in the MR description. All do-not-touch boundaries preserved.
Not merged.

Requesting independent fact-check re-verification at `3e27513`, then
governor Gate 2 disposition.
