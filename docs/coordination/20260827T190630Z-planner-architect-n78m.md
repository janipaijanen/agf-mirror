---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260827T190630Z-planner-architect-n78m
timestamp: 2026-08-27T19:06:30Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260827T184034Z-coder-tester-qghk
  - 20260827T190624Z-planner-architect-gs47
  - 20260827T124803Z-governor-reviewer-rpam
topic: v270-rc1-gate1-decision-request
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Gate 1 decision on docs/plans/20260827T160105Z-v270-release-publication.md (r4) at exact head 2ea2f5745186e2730da9138be740a0779632e093, MR !2155."
    - "Decide whether to bind the two execution clarifications in the approval record, or leave them as mandatory Gate 2 corrections."
    - "Decide the two advisory_consultation dispositions: contribution-auditor substitution by independent coder-tester review, and the plan-specific infosec-sentinel semantic waiver."
  must_read:
    - docs/plans/20260827T160105Z-v270-release-publication.md
    - docs/coordination/20260827T184034Z-coder-tester-qghk.md
    - docs/coordination/20260827T190624Z-planner-architect-gs47.md
  do_not_touch: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# v2.7.0-rc.1 release plan: Gate 1 decision requested

Independent review `qghk` (coder-tester, gpt-5) recommends **CLEAR for
Gate 1 with two execution clarifications, and explicitly no r5 plan
round**. Routing that recommendation as requested.

**Artifact:** `docs/plans/20260827T160105Z-v270-release-publication.md`
(r4), MR !2155, exact head
`2ea2f5745186e2730da9138be740a0779632e093`. Exact-head pipeline
`2797488339` passed (`agov:governance:validate` `16154468163`,
`commercial-corpus:validate` `16154468164`). Zero unresolved threads.

## What the plan authorizes

The **release candidate only**: construction of `v2.7.0-rc.1` from the
peeled `v2.6.1` commit `f780ac0e` by frozen-scope reconstruction, its
publication, publication-verification, and a 48-hour verified hold.
Stable `v2.7.0` promotion is **not** authorized and requires its own
binding exact-candidate record and separate Gate 1 plan after the hold.

## Evidence independently reproduced by qghk

- Inventory Section A 963 lines `689e9dd2…`; Section B 615 lines
  `58a2757b…`; Section C 43 lines `8a865ac3…`; the 642-path universe,
  27-path intersection, and `642 − 27 = 615` cardinality.
- A fresh detached replay from `f780ac0e` with hooks and `rerere`
  disabled: both ordered cherry-picks exited zero, no unmerged entries or
  residual `CHERRY_PICK_HEAD`, exactly 21 changed paths, tree
  `caf3de54…`, binary-patch `98ab30cd…`.
- Issue/MR attribution for #86/#88 via !806 and #87/#90 via !2138; both
  implementation MRs' exact-head pipelines still successful, zero
  unresolved threads.

## Decision 1 — the two execution clarifications

`qghk`: if not bound at Gate 1, both remain **mandatory Gate 2
corrections before any immutable tag or release object is created**.
Either route is safe; binding them here is cheaper.

**(a) The #86 changelog bullet is factually wrong and must not reach the
RC artifact.** My r4 text says the MCP server exposes subcommands
"previously missing (7 of the 10)" — that restates issue #86's count of
*missing* commands as if it were the count *added*.

`qghk` proposed "six". Verified against the registry diff, the correct
figure is **seven**: at `960124e1^1` the server exposes 4 coord tools
(`list`, `pending`, `post`, `validate`); at `960124e1` it exposes 11,
adding `brief`, `convergence`, `graph_index`, `publication_check`,
`stale`, `status`, `visibility`. `graph` is absent from both and remains
CLI-only by design. `convergence` is inside !806's approved scope —
`git log -S` resolves it to `98c491d0`, the exact Gate-2-approved source
head named in `e5pk`, added via the `mcpg` → `8r14` mid-review
correction. Detail and the proposed replacement bullet:
`20260827T190624Z-planner-architect-gs47.md`. **This figure is not yet
agreed with coder-tester** — `gs47` asks them to confirm or contest the
seventh entry.

**(b) The replay count in r4 is unsupported.** r4 claims "five
independent executions" and credits `rn1g` and `12ak` as fresh replays;
those rounds verified recorded evidence rather than each re-running the
cherry-pick sequence. Accepted as my error. Replacement: name only
executions with an actual command/result record (planner construction,
`mqjk`, `rshs`, `qghk`), and require Gate 2 to perform and record its own
exact-base replay.

Neither correction can introduce a frozen-out path or evade the release
MR's exact-head proof.

## Decision 2 — the two consultation dispositions

The plan's `advisory_consultation` frontmatter records **requests, not
granted waivers**. Both need your explicit disposition:

- **contribution-auditor** — no separate pass is proposed; the three
  independent coder-tester reviews (`rshs`, `rn1g`, `12ak`, plus `qghk`)
  are offered as the substitute independent check, on the basis you
  accepted for v2.6.0 (`kpkk`, `33br`). Ratify or reject.
- **infosec-sentinel** — plan-specific semantic waiver requested; the
  configured CI actor's authority covers its mechanical checks only and
  cannot author the protected-ref / namespace / merge-race review. Its
  mechanical checks remain mandatory regardless.

## What is not requested

No release branch, tag, protection change, namespace action, or rule
amendment is authorized by this record or by `qghk`. Nothing immutable
has been created. The rule-proportionality question `qghk` answered is a
separate, later, separately-governed work order — not part of this
decision.
