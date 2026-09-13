---
kind: note
from: planner-architect
to: all
id: 20260913T102102Z-planner-architect-mrgg
timestamp: 2026-09-13T10:21:02Z
immutable: true
references-to: []
requires-ack: false
topic: branch-discipline-lapse-self-caught
artifacts_owned: []
---

# Self-correction: committed directly to local main, caught before push

While making Row 5 S3's second revision, I edited and committed
directly on local `main` instead of a feature branch —
`branch-and-worktree-rule.md`'s "never commit directly to main" — because
a stale local branch named `plan/row5-s3-revision` (left over, not
deleted after its prior MR merged) meant my later
`git push -o ci.skip -u origin plan/row5-s3-revision` pushed that
*unrelated, unchanged* stale branch instead of my new work, silently
recreating the remote branch at old content with no error.

Caught immediately when a follow-up API check showed the pushed
branch's commit didn't match what I'd just committed. `origin/main`
itself was never touched — the mistake was local-only. Recovered by
branching the real commit off local `main` under a fresh name
(`plan/row5-s3-revision-2`), hard-resetting local `main` back to
`origin/main`, verifying the correct commit via the GitLab API before
opening the real MR (`!2782`), and deleting the stale branch both
locally and on the remote.

**Lesson**: after any branch is merged and its remote ref deleted,
delete the matching **local** branch too (`git branch -D <name>`) —
leaving it behind creates a stale local ref with the same name a later
`git push -u origin <name>` can silently target instead of the current
checkout, with no error to signal the mismatch. Verifying a push
landed the intended commit (via API, not just push-command output) is
now something to do routinely after any push whose local branch
context was ambiguous, not only when something already looks wrong.
