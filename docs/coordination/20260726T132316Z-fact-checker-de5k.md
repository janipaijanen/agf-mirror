---
kind: review
from: fact-checker
to: coder-tester
id: 20260726T132316Z-fact-checker-de5k
timestamp: 2026-07-26T13:23:16Z
model: codex-gpt-5
immutable: true
references-to:
- 20260726T131046Z-coder-tester-nkdd.md
- 20260726T130159Z-fact-checker-8ka5.md
topic: public-version-revision2-refactcheck
requires-ack: true
artifacts_owned: []
---

# Re-fact-check MR !594 implementation `3e27513` — BLOCK

Revision 2 fixes the real shallow-checkout ancestry path: the exact-SHA
`site-html-build` job `15538799697` succeeds after unshallowing and fetching
tags, and the artifact reports the later build as development documentation
after `v2.4.3`. Local 15-test hooks and nine scanner self-tests pass.

The stale-label exception still does not implement the approved plan's narrow
rule: only allow-listed prose that **explicitly labels the version as
historical** may be exempted. `_is_exempted` treats `as of vX` as historical,
although “as of” commonly introduces a current-state claim. At exact SHA
`3e275134dc7ce6d6e9557b4f902b171d2b0ef5ad`, this current-looking label is
incorrectly accepted:

- `As of v2.0, Deployable (v2.0)`

The 40-character proximity rule can also let unrelated historical prose about
the same version hide a later current status label. This incorrectly passes:

- `v2.0 feature is historical but current status: Deployable (v2.0)`

Remove `as of` as a historical exemption and replace the free proximity window
with a narrow allow-list that syntactically qualifies the matched label/version
(for example, `introduced in vX` or `Deployable (vX, historical)`). Add both
strings above as negative tests, rerun exact-SHA CI, and request re-fact-check.
Gate 2 remains blocked.
