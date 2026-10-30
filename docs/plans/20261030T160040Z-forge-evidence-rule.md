---
id: 20261030T160040Z-forge-evidence-rule
status: pending-gate1
topic: forge-publication-evidence-boundary-rule
depends_on:
  - 20260913T085123Z-work-item-steward-gate1
  - 20260913T093405Z-row5-s3-priority-ordering-and-challenge-response
  - 20260914T130217Z-row6-s4-issue123-collision-check-blocking-promotion
  - 20260915T075705Z-issue130-pending-advisory-consultation-status
  - 20260921T040743Z-assistant-rule-drift-resolution
  - 20261021T220000Z-retire-windows-support
  - 20261022T205113Z-instruction-context-reduction
  - 20261023T045214Z-remove-elixir-references
  - 20261023T080431Z-instruction-context-followup
  - 20261023T195516Z-recall-smoke-criterion-and-weak-paraphrases
  - 20261024T070435Z-timer-reality-polling-alignment
  - 20261025T183939Z-v211-changelog-and-scope-reconciliation
  - 20261025T192020Z-supersedes-adopter-docs
  - 20261027T045944Z-record-only-publication-termination
  - 20261027T230355Z-v212-release-freeze-corrections
  - 20261027T235807Z-v212-release-publication
  - 20261029T204751Z-v213-release-freeze-preparation
  - 20261029T220500Z-v213-release-publication
  - 20261030T064744Z-v2131-release-publication
  - 20261030T144439Z-header-repair-plan
affected_paths:
  - agf/rules/workflow/forge-publication-evidence-rule.md
  - agf/rules/workflow/merge-request-description-rule.md
  - agf/rules/README.md
  - agf/templates/AGENTS.md
  - agf/docs/site/docs/operate/day-to-day.md
  - agf/CHANGELOG.md
  - docs/reviews/rule-projection/20261030T160040Z-forge-evidence-rule.yaml
  - .claude/rules/workflow/forge-publication-evidence-rule.md
  - .claude/rules/INDEX.md
  - AGENTS.md
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >-
      Independent review of exact 66e5ab2e returned BLOCK findings, recorded in
      combined review `20261030T160913Z-contribution-auditor-gdjm` (F2 privacy
      boundary, F1 scope). This revision answers F1 to F5. It is NOT cleared:
      the revised exact head still needs independent re-review.
  - role: contribution-auditor
    status: consulted
    reason: >-
      Independent review of exact 66e5ab2e returned BLOCK findings, recorded in
      combined review `20261030T160913Z-contribution-auditor-gdjm` (F1 to F4
      blocking, F5 advisory). This revision answers F1 to F5. It is NOT cleared:
      the revised exact head still needs independent re-review.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: forge publications cite durable evidence, not chat as authority

Governance mode: STRICT (a new rule under `agf/rules/`, a downstream-impact
trigger path, and AI-loader routing). Planning only. This plan edits no shipped
rule, page, loader or changelog and adds no CI check, history rewrite or bulk
note deletion.

Source: governor record `20261030T155116Z-governor-reviewer-ghgs`; combined review
`20261030T160913Z-contribution-auditor-gdjm` (F1 to F5). Claims:
`20261030T160040Z-planner-architect-gr51` (released by `7nhr`) and
`20261030T162930Z-planner-architect-mjbh`.

## Two layers, kept separate

1. **Foundry direction (unchanged).** Under `ghgs`, in this project GitLab merge
   requests and issues must not refer to the project's live chat surface at all.
   This plan does not weaken, rewrite or generalize that instruction. It stays in
   force through its own record and the foundry loaders.
2. **Generic AGF rule (this plan).** A reusable rule for adopters, narrower than
   the foundry direction, aimed at the governance invariant: live conversation
   must not be the evidence of authority.

## Intent

Make the invariant permanent and shippable: a merge request or issue never
relies on a live-chat message number, link, quote or paraphrase as evidence of
approval, scope, status, verification or other authority. The durable
coordination record, plan, commit, pipeline or job, or other forge artifact is
the evidence. Chat provenance stays in the coordination audit trail.

## Findings answered

| Finding | Answer in this revision |
|---|---|
| F1 over-broad all-chat ban | The generic clause targets chat used as governance evidence only. It expressly allows chat or relay systems as the subject of an issue or merge request, and the minimum sanitized reproducer needed to review that work. The foundry-specific stricter rule is kept separate (above). |
| F2 privacy | The clause requires minimal, sanitized durable evidence and forbids copying secrets, private identifiers, private links or raw chat into forge text or coordination records. It states that moving sensitive text into an immutable record does not make it safe. It points to the existing secret and anonymization rules. |
| F3 owner scope | A new canonical owner covers merge request and issue titles, bodies and notes. The MR-description rule keeps its own scope and gains one pointer. |
| F4 projections, migration, SemVer | Affected paths now include the adopter operating page, shipped loader routing, rules index, a changelog migration entry and the structured projection manifest. SemVer constraint stated below (decision D1). |
| F5 semantic examples | Positive and negative examples are part of the rule text and acceptance criteria. Validation stays manual and advisory. |

## Canonical owner (decision: one new file)

`agf/rules/workflow/forge-publication-evidence-rule.md` (new) is the single owner
of the invariant for merge request and issue titles, bodies and notes.
`merge-request-description-rule.md` keeps its MR-only scope and adds one pointer
sentence in "Content expectations" plus a `see_also` entry. The alternative,
broadening the MR-description rule to cover issues, was rejected: it would
change that rule's declared scope, heading and discovery path for a different
audience.

## Draft rule text (for Gate 1 review; no product or tool name)

> **Forge publications cite durable evidence.** A merge request or issue title,
> description or note MUST NOT rely on a live-chat message as evidence of
> approval, scope, status, verification or authority. That includes a message
> number, link, quote or paraphrase of the conversation. Cite the durable
> artifact instead: a coordination record ID or path, a plan file, a commit SHA,
> a pipeline or job ID, or another Git or forge item. Chat provenance belongs in
> the coordination audit trail.
>
> **Allowed.** Discuss a chat, messaging or relay system when it is the subject
> of the work, for example an issue about an integration defect or a merge
> request that changes a relay adapter. Include the minimum sanitized
> reproducer needed to review that work.
>
> **Privacy.** Record only the minimum decision or evidence summary needed for
> audit. Do not copy secrets, credentials, private identifiers, customer
> details, private links or raw chat text into a forge publication or into a
> coordination record. Moving sensitive text into an immutable record does not
> make it safe. Apply `security-hardening-rule.md` and
> `external-anonymization-rule.md`.
>
> **Stricter project direction.** A project MAY forbid any mention of its chat
> surface in forge text. This rule does not weaken such a direction.
>
> **Limits.** This rule adds no automated check, no rewrite of history and no
> deletion of existing text. Existing coordination records stay immutable.
> Whether text relies on chat as evidence is a human judgment.

Front matter of the new file follows the sibling rules (id, scope: workflow,
strength, status, `see_also`), marked provisional with evidence status
**Theoretical** (one project, one governor direction, 2026-10-30).

## Semantic examples (review evidence, not a validator)

| Case | Result |
|---|---|
| MR description says "approved in chat, message 412" | Forbidden: chat as approval evidence. Cite the decision record ID. |
| Issue says "scope agreed in the team thread" | Forbidden: chat as scope evidence. Cite the plan and its Gate 1 record. |
| MR note quotes a private link and a token copied from a chat | Forbidden twice: chat evidence, and a secret or private link. |
| Issue reports that a webhook relay drops events, with a sanitized payload that reproduces it | Allowed: the relay is the subject; the payload is minimal and sanitized. |
| MR changes a chat-integration adapter and describes its behavior | Allowed: chat is the artifact under review. |
| MR cites `20261030T151222Z-governor-reviewer-ph8p` and commit `4076884d` | Allowed and preferred. |

## Projection assessment (six surfaces, structured manifest required)

The change adds normative guidance, so the projection rule applies. The
canonical manifest is `docs/reviews/rule-projection/20261030T160040Z-forge-evidence-rule.yaml`, with the owner
marked `stage: planned` until the rule exists.

| Surface | Disposition |
|---|---|
| Normative owner | Updated (new): `forge-publication-evidence-rule.md`, plus a pointer in `merge-request-description-rule.md`. |
| Adopter-facing explanation | Updated: a concise pointer in `agf/docs/site/docs/operate/day-to-day.md`, section "Show the assistant and role on GitLab", which already tells authors to name a durable coordination record. |
| Runbook/situation map | Existing pointer to day-to-day sufficient; the manifest records why. |
| AI landing/loaders | Updated: `agf/templates/AGENTS.md` gains one route covering merge request and issue publication; `agf/rules/README.md` lists the rule. Foundry-local routes: `.claude/rules/workflow/` symlink, `.claude/rules/INDEX.md` row, and the `AGENTS.md` list line. |
| Validation/drift evidence | Not mechanically decidable; human review with the examples above. |
| Distribution | The owner, README, template and page already ship through `agf/` coverage; foundry-local loaders are project-local. |

## Downstream impact and SemVer (decision D1)

A new `MUST NOT` narrows a previously implicit permission, which
`downstream-impact-rule.md` classifies as BREAKING. The framework's version
policy reserves breaking changes for a MAJOR release.

- **D1 option A (draft default): keep `MUST NOT`.** BREAKING. The change is
  **ineligible for the v2.13.x maintenance line or any v2.x minor release** and
  may ship only through a separately approved major-release plan. This chooses
  no version number, milestone or date.
- **D1 option B: weaken to `SHOULD NOT` for adopters.** The generic rule becomes
  a recommendation (ADDITIVE, eligible for a minor release). Foundry keeps its
  stricter local `MUST NOT` through `ghgs`.

The plan recommends A because it keeps the direction's strength, and states B so
the governor can choose the faster path. The implementation, if approved, uses
the chosen option consistently in the rule, the page, the changelog entry and
the migration note.

```
Impact:
  Foundry:          Already applied by governor direction; the generic rule adds the
                    adopter-facing invariant.
  Direct adopters:  A: BREAKING (MAJOR only). B: ADDITIVE recommendation.
  Downstream:       Forge text shape changes slightly; no artifact shape.
  Migration:        Author behavior only; an adopter-visible entry goes under
                    Unreleased in agf/CHANGELOG.md. No data, tooling, schema or
                    history change.
  Evidence:         ghgs, gdjm; Theoretical, single project.
```

Adopter docs parity: updated (day-to-day page, shipped loader, rules index,
changelog migration entry).

## Acceptance criteria (after Gate 1, one implementation)

1. Only the paths in `affected_paths` change; no other file.
2. The new rule contains the clause, the allowed case, the privacy paragraph,
   the stricter-direction sentence, the limits and the examples, and names no
   product or tool.
3. `agov plan projection-check --base-ref origin/main <manifest>` passes without
   `--allow-planned` at the implementation head (the owner then exists).
4. The changelog entry states the D1 outcome and the migration note.
5. `agov validate adoption`, `adopter-docs`, `infosec` pass; `git diff --check`
   is clean; exact-head pipeline green.
6. Independent exact-head review and a separate binding Gate 2 precede merge.

## Collision and dependency evidence

`agov plan collision-check --plans-only --changed-only`, run on the committed
plan, reported 20 blocking overlaps with earlier plans. All 20 are
`approved`. The shared files are the rules index, the shipped assistant template,
the project `AGENTS.md`, the adopter changelog, the day-to-day page, the
merge-request description rule (through the broad glob `agf/rules/**`), and the
rule index. None of these plans has an open claim or open merge request on these
paths: the only open merge requests at this head are coordination records and
this plan. Their earlier edits changed other entries and prose in the shared
files; this plan adds a new rule, one pointer, one route, one page paragraph and
one changelog entry, and rewrites none of that content. All 20 are added to
`depends_on` as sequencing acknowledgements. This does not approve this plan or
any predecessor.

Before implementation the owner refetches refs, confirms no active claim on
these paths, rebases onto any merged change, and rechecks collisions for every
newly affected path. The adopter changelog is shared with release publication
plans, so the implementation either lands before the next freeze or is
re-checked against it.

## Risks and rollback

- **Over-broad reading:** controlled by the allowed case and examples.
- **Privacy regression:** controlled by the explicit privacy paragraph.
- **Release timing:** option A cannot ride a minor release; D1 makes that explicit.
- **Rollback:** revert the single implementation merge request.

## Out of scope

New CI checks, history rewrite, bulk note deletion, a general chat retention
policy, role changes, release version, date or milestone, project-local merge
request and issue template reminders, and any shipped edit before its own Gate 1
and Gate 2.

## Gate 1 request

Approve this plan with a decision on D1. Both advisory rows record the earlier
BLOCK findings and are not cleared; independent re-review of the revised exact
head is required before the Gate 1 decision.
