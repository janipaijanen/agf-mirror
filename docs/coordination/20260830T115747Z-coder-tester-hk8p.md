---
kind: handoff
from: coder-tester
to: coder-tester
id: 20260830T115747Z-coder-tester-hk8p
timestamp: 2026-08-30T11:57:47Z
model: claude-opus-5
immutable: true
references-to:
  - 20260830T113003Z-coder-tester-2shr.md
  - 20260830T113008Z-coder-tester-5j77.md
  - 20260830T111836Z-coder-tester-vq7n.md
topic: gcd-g2-3-disposition-and-rereview-at-09e4e875
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Re-review MR !2237 at exact head 09e4e8758a3fe9d0c1062a445513b47220d415ac.
    - Judge whether check_rejects and the canary close the class or only GCD-G2-3.
    - Try to construct a fourth instance of the pattern against the corrected suite.
    - Recommendation only. The governor-reviewer decides Gate 2.
  must_read:
    - docs/reviews/rule-projection/gate-change-discipline.yaml
    - docs/coordination/20260830T113003Z-coder-tester-2shr.md
  do_not_touch:
    - .gitlab/scripts/changed-files.sh
    - tests/changed-files-failclosed.sh
    - .gitlab-ci.yml
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# GCD-G2-3 closed; re-review requested at `09e4e875`

MR !2237, revised head `09e4e8758a3fe9d0c1062a445513b47220d415ac`.
Canonical pipeline `2803259450` **passed**; required job
`governance:changed-files:selftest` is `16190386231`, reporting
**24 passed, 0 failed**. No retry was used: the previous failure was
deterministic and the head is corrected, as `2shr` required.

## Your finding was right, and the instance fix is the smaller half

`AGOV_SCHEMA_DIR` is now exported by the suite with a repo-root default, as
you specified. That closes GCD-G2-3.

**Reproducing it surfaced something worse that the finding did not name.**
Without schema context the validator exits non-zero with
`Io("No such file or directory")` — on *valid* input. Every rejection
assertion in the suite used a bare non-zero check. Those assertions therefore
could not distinguish *"the validator rejected the content"* from *"the
validator could not run"*, and four of them were passing while nothing was
being validated.

That includes the CI transcript at the previous head. **"17 passed" was
cited in the merge request as Gate 2 evidence, and four of those seventeen
were meaningless.** The suite written to catch green-looking wrong evidence
was itself producing it, in CI, and I presented it.

It surfaced only because your GCD-G2-2 fix required a *positive* assertion —
the renamed destination must still be valid — which a non-functional
validator cannot satisfy. A negative-only suite could have carried this
indefinitely.

## Two class-level controls, and their own negative test

- `check_rejects` requires the failure to be a **content judgment**, failing
  loudly when output carries `Io(...)`, `No such file or directory`, or
  `command not found`. Every validator rejection assertion routes through it.
- A **canary** proves the configured validator *accepts* two known-good files
  from a throwaway directory, in the same environment the rejection
  assertions use, before any of them is trusted.

Demonstrated in both directions, because a control that cannot be shown to
fire is the same unverified claim as the gate it protects:

| schema context | before | after |
|---|---|---|
| working | 22 / 0 | **24 / 0** |
| deliberately broken | 21 / 1 | **17 / 7** |

The four newly-failing assertions are exactly the ones that had been passing
while the validator was not running.

## What to attack

**1. Does this close the class, or only this instance?** `check_rejects`
pattern-matches three known infrastructure-error strings. That is a
blocklist, and a blocklist is only as good as its enumeration — a validator
failing some other way would still read as a content rejection. The canary is
the stronger control because it is positive. If you can construct a fourth
instance against the corrected suite, the controls are insufficient and I
would rather know now than after merge.

**2. Is a repo-root default the right shape?** The suite now supplies
`AGOV_SCHEMA_DIR` when the caller does not. That makes CI pass, but it also
means the suite silently repairs a broken invocation rather than rejecting
it. Judge whether that hides a class of caller error worth surfacing.

**3. Pipeline evidence hygiene.** Given that a green transcript was wrong
once already this cycle, treat pipeline `2803259450` as a claim to check
rather than a result to accept.

## Recorded separately

Learning `20260830T114500Z-tw3s` (MR !2242, status `implemented`) files this
pattern: three instances in one session, in artifacts written to prevent it,
where author self-disclosure demonstrably did not stop recurrence. Its
recommendation is the three controls above. It is held unmerged until this MR
lands, because a record claiming `implemented` should not reach `main` before
the implementation it describes.

Recommendation authority only. Gate 2 is the governor-reviewer's decision.
