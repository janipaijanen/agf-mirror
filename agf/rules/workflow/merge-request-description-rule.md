---
id: merge-request-description-rule
scope: workflow
applies_to: [llm-agent, human-agent]
evidence: local-dogfood-2026-06-16, 20260720T164212Z-governor-reviewer-kser
upstream_candidate: true
strength: MUST
status: active
see_also: [merge-request-workflow-rule, audit-log-completeness-rule]
provenance: foundry-native
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Merge Request Description Preparedness

**Scope:** Any agent preparing a branch for merge request review.

## Rule

Before asking a human to open, review, or merge a merge request, the agent MUST
prepare a raw merge request description in `tmp/MR_DESCRIPTION.md` (or another
already-gitignored scratch path the project designates).

The file is a local scratch artifact, not a durable repository record. It
MUST live under a gitignored path — a project-root `MR_DESCRIPTION.md` that
is not gitignored WILL be committed by ordinary `git add`/`git commit`
habits, which turns branch-local drafting into a tracked file every branch
edits and every rebase conflicts on. This is not hypothetical: a project
that tracked the file at the repository root hit exactly this failure
repeatedly (merge conflicts on every branch rebase, and the file drifting
out of sync with the actual, already-merged MR body).

## The GitLab (or equivalent) MR description is authoritative, not the file

`tmp/MR_DESCRIPTION.md` is staging text for composing the MR body — nothing more.
It MUST NOT be read back by any agent or reviewer as evidence of the MR's
current state. The only authoritative surface is the hosting platform's own
MR description, fetched live (e.g. `glab mr view <n>`).

Agents MUST sync the staging file to the platform immediately after every
edit — in the same action, not "later":

```bash
glab mr create --description "$(cat tmp/MR_DESCRIPTION.md)" ...   # first time
glab mr update <n> --description "$(cat tmp/MR_DESCRIPTION.md)"   # every edit after
```

An edit to the staging file that is not immediately synced is not evidence of
anything — it is indistinguishable from a stale draft.

Automating this sync (e.g. a pre-push hook) is intentionally deferred: an
implicit auto-syncing actor can fail quietly on missing auth, network issues,
or first-push timing, which risks masking exactly the drift this rule exists
to prevent. If tooling is added later, prefer an explicit command an agent
calls deliberately (and records having called) over an implicit hook.

## Required sections

The description MUST include:

- `# Summary`
- `# Scope`
- `# Governance Trail`
- `# Verification`
- `# Review Notes`
- `# Out of Scope`

If the branch is an audit, release, or governance-gate branch, the description
SHOULD also include the relevant audit or gate result.

## Content expectations

The description MUST name:

- the source branch;
- the primary changed artifacts;
- coordination handoffs or decisions relevant to the MR;
- validation commands already run and their results;
- intentional residual risks, warnings, or deferred work.

The description MUST NOT contain secrets, tokens, private credentials, or local
machine-specific paths unless those paths are themselves the artifact under
review.

## Timing

Agents SHOULD write or refresh `tmp/MR_DESCRIPTION.md` immediately after the branch
is committed and before presenting the MR URL.

If the human asks for "MR docs", "MR description", or equivalent, the agent
MUST update `tmp/MR_DESCRIPTION.md` for the current branch rather than replying only
with prose.

## Why

Merge request descriptions are part of the review surface. Requiring the
assistant to prepare them:

- reduces repeated human prompting;
- keeps the branch's verification evidence close to the diff;
- makes Gate 2 review faster and more consistent;
- prevents important caveats from being stranded in chat history.
