---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260727T081257Z-coder-tester-rrj8
timestamp: 2026-07-27T08:12:57Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260727T073930Z-governor-reviewer-4hrb
  - 20260727T080327Z-fact-checker-dj7s
  - 20260727T080326Z-contribution-auditor-mtm2
topic: issue70-downstream-operations-runbook
requires-ack: true
artifacts_owned:
  - agf/docs/site/docs/operate/runbook.md
  - agf/docs/site/docs/operate/session-continuity.md
  - agf/docs/site/docs/operate/poll-and-coordinate.md
  - agf/docs/build/mkdocs.yml
  - agf/docs/build/hooks/ai_evaluation_dossier.py
  - agf/docs/build/hooks/test_operations_runbook.py
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Review issue #70 implementation commit 4d98d68 and the feature branch exact head containing this handoff for Gate 2. Confirm the public runbook is a decision-oriented map, continuity preserves unknown state, polling covers coordination plus forge changes/comments and issues/comments, and GitLab/Claude sections remain removable adapters."
    - "Inspect the exact-head site-html-build pipeline before any merge."
  must_read:
    - docs/plans/issue70-downstream-poll-coordinate-runbook.md
    - agf/docs/site/docs/operate/runbook.md
    - agf/docs/site/docs/operate/session-continuity.md
    - agf/docs/site/docs/operate/poll-and-coordinate.md
  do_not_touch: []
---

# Issue #70 operations and continuity runbook ready for Gate 2

The approved v2.4.5 documentation slice is implemented at source commit
`4d98d68`. The canonical Pages map now routes start/resume, graceful context
cleanup and planned reboots, abrupt-loss recovery, full polling and
coordination, governed changes, review/release, commissioning/upgrades, and
exit without duplicating the detailed guides.

The continuity procedure distinguishes local, shared, and unknown evidence and
covers dirty trees, local-only commits, remote branches without handoffs, lost
IDE buffers, missing network/base refs, and fresh-clone recovery. The polling
procedure covers coordination, MRs/change requests and their comments,
issues/tickets and their comments, exact-head CI, split-work ownership,
progress, handoffs, and honest wake-up limits.

`/compact-agf` now says validation alone is not publication. It requires a
pushed record or visible bridge for cross-machine/person/agent continuity and
reports remaining local-only state.

Validation at `4d98d68`:

- `/usr/bin/python3 -m unittest discover -s agf/docs/build/hooks -p 'test_*.py'`: 23 passed;
- strict MkDocs build: passed;
- palette contrast check and self-test: passed;
- stale-version-label check and self-test: passed;
- `git diff --check`: passed;
- documented `agov coord list --since` availability checked against the Rust
  binary help; and
- rendered navigation includes the three new Operate pages and the dossier
  corpus test includes them in explicit order.

No version bump, tag, release, CLI command, background scheduler, or role
authority change is included.
