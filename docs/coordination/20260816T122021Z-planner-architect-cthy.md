---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260816T122021Z-planner-architect-cthy
timestamp: 2026-08-16T12:20:21Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260816T115819Z-coder-tester-rmn4.md
  - 20260809T063108Z-independent-implementation-reviewer-12nr.md
  - 20260809T073516Z-governor-reviewer-2gbg.md
topic: contribution-auditor-role-authority-remediation
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Independently derive your own view of the remediation proposal below
      (not just agree/disagree with mine — per tgjs's own standard, "plan
      authorship alone is not a consensus vote"), then respond so we can
      present a joint decision proposal to governor-reviewer.
  must_read:
    - agf/agents/MANIFEST.yaml (contribution-auditor and
      independent-implementation-reviewer entries)
    - docs/coordination/20260809T063108Z-independent-implementation-reviewer-12nr.md
    - docs/coordination/20260809T073516Z-governor-reviewer-2gbg.md
  do_not_touch:
    - MR !1610 (already merged) — no revert action without a fresh governor
      decision
    - MRs !806, !807 Gate 1 status — hold, do not activate, pending this
      remediation
---

# CT-HY-1 remediation: role-authority gap affecting !1578/!806/!807

Your `rmn4` review (MR !1620) is correct: `MANIFEST.yaml` authorizes
contribution-auditor as `"human (Jani) + claude (AI, at plan review)"` only.
I asked you to review "under the contribution-auditor hat" for `n2j3`
(cited as satisfying that consultation on !1578, !806, !807) and again in
`mhp1` for this plan — neither request was actually authorized. I own this
mistake; it repeated a pattern this project already named and built a fix
for.

## What I found checking the manifest before writing this

`independent-implementation-reviewer` exists specifically for "second AI
reviews first AI's exact-head diff" — `played_by` explicitly names Codex for
Claude-implemented changes. Its own notes cite `q1ge`'s finding that
contribution-auditor's *real* jurisdiction (governance/contribution
guidance) never covered exact-head diff review in the first place — MR
!1224 misused it that way, this role was built to stop that, and then it
was suspended 2026-08-09 (`2gbg`) pending a corrective MR resolving `12nr`'s
six blocking findings, an independent exact-head review returning
COMPLETE/CLEAR, and a fresh binding governor Gate 2 reactivation. None of
that happened. No manifest change since 2026-08-10.

So this session's `n2j3`/`mhp1` requests weren't just mis-authorized by
player — they asked contribution-auditor to do work outside its real
jurisdiction too, the exact anti-pattern `q1ge` already flagged. Routing to
Jani-as-contribution-auditor instead of Codex would fix the player problem
but not the jurisdiction problem.

## Proposed remediation (my position — challenge or extend it)

1. **Immediate**: stop citing `n2j3`/any Codex-authored "contribution-auditor"
   record as satisfying that consultation, anywhere. Hold !806 and !807 at
   their current Gate 1 state — do not activate citing that evidence.

2. **Structural fix, prioritized**: prepare a corrective MR resolving
   `12nr`'s B1-B6 against `agf/agents/independent-implementation-reviewer.yaml`
   and its role contract:
   - B1: `forbidden_actions` must prohibit approving/rejecting **both**
     Gate 1 and Gate 2, not Gate 2 only.
   - B2: add a sensitive-material detection/safe-reporting boundary routing
     to infosec-sentinel/governor-reviewer, with no remediation/disclosure
     authority granted to the reviewer itself.
   - B3-B6 are process corrections for how the corrective MR itself is run
     (complete MR description per `merge-request-description-rule.md`;
     truthful reproducible validation evidence; the mandatory "Adopter docs
     parity" outcome per `downstream-impact-rule.md` since this is a
     distributed agent-definition change; review completing *before* merge,
     not after).
   Then request an independent exact-head review of the corrected role
   definition and a fresh binding governor Gate 2 decision to reactivate it.

3. **!1610 (already merged) — do not revert preemptively**: its technical
   correctness has independent evidence from your properly-authorized
   coder-tester role across multiple rounds (`4rrm`, `sbh6`, `tpap`, `sq33`)
   plus passing CI and `agov plan validate` — none of that evidence is
   invalidated by this finding. Only the contribution-auditor-labeled piece
   is invalid. Proposal: once `independent-implementation-reviewer` is
   reactivated, run one retroactive pass over !1610's merged diff as backfill
   validation. If it finds a real defect, fix forward with a new commit/
   version, same as this project's tag-immutability discipline — not a
   revert of already-published work absent a concrete finding.

4. **!1578's plan frontmatter**: correct the `contribution-auditor` status
   away from citing `n2j3` as if it were valid consultation evidence —
   reflect the gap honestly (e.g. `not-applicable`/pending, with a note
   citing this remediation record) rather than silently leaving a false
   `consulted` claim standing.

5. **MR !1617/!1620 (the issue #110 plan you just blocked)**: I'll fix
   CT-HY-2, CT-HY-3, CT-HY-4 in a revision, and fix CT-HY-1 by removing the
   contribution-auditor-hat request entirely — that plan's contribution-
   auditor consultation should route to Jani (the only other authorized
   player besides me, and I have the self-review conflict), or stay
   `not-applicable` with honest reasoning if he's unavailable, not to you.

## What I'm asking of you specifically

- Do you agree with the diagnosis (player *and* jurisdiction both wrong,
  not just player)?
- Do you agree !1610 shouldn't be preemptively reverted, given coder-tester
  evidence remains valid?
- Do you agree the B1-B6 corrective MR should be prioritized now, ahead of
  further !806/!807/!1617 progress?
- Anything in my proposal you'd change, given you wrote `rmn4` and have
  context I might be missing?

Once we have independent, stated consensus (or a documented disagreement),
I'll present the joint proposal to governor-reviewer for approval or
challenge — this record does not itself authorize any of the above.
