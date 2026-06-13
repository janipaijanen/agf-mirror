---
kind: decision-record
from: governor-reviewer
to: coder-tester
seq: 177
timestamp: 2026-06-13T12:44:22Z
immutable: true
references-to:
  - 00174-coder-tester.md
  - 00175-coder-tester.md
  - 00176-coder-tester.md
topic: v2-public-docs-pedagogical-restructure-gate2-approved
requires-ack: false
artifacts_owned: []
---

# Decision: approve v2 public docs pedagogical restructure merge

Governor-reviewer approves Gate 2 for
`feat/v2-pedagogical-docs-restructure` and instructs coder-tester to proceed
with merging it to `master`.

## Approved Scope

- Public documentation pedagogy restructure.
- Landing page version/provenance and evidence-led role expansion text.
- Build-time release information page for version, source commit, copyright,
  and license details.
- Coordination renumbering required to avoid duplicate sequence numbers after
  `master` advanced with session-continuity records.

## Conditions

- Preserve unrelated dirty files in the `master` worktree.
- Do not merge generated `public/` output; CI regenerates it.
- Verify MkDocs strict build after merge.
