---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260727T081811Z-coder-tester-qhap
timestamp: 2026-07-27T08:18:11Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260727T081257Z-coder-tester-rrj8
topic: issue70-downstream-operations-runbook
requires-ack: true
artifacts_owned:
  - agf/docs/site/docs/operate/runbook.md
  - agf/docs/site/docs/operate/session-continuity.md
  - agf/docs/site/docs/operate/poll-and-coordinate.md
  - agf/docs/build/hooks/test_operations_runbook.py
  - agf/templates/AGENTS.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Review revised issue #70 implementation through source commit 35ae7c1 after contribution-auditor and fact-checker corrections; use the later empty CI-trigger commit as the exact Gate 2 head."
    - "Require the exact-head site-html-build pipeline to pass before merge."
  must_read:
    - docs/plans/issue70-downstream-poll-coordinate-runbook.md
    - agf/docs/site/docs/operate/runbook.md
    - agf/docs/site/docs/operate/session-continuity.md
    - agf/docs/site/docs/operate/poll-and-coordinate.md
  do_not_touch: []
---

# Revised issue #70 Gate 2 handoff

This handoff supersedes `20260727T081257Z-coder-tester-rrj8` for review
evidence. Required advisory corrections are incorporated:

- the runbook now points to adopter-designated local role authority and labels
  the shipped manifest as example/reference until explicitly designated or
  customized;
- recovery tests now encode input-to-classification/action contracts for no
  checkpoint, dirty tree, local-only commit, pushed branch without handoff,
  missing network/base, fresh clone, and ambiguity escalation;
- a cross-surface invariant checks that adapters and internal references point
  to the canonical public source without copying its route table; and
- both invalid template literals `agov coord graph-index docs/coordination/`
  now use supported `agov coord graph-index` syntax, protected by a regression
  test and verified against Rust CLI help.

Validation at source commit `35ae7c1`:

- 26 documentation-hook tests passed;
- strict MkDocs build passed;
- `git diff --check` passed with no EOF or whitespace warnings;
- `agov coord graph-index --help` and `agov coord list --help` passed against
  the Rust binary.

No implementation scope beyond the approved documentation plan was added.
