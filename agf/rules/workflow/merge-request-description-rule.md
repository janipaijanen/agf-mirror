---
id: merge-request-description-rule
scope: workflow
applies_to: [llm-agent, human-agent]
evidence: local-dogfood-2026-06-16
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
prepare a raw merge request description in `MR_DESCRIPTION.md` at the repository
root.

The file is a local scratch artifact by default. It SHOULD remain untracked
unless the project explicitly asks to version it.

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

Agents SHOULD write or refresh `MR_DESCRIPTION.md` immediately after the branch
is committed and before presenting the MR URL.

If the human asks for "MR docs", "MR description", or equivalent, the agent
MUST update `MR_DESCRIPTION.md` for the current branch rather than replying only
with prose.

## Why

Merge request descriptions are part of the review surface. Requiring the
assistant to prepare them:

- reduces repeated human prompting;
- keeps the branch's verification evidence close to the diff;
- makes Gate 2 review faster and more consistent;
- prevents important caveats from being stranded in chat history.
