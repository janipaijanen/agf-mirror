---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20260724T151451Z-governor-reviewer-h737
timestamp: 2026-07-24T15:14:51Z
model: human-governor
immutable: true
references-to:
  - 20260724T151100Z-governor-reviewer-ksjq
  - 20260724T151121Z-contribution-auditor-s36p
  - 20260724T150622Z-planner-architect-7km2
topic: v242-gate1-plan-revision-required
requires-ack: true
artifacts_owned:
  - docs/plans/v2.4.2-public-experience-and-evidence-presentation.md
---

# Governor decision: revise v2.4.2 plan before implementation

The governor requires planner-architect/Claude to revise the v2.4.2 Gate 1
plan in response to contribution-auditor review `s36p`.

This decision supersedes `ksjq` for implementation authority. Although `ksjq`
recorded Gate 1 approval before `s36p` became visible, **no implementation is
authorized under the currently approved plan or MR !476 SHA**. A revised plan
must pass contribution-auditor review and receive a fresh governor Gate 1
decision at its new exact SHA before coder-tester begins implementation.

Claude must return a correction matrix and revise the plan to:

1. freeze the v2.4.2 scope before Gate 1 by either dispositioning every
   carried-forward WARN/INFO finding now or excluding unverified items pending
   a separate plan amendment/new Gate 1;
2. define the exact implementation slice/MR structure rather than choosing it
   after approval;
3. name the canonical artifact for the assistant-byline convention, clarify
   its prospective boundary, and identify representative acceptance records;
4. require explicit fact-checker review of the exact future marketing,
   data-boundary, maturity, command-inventory, and dossier text before Gate 2;
5. reconcile already-merged MR !328 and state whether v2.4.2 absorbs and
   resolves open MR !400's dossier-planning request;
6. distinguish the v2.4.1 release-content baseline from the current branch
   base, rebase, and re-run live-status and plan validation checks.

The homepage-table placement decision itself is settled as **CONDENSE** and
does not need another consultation. No merge, version change, tag, release,
or deployment authority is granted by this decision.
