---
kind: handoff
from: contribution-auditor
to: planner-architect
id: 20260728T161947Z-contribution-auditor-pmjr
timestamp: 2026-07-28T16:19:47Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260726T231147Z-governor-reviewer-3ess
topic: open-portfolio-release-disposition
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - replace the blocked v2.5.0 portfolio head with a current issue-and-MR disposition
    - return amended issue 75 and issue 76 plans for Gate 1 decisions
    - propose one preservation-and-closure batch for completed coordination MRs
  must_read:
    - GitLab issue 57 and MR 661
    - GitLab issues 75 through 81
    - GitLab MRs 340, 463, 654, 739, 745, 746, and 748 through 750
  do_not_touch:
    - release tags or version files before a new release plan is approved
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Current portfolio and open-MR disposition after v2.4.5

## Result

There is **no approved next-release roadmap**. Release v2.4.5 is complete,
issue #57 remains only a Level 1 discovery umbrella, and MR !661 has a durable
human Gate 1 BLOCK in `20260726T231147Z-governor-reviewer-3ess`. Its current
head must not be merged or used as implementation authority.

Do not declare v2.4.6 or begin v2.5.0 implementation from the present open-MR
queue. First replace or comprehensively revise !661 on a current-main branch,
with discovery acceptance separated from implementation authorization and
with genuinely independent consultation where claimed.

## Release-position lanes

| Lane | Current work | Position |
|---|---|---|
| Patch-candidate screen | #67, #78, #81 and the still-open implementation in !340 | Assess adopter impact and compatibility first. #78's stale Pages links and #81's shipped runbook-test failure are concrete release-artifact defects; this is not yet authorization for v2.4.6. |
| Active Gate 1 planning | #75 / !745 and #76 / !746 | Keep outside any release promise until their plans are amended, reviewed, and approved. |
| Cross-repository intake | #77 / !748 | Design lane only. Reuse #75 lifecycle semantics while keeping origin, audience, evidence status, and intake resolution orthogonal. |
| Governance-contract candidates | #79 and #80 | Require explicit compatibility and semantic-version analysis before assignment to a patch or minor release. |
| v2.5 discovery backlog | #31-#49 and #53 | Re-screen under #57; listing is not adoption. Preserve the earlier bounded trust-chain recommendation rather than importing the whole backlog. |
| Radar | #59-#62 | Keep outside release workflow and board lifecycle. |
| Historical intake | #17-#19 | Verify that surviving findings/learnings have canonical destinations, then close or archive rather than carry them indefinitely. |

## Merge assessment

No open MR is ready for an immediate product merge solely from its present
forge status. A green historical pipeline is not current Gate 2 authority.

Bounded candidates, in recommended order:

1. **!739** — corrected after the fact-check ADAPT and currently green. Obtain
   an exact-head fact-check/CLEAR for `cf3be32b`, then a human Gate 2 decision.
2. **!745** — add the one-line #75/#77 resolution-vocabulary boundary already
   requested in !750, revalidate, and return the exact head for Gate 1.
3. **!746** — ADAPT before Gate 1. The plan currently changes the default CLI
   scan contract while calling contribution audit not applicable and leaves
   the exact default to implementation. Split the semantics-preserving batch
   optimisation from any default/flag behavior change, or specify and review
   the compatibility contract now. Incorporate #78's stale-ref and visibility
   evidence. Do not introduce an implicit GitLab/open-MR network dependency.
4. **!340 and !463** — potentially useful implementations, but both need a
   current-main rebase, current CI, confirmation that their issues are not
   already superseded, and fresh exact-head Gate 2 review.
5. **!748-!750** — coordination-publication MRs, not product changes. Their
   obsolete literal CI-skip markers were removed and real lightweight
   pipelines started at `89af35f`, `0b5698c`, and `104816c`. They may use the
   governed coordination fast path after those pipelines pass and the target
   tip is rechecked; they do not authorize the work they describe.

## Close, preserve, or rebuild assessment

- **Close after one verified fresh-main coordination archive:** completed
  MR !719 review/approval records !723, !726-!736 (except the still-useful
  issue handoffs !725 and !728), and completed v2.4.5 release records
  !738 and !740-!744. Preserve unique immutable coordination files first;
  do not merge stale source histories or delete branches as part of closure.
- **Keep until successor action is visible:** !725, !728, !747, !748-!750,
  and the pre-v2.5 inputs !671-!674. Close these only after their records are
  on `main` or a successor plan links and incorporates them.
- **Replace rather than merge the current head:** !661. Publish the corrected
  portfolio on a fresh current-main branch, then close !661 as superseded.
- **Explicitly blocked or stale/conflicted heads:** !341, !349, !407, !430,
  !459, !465, and !658 must not merge as-is. Preserve still-valid findings in
  current issues/plans, then close the old heads and rebuild only adopted work.
- **Decision needed, not silent drift:** draft !461 and plan !654. Either
  establish their current value and next gate on a fresh head or close them
  with a reason; a past review request is not a permanent roadmap position.

## Requested planner response

Return a compact current-state table that assigns every open issue and MR one
of: `act-now`, `candidate-next-patch`, `v2.5-discovery`, `research`, `radar`,
`archive/close`, or `superseded/rebuild`. For every `act-now` item, name the
next required review or gate and its exact current head. Recommend a release
number only after the #67/#78/#81 defect screen and #79/#80 compatibility
analysis establish a coherent release character.

This handoff is an assessment and planning obligation. It is not Gate 1,
Gate 2, merge, issue-closing, version, or release authorization.
