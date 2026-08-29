---
kind: handoff
from: coder-tester
to: coder-tester
id: 20260829T230155Z-coder-tester-argd
timestamp: 2026-08-29T23:01:55Z
model: claude-opus-5
immutable: true
references-to:
  - 20260829T223950Z-coder-tester-j1ht.md
  - 20260829T224246Z-coder-tester-48kq.md
topic: live-governance-gate-failopen-scoping-question
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Assess whether the live fail-open in agov:governance:validate should be fixed as its own gated change, separate from MR !2214.
    - Verify or refute the shallow-clone hypothesis below empirically; it is a mechanism argument, not a demonstrated CI failure.
    - Judge the collision risk against MR !2214 slice B1, which edits the same lines.
    - Recommendation only. The governor-reviewer decides scope and Gate 1.
  must_read:
    - .gitlab-ci.yml
    - docs/coordination/20260829T223950Z-coder-tester-j1ht.md
  do_not_touch:
    - .gitlab-ci.yml
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Scoping question: your GCD-R4 finding may describe a live, currently-firing gate failure

Your `j1ht` finding GCD-R4 was scoped to slice B2's proposed adopter
correction. Following it into foundry's own configuration suggests it also
describes a defect in the **live** `agov:governance:validate` job, right now,
independent of MR !2214 and of any Gate 1 decision.

The governor asks for your assessment before this is separated into its own
work order.

## The mechanism

Your reproduction, re-run in this checkout:

```
$ sh -c 'set -eu; git diff --name-only -z does-not-resolve...HEAD -- \
    "docs/plans/*.md" | xargs -0r true; echo PIPELINE_EXITED_ZERO'
fatal: bad revision 'does-not-resolve...HEAD'
PIPELINE_EXITED_ZERO
exit=0
```

`.gitlab-ci.yml:866-871` uses exactly that shape under `set -eu`.

The part that makes this more than theoretical: **`agov:governance:validate`
runs on a shallow clone.** The project's default `GIT_DEPTH` is 20 — stated
in this repository at `.gitlab-ci.yml:981` and there attributed to the GitLab
project API. Only `commercial-corpus:validate` overrides it to `0`
(`.gitlab-ci.yml:987`), and it does so for an unrelated reason. The governance
job inherits the default.

`git diff <base>...HEAD` requires a merge base. On a depth-20 clone whose
`git fetch --no-tags origin <target>` is also shallow, a branch that diverged
more than roughly twenty commits back may have no merge base in the local
object graph. The diff then fails, `xargs` returns 0, and the job reports
success having validated nothing.

Given this project's merge rate, a branch open across twenty merges is
routine rather than exceptional — several currently-open MRs qualify.

## What I am claiming, and what I am not

I am claiming the preconditions exist and the shape matches. I am **not**
claiming a demonstrated CI failure: I have not caught the job passing green
with an unvalidated changed file. That distinction matters here, because this
session has already published one confident diagnosis that a timestamp check
would have falsified (`r3ad`, retracted by `rmj1`), and one derived closure
resting on a premise your `k5dq` disproved against the real artifact.

So this is a hypothesis with a mechanism, offered for verification rather than
as a finding. Falsifiers worth trying: GitLab's target-ref fetch may unshallow
enough to keep a merge base available; `git diff A...B` may degrade to
something non-failing in the relevant case; or the base-resolution guard above
may already exclude the failing shapes.

You have twice reached the right answer by going to the artifact rather than
the source — the packaged binary for `k5dq`, the pipe semantics for GCD-R4.
This is the same kind of question.

## The three questions

**1. Separate work order, or inside !2214?** Fixing it separately means an MR
touching `.gitlab-ci.yml:866-871` while slice B1 also rewrites those exact
lines. That is a direct collision on the same hunks, and !2214 has now taken
three BLOCK rounds and may take more. My inclination is that a live fail-open
should not wait on a plan still converging, but the collision is real and I
would rather have your read than assert mine.

**2. What shape should the fix take?** GCD-R4's resolution condition names
proving a merge base, capturing `git diff` success before invoking `xargs`,
and failing closed on empty target, fetch failure, missing merge base, and
diff failure. Should that live as a shell helper inside the extracted
`.gitlab/scripts/governance-validate.sh`, or as an `agov` subcommand that
emits the changed-file list and fails closed itself? The second is testable in
Rust with fixtures; the first is smaller and has no packaging path.

**3. Does the shallow-clone reading change the severity?** If a merge base can
genuinely be absent at depth 20, the gate is not merely capable of failing
open under contrived input — it may be doing so on ordinary long-lived
branches. That would make this more urgent than either blocked plan.

## Out of scope

Do not revise MR !2214, MR !2210, or any plan. This is a scoping assessment
and an empirical verification request only.

Recommendation authority only. The governor-reviewer decides.
