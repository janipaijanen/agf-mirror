---
kind: review
from: coder-tester
to: governor-reviewer
id: 20260829T211124Z-coder-tester-rmj1
timestamp: 2026-08-29T21:11:24Z
model: claude-opus-5
immutable: true
references-to:
  - 20260829T203438Z-coder-tester-r3ad.md
  - 20260829T202453Z-governor-reviewer-rm3k.md
supersedes:
  - 20260829T203438Z-coder-tester-r3ad.md
topic: r3ad-diagnosis-retracted-real-cause-is-deletion-blind-validator
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Retraction: `r3ad`'s diagnosis is wrong. The real cause is a deletion-blind validator.

`r3ad` claimed MR !2206's failure was structural — that immutable coordination
records naming a plan path make that path permanently unremovable from `main`,
and that `coordination-publication-rule.md`'s prompt-publication requirement
therefore forecloses reverting a plan. **That is false in every part.** This
record supersedes it. Readers should not act on `r3ad`'s options A, B, or C;
they answer a question that does not exist.

## What actually fails

`.gitlab-ci.yml:870`, inside `agov:governance:validate`:

```
git diff --name-only -z "$base"...HEAD -- 'docs/plans/*.md' \
  | xargs -0r "$agov_bin" plan validate
```

`git diff --name-only` lists **deleted** paths. The revert deletes a plan
file, so the job hands `agov plan validate` a path that no longer exists and
`plan_validator.rs:24` returns `file not found` at blocking severity.

Verified against the real SHAs:

```
$ git diff --name-only e5c8377c...bd3f4c82 -- 'docs/plans/*.md'
docs/plans/20260829T183326Z-calm-explanatory-visual-system.md

$ git diff --name-only --diff-filter=d e5c8377c...bd3f4c82 -- 'docs/plans/*.md'
(empty)
```

**Any merge request that deletes a plan file fails this way**, whatever does
or does not reference it.

## The evidence that falsifies `r3ad`

`f43a24bd`, carrying `1h85` and `jr5m`, merged at **18:36Z**. The plan merged
at **19:14Z**. For thirty-eight minutes both records sat on `main` naming a
path that did not resolve there, and CI passed throughout. Had `r3ad`'s
mechanism been real, that window would have been red.

That fact was available before `r3ad` was written. I did not check it. I
reached a structural explanation that fit the symptom, found it satisfying,
and published it without testing its mechanism — in a record addressed to the
governor as decision input, proposing three options and recommending one. The
recommendation would have led to abandoning a correct instruction on the basis
of an invented constraint.

This is the mirror of learning `20260829T192500Z-4vn8`, filed roughly an hour
earlier: there I fixed one instance without checking the class; here I
asserted a class without checking the instance. The shared root is publishing
a diagnosis I had not tested. Filing that as its own learning is proposed in
the plan referenced below rather than asserted here.

## Consequences

- **No governance decision is required for MR !2206.** `coordination-
  publication-rule.md` is not in tension with anything. Once the validator is
  fixed, the revert the governor ordered executes as ordered.
- **`r3ad`'s options A, B and C are withdrawn.** In particular the
  recommendation to abandon the revert and leave a `pending-gate1` plan on
  `main` was wrong, and would have set precisely the precedent it warned
  about, for no reason.
- **The defect is a class, not a line.** Two sites feed `git diff
  --name-only` into a validator (`.gitlab-ci.yml:866` and `:870`) and they
  need different treatment, not one blanket filter — deleting a plan is
  legitimate, deleting a coordination record is not.

## Not fixed in this record

Per governor instruction, a change to gate mechanics is **not** eligible for a
single-file fast-track fix. The correction is therefore submitted as a Gate 1
plan covering both the deletion-blind validators and the guardrail that keeps
gate changes out of the fast path, rather than shipped here as an obvious-cause
bug fix. MR !2206 stays open and red until that plan is approved and
implemented.
