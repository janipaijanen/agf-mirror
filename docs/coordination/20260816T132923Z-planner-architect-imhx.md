---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260816T132923Z-planner-architect-imhx
timestamp: 2026-08-16T13:29:23Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260816T124942Z-governor-reviewer-pehh.md
  - 20260816T130941Z-governor-reviewer-p4hk.md
  - 20260816T130240Z-coder-tester-g1k1.md
topic: reactivation-and-staffing-implementation-review
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Review MR !1632 (independent-implementation-reviewer reactivation
      implementation) at its exact head.
    - Review MR !1633 (contribution-auditor Codex staffing implementation)
      at its exact head.
  must_read:
    - docs/plans/20260816T130924Z-independent-implementation-reviewer-reactivation.md
    - docs/plans/20260816T130924Z-contribution-auditor-codex-staffing.md
  do_not_touch:
    - Neither MR is mergeable without governor-reviewer's own binding Gate
      2 decision — both plans state this explicitly, this is not a normal
      coder-tester-clears-then-merges flow.
---

# Gate-1-approved plans implemented; requesting review before Gate 2

Governor approved Gate 1 for both plans ("gate 1 approved. proceed"). I've
implemented both as separate MRs, matching each plan's own "Proposal"
section exactly.

## MR !1632 — independent-implementation-reviewer reactivation

`agf/agents/MANIFEST.yaml`: `independent-implementation-reviewer.status`
`suspended` → `active`, citing `g1k1` and MR !1289's exact head. The
`reason` field is explicit that `status: active` here is the *proposed*
end state — not in effect until governor's own fresh binding Gate 2
reactivation decision merges this change. No change to `played_by`.

Please independently verify: the citations are accurate (I already
cross-checked `g1k1`, `q86p`, and MR !1289's exact head directly before
writing this); `agov validate agents` and bare `agov validate` pass (I ran
both, clean); no other manifest fields drifted.

## MR !1633 — contribution-auditor Codex staffing

`agf/agents/MANIFEST.yaml`: adds Codex to `contribution-auditor.played_by`,
scoped to "only for contributions not authored or materially co-authored by
codex". `agf/agents/contribution-auditor.yaml`: adds a new
`cross_llm_independence` list under `boundaries` — `p4hk`'s exact five
points, not a paraphrase — and a `jurisdiction_boundary` note
cross-referencing `independent-implementation-reviewer.yaml`'s existing
header language rather than duplicating it.

Please independently verify: the cross-LLM policy text matches `p4hk`
exactly (I copied it, but please re-check rather than trust that); the
jurisdiction boundary doesn't accidentally expand scope; `agov validate
agents` passes (confirmed clean on my end).

## Why I'm asking for review before presenting for Gate 2

Both plans explicitly require it: !1632's plan says "governor-reviewer
records a fresh binding Gate 2 decision reactivating the role" as a
distinct step; !1633's plan says its own Gate 2 is self-referential and
"neither Claude nor Codex may self-supply the binding audit or gate
decision for that change." Your coder-tester review is the exact-head
technical check both plans want in place before I present either for
governor's Gate 2 decision — it is not itself Gate 2, and per `p4hk`,
until his decision merges either MR, Codex remains authorized only under
its current roles (which does not yet include contribution-auditor).

## Verification (my side, before this handoff)

- Both MRs: `agov validate agents`, bare `agov validate` — clean (only
  pre-existing, unrelated WARNs).
- !1632: `agov plan projection-check --base-ref origin/main` on the
  existing role's rule-projection file — OK.
- `git diff --check` on both — clean.
