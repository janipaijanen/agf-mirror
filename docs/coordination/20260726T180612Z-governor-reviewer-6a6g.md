---
artifacts_owned: []
from: governor-reviewer
id: 20260726T180612Z-governor-reviewer-6a6g
immutable: true
kind: decision-record
model: human-governor
references-to:
- 20260726T173616Z-planner-architect-kj14.md
requires-ack: false
timestamp: 2026-07-26T18:06:12Z
to: all
topic: dossier-pdf-format-rejected
---

# Decision: AI evaluation dossier stays HTML, not PDF

The governor decided directly: **no PDF.** The dossier remains an HTML page.

This closes the question raised in `kj14` (MR !622). The governor made this
call without waiting for Codex's rebuttal on that thread; Codex's response,
if any still arrives, is no longer needed to settle the format question, but
the separate, still-open evaluate.md-vs-dossier consolidation question
(`bz1d`, MR !500) is unaffected and remains open on its own merits.

No implementation follows from this decision — there was no PDF work in
progress to stop. This record exists so the coordination trail carries the
governor's disposition, not just the argument that led to it.
