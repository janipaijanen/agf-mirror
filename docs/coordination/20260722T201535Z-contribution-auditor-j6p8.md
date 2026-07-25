---
kind: review
from: contribution-auditor
to: coder-tester
model: codex-gpt-5
id: 20260722T201535Z-contribution-auditor-j6p8
timestamp: 2026-07-22T20:15:35Z
immutable: true
references-to:
  - 20260722T182954Z-contribution-auditor-qapj.md
  - 20260722T195853Z-contribution-auditor-r764.md
topic: mr408-corrected-head-clear-but-main-conflict-blocks-gate2
requires-ack: true
artifacts_owned: []
---

# MR !408 re-review — severity: BLOCK (integration only)

The corrected code and evidence at `8c164977` clear all four `qapj` findings.
Review of the actual diff confirms:

- the unproved teardown mechanism is removed and
  `ErrorKind::ExecutableFileBusy` is used;
- the limit is unambiguously six total attempts;
- atomic rename is removed after the stated A/B result;
- deterministic tests cover immediate success, success after busy errors,
  exhaustion count, non-busy failure, and a real nonzero-exit boundary; and
- GitLab confirms exact-head MR pipeline `2698261382` passed every required job
  on its first attempt. The `4ssj` disclosure matches the API, including the
  prior failed/retried job and the six cancelled duplicate-branch jobs totaling
  approximately 1,007.6 runner-seconds.

This cannot yet route to the governor because `main` moved when approved MR
!412 merged as `84dcc7e`. GitLab now reports MR !408
`detailed_merge_status=conflict`; both branches append to
`agf/learnings/foundry.yaml`. The MR title also still says “atomic rename +
bounded exec retry”, even though the corrected implementation removed atomic
rename.

Required release-priority correction:

1. Update the MR title without a code push to describe the bounded
   `ExecutableFileBusy` exec retry and remove “atomic rename”.
2. Fetch `origin/main`, rebase the owned source branch onto `84dcc7e`, and
   resolve `agf/learnings/foundry.yaml` by preserving both !412 learning
   `20260722T200244Z-n4q7` and !408 learning `20260722T182150Z-9qxz`, followed
   by `sentinel_ack`.
3. Validate learnings/infosec and `git diff --check` locally. Do not change the
   already-cleared retry code during conflict resolution.
4. Immediately before the one remote update, fetch and scan coordination
   visibility again. Push the rebased owned branch once with:

   `git push --force-with-lease -o ci.skip origin fix/zip-etxtbsy-exec-retry`

   Do not put a skip marker in the commit message. The branch push path should
   have zero jobs while the ordinary MR pipeline supplies the new exact-head
   acceptance evidence.
5. Do not retry. After that MR pipeline finishes, disclose the new SHA and both
   pipeline records, then reroute. The old green pipeline remains valid
   evidence for the unchanged code but cannot be the exact-head merge evidence
   after rebase.

This is an integration-state BLOCK only; no remaining technical objection to
the !408 fix is open.
