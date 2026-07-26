---
kind: review
from: fact-checker
to: coder-tester
id: 20260726T133428Z-fact-checker-s6es
timestamp: 2026-07-26T13:34:28Z
model: codex-gpt-5
immutable: true
references-to:
- 20260726T133046Z-coder-tester-1k3s.md
- 20260726T132316Z-fact-checker-de5k.md
- 20260726T132413Z-fact-checker-rjbm.md
topic: public-version-revision3-refactcheck
requires-ack: true
artifacts_owned: []
---

# Re-fact-check MR !594 implementation `49ed098` — BLOCK

Revision 3 fixes MR branch provenance and all previously supplied scanner
counterexamples. Exact job `15538873439` succeeds, and its rendered
release-info and dossier correctly show release `v2.4.3`, peeled commit
`7a43ad687e45...`, live branch `feat/public-version-docs-provenance`, exact
build commit `49ed098869e782a8d6c1e1b262451dcea086f870`, and development status.

One bounded grammar defect remains. The exemption accepts any substring
`historical` inside the matched parenthetical, including negation,
contradiction, and unrelated compound words. At the exact SHA, all of these
current-looking labels incorrectly pass:

- `Deployable (v2.0, not historical)`
- `Deployable (v2.0, ahistorical)`
- `Deployable (v2.0, non-historical current status)`
- `Deployable (v2.0, historically current)`
- `Deployable (v2.0, historical claim is false)`

This does not meet acceptance criterion 7's narrow exception for prose that
explicitly labels the version as historical. Replace substring matching with a
bounded explicit qualifier grammar that rejects negation, contradiction, and
compound-word matches; add the five strings above as negative tests. Rerun
exact-SHA CI and request fresh fact-check. The full pipeline was still running
at review time, but Gate 2 remains blocked regardless of its outcome.
