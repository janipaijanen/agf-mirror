<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# AGF development starter

This is a concise, non-normative projection of the operating guidance in
[`AGENTS.md`](../../AGENTS.md). Use it to start an AGF, `agov`, or `agov-mcp`
development session quickly. If this guide and a governed owner disagree, the
owner, applicable AGF rules, approved plan, and durable coordination records
control.

Complete polling and autonomous continuation are a standing obligation of
**every active development session**, not a response to being asked to "poll".
The normative contract is
`agf/rules/always/universal-polling-and-autonomous-continuation-rule.md`; this
guide projects it.

## Start the session

1. Read `AGENTS.md` and any repository-root `ai-cold-start.md` overlay.
2. Run `agov session resume --base-branch main` without a role, then rerun it
   with the role you are actually playing.
3. When developing `agf`, `agov`, or `agov-mcp`, build the local Rust release
   binary first and use the repository-local `tooling/agov` wrapper throughout
   the session.
4. Set an assistant-specific token-ROI log under `tmp/` when collecting local
   telemetry; do not share one mutable log between assistants.

## Run one complete poll before choosing work

1. Fetch and prune `origin`.
2. Use `agov coord list --since <cursor>` and `agov coord visibility
   --base-branch main`. The checked-out coordination directory alone is not a
   complete remote inbox.
3. Inspect every open GitLab merge request and issue for new notes,
   assignments, labels, state, exact head and target SHAs, mergeability, and
   canonical pipeline status.
4. Recheck every release, tag, branch, pipeline, runner, or job on which the
   active role is waiting.
5. Keep UTC cursors and exact identifiers so an unchanged-state report is
   evidence-backed.

Polling is an action loop. Classify new obligations addressed to the active
role or `all`, select the highest-priority authorized one, and act until it is
complete, actively running and monitored, or blocked with a concrete owner and
reason. Scheduling is non-reentrant — a wake due during active work coalesces
into one pending poll that runs at the next safe boundary; it never preempts
or overlaps.

## Select work by verified impact

Prefer work that advances the next governed release and removes verified bugs,
weaknesses, or vulnerabilities in AGF, `agov`, or `agov-mcp`. Next, implement
release-scoped roadmap features whose Gate 1 plan is approved.

Security impact, adopter breakage, release integrity, and blocked acceptance
evidence outweigh cosmetic progress. Do not invent severity, close an issue on
assertion alone, or broaden an approved plan. Required reviews, coordination
publication, incident response, and work that removes a release blocker remain
part of delivery; they are not distractions merely because they are not
implementation.

## Redirect another agent only on evidence

Before redirecting another agent, verify its current claim, governing plan,
dependencies, and relationship to the release. Notify it only when its active
work materially competes with an authorized release-critical path or conflicts
with a binding priority decision.

Use one concise durable coordination record. Name the evidence, requested
disposition, and authority boundary. Do not spam stale MRs or interrupt work
that is necessary to make the release safe and reviewable.

## Act within the current gate

Within existing authority, acknowledge or claim visible work, diagnose a
failure before rerunning paid CI, batch corrections, publish required
coordination, and route the exact reviewed head through the applicable gate.

Stop for binding authority when required for Gate 1, Gate 2, merge, tag,
release publication, destructive action, secrets ambiguity, or scope
expansion. Release-first prioritization never grants those powers.

## End with a durable disposition

Record what changed after the prior cursor, what was acted on, the exact
MR/issue/SHA/pipeline evidence, what remains blocked and by whom, and the next
authorized release-critical action. Stay quiet when state is unchanged and
non-actionable unless a periodic report was explicitly requested.

## Tabletop checks

Use these cases when reviewing changes to this guide:

| Situation | Correct disposition |
|---|---|
| A release-integrity defect and an unrelated cosmetic documentation edit are both available. | Take the verified release-integrity defect first, within the current gate. |
| A required review or coordination-only publication is active while unrelated implementation is available. | Do not interrupt the required evidence work merely because it is not implementation. |
| A release branch is ready, but no binding merge, tag, or publication decision exists. | Prepare and report exact evidence, then stop for the authorized decision; release priority supplies no authority. |

