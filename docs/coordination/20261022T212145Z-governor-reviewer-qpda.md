---
kind: decision-record
from: governor-reviewer
to: all
id: 20261022T212145Z-governor-reviewer-qpda
timestamp: 2026-10-22T21:21:45Z
model: human-governor
immutable: true
references-to:
  - 20261022T211716Z-governor-reviewer-k1ng
topic: narrow-temporary-joint-governor-delegation
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Delegation Artifact: temporary joint governor-reviewer authority

This record narrows and completes `k1ng` under
`agf/rules/workflow/deputy-governor-rule.md`. It supersedes every broader or
conflicting statement in `k1ng`; it does not widen that record. The human
governor confirmed the delegation directly to both deputies and instructed
them to add the red list.

## Required fields

- **Delegator:** Jani Päijänen, Human Governor, 2026-10-22.
- **Deputies:** `codex-foundry@2026-10-22T21:17Z` and
  `claude-planner@2026-10-22T21:17Z`. They act jointly; both must expressly
  agree on the same bounded decision. Neither may act alone.
- **Expiry:** `2026-10-23T08:00:00Z` (2026-10-23 10:00 EET, UTC+02:00).
- **Severity ceiling:** WARN. Any finding above WARN refuses delegation and
  returns the item to the Human Governor.
- **Audit trigger:** five delegated decisions in total. After decision five,
  both deputies stop until the Human Governor completes the required audit
  and issues a fresh Delegation Artifact.
- **Revocation:** automatic on the Human Governor pulling any item, any agent
  challenging a delegated decision as wrong, expiry, or explicit Human
  Governor revocation. A challenge stops use immediately; it need not first
  be upheld.

## Delegable categories

Only the following categories are delegated, and only when no red-list item
is present:

1. Gate 1 decisions for bounded, reversible INFO/WARN work that changes no
   security boundary, architecture, data model, dependency, integration,
   framework rule, agent definition, assistant loader, gate mechanics,
   release mechanics, schema, legal artifact, or other red-list surface.
2. Gate 2 decisions and merge authorization for exact-head changes under an
   already approved plan when the implementation remains within that same
   boundary, all required checks pass, and every finding is INFO or WARN.
3. Coordination-only publication and closure records that report an already
   verified decision or repository state and do not change coordination
   integrity mechanics, authority semantics, schemas, or publication holds.
4. Non-normative documentation and tracker metadata outside every red-list
   surface, with no security, legal, compliance, release, architectural, or
   authority effect.

Near-boundary or ambiguous work is out of scope and returns to the Human
Governor. Each delegated decision is a separate immutable record that cites
this artifact, identifies the change, summarizes both deputies' reasoning,
lists every finding and severity, and states the running decision count.

## Red list: never delegated

The deputies must refuse and return all of these to the Human Governor even
when both deputies agree:

1. Irreversible or destructive operations governed by
   `destructive-op-preflight-rule.md`, including history rewrite, force push,
   deletion, destructive database work, or tag movement.
2. Security-boundary changes: authentication, authorization, secrets,
   permissions, cryptography, tenant isolation, runner trust boundaries, or
   credential scope.
3. Architecture or data-model changes, new external dependencies, or new
   integrations.
4. Any critical-severity item.
5. Delegation scope: widening, amendment, extension beyond the stated expiry,
   new delegations, or red-list changes.
6. Rule and agent definitions, including `agf/rules/**`,
   `agf/agents/*.yaml`, `agf/agents/MANIFEST.yaml`, `agf/framework.md`,
   `AGENTS.md`, and tool-specific assistant loaders such as `.claude/**`.
7. Compliance, licensing, or legal artifacts, including SPDX, LICENSE,
   consent, and risk-acceptance records.
8. Areas governed by `high-risk-operations-rule.md`: coordination integrity
   mechanics; gate and role-authority validation logic; release, tag,
   publication, or deployment mechanics and any actual release or deploy;
   breaking plan/message schema changes; and publication-hold enforcement.
9. Governor-reviewer role status or `played_by` changes.

Consequently, the instruction-context work in issue #138 and MR !3114 is
red-listed because it changes `AGENTS.md` and `.claude/**`. Codex and Claude
may research, review, test, and prepare its plan, but its Gate decisions must
come directly from the Human Governor.
