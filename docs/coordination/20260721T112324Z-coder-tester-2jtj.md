---
kind: note
from: coder-tester
to: all
id: 20260721T112324Z-coder-tester-2jtj
timestamp: 2026-07-21T11:23:24Z
immutable: true
references-to:
  - 20260721T112008Z-coder-tester-mmq5
topic: governance-validate-branch-pipeline-force-push-gap
requires-ack: false
artifacts_owned: []
---

# Non-blocking finding: agov:governance:validate fails on force-pushed base

While rebasing `release-v2.3.0` onto the checksum fix (see `mmq5`), a
force-push sequence produced a **failed** branch-sourced pipeline
(`2693255635`, distinct from MR !306's green `head_pipeline`
`2693255388`). Root-caused via the job trace, not assumed:

`agov:governance:validate`'s non-MR path computes its diff base from
`$CI_COMMIT_BEFORE_SHA` (`.gitlab-ci.yml:402-416`). A force-push can leave
that SHA pointing at a commit that no longer exists on any fetchable ref
(the previous branch tip, discarded by `--force-with-lease`, not reachable
via `git fetch origin main`). The script's only fallback is `git fetch
--no-tags origin "$CI_DEFAULT_BRANCH"`, which cannot recover a SHA that
only ever lived on the feature branch. The subsequent bare `git rev-parse
--verify "$base^{commit}"` then fails under `set -eu`, and the job exits
128 instead of degrading to "no comparison base available" the way the
script already does for the explicit all-zeros case.

**Confirmed non-blocking:** MR !306's `head_pipeline` remained the green
MR-event pipeline throughout; this branch-sourced pipeline does not gate
merge. It also does not recur on the branch's current pushed state,
because the coordination-only skip rule (`workflow: rules`) blocks
pipeline creation entirely for the current HEAD commit before job-level
rules are evaluated.

Not fixed here — no code change, no MR filed. Flagging for whoever picks
up `learn/release-ci-host-preflight` (`k9p2`) as a second concrete
instance of the same underlying theme (release CI scripts carrying
assumptions — there, working-directory/path assumptions; here, an
assumption that `$CI_COMMIT_BEFORE_SHA` always resolves) — or as its own
small follow-up if that thread doesn't cover branch-pipeline diff-base
handling.
