---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260725T180846Z-coder-tester-pgah
timestamp: 2026-07-25T18:08:46Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260725T175956Z-fact-checker-st61
  - 20260725T175624Z-coder-tester-jss7
topic: v243-doc-quality-at-smoke-test
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Provide or arrange the approved plan's named AT smoke test for MR !527 exact SHA 25bfeb35d40f0b44bdc8485f23f2c503334f331e: VoiceOver/current Safari on macOS, or current NVDA/Firefox on Windows. Record OS, browser, AT versions, and whether all seven figures announce descriptive alt text and captions rather than bare titles."
  must_read:
    - docs/plans/v2.4.3-doc-quality-and-ia-backlog.md
  do_not_touch: []
---

# External AT smoke test required before MR !527 Gate 2

### Codex · coder-tester

MR !527 exact SHA `25bfeb35d40f0b44bdc8485f23f2c503334f331e`
now has successful exact-head pipelines `2705597471` and `2705597453`, strict
MkDocs and contrast validation, desktop and 390px rendered inspection, and
fact-check `st61` (**ADOPT x4**).

Chromium's full accessibility tree was inspected directly. Each of the seven
new `<figure>` nodes contains its descriptively named image and its matching
numbered `Figcaption`; no bare-title accessible image name remains.

The current host is Linux and cannot run either exact AT/browser pair required
by the approved plan. Before Gate 2, provide one real smoke test using:

- VoiceOver with current Safari on macOS; or
- current NVDA with Firefox on Windows.

Record the OS, browser, and AT versions and confirm whether all seven figures
are announced with descriptive alt text and captions rather than bare titles.
No Gate 2 request or approval should be issued until this evidence is visible,
or the governor explicitly returns the scope to Gate 1 to change the approved
acceptance boundary.
