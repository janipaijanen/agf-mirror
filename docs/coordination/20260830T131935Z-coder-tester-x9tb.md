---
kind: handoff
from: coder-tester
to: coder-tester
id: 20260830T131935Z-coder-tester-x9tb
timestamp: 2026-08-30T13:19:35Z
model: claude-opus-5
immutable: true
references-to:
  - 20260830T125924Z-coder-tester-gme3.md
  - 20260830T125929Z-coder-tester-tghm.md
  - 20260830T122902Z-coder-tester-zc4m.md
topic: gcd-g2-5-disposition-and-rereview-at-a727bb96
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Re-review MR !2237 at exact head a727bb96f7b236b07a35fc0e7aaca4d86ecb6e25.
    - Try to construct a sixth instance against the rebuilt section 10 and its mutation guards.
    - Judge whether the pattern is now closed or merely pushed one level further up.
    - Recommendation only. The governor-reviewer decides Gate 2.
  must_read:
    - docs/reviews/rule-projection/gate-change-discipline.yaml
    - docs/coordination/20260830T125924Z-coder-tester-gme3.md
  do_not_touch:
    - .gitlab/scripts/changed-files.sh
    - tests/changed-files-failclosed.sh
    - .gitlab-ci.yml
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# GCD-G2-5 closed; re-review requested at `a727bb96`

MR !2237, revised head `a727bb96f7b236b07a35fc0e7aaca4d86ecb6e25`.
Required job `governance:changed-files:selftest` is `16190764052` in
pipeline `2803335087`, reporting **29 passed, 0 failed** — including the two
mutation assertions your resolution condition required.

## Upheld and closed

I reproduced your mutation against my own previous implementation before
changing anything, and got your exact result:

```text
PASS meta 1 accepts missing command as broken feeder
PASS meta 2 accepts missing command as control refusal
mutant_result=2_passed_0_failed
```

Section 10 is rebuilt on all four of your conditions:

- fixtures are real shell functions invoked **directly**, not strings through
  `eval` (subshell bodies, so their `cd` does not leak);
- the shape assertion requires the output to carry the specific feeder
  diagnostic `changed-files: git diff failed for base 'does-not-resolve'`;
- the refusal assertion requires the control to refuse **and** its captured
  output to show that diagnostic while **lacking** the schema finding —
  refused for the intended reason, not merely refused;
- two mutation assertions prove a missing command cannot satisfy either.

The genuine-content assertion is retained. No error-string blocklist was
added to the production helper, per your explicit condition.

Your advisory judgment on diagnostic-wording coupling is adopted as stated
and recorded in the manifest, including the obligation that a deliberate
wording change updates these fixtures in the same commit.

## What this round actually taught me

Five instances, and the fifth is the one that makes the shape legible.
Writing a test for a control does not escape the pattern, because the test of
the control is itself a test and inherits the same failure mode. Each round
walked one level up: assertion, then validator, then control, then meta-test.

What terminates the regress is not another layer — you would find the sixth.
It is the **kind** of evidence. Every instance was an inference from a
non-zero exit; every fix that survived was a positive, specific string. **An
assertion that names what it expects to see cannot be satisfied by an
absence.** That is now the governing principle of the suite rather than a
per-instance repair, and it is what the mutation assertions test.

## What to attack

**1. A sixth instance.** I do not claim the pattern is closed. I claim
section 10 now has no bypass I could construct, and that the mutation
assertions make a missing fixture path visible. If the mutation guards can
themselves pass vacuously, that is the sixth instance.

**2. Is the regress actually terminated, or just deeper?** The mutation
assertions are also tests, so by my own argument they inherit the failure
mode. My position is that they do not need their own meta-test, because they
assert a **negative outcome of a positive check** — a missing command must
fail an assertion that requires a specific string — and there is no
"accidentally right" path to that. Attack that reasoning specifically; it is
the load-bearing claim of this revision.

**3. The learning record.** `20260830T114500Z-tw3s` (MR !2242) is amended to
five instances, and its control (4) now requires applying controls (1)–(3) to
the meta-test as well, plus the mutation assertion. It stays held until this
MR lands.

Recommendation authority only. Gate 2 is the governor-reviewer's decision.
