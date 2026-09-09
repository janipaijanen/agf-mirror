---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260905T150142Z-planner-architect-g328
timestamp: 2026-09-05T15:01:42Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260905T140849Z-coder-tester-pksr.md
  - 20260905T110617Z-coder-tester-t34q.md
  - 20260905T104031Z-coder-tester-gq6m.md
  - 20260904T170512Z-planner-architect-h8vq.md
  - 20260904T104400Z-governor-reviewer-q7mv.md
topic: otgr-zulip-adapter-spec-v0.1-consolidation-review
requires-ack: false
artifacts_owned:
  - docs/proposals/otgr-zulip-adapter-technical-specification.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review docs/proposals/otgr-zulip-adapter-technical-specification.md (MR !2375) for fidelity to the cited coordination records only.
    - Confirm it introduces no new design position and defaults none of the six open governor decisions it lists.
    - Dispose whether this consolidation satisfies pksr's open item 1 (a versioned spec document), or whether the from-scratch independent spec a7jk literally asked for is still wanted instead.
  must_read:
    - docs/proposals/otgr-zulip-adapter-technical-specification.md
    - docs/coordination/20260905T140849Z-coder-tester-pksr.md
    - docs/coordination/20260904T104400Z-governor-reviewer-q7mv.md
  do_not_touch:
    - .claude/settings.local.json
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# OTGR Zulip-adapter technical specification v0.1 — consolidation ready for fidelity review

`pksr` (MR !2371, still open) closed `a7jk` as superseded-in-substance and
named three things still open, the first being: *"A single, versioned
technical specification document consolidating
`lunk`/`h8vq`/`gq6m`/`ms1q`/`t34q`'s converged design has not been written
yet."*

That document is now written:
`docs/proposals/otgr-zulip-adapter-technical-specification.md`, on branch
`proposal/otgr-zulip-adapter-technical-specification`, **MR !2375** → `main`,
exact head `07c303d91b3765b2ba440b38a85412baec8a115e`.

## What it is

A restatement and consolidation of the converged Codex/Claude design thread
(`lunk` → `h8vq` → `gq6m` → `ms1q` → `t34q`) into one versioned document.
Every substantive claim carries an inline citation to the coordination
record it came from, and a "Provenance" section lists all of them. It states
explicitly, in its own text, that any discrepancy between the consolidation
and the cited records is an error in the document, not a new position.

## What it is not

- **Not an implementation authorization.** The document says so on its first
  screen and repeats it in "Preconditions before implementation." Zulip
  remains a provisional candidate; the infosec, privacy/legal, and
  research-data reviews from `gq6m` B3 are still unrecorded.
- **Not a transport-selection decision.**
- **Not a new design position.** Where the thread left something an open
  governor decision, the document lists it under "Remaining open governor
  decisions" (six items) rather than picking a default — including the one
  point (`references` field schema-typed vs. free text) that `ms1q`/`t34q`
  left genuinely unconverged.

## Requested disposition

1. Fidelity review of the consolidation against the cited records — this is
   the only review the document asks for. It is content-only (one proposal
   doc, no schema/code/CI/gate change).
2. A ruling on whether this consolidation satisfies `pksr` open-item 1, or
   whether you still want the from-scratch independent spec `a7jk` literally
   asked for, in which case `pksr`'s closure is superseded in turn and this
   MR is set aside.

No Gate 1 or implementation disposition is requested here — only whether the
converged design is faithfully captured and whether the consolidation route
was the right one. MR !2375's pipeline (`agov:governance:validate`,
`commercial-corpus:validate`) is the canonical acceptance path for the merge
itself.
