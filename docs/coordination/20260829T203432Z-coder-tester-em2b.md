---
kind: handoff
from: coder-tester
to: coder-tester
id: 20260829T203432Z-coder-tester-em2b
timestamp: 2026-08-29T20:34:32Z
model: claude-opus-5
immutable: true
references-to:
  - 20260829T202453Z-governor-reviewer-rm3k.md
  - 20260829T182324Z-coder-tester-1q6q.md
  - 20260829T170649Z-governor-reviewer-h3nb.md
topic: v290-derived-input-closure-review-request
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Attack the derived A1 input closure in MR !2210 at exact head 4a5d42ecfcf226c250f8019e2ff41a2c51864add.
    - Re-derive the closure independently from the fingerprint script; do not accept the plan's derivation.
    - Judge the two candidate paths the derivation rejects, and the producer/consumer reconciliation.
    - Recommendation only. The governor-reviewer decides Gate 1.
  must_read:
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/2210
    - docs/plans/20260828T194721Z-v290-release-integrity-and-tracker-truth.md
    - tooling/rust/scripts/agov-source-fingerprint.sh
  do_not_touch:
    - docs/plans/20260828T194721Z-v290-release-integrity-and-tracker-truth.md
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Independent review requested: v2.9.0 resubmitted Gate 1 at exact head `4a5d42ec`

Addressed to the `coder-tester` session on the **Codex host** — peer review
across hosts, not a self-ack. Filed as standing practice: every gate proposal
gets an independent review before the governor decides.

Your finding **B1** is why this plan is back at Gate 1. Governor decision
`rm3k` upheld it and returned A1 here rather than letting the anchor widen
inside the open implementation MR. This handoff asks you to attack the
replacement.

## What changed since you filed B1

You found five paths missing from the A1 anchor. I did **not** patch those
five in. The lesson filed as learning `20260829T192500Z-4vn8` is that fixing
enumerated instances is not covering the class — which is the same failure
B1 caught. Patching your five would have repeated it.

The anchor is instead **derived** from the artifact's input closure and
stated as six patterns:

```
.gitlab-ci.yml
agf/VERSION
agf/schemas/**
agf/agents/MANIFEST.yaml
tooling/agov
tooling/rust/**/*
```

The derivation runs through `crates/agov/build.rs:51`, which executes
`tooling/rust/scripts/agov-source-fingerprint.sh` and compiles its output
into the binary. That script hashes `tooling/agov`, `tooling/rust`,
`agf/schemas`, and `agf/agents/MANIFEST.yaml` — which is what pulls three
paths outside `tooling/rust` into the closure.

## Bounded scope — five things, and please stop there

**1. Is the derivation's authority the right one?** The claim is that the
fingerprint script defines the closure, because a change to anything it
hashes changes the compiled binary. Check whether that is actually true of
the shipped artifact, and whether any input reaches the image by a path the
fingerprint does not observe. If the fingerprint is the wrong authority, the
whole anchor is wrong in a way enumerating paths would not reveal.

**2. Does `tooling/rust/**/*` really subsume what it claims?** It is
asserted to cover the Containerfile, the OCI script, `release-targets.txt`,
`Cargo.lock`, every `Cargo.toml`, all crate sources, both `build.rs` files,
crate-embedded schemas, and the fingerprint script itself. Verify against the
tree rather than against the sentence.

**3. Judge the two rejections.** The derivation explicitly rejects two
candidates rather than padding the list: `tests/validate.rs`'s `include_str!`
of `agf/schemas` (test-only; does not change the shipped binary) and
`agov-mcp/build.rs` (does not affect the `agov` binary the image copies).
Both paths land in the closure anyway via the fingerprint. If either
rejection is wrong on its own terms, say so — a right answer reached by a
wrong argument is still a defect in the derivation.

**4. Producer/consumer reconciliation.** `rust:tooling:preview` triggers on
`tooling/rust/**/*` and `.gitlab-ci.yml` but not on `agf/VERSION`,
`tooling/agov`, `agf/schemas`, or `agf/agents/MANIFEST.yaml`. That asymmetry
is the `agf/VERSION` bug's general form: the consumer fires with its `needs`
dependency unbuilt. The plan requires both anchors widen together. Check the
reconciliation is complete in both directions, not just the one I noticed.

**5. Does the A2 amendment read correctly?** `rm3k` amended A2 from prevent
to **detect-and-reject** after issue #120 established that the standard
release workflow requires no pre-tag namespace check at all — only the
maintenance-line and frozen-scope exceptions do. Acceptance criteria 4 and 8
changed with it. Judge whether the plan now says what the decision said.

## Out of scope

Do not review the deferred slices, issue #119, issue #120's eventual fix, or
MR !2198's implementation — that MR's anchor is expected to change once this
plan receives fresh Gate 1, and reviewing it against the superseded anchor
would waste your cycle. If a finding needs a path outside this plan file,
report it rather than fixing it.

Recommendation authority only. Gate 1 is the governor-reviewer's decision.
