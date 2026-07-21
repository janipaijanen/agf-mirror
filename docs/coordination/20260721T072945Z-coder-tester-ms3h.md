---
kind: note
from: coder-tester
to: coder-tester
id: 20260721T072945Z-coder-tester-ms3h
timestamp: 2026-07-21T07:29:45Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260721T072153Z-coder-tester-p3nj
topic: correct-p3nj-ci-pipeline-claim
requires-ack: false
artifacts_owned: []
---

# Correction: p3nj's CI-pipeline claim was wrong

`p3nj`'s BLOCK 2 response claimed: "This response's content-touching
commit will trigger a normal pipeline, giving fresh CI evidence for the
corrected head." That was incorrect, caught by the human governor.

`p3nj` was itself pushed in the same `git push` as the preceding
content-fix commit (`5264a7a`), as two commits in one push. GitLab's
`workflow.rules` in `.gitlab-ci.yml` evaluates `$CI_COMMIT_TITLE` against
the **push's HEAD commit**, not each commit in the push individually.
Because that push's HEAD commit was `p3nj`'s own coordination-only commit
(title starting `coordination:`), the entire push — including
`5264a7a`'s real content change — was skipped from CI. No pipeline ran.

This is exactly the "mixed commits must run normal CI" case the CI
config's own comment warns about, self-inflicted by batching a
content-fix commit and a coordination-only commit into one push with the
coordination commit last. Corrected going forward: when a push must
include both a content commit and a coordination-record commit, order
them so the coordination-only commit is not the final one, or push them
separately with the content commit last.

MR_DESCRIPTION.md added on this branch (non-coordination-titled commit,
pushed after this note) both fixes the stale MR description language the
human governor flagged and, as a side effect of not matching the skip
regex, gets this branch's actual first real CI evidence for the wording
fix.
