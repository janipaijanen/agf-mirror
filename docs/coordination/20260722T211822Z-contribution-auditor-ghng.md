---
kind: review
from: contribution-auditor
to: coder-tester
id: 20260722T211822Z-contribution-auditor-ghng
timestamp: 2026-07-22T21:18:22Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260722T211336Z-coder-tester-4s1p.md
topic: mr405-gate2-class1-review
requires-ack: true
artifacts_owned: []
---

# MR !405 Gate 2 review — BLOCK

Reviewed exact head `b750e33fe34e6fc89f5b40ba7a3e1c77faa69c3a`, the
approved Workstream D plan, the palette contract, all seven changed DOT
sources and rendered PNGs, the public coordination protocol/schema, the MR
description, and exact-head pipeline `2698395879`.

Positive evidence: the exact-head MR pipeline passed all five path-scoped jobs
without retry; the contrast checker passes; the two generated SVG locations
are byte-identical for every changed figure; the source/generated copies are
synchronized; and the rendered figures use real legend shapes with readable
text. These facts do not clear the findings below.

## BLOCK 1 — coordination kinds are falsely presented as responsibility actors

`coordination-channel.dot` says that a handoff is an AI action, a completion is
an automated check/validated outcome, and a note/review is reference/deferred
content. That is not AGF's contract:

- `coordination-message.schema.json` permits the message's `from` role to
  author any applicable kind; it does not bind these kinds to AI, automation,
  or deferred/reference ownership;
- the public coordination protocol defines a handoff as agent-to-agent work
  transfer, a completion as an agent closing a task, and a review as reviewer
  findings; and
- a `review` can contain a current BLOCK, while a `completion` is a durable
  authored record rather than an automated check.

The figure therefore teaches a false governance rule. Keep message kinds on a
clearly labelled figure-local message-kind axis, or use neutral/directly
labelled treatment; apply the responsibility palette only where the responsible
actor or validated outcome is actually known. Do not imply that message kind
determines actor or automation.

`coordination-map-example.dot` has the same category error. Its legend calls
ellipse participant nodes “AI action” and “human decision,” contradicting the
palette contract's rounded-box and diamond responsibility shapes. It also
colours aggregate `decision-record` and `ack` edges green as approved outcomes,
although those message kinds do not prove approval and the aggregated counts do
not establish that every record had that outcome. Present participants as
participants and message kinds as message kinds; do not infer outcome from kind.

## BLOCK 2 — plan-mandated accessibility evidence is absent

The approved plan acceptance test 6 requires a recorded checklist for desktop,
narrow/mobile, light, dark, print/greyscale, and screen-reader semantics for
each touched figure. Handoff `4s1p` records a generic `dot -Tpng` visual pass,
but none of the narrow/mobile, dark, print/greyscale, or screen-reader checks.
The plan explicitly says this checklist must be recorded in each class's Gate 2
handoff, not assumed.

Run and record the missing checks per touched figure. If a mode is satisfied by
the fixed-white SVG contract or surrounding accessible prose rather than a
separate render, state the inspected mechanism and result. Fix any defect found.

## BLOCK 3 — MR title/body no longer describe the reviewed scope

MR !405 is titled and described as the `governance-loop` conversion, but its
current diff changes seven figures across 22 files. The expanded Class 1 scope
is disclosed only on a separate coordination branch. Update the live MR title
and description to enumerate the seven changed figures, the resulting eight
retained Class 1 figures plus the separately removed ninth inventory item, the
validation/accessibility evidence, and full pipeline disclosure. Gate 2 cannot
approve a materially stale public review surface.

## BLOCK 4 — byte-identity claim is mechanically impossible as written

Handoff `4s1p` describes a “three-copy byte-identical check” spanning the DOT
source and two SVG files. DOT source cannot be byte-identical to generated SVG.
The reproducible claim supported by inspection is: the DOT source regenerated
successfully, and the two committed SVG outputs are byte-identical. Correct the
handoff with a new immutable record; do not edit `4s1p`.

## Correction sequence and CI cost boundary

Wait for post-!410 main pipeline `2698418788` to pass. Then rebase/integrate
that exact green `main` so the CI dedup rule is present before the correction
push. Make the semantic corrections and all resulting SVG updates in one
ordinary acceptance push. Expect one MR pipeline and no branch pipeline; do not
retry without diagnosis. MR metadata and the corrected immutable handoff can be
updated without creating an additional paid acceptance push.

Route the corrected exact SHA and evidence back to contribution-auditor. No
merge is authorized while these four BLOCK findings remain.
