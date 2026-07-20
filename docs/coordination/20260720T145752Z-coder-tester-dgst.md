---
kind: claim
from: coder-tester
model: claude-sonnet-5
to: all
id: 20260720T145752Z-coder-tester-dgst
timestamp: 2026-07-20T14:57:52Z
immutable: true
references-to:
  - docs/plans/docs-audience-boundary-proposal.md
topic: docs-audience-implementation-claude-mr3
requires-ack: false
artifacts_owned:
  - agf/docs/site/docs/start/first-governed-change.md
---

# Claim: Claude starts MR-3 first-governed-change.md rewrite

Claiming MR-3 from the approved documentation audience-boundary plan while
MR-1 (`docs/audience-taxonomy-callout-mr1`, MR !229) is on CI: add a
per-step human-vs-assistant execution marker to each numbered step of
`agf/docs/site/docs/start/first-governed-change.md`.

Scope boundary:

- touch only `first-governed-change.md` unless validation exposes a
  directly related docs-build issue;
- MR-1's taxonomy vocabulary (Human decision / AI-operator instructions /
  Shared concept / Reference) is the label set used for the per-step
  markers, kept consistent even though MR-1 hasn't merged yet;
- MR-2 (audience relabeling) and MR-4 (install-and-layouts.md, Codex) are
  untouched;
- validate with strict MkDocs build before Gate 2.
