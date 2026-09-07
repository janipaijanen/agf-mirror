---
kind: handoff
from: coder-tester
to: coder-tester
id: 20260906T213043Z-coder-tester-aks5
timestamp: 2026-09-06T21:30:43Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260906T203532Z-coder-tester-5bjb.md
  - 20260906T204742Z-coder-tester-qp1q.md
  - 20260906T205648Z-coder-tester-rqs6.md
topic: docs-slice-hooktest-fix-reintegrate-e694aecf
requires-ack: true
artifacts_owned:
  - agf/docs/site/docs/operate/poll-and-coordinate.md
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - >-
      Re-integrate the Claude documentation slice at the corrected head
      e694aecfda0b3aa5917e26aed9119a31f5a4a7e0 (was 78c46edb). One added
      commit on feat/universal-polling-docs-slice fixes the site-html-build
      failure on !2463 pipeline 2824733364. The integrated head !2463 /
      78385489 and its independent-review request (rqs6) are stale and need
      to advance to the re-integrated head.
  must_read:
    - Branch feat/universal-polling-docs-slice @ e694aecf
  do_not_touch:
    - .claude/settings.local.json
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Docs slice hook-test fix — re-integrate at `e694aecf`

## Failure

`!2463` canonical pipeline `2824733364` at `78385489` failed on
`site-html-build`:

```
FAIL: test_poll_includes_every_shared_activity_surface
  (agf/docs/build/hooks/test_operations_runbook.py:72)
AssertionError: 'tickets/issues' not found in '# Poll and coordinate...'
```

## Cause and fix — mine

The docs-slice edit to `poll-and-coordinate.md` step 5 changed the literal
phrase `tickets/issues` to `ticket/issue`. `test_operations_runbook.py`
asserts the exact string `tickets/issues` is present as a named shared
activity surface. One-line fix on `feat/universal-polling-docs-slice`:

- head `78c46edb` -> `e694aecfda0b3aa5917e26aed9119a31f5a4a7e0`
- step 5 now reads: "Inspect **every** open item in tickets/issues,
  regardless of assignee or apparent relevance: new issues, state and label
  changes, and issue comments." — keeps the whole-surface emphasis and
  restores the required literal.

## Evidence at `e694aecf`

- `python3 -m unittest discover -s agf/docs/build/hooks` — 27 tests, `OK`
  (was 1 failure).
- `agov validate adopter_docs` OK; `assistant_loader` OK; `git diff --check`
  clean.
- The fix touches only `poll-and-coordinate.md`; the other 5 slice files are
  unchanged from `78c46edb`.

## Next

Codex (integration owner): re-integrate `e694aecf`, refresh the `!2463`
integrated head, re-run the canonical pipeline, and re-point the independent
whole-head review request (`rqs6`) at the new integrated SHA. The
non-conflicted-reviewer ask in `am5a` still stands — this session cannot
supply that review.
