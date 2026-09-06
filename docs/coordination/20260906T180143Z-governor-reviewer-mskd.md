---
kind: decision-record
from: governor-reviewer
to: all
id: 20260906T180143Z-governor-reviewer-mskd
timestamp: 2026-09-06T18:01:43Z
model: human
immutable: true
references-to:
  - 20260906T071102Z-contribution-auditor-p6j5
  - 20260906T173031Z-contribution-auditor-6pss
topic: layered autonomous poll guidance for all AGF development AIs
requires-ack: true
artifacts_owned:
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/2405
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Adopt the layered autonomous poll model

The governor-reviewer directs every current AGF development AI to use the
following layered poll/action model immediately wherever it does not conflict
with a more specific governed rule. Planner-architect owns converting this
direction into the corrected Gate 1 plan on MR !2405 so the model is projected
to every supported startup surface for future AGF development AIs.

This decision establishes the operating objective and the planning obligation.
It does not approve implementation changes to rules, loaders, specialist files,
runbooks, schemas, or tooling before the corrected plan passes Gate 1.

## Layer 1 — complete discovery cycle first

Run, in order:

1. `git fetch --prune origin`;
2. `agov coord list --since <cursor>`;
3. `agov coord visibility --base-branch main`;
4. `agov handoff next <role>`;
5. `agov coord pending`;
6. scan every open merge request, including new discussions, approvals, draft
   state, source and target SHA, mergeability, and exact-head pipeline/jobs;
7. scan issues for notes, assignments, labels, and state changes;
8. recheck every exact external state the active role started or awaits.

Priorities cannot rank work that discovery has not surfaced. Polling is
non-reentrant: if authorized work is active when a trigger fires, coalesce the
trigger and run one complete cycle at the next safe completion, handoff, gate,
or blocker boundary. Advance the cursor only after discovered obligations are
classified and the selected action is completed, running and monitored, or
concretely blocked.

## Layer 2 — select the first matching action

Use this order:

0. A screened security, vulnerability, data-integrity, or tenant-isolation
   issue with a credible reporter and substantiated claim preempts release
   work. Sensitive detail follows InfoSec routing rather than the open poll
   flow. Authority and gate requirements still apply.
1. Act on the highest-priority obligation addressed to the active role or
   `all` that is inside the role, approved scope, and current gate.
2. Advance an item genuinely ready at its gate: required pipeline green,
   findings resolved, and exact head confirmed. Surfacing or unblocking a
   not-ready item is advancement; forcing it past its gate is not.
3. Advance next-release readiness by disposing open merge requests and issues
   against the tag boundary with a recorded cause.
4. Improve existing-feature quality or reliability as one evidence-bearing
   unit with its tests, documentation, changelog, roadmap state, and release
   evidence.
5. Advance approved features or bounded experiments already through Gate 1.

The action loop does not enlarge authority. Within the selected item, agents
autonomously perform already-authorized in-role work and stop only at a genuine
gate, concrete blocker, or authority boundary.

## Layer 3 — bounded idle work

When no authorized in-role work is actionable, do not manufacture work, invent
release scope, or generate low-signal coordination. Perform bounded read-only
readiness analysis, decompose known backlog items where authorized, and
propose—not implement—new scope through the issue board and coordination
channel.

## Invariants

AI agents do not grant Gate 1 or Gate 2 approval. Do not publish a release or
tag, expand scope, create role authority, or perform destructive operations
without explicit authorization. Do not implement without an approved plan
except through an applicable governed exception, such as the narrow
sources/radar research-maintenance path. Every action creating a downstream
obligation gets a validated and visible coordination record. Stop and request
the binding decision at each authority boundary.

## Mandatory plan and projection work

Planner-architect MUST revise MR !2405 rather than create a competing plan. The
revision must resolve contribution-auditor finding `p6j5`, including the
collision between mandatory pre-mutation claims and the simplicity rule, and
must select one canonical normative owner for the polling model.

The plan and eventual Gate 2 review must provide the structured six-row matrix
required by `agf/rules/workflow/documentation-rule-projection-rule.md`:

1. normative owner;
2. adopter-facing explanation;
3. runbook/situation map;
4. AI landing/loaders, including `AGENTS.md`, `agf/templates/AGENTS.md`,
   `.claude/CLAUDE.md`, and every supported assistant specialist/startup file;
5. validation and drift evidence;
6. distribution in the relevant AGF or tooling artifacts.

The corrected exact head requires contribution-auditor and InfoSec review
before governor Gate 1. After approval, implementation must make the canonical
owner discoverable to every current and future AGF development AI without
copying divergent normative text across projections.
