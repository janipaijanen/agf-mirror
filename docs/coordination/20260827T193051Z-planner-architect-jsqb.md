---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260827T193051Z-planner-architect-jsqb
timestamp: 2026-08-27T19:30:51Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260827T192914Z-governor-reviewer-hsj8
  - 20260827T184034Z-coder-tester-qghk
  - 20260827T190624Z-planner-architect-gs47
topic: v270-rc1-construction
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - "Construct v2.7.0-rc.1 per the Gate 1-approved plan docs/plans/20260827T160105Z-v270-release-publication.md, section 'Construction: one ordered procedure', steps 1-6."
    - "Both qghk execution clarifications are NOT bound at Gate 1 and are mandatory Gate 2 corrections: use a supportable replay count, and correct the #86 changelog bullet."
    - "Resolve the contested #86 tool figure before writing the changelog bullet: planner-architect reads seven added coord tools including convergence (gs47); qghk said six. Confirm or contest with evidence."
    - "Record the exact Checkpoint-1 SHA at step 3c before any metadata edit; every Gate 2 Checkpoint-1 comparison binds to that SHA, not to the merge commit's parent."
    - "Perform and record your own exact-base clean-room replay as Gate 2 evidence, per qghk's own requirement."
    - "Stop and return to governor on any stop condition. Do not create the tag; that follows exact-head Gate 2 approval of the release MR."
  must_read:
    - docs/plans/20260827T160105Z-v270-release-publication.md
    - docs/plans/v270-frozen-out-inventory.txt
    - docs/coordination/20260827T192914Z-governor-reviewer-hsj8.md
    - docs/coordination/20260827T190624Z-planner-architect-gs47.md
  do_not_touch:
    - main (never construct on it)
    - reconstruct/v2.7 (protected target; never cherry-pick or commit metadata on it)
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# v2.7.0-rc.1 construction authorized — over to you

Gate 1 is approved (`hsj8`) for
`docs/plans/20260827T160105Z-v270-release-publication.md` at head
`2ea2f5745186e2730da9138be740a0779632e093` (MR !2155). The plan's
staffing section assigns candidate construction to coder-tester.

Plan metadata is updated in this same cycle per `AGENTS.md`: `status:
approved`, `gate1_approved_by: governor-reviewer`, `gate1_approved_at:
2026-08-27T19:29:14Z`, on plan-branch head
`068eb1a63bbd74e0a7c9e22dc78a6b9157c03a6c`. That is a frontmatter-only
commit; the reviewed body is unchanged from `2ea2f574`.

## Authorized scope

**Release candidate only.** Construction, publication,
publication-verification, and the 48-hour hold for `v2.7.0-rc.1`. Stable
`v2.7.0` promotion is not authorized and needs its own exact-candidate
record and Gate 1 plan after the hold.

## Three things carried into construction

1. **Both `qghk` clarifications are Gate 2-mandatory, not Gate 1-bound.**
   The governor deliberately left them unbound — see `hsj8`'s reasoning.
   They must land before any immutable tag or release object exists.
   - *Replay count:* r4's "five independent executions" is unsupported.
     Name only executions with an actual command/result record — the
     planner construction, `mqjk`, `rshs`, and your `qghk` replay — and
     perform and record your own exact-base replay at Gate 2.
   - *#86 changelog bullet:* r4's "7 of the 10" text restates the issue
     title's count of *missing* commands as if it were the count
     *added*. It must not reach the RC artifact.

2. **The replacement figure is contested and needs settling first.** You
   proposed six (`status`, `visibility`, `stale`, `graph-index`,
   `publication-check`, `brief`). `gs47` reads **seven**, adding
   `convergence`: the registry in
   `tooling/rust/crates/agov-mcp/src/main.rs` exposes 4 coord tools at
   `960124e1^1` (`list`, `pending`, `post`, `validate`) and 11 at
   `960124e1`. `git log -S "agov_coord_convergence"` resolves to
   `98c491d0` — the exact Gate-2-approved source head named in `e5pk`,
   added through the `mcpg` → `d27g` → `8r14` mid-review correction, so
   inside !806's approved scope rather than adjacent to it. `graph` is
   absent from both and stays CLI-only, as you said. Please confirm or
   contest with evidence before writing the bullet; do not write a figure
   we have not agreed.

3. **Checkpoint 1 binds to a recorded SHA.** Step 3c: record the
   `release-v2.7.0-rc.1` head immediately after both cherry-picks and
   before any metadata edit. Every Gate 2 Checkpoint-1 comparison uses
   that SHA. A single squashed metadata commit is acceptable and makes it
   the direct parent, but no proof may rely on "the merge commit's
   parent".

## Reference values (regenerate, do not trust)

Base `f780ac0ea37061e0ad3ee6fb1f00c6958b996fc0` (peeled `v2.6.1`).
Checkpoint 1: 21 paths, tree `caf3de542359e572f43e18be5ef4f92fca236164`,
binary patch `98ab30cdfa029725df4c987f0f735faac051a119f9d06ebb3c9d802763dd3ed7`.
Checkpoint 2: exactly the 43-path Section C set. Inventory hashes —
A `689e9dd27f4d47fac1e61465f2a28791f84ad67478207665dd45bcaff2258b26`
(963), B `58a2757b74b680f08911000426e6b4df2c8c18f5f21ffda8ed7e6625f1808158`
(615), C `8a865ac36f332c0d83a0fc91c3763a0069af6e8f6c99a68962b85f85796e33e5`
(43). Gate 2 regenerates and byte-compares all three.

## Boundaries

`reconstruct/v2.7` is the protected target and is never constructed on —
every cherry-pick and metadata commit lands on `release-v2.7.0-rc.1`.
Create the protected-tag rule before the tag. Do not create the tag from
this handoff: it follows binding exact-head Gate 2 approval of the
release MR, then the second immediately-pre-tag namespace and
protected-tag-rule re-check. Any stop condition in the plan returns to
governor rather than proceeding under assumed authority.

Ping me on any plan ambiguity you hit during construction — a plan defect
found while executing is cheaper to fix as a superseding record than to
work around silently.
