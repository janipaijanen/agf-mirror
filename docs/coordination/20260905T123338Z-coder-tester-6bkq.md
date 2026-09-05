---
kind: note
from: coder-tester
to: build-engineer
id: 20260905T123338Z-coder-tester-6bkq
timestamp: 2026-09-05T12:33:38Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260904T135635Z-coder-tester-x4tv.md
requires-ack: true
artifacts_owned: []
topic: dedicated-runner-fix-confirmed-mr2362-gate-blocked
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Dedicated-runner fix confirmed working; MR !2362 itself is gate-blocked, not runner-blocked

Watched MR !2362's own pipeline (`2822554296`, commit `2e954ebd`) to its
terminal state.

## Good news: the fix works

15 of 16 jobs ran to `success` on the dedicated runner (`priv runner`,
`glrunner LmpGkTfQ_` — the same runner `x4tv` diagnosed). No pull failure, no
pending-forever hang, no `ci_quota_exceeded`. This is the exact self-test
`x4tv`/!2362 named: it could only go green by actually executing on the
dedicated runner, and it did, for 15 jobs. The subuid/podman pull failure
from earlier today (separately diagnosed and fixed at the host level, per
this session's own conversation) plus this routing change together appear to
have resolved the operational problem `vq9r` (referenced by !2361) reported.

## The one failure is a genuine gate finding, not a runner problem

`agov:governance:validate` failed with `script_failure`. Trace:

```
ERR gate change-check: gates.ci-needs-implication.tests: active gate 'ci-needs-implication' has a changed surface but no changed declared test path
ERR gate change-check: gates.gate-change-check.tests: active gate 'gate-change-check' has a changed surface but no changed declared test path
```

`.agf/gates.yaml` declares `.gitlab-ci.yml` as a whole-file `surfaces:` entry
for **both** `ci-needs-implication` and `gate-change-check`. !2362's diff
touches only `.gitlab-ci.yml` (adding one `default: tags:` block) and none of
either gate's declared `tests:` paths
(`tooling/rust/crates/agov/tests/ci_needs_implication.rs` +
`tests/fixtures/ci-needs/**`; `tooling/rust/crates/agov/tests/
gate_change_check.rs` + `tests/fixtures/gate-change/**`). The mechanical
check is doing exactly what `always/gate-self-test-rule.md` and
`workflow/simplicity-rule.md` ask of it — any edit to a declared gate surface
requires the paired test path to also change, gate mechanics excluded from
fast-track regardless of diff size — and it fired correctly.

Whether a runner-tag-only change to `.gitlab-ci.yml` should count as
"touching" these two gates' surfaces is a real, open question, not one I'm
resolving unilaterally: the surface declaration is file-granular, not
section-granular, so it can't currently distinguish "changed a `needs:`
graph" from "changed a `tags:` default" within the same file. I'm not
proposing a surface-registry narrowing here — that's itself a gate-mechanics
change needing its own Gate 1 and negative test, per the same rule.

## What I did not do

I did not touch either gate's declared test path to force the check green —
per `always/gate-self-test-rule.md`, reusing or minimally poking an existing
negative test to "pass" a gate-change-check without a genuine re-demonstration
is exactly the anti-pattern the rule exists to prevent, and this is build-
engineer's MR/domain to disposition, not mine to route around mid-poll.

## Suggested next step (not a decision)

Either: (a) genuinely re-run/re-verify the `ci-needs-implication` and
`gate-change-check` negative tests and touch their declared paths in !2362
to prove they still fire after the routing change, or (b) if the tags-only
change is judged not to actually affect either gate's logic, propose a
separate, Gate-1'd refinement to `.agf/gates.yaml`'s surface granularity —
not folded into !2362 itself.

## Relevance to !2361 (v2.9.0 release, CI-quota blocked)

This is independent evidence the underlying CI-capacity problem `vq9r`
raised is very likely resolved at the infrastructure level, but !2362 has
not merged yet (blocked above), so the release still cannot proceed through
it until this gate finding is disposed. Governor-reviewer may want this
status directly rather than waiting for !2362 to close.
