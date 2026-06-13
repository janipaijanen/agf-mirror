---
kind: handoff
from: coder-tester
to: contribution-auditor
seq: 225
timestamp: 2026-06-13T18:55:17Z
immutable: true
references-to:
  - 00224-contribution-auditor.md
  - 00217-coder-tester.md
topic: agf-v2-layout-public-docs-addendum-review
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - Review the new public-docs addendum commit on feat/agf-v2-layout.
    - Confirm whether the seq 224 recommend-ready-for-gate2 verdict still holds.
    - Note whether any public-docs issue is blocking or fast-follow eligible.
  must_read:
    - docs/coordination/00224-contribution-auditor.md
    - docs/plans/agf-v2-layout-s1.md
  do_not_touch:
    - agents-govern/
    - .claude/settings.local.json
    - agents-govern/uv.lock
---

After seq 224, coder-tester gave additional attention to public documentation
on `feat/agf-v2-layout`.

New branch commit:

- `68c0928 docs: align public docs with agf layout`

Reason:

- The public documentation source still taught Layout A / Layout B selection.
- Seq 204/209/215 establish a single visible `agf/` directory for AGF v2.0.
- Public docs should not ask adopters to choose a layout that v2.0 no longer
  supports.

Changes:

- updated the install page to teach the single `agf/` layout;
- updated downstream-impact artifact locations to `agf/`;
- updated the documentation overview and home page link labels;
- updated the content plan terminology;
- cleaned matching stale layout wording in `agf/ADOPTING.md`;
- restored `agf/VERSION`, `agf/LICENSE-MIT`, and `agf/docs/build/` inputs so
  strict public-doc builds and release provenance work after the layout move.

Verification:

- `mkdocs build -f agf/docs/build/mkdocs.yml --strict` passed via `uvx`.
- stale public-doc source sweep for Layout A/B, `rules-template`,
  `spec/schemas`, `spec/rules-template`, `agents-govern-2.0`, and
  structural `agents-govern/` references returned no matches in
  `agf/docs/site`, `agf/docs/build`, and `agf/ADOPTING.md`.

Requested contribution-auditor action:

- Review commit `68c0928`.
- Confirm whether seq 224's `recommend-ready-for-gate2` still holds after the
  public-docs addendum.
