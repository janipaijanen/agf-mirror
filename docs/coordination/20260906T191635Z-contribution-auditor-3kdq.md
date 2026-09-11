---
kind: note
from: contribution-auditor
to: governor-reviewer
id: 20260906T191635Z-contribution-auditor-3kdq
timestamp: 2026-09-06T19:16:35Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260906T123407Z-contribution-auditor-52nt.md
topic: b4-stale-july-mr-disposition-executed
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# B4 stale-MR disposition — executed on governor direction

Per the governor's "proceed as proposed" plus "assist with need-decision and
then close-or-refile" (2026-09-06 session), the B4 verify-each pass from the
release-scope inventory (MR !2439 Part B4) has been acted on. Each close has a
citing comment on the MR naming the current-main artifact that covers the
theme. All closes are reversible (reopen or re-file).

## Closed (8)

| MR | Cause |
|---|---|
| !465 autonomous-polling cross-branch gap | insight is the standing poll contract (`coord visibility --base-branch main`, cross-branch scan, `mskd` Layer 1) |
| !829 atomic MR ≠ pipeline dedup | captured near-verbatim in AGENTS.md first-push-limitation paragraph |
| !932 coordination-title per-commit mistake | consequence mechanically caught by `agov coord publication-check`; `coordination-publication-rule` covers title semantics |
| !461 assistive-tech boundary (DRAFT) | abandoned draft, 6+ wk stale, no theme landed |
| !341 AGF/ADR/tracker coexistence page | 6+ wk stale, Gate-2-bounced, position/fast-path docs evolved; routed to issue #118 if still needed |
| !349 coordination-before-bridge-notes | discipline established in `coordination-publication-rule.md` + AGENTS.md Visibility rule; record also mis-targeted the bootstrap `agents-govern/learnings/` dir |
| !430 tooling doc-impact assessment | `downstream-impact-rule.md` covers `tooling/**` + impact classification; re-propose the 41-line addition standalone if it has novel mechanism |
| !850 release-transition duplicate matrix | theme tracked in issues #53 + #106; fold novel specifics there |

## Left open — recommend LAND, governor/planner-architect disposition (2)

| MR | Why not closed |
|---|---|
| !739 commit-message `[ci skip]` substring FP | learning `m969` is a genuinely uncaptured hazard: GitLab honours `[ci skip]`/`[skip ci]` **in descriptive prose**, not just as a directive (4 zero-job pipelines on MR !737). Current `coordination-publication-rule.md` forbids the marker but does not warn against quoting the literal string when documenting the rule. Recommend landing `m969` or a one-line rule warning. |
| !776 release agent wrong-project identity | high-consequence uncaptured failure mode (release ops against the wrong project via an inherited checkout). AGENTS.md's "confirm Gate 2 target/remote with the governor" is adjacent, not equivalent. Recommend landing the record. |

## Left open — governor call (1)

| MR | Recommendation |
|---|---|
| !407 push discipline / pipeline source multiplication (2 records, ~170 lines) | themes covered (AGENTS.md first-push paragraph, "don't retry a paid job for green", `branch-and-worktree-rule` pre-push checks). Recommend close unless a skim finds novel material. |

## Not touched (outside this B4 set)

!1723 / !1750 (commercial-licensing) remain the separate governor
disposition from digest `52nt` / MR !2437. !1446 / !1691 / !1718 / !1722
(v2.5.0-era external DD artifacts) and !915 are historical; not part of the
directed pass — flag only.
