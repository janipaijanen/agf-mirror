---
kind: handoff
from: contribution-auditor
to: planner-architect
id: 20260728T181205Z-contribution-auditor-rmhr
timestamp: 2026-07-28T18:12:05Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260728T174308Z-planner-architect-ag3m
  - 20260728T174906Z-contribution-auditor-4g77
  - 20260728T180140Z-contribution-auditor-q4hn
topic: v250-roadmap-argument-defence
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Defend or replace the proposed ordered release train through v2.5.0.
    - Place every currently open issue and merge request in a release, work package, or evidence-backed closure or parking disposition.
    - Argue the dependency order and number of releases against the stated counterproposals.
    - Publish dissent, evidence that would change the recommendation, and one adjudicated Gate 1 plan for governor-reviewer review.
  must_read:
    - docs/coordination/20260728T174906Z-contribution-auditor-4g77.md
    - docs/coordination/20260728T180140Z-contribution-auditor-q4hn.md
    - agf/docs/contracts/contract-stability.md
  do_not_touch:
    - agf/VERSION
    - .claude/settings.local.json
    - tooling/elixir
---

# Handoff: argue and defend the release roadmap through v2.5.0

The governor has requested an ordered Statement of Work that may contain
several releases before v2.5.0, and requires consensus through argument and
defence rather than a bare agreement count.

## Evidence snapshot

At approximately 2026-07-28T18:06Z, GitLab had 48 open merge requests and 39
open issues in the reviewed portfolio. The MR snapshot includes branches from
0 to 957 commits behind current `main`; GitLab statuses such as `mergeable`,
`unchecked`, or `ci_must_pass` are not Gate evidence. MR !753 at `ed99375d`
contains the contribution-auditor constraints and passed pipeline 2712933857.

Three parallel same-service role-lens sessions reviewed the initial SoW. They
were separated and did not receive each other's results, but remain correlated
Codex evidence rather than independent corroboration. The authoritative current
manifest marks `build-engineer` active; stale contrary statements were rejected.

## Contested release-train hypotheses

The initial contribution-auditor hypothesis was:

1. R0 portfolio reconciliation and preservation;
2. v2.4.6 recovery patch;
3. v2.5.0-alpha.1 current-contract baseline (#79/#80);
4. alpha.2 declared-input/provenance baseline (#49/#82/#35 limits);
5. alpha.3 accountability/review topology (#45/#47/#82);
6. beta.1 threat/enforcement integration (#42/#34/#35);
7. beta.2 bounded operational reliability (#76/#53); and
8. rc.1, defect-driven later RCs, then v2.5.0.

The fact-checker role lens rejects that breadth and recommends exactly three
planned identities:

1. **v2.4.6** — #81 plus verified #67/#78 documentation and package truth,
   contract/changelog correction, and a current-main rebuild of !340 only if
   its compatibility defect still reproduces. No new contracted surface.
2. **v2.5.0-rc.1** — additive #79/#80 contract repair, remaining #67 semantic
   repairs, and bounded semantics-preserving #76 only. Create rc.2 only if the
   candidate changes.
3. **v2.5.0** — the same functional content as the accepted RC, with release
   metadata and post-publication verification.

Its argument is that #34/#42/#45/#47/#49 are plausible later-minor themes, not
dependencies for restoring upgrade safety and contract truth. Including them
would turn a stabilization release into an evidence-immature trust-platform
expansion. Conversely, putting #79/#80 in v2.4.6 would repeat v2.4.5's patch
SemVer contradiction.

## Required defence

Respond to the GitLab MR !752 argument request and publish the resulting
planner coordination. Do not simply choose the median number of releases.
Defend the chosen train against:

- one large direct-to-v2.5 release;
- many stable patch/minor releases;
- one recovery patch plus bounded minor prereleases; and
- stabilization-only sequencing that parks all new trust-chain work.

For each open issue and MR, give an observed-at timestamp, exact evidence,
target release or work package, disposition, dependency, and closure condition.
Explicitly distinguish `archive-close`, `incorporated`, `superseded`, `rebuild`,
`park`, and `discard`. A parked item needs a named revisit trigger. A release
checkpoint needs a falsifiable exit condition; do not create ceremonial version
numbers.

Use the read-only MR inventory already authorized by the referenced review.
Do not retrigger, merge, close, relabel, delete branches, implement, version,
tag, or release while producing the argument. The final roadmap requires a
separate schema-valid Gate 1 plan and governor-reviewer approval before work
begins.
