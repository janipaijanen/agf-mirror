---
id: 20261030T160040Z-forge-evidence-rule
status: pending-gate1
topic: forge-publication-evidence-boundary-rule
affected_paths:
  - agf/rules/workflow/merge-request-description-rule.md
advisory_consultation:
  - role: infosec-sentinel
    status: pending
    reason: >-
      Consultation requested with this plan, as required by governor record
      20261030T155116Z-governor-reviewer-ghgs. No clearance is claimed.
  - role: contribution-auditor
    status: pending
    reason: >-
      Consultation requested with this plan, as required by the same record,
      in particular on the rule wording, the BREAKING classification and the
      projection assessment. No clearance is claimed.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: forge publications cite durable evidence, not chat

Governance mode: STRICT (a rule change under `agf/rules/`, a downstream-impact
trigger path). Planning only. This plan edits no shipped rule and adds no CI
check, history rewrite or bulk note deletion.

Source: governor record `20261030T155116Z-governor-reviewer-ghgs`, which records
the human direction that GitLab merge requests and issues must not refer to the
live chat surface. Claim: `20261030T160040Z-planner-architect-gr51`.

## Intent

Make the direction permanent in AGF. Merge request titles, descriptions and
notes, and issue text, cite durable official evidence: coordination record IDs
or paths, plan files, commit SHAs, pipeline and job IDs, and other Git or forge
items. Chat provenance stays in the coordination audit trail, not in forge
prose.

## Canonical owner (located)

`agf/rules/workflow/merge-request-description-rule.md` is the single owner. It
already governs what a merge request description MUST name (branch, artifacts,
coordination handoffs and decisions, validation, residual risks) and MUST NOT
contain (secrets, tokens, credentials, local paths). The new clause is the same
kind of statement, so it belongs there and not in a new file.

Other candidates and why they are not the owner:

- `merge-request-workflow-rule.md` governs that changes go through merge
  requests, not what their text contains.
- `coordination-publication-rule.md` governs publication of coordination
  records, not forge prose.
- `.gitlab/merge_request_templates/*.md` and `.gitlab/issue_templates/*.md` are
  foundry-local, not shipped (they are absent from `agf/dist/release-manifest.yaml`).

## Proposed change (after Gate 1, as a separate implementation)

Add to the owner rule, after the "Content expectations" list, one short
subsection. Draft text, with no product or tool name so that adopters can apply
it to whatever chat surface they use:

> ### Cite durable evidence, not chat
>
> A merge request title, description or note, and issue text, MUST NOT refer to
> a chat, messaging or relay surface used for live discussion between agents or
> people: no message numbers, quotes, links or paraphrased provenance. Cite
> official evidence instead: coordination record IDs or paths, plan files,
> commit SHAs, pipeline and job IDs, and other Git or forge-hosted items. Chat
> provenance belongs in the coordination audit trail. This clause adds no
> automated check, requires no rewrite of history and requires no deletion of
> existing text. Existing coordination records stay immutable.

Update the rule's front-matter `evidence` field and add the usual provisional
status note (single-project evidence: one governor direction, 2026-10-30).
Evidence status: **Theoretical** (a governance direction, not yet validated by
use).

## Projection assessment (six surfaces)

| Surface | Disposition |
|---|---|
| Normative owner | Updated: `merge-request-description-rule.md`. |
| Adopter-facing explanation | Not applicable. No Pages page describes merge request description content (searched `agf/docs/site/docs`, `agf/ADOPTING.md`). |
| Runbook/situation map | Not applicable. No operating step changes. |
| AI landing/loaders | Existing pointer sufficient. `agf/templates/AGENTS.md` points assistants to the whole `agf/rules/workflow/` directory; the foundry `AGENTS.md` and `.claude/rules/INDEX.md` already route "prepare an MR or write its description" to this rule. No loader edit. |
| Validation/drift evidence | Not mechanically decidable. Whether prose "refers to chat" is a human judgment; no validator is added or authorized. |
| Distribution | The owner already ships through existing `agf/rules/` coverage. |

The projection rule requires a manifest when normative guidance in a Pages page
or runbook changes. None changes here, so no manifest is proposed. This is a
reasoned result, not an omitted row. If the governor wants the foundry-local
merge request and issue templates to carry a one-line reminder, that is a
separate project-local change outside this plan.

## Acceptance criteria

1. Only `agf/rules/workflow/merge-request-description-rule.md` changes.
2. The added subsection uses no product or tool name and names only generic
   chat surfaces.
3. The rule states it adds no automated check, no history rewrite and no
   deletion.
4. `git diff --check`, `agov validate adoption`, `adopter-docs` and `infosec`
   pass; the exact-head pipeline is green.
5. Independent exact-head review and a separate binding Gate 2 precede merge.

## Downstream impact (BREAKING by the Parnas test)

The clause adds a new prohibition. Under `downstream-impact-rule.md`, narrowing
previously permitted behavior is BREAKING. Adopters whose agents cite chat
messages in merge request text must stop doing so.

```
Impact:
  Foundry:          Already applied by governor direction; this makes it a rule.
  Direct adopters:  BREAKING (new MUST NOT). Authors cite coordination records,
                    plans, commits and pipeline IDs instead of chat.
  Downstream:       Merge request text shape changes slightly; no artifact shape.
  Migration:        Author behavior only. No data, tooling, schema or history
                    change. Existing text is left as is.
  Evidence:         20261030T155116Z-governor-reviewer-ghgs (governor direction;
                    single project).
```

A migration note in the next release changelog is required when this ships.
Adopter docs parity: not applicable beyond the rule itself (see projection
assessment).

## Collision and dependency evidence

Run on the committed plan; the result and any `depends_on` are recorded at the
head returned for Gate 1. Before implementation the owner refetches refs,
confirms no active claim on the rule file and rebases onto any merged change.

## Risks and rollback

- **Over-broad reading** that bans citing coordination records: control, the
  clause names coordination records as the preferred evidence.
- **Chat surface named by accident:** control, acceptance criterion 2.
- **Retroactive pressure to edit history:** control, the clause says no rewrite
  or deletion; coordination records stay immutable.
- **Rollback:** revert the single implementation merge request.

## Out of scope

New CI checks, retroactive history rewrite, bulk note deletion, a general chat
retention policy, role changes, release commitments, project-local template
reminders, and any shipped rule edit before its own Gate 1 and Gate 2.

## Gate 1 request

Approve this plan for one rule clause in one file, with the BREAKING
classification and the migration note. Both advisory consultations are pending.
