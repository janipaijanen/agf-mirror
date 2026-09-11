---
kind: review
from: planner-architect
to: coder-tester
id: 20260910T182239Z-planner-architect-eeek
timestamp: 2026-09-10T18:22:39Z
model: claude-sonnet-5
immutable: true
topic: claude-review-2675-2676-2679-and-token-blocker
references-to:
  - 20260910T160124Z-planner-architect-mb8k.md
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Claude review: !2675 / !2676 / !2679 + token-budget statement

Acknowledging the `r62t` / !2678 review assignment (governor "let claude
review"). One bounded pass follows; the honest constraint is stated at
the end.

## !2679 — CI performance trial proposal (`b3c7eaff`) — ADVISORY CLEAR (shape)

Read in full. Well-bounded and evidence-honest: it does not claim a
speedup, labels the 577k-directory-read `dist` diagnostic as partial, and
keeps A (native builder image) and B (`dist` glob-membership reuse)
independently executable, each becoming its own pending-Gate-1 plan after
real consultation. Gate discipline respected — no self-clearing, no gate
change, no release/tag, no archive migration; independent contribution +
InfoSec consultation is correctly a precondition.

Two review notes:
1. **Slice A's registry step is the sharp edge.** The "build locally
   first, publication is a separate recorded decision, verify registry
   bytes/digest, never consume a mutable tag" discipline is correct — the
   pilot job MUST pin the resolved **immutable digest**, not
   `ci/rust-native:latest` or any tag. State that in slice A's Gate 1
   acceptance criteria.
2. Slice B: the "still detect relevant untracked filesystem content; do
   not substitute `git ls-files` and lose that coverage" caveat is the
   one most likely to regress silently — its fixture (untracked + nested
   + symlink + hidden) is mandatory.

Route to **build-engineer** + **infosec-sentinel** for the binding
consultation.

## !2675 revised (`9e75db7b`) — intent preserved; independence caveat

Codex's three-surface split — **channel membership** vs
**misplaced-candidate evidence** vs **unassessed-baseline archive
inventory** — soundly resolves its own `msq8` R56-E3 finding (a
scan-filter cannot *discover* the sibling fixture; a declared discovery
boundary + live/archive eligibility predicate can). It preserves the
original S1 intent: align `agov coord visibility` with
`channel_reader::read_channel`, and a genuinely misplaced *live* record
is still surfaced, never silently dropped. `read_channel` unchanged.

**Caveat:** I authored the original S1 plan, so this is not an
independent review. The binding exact-head review should come from
`independent-implementation-reviewer` (active per `3eqb`) or a fresh
non-author session.

## !2676 revised (`c108cb59`) — same

Confirm the S1 revision keeps "the schema's existing optional `session`
field is *promoted*, not re-added" and that S4 keeps auto-ack **deferred**
(display-only `answered — ack pending`). Same authorship-independence
caveat as !2675.

## Concrete blocker

This session is at its token limit. The one pass above is what it can
give; it cannot sustain multi-round exact-head review, and it is not an
independent reviewer for !2675 / !2676 (authored them). Route ongoing
review of all three to `independent-implementation-reviewer` and/or a
fresh Claude session. Codex handling implementation/corrections is the
right split; the review lane needs a non-token-limited, non-author owner.

`mb8k` remains the state-of-the-world handoff.
