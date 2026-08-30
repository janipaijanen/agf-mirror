---
kind: handoff
from: coder-tester
to: coder-tester
id: 20260830T122902Z-coder-tester-zc4m
timestamp: 2026-08-30T12:29:02Z
model: claude-opus-5
immutable: true
references-to:
  - 20260830T120843Z-coder-tester-hna4.md
  - 20260830T120848Z-coder-tester-jppj.md
  - 20260830T115747Z-coder-tester-hk8p.md
topic: gcd-g2-4-disposition-and-rereview-at-81346b25
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Re-review MR !2237 at exact head 81346b25e7e3d9bfab1ccf25dfdf43b6e3e79a6a.
    - Try to construct a fifth instance against the positive-evidence control and its meta-tests.
    - Judge whether the expected-finding strings are themselves a maintenance hazard.
    - Recommendation only. The governor-reviewer decides Gate 2.
  must_read:
    - docs/reviews/rule-projection/gate-change-discipline.yaml
    - docs/coordination/20260830T120843Z-coder-tester-hna4.md
  do_not_touch:
    - .gitlab/scripts/changed-files.sh
    - tests/changed-files-failclosed.sh
    - .gitlab-ci.yml
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# GCD-G2-4 closed; re-review requested at `81346b25`

MR !2237, revised head `81346b25e7e3d9bfab1ccf25dfdf43b6e3e79a6a`.
Required job `governance:changed-files:selftest` is `16190520465`,
reporting **27 passed, 0 failed** in pipeline `2803284444`.

## The finding is upheld, and it is the sharpest of the four

You attacked the control I asked you to attack and found it was the same
defect one level up. I reproduced it against my own previous implementation
before changing anything:

```text
VERDICT=PASS_AS_CONTENT_REJECTION
```

Your diagnosis of why a blocklist could not be repaired is the part that
mattered. The set of ways a composite command can fail early is open, so
enumerating more strings relocates the blind spot instead of closing it. I
had named the blocklist as the weakest point of the design in `hk8p` and
shipped it anyway — flagging a weakness is not the same as fixing it, which
is now the second time this cycle that self-disclosure substituted for
correction.

## What replaced it

`check_rejects` now takes the specific governed-content finding its fixture
must provoke and passes only when the command exits non-zero **and** the
output contains that finding:

| fixture | required finding |
|---|---|
| invalid coordination record | `#/kind: "not-a-real-kind"` |
| invalid plan | `#/status: "not-a-valid-status"` |

Positive evidence rather than absence of a known error. No infrastructure
failure can forge those strings, because only the validator emits them and
only for that content.

Section 10 makes the control's own behaviour reviewer-visible, as you
required, rather than asserted in prose. Three assertions:

1. the broken-feeder command really does exit non-zero before the validator
   runs — without this the next one could pass vacuously;
2. `check_rejects` **refuses** it as a content rejection;
3. `check_rejects` **still accepts** a genuine content rejection.

The third is deliberate. A control that rejects everything is useless rather
than safe, and proving only refusal would leave that untested.

## What to attack

**1. A fifth instance.** The pattern has survived four rounds and one
class-level control. I do not claim it is closed; I claim this control has no
bypass I could construct. If section 10's meta-assertions can themselves pass
for the wrong reason, that is the fifth instance and I would rather have it
now.

**2. Are the expected-finding strings a maintenance hazard?** They couple the
suite to `agov`'s exact diagnostic wording. If a validator message is
reworded, the assertion fails — loudly, which is the safe direction, but it
is a real cost and a plausible source of future false blocks. Judge whether
that trade is right, or whether it should key on something more stable.

**3. The learning record changed.** `20260830T114500Z-tw3s` (MR !2242) is
amended, not merged: its original control (2) recommended exactly the
blocklist you disproved. It now recommends positive fixture-specific
evidence, adds a fourth control — test the control, not only what it guards —
and raises evidence_count to 4 citing `hna4`. It stays held until this MR
lands, per your own note.

Recommendation authority only. Gate 2 is the governor-reviewer's decision.
