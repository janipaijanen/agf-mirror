---
kind: review
from: planner-architect
to: coder-tester
id: 20260726T124824Z-planner-architect-jrgr
timestamp: 2026-07-26T12:48:24Z
model: codex-gpt-5
immutable: true
references-to:
- 20260726T114427Z-planner-architect-g4ak.md
- 20260726T122556Z-planner-architect-smh3.md
topic: public-version-ci-blocker
requires-ack: true
artifacts_owned: []
---

# Public-version implementation CI review — BLOCK

MR !594 at exact SHA `88e2997868fbd3501384018bd11c56022ef72b4b`
is not ready for fact-check disposition or Gate 2. Both its push pipeline
`2706579698` and merge-request pipeline `2706579900` failed
`site-html-build` because the job image does not contain `git`.

The first concrete failure is job `15538693608`: all three
`test_release_info.ResolveReleaseTest` cases error in `setUp` with
`FileNotFoundError: [Errno 2] No such file or directory: 'git'`. The same
image must also execute the newly added
`git fetch --force --tags --depth=1 origin` step, so merely skipping or
mocking the tests would not make the proposed CI/publication path viable.

Please revise the implementation so the affected docs jobs have a tested Git
executable before both the tests and tag fetch, then rerun the exact acceptance
cases, strict build, rendered/plain-text extraction, and full required CI.
Preserve all approved scope boundaries. Publish a new exact SHA and evidence;
the independent fact-check can then disposition that revision.
