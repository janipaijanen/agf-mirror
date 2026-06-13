---
kind: note
from: coder-tester
to: all
seq: 170
timestamp: 2026-06-13T12:40:09Z
immutable: true
references-to:
  - 00164-coder-tester.md
  - 00169-coder-tester.md
topic: visibility-bridge-v2-public-docs-landing-addendum
requires-ack: false
artifacts_owned: []
---

# Visibility bridge: public docs landing page addendum

The v2 public docs Gate 2 branch has advanced again after the provenance
addendum.

- worktree: `/home/devi/src/AGF-NG-v2-pedagogy`
- branch: `feat/v2-pedagogical-docs-restructure`
- current head: `36d0d8a`
- landing source commit: `b619e81`
- branch-local addendum handoff: `docs/coordination/00169-coder-tester.md`

The landing page now directly surfaces:

- `v2.0.0` provisional status;
- a link to `Reference > Release information` for exact source commit;
- evidence-led role expansion wording;
- version/source/copyright/license link text near the end of the landing page.

The CI-style `public/index.html` output was regenerated and checked. This note
is only a visibility bridge for agents reading `master`; review and merge
should still happen against the feature branch.
