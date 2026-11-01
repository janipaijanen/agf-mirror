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
  - 20261030T163253Z-pages-structure-and-voice
  - 20261030T210529Z-pages-source-consistency-g1-g5
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
      Two independent reviews returned BLOCK findings. Round 1, exact 66e5ab2e:
      `20261030T160913Z-contribution-auditor-gdjm` (F2 privacy boundary, F1
      scope). Round 2, exact 5e2f8221: `20261030T165922Z-contribution-auditor-q48r`
      (F2 option-B privacy wording and destination boundary). This revision
      answers both rounds. It is NOT cleared: the revised exact head still needs
      independent re-review.
  - role: contribution-auditor
    status: consulted
    reason: >-
      Two independent reviews returned BLOCK findings. Round 1, exact 66e5ab2e:
      `20261030T160913Z-contribution-auditor-gdjm` (F1 to F4 blocking, F5
      advisory). Round 2, exact 5e2f8221: `20261030T165922Z-contribution-auditor-q48r`
      (F1 strength propagation, F2 privacy wording, F3 major-release sequencing;
      3 blocking). This revision answers both rounds. It is NOT cleared: the
      revised exact head still needs independent re-review.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: forge publications cite durable evidence, not chat as authority

Governance mode: STRICT (a new rule under `agf/rules/`, a downstream-impact
trigger path, and AI-loader routing). Planning only. This plan edits no shipped
rule, page, loader or changelog and adds no CI check, history rewrite or bulk
note deletion.

Source: governor record `20261030T155116Z-governor-reviewer-ghgs`; combined review
`20261030T160913Z-contribution-auditor-gdjm` (F1 to F5); revision review
`20261030T165922Z-contribution-auditor-q48r` (F1 to F3, round 2). Claims:
`20261030T160040Z-planner-architect-gr51` (released by `7nhr`),
`20261030T162930Z-planner-architect-mjbh` and the round-2 claim named in the
revision handoff.

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
| F2 privacy | The rule keeps the inherited secret and anonymization exclusions at their own strength, adds advisory minimization and a destination paragraph (revised in round 2 below), and states that moving sensitive text into an immutable record does not make it safe. It points to the existing secret and anonymization rules. |
| F3 owner scope | A new canonical owner covers merge request and issue titles, bodies and notes. The MR-description rule keeps its own scope and gains one pointer. |
| F4 projections, migration, SemVer | Affected paths now include the adopter operating page, shipped loader routing, rules index, a changelog migration entry and the structured projection manifest. SemVer constraint stated below (decision D1). |
| F5 semantic examples | Positive and negative examples are part of the rule text and acceptance criteria. Validation stays manual and advisory. |

### Round 2: answers to `q48r` (not cleared)

| Finding | Answer in this revision |
|---|---|
| F1 strength propagation | The projection section now lists every surface that expresses the behavior and requires the D1 strength on all of them: owner front matter and text, manifest statements, page, template route, rules index and changelog. Pointers that only name the owner stay neutral. The foundry `AGENTS.md` route keeps the stricter `ghgs` direction in its own words and does not route to the weaker option. |
| F2 privacy wording | The privacy paragraph is split into inherited mandatory exclusions and advisory minimization. It names the destination boundary and removes the absolute ban on private references in access-controlled coordination records. Under both options the new privacy guidance is advisory. |
| F3 major-release sequencing | The plan records no D1 choice. If the governor selects option A, the implementation stays unmerged until a separate, approved major-release sequence exists. If the governor selects option B, the additive test below must hold across the whole projected change. |

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
> description or note MUST NOT [option B: SHOULD NOT] rely on a live-chat
> message as evidence of
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
> **Privacy: inherited exclusions.** This rule adds no new privacy
> prohibition. Secrets and credentials never belong in a forge publication or a
> coordination record. Identifying or sensitive project or customer data stays
> out of any text that crosses the codebase boundary. Those exclusions come from
> `security-hardening-rule.md` and `external-anonymization-rule.md` and keep
> their own strength. Moving sensitive text into an immutable record does not
> make it safe.
>
> **Privacy: minimization (advisory).** Authors SHOULD record only the minimum
> decision or evidence summary needed for audit and SHOULD avoid copying raw
> conversation text that the audit does not need.
>
> **Destination.** Public or external forge text and access-controlled internal
> coordination are different surfaces. When evidence crosses the codebase
> boundary, a sanitized summary is the default. A private internal coordination
> record MAY keep a necessary, access-appropriate reference when its governing
> privacy and access rules allow it.
>
> **Stricter project direction.** A project MAY forbid any mention of its chat
> surface in forge text. This rule does not weaken such a direction.
>
> **Limits.** This rule adds no automated check, no rewrite of history and no
> deletion of existing text. Existing coordination records stay immutable.
> Whether text relies on chat as evidence is a human judgment.

Front matter of the new file follows the sibling rules (id, scope: workflow,
strength, status, `see_also`), marked provisional with evidence status
**Theoretical** (one project, one governor direction, 2026-10-30). Its `strength`
value is the D1 choice: `MUST` under option A, `SHOULD` under option B. Only the
evidence clause carries that strength. Both privacy tiers and the destination
paragraph read the same under both options.

## Semantic examples (review evidence, not a validator)

| Case | Result |
|---|---|
| MR description says "approved in chat, message 412" | Forbidden: chat as approval evidence. Cite the decision record ID. |
| Issue says "scope agreed in the team thread" | Forbidden: chat as scope evidence. Cite the plan and its Gate 1 record. |
| MR note quotes a private link and a token copied from a chat | Forbidden twice: chat evidence, and a secret or private link. |
| Issue reports that a webhook relay drops events, with a sanitized payload that reproduces it | Allowed: the relay is the subject; the payload is minimal and sanitized. |
| MR changes a chat-integration adapter and describes its behavior | Allowed: chat is the artifact under review. |
| Public issue pastes a customer name and a private thread link as evidence | Forbidden: chat as evidence, and identifying data crossing the boundary. Use a sanitized summary. |
| Internal coordination record cites a private design thread by an access-appropriate reference, with no sensitive content copied | Allowed where the project's privacy and access rules allow it. A decision record ID is still preferred as evidence. |
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

### Strength propagation (answers `q48r` F1)

The governor's D1 choice sets one strength. The implementation MUST apply that
strength, and no other, to every surface that expresses the behavior:

| Surface | Required handling |
|---|---|
| New owner front matter (`strength`) and text | `MUST` under A, `SHOULD` under B, in the clause and the front matter. |
| Projection manifest (`reason` text for each row) | States the chosen strength. It does not call the owner an absolute invariant under B. |
| Day-to-day page paragraph | Uses the chosen strength in its own words, or only names the owner. |
| `agf/templates/AGENTS.md` route | Names the owner and uses the chosen strength, or stays a neutral pointer. |
| `agf/rules/README.md` index line | Neutral pointer; names no strength. |
| `agf/CHANGELOG.md` entry and migration note | States the chosen strength and the matching classification (A: BREAKING; B: ADDITIVE). |
| `merge-request-description-rule.md` pointer | Neutral pointer to the owner. |
| `.claude/rules/INDEX.md` row and `.claude/rules/workflow/` symlink | Neutral foundry-local discovery pointers. |
| Foundry `AGENTS.md` route line | Keeps the stricter `ghgs` direction (no mention of the live chat surface in merge request or issue text) in the line's own words. It cites the generic owner as a floor and never replaces or weakens `ghgs`, even when the governor picks B. |

A pointer that only names the owner stays neutral and needs no strength. A
surface that restates behavior must match the D1 strength or be removed. Gate 2
review checks each row of this table against the diff.

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
the governor can choose the faster path. This plan and its reviews do not choose
D1. Only a later `governor-reviewer` record can choose it, and Gate 1 approval
must record that choice. The implementation, if approved, applies the chosen
option on every surface in the strength-propagation table above.

**If the governor chooses A.** The implementation stays unmerged and unshipped
until a separately approved major-release sequence exists and fresh,
release-specific acceptance covers it. This plan supplies no release authority,
version, milestone or date. Gate 2 for the implementation MUST cite that
separate sequence.

**If the governor chooses B.** The classification is ADDITIVE only if all of
these hold across the complete projected change: the evidence clause and the
front matter say `SHOULD`; no surface restates the behavior as `MUST` or as an
absolute invariant; the privacy paragraphs add no prohibition beyond the
inherited owners (the minimization tier is advisory); and the changelog entry
says ADDITIVE. Gate 2 checks each condition. Any surface that fails one makes the
package BREAKING and returns it to this plan's option A path.

Making the new minimization tier mandatory would be a further breaking change.
This plan does not propose it.

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
2. The new rule contains the clause, the allowed case, both privacy tiers and the
   destination paragraph, the stricter-direction sentence, the limits and the
   examples, and names no product or tool. Its front-matter `strength` equals the
   governor's D1 choice.
3. `agov plan projection-check --base-ref origin/main <manifest>` passes without
   `--allow-planned` at the implementation head (the owner then exists).
4. The changelog entry states the D1 outcome and the migration note. Every row of
   the strength-propagation table is checked against the diff and the result is
   recorded in the Gate 2 handoff.
4a. Option A only: the Gate 2 handoff cites the separately approved major-release
   sequence, and the implementation merge request is not merged before it exists.
5. `agov validate adoption`, `adopter-docs`, `infosec` pass; `git diff --check`
   is clean; exact-head pipeline green.
6. Independent exact-head review and a separate binding Gate 2 precede merge.

## Collision and dependency evidence

`agov plan collision-check --plans-only --changed-only`, run on the committed
plan, reported 20 blocking overlaps with earlier plans. All 20 are
`approved`. The shared files are the rules index, the shipped assistant template,
the project `AGENTS.md`, the adopter changelog, the day-to-day page, the
merge-request description rule (through the broad glob `agf/rules/**`). None of these plans has an open claim or open merge request on these
paths: the only open merge requests at this head are coordination records and
this plan. Their earlier edits changed other entries and prose in the shared
files; this plan adds a new rule, one pointer, one route, one page paragraph and
one changelog entry, and rewrites none of that content. All 20 are added to
`depends_on` as sequencing acknowledgements. This does not approve this plan or
any predecessor.

Round-2 refresh (checked against `origin/main` at `37828e4a`): two further
approved plans landed after the first scan and overlap the day-to-day page and
the adopter changelog: `20261030T163253Z-pages-structure-and-voice` and
`20261030T210529Z-pages-source-consistency-g1-g5`. Both are now in `depends_on`.
The target section "Show the assistant and role on GitLab" still exists on
`origin/main`. The same implementation-time refetch and recheck applies.

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
BLOCK findings (rounds 1 and 2) and are not cleared; independent re-review of
the revised exact head is required before the Gate 1 decision. Status stays
`pending-gate1` until a `governor-reviewer` record chooses D1 and approves.
