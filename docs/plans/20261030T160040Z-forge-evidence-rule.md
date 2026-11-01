---
id: 20261030T160040Z-forge-evidence-rule
status: approved
gate1_approved_by: 20261101T120448Z-governor-reviewer-saps
gate1_approved_at: "2026-11-01T12:04:48Z"
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
  - .claude/settings.json
  - AGENTS.md
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >-
      Four independent review rounds. Rounds 1 to 3 returned BLOCK findings, all
      answered. Round 1, exact 66e5ab2e: `20261030T160913Z-contribution-auditor-gdjm`.
      Round 2, exact 5e2f8221: `20261030T165922Z-contribution-auditor-q48r`.
      Round 3, exact ff368bdb: an independent InfoSec and privacy review (two
      blocking findings on the privacy examples and on the attribution of
      existing exclusions, two advisory), recorded in
      `20261101T111215Z-contribution-auditor-6kn8`. Round 4, exact 6fe9948f: an
      independent re-review returned CLEAR with three advisory wording findings,
      recorded in `20261101T112422Z-contribution-auditor-432p`. A narrow
      exact-diff review at 5a33651b returned CLEAR with three advisory items,
      recorded in `20261101T114325Z-contribution-auditor-m2ee`. The reviews are
      by AI agents, not the CI infosec-sentinel. Gate 1 was decided in
      `20261101T120448Z-governor-reviewer-saps`.
  - role: contribution-auditor
    status: consulted
    reason: >-
      Three independent review rounds returned BLOCK findings. Round 1, exact
      66e5ab2e: `20261030T160913Z-contribution-auditor-gdjm`. Round 2, exact
      5e2f8221: `20261030T165922Z-contribution-auditor-q48r`. Round 3, exact
      ff368bdb: an independent contribution audit (one blocking finding, a
      missed foundry-local surface; six advisory), recorded in
      `20261101T111215Z-contribution-auditor-6kn8`. Round 4, exact 6fe9948f:
      an independent re-review resolved every earlier finding and returned
      BLOCK on one stale sentence that let the implementation land before the
      next freeze, plus advisory wording findings, recorded in
      `20261101T112422Z-contribution-auditor-432p`. That was fixed, and a narrow
      exact-diff review at 5a33651b returned CLEAR with three advisory items,
      recorded in `20261101T114325Z-contribution-auditor-m2ee`. Gate 1 was
      decided in `20261101T120448Z-governor-reviewer-saps`.
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
`20261030T165922Z-contribution-auditor-q48r` (F1 to F3, round 2); governor
decision `20261101T105902Z-governor-reviewer-kqgp` (D1 is option A); independent
exact-head review of `ff368bdb` (round 3, below). Claims:
`20261030T160040Z-planner-architect-gr51` (released by `7nhr`),
`20261030T162930Z-planner-architect-mjbh` and `20261101T100151Z-planner-architect-e22d`.

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
| F2 privacy | The rule keeps the existing secret and anonymization rules at their own scope and strength, adds advisory minimization and a destination paragraph (revised in round 2 below), and states that moving sensitive text into an immutable record does not make it safe. It points to the existing secret and anonymization rules. |
| F3 owner scope | A new canonical owner covers merge request and issue titles, bodies and notes. The MR-description rule keeps its own scope and gains one pointer. |
| F4 projections, migration, SemVer | Affected paths now include the adopter operating page, shipped loader routing, rules index, a changelog migration entry and the structured projection manifest. SemVer constraint stated below (decision D1). |
| F5 semantic examples | Positive and negative examples are part of the rule text and acceptance criteria. Validation stays manual and advisory. |

### Round 2: answers to `q48r` (not cleared)

| Finding | Answer in this revision |
|---|---|
| F1 strength propagation | The projection section now lists every surface that expresses the behavior and requires the D1 strength on all of them: owner front matter and text, manifest statements, page, template route, rules index and changelog. Pointers that only name the owner stay neutral. The foundry `AGENTS.md` route keeps the stricter `ghgs` direction in its own words and does not route to the weaker option. |
| F2 privacy wording | The privacy text was split into existing-rule restatement and advisory minimization, with a destination boundary and no absolute ban on private references in access-controlled coordination records. Round 3 below corrected how the existing rules are described. |
| F3 major-release sequencing | Round 2 bound option A to a separately approved major-release sequence. The governor has since selected option A (below); the plan now carries only that path. |

### D1 is decided: option A

Governor decision `20261101T105902Z-governor-reviewer-kqgp` selects option A.
This revision removes the option B wording and its additive test, so a reader of
the plan sees one path. Option B appears only as a "Not selected" note in the
D1 section. The decision does not approve Gate 1.

### Round 3: independent exact-head review of `ff368bdb` (not cleared)

| Finding | Answer in this revision |
|---|---|
| InfoSec F2-R1: example row 3 called a "private link" forbidden | The row now names only a credential or token. A separate row covers a private thread link without a secret: the privacy text adds no ban, and chat-as-evidence still applies. |
| InfoSec F2-R2: inherited-exclusion sentences overstated the owner rules | The paragraph now restates the owners' real scope: secrets stay out of committed text; across the codebase boundary only the project codename appears. It says this rule extends neither. The claim "adds no new privacy prohibition" is true of the new text. |
| InfoSec A1: AI-provider prompts are an external destination | Added to the destination paragraph. |
| InfoSec A2: chat provenance must be a pointer | The clause now says a minimal reference, not a transcript, with no credential or tokenized URL. |
| Contribution F1-miss: `.claude/settings.json` omitted | Added to `affected_paths`, the propagation table and the manifest. The implementation adds the new rule's exclude glob there, neutral and with no strength. |
| Contribution A1: manifest loader paths incomplete | The manifest lists `AGENTS.md` and `.claude/rules/INDEX.md` and records `.claude/CLAUDE.md` as unchanged. |
| Contribution A2: uncited SemVer claim | Cites the framework versioning section. |
| Contribution A3: approver of the major-release sequence | Criterion 4a names a binding `governor-reviewer` decision and an unmerged implementation merge request. |
| Contribution A4: "inherited" overstated | Fixed with InfoSec F2-R2. |
| Contribution A5: template placement | Names the "Rules" section of `agf/templates/AGENTS.md`, after the directory table. |
| Contribution A6: D1 not recorded in the plan | Recorded above, with a link to the governor decision. |

### Round 4: independent re-review of `6fe9948f` (not cleared)

Every round-3 finding was found resolved. Two new reviewers returned: InfoSec and
privacy CLEAR with three advisory wording findings; contribution audit BLOCK on
one finding and advisory wording findings.

| Finding | Answer in this revision |
|---|---|
| Contribution N1 (BLOCK): a leftover sentence let the implementation land before the next freeze | Replaced. The implementation MUST NOT land before the next freeze or in any v2.x release, stays Draft until the D1 major-release decision exists, and is then rechecked against that release's freeze. |
| Contribution N2 and N3, InfoSec N2: examples over-attributed or contradicted the evidence clause | Example rows 3 and 4 reworded: a credential in forge text is unsafe but this rule adds no separate ban; a private thread link cited as evidence of authority is forbidden as chat evidence. "Inherited" wording replaced. |
| InfoSec N1: the "no credential or tokenized URL" sentence read as a new absolute | Now "SHOULD carry no credential", with a pointer to the existing rule. |
| InfoSec N3: "sanitized summary is the default" could read as an exception | Now points at `external-anonymization-rule.md` and calls a sanitized summary the usual way to comply. |
| Contribution N4: reviews named only by handoff | Named directly. The manifest keeps the foundry symlink in its reason text, not a path entry, because the file does not exist until implementation. |

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
> the coordination audit trail as a minimal reference, not a transcript. The
> reference SHOULD carry no credential, which `security-hardening-rule.md`
> already keeps out of committed text, and no signed or tokenized URL.
>
> **Allowed.** Discuss a chat, messaging or relay system when it is the subject
> of the work, for example an issue about an integration defect or a merge
> request that changes a relay adapter. Include the minimum sanitized
> reproducer needed to review that work.
>
> **Privacy: existing rules, unchanged.** This rule adds no new privacy
> prohibition and extends the scope of no existing one. Secrets and credentials
> stay out of committed text (`security-hardening-rule.md`;
> `systematic-code-review-rule.md`, section 3). Across the codebase boundary
> only the project codename may appear, and text an AI assistant sends to a
> provider is an external destination (`external-anonymization-rule.md`). Those
> rules keep their own scope and strength. Moving sensitive text into an
> immutable record does not make it safe.
>
> **Privacy: minimization (advisory).** Authors SHOULD record only the minimum
> decision or evidence summary needed for audit and SHOULD avoid copying raw
> conversation text that the audit does not need.
>
> **Destination.** Public or external forge text and access-controlled internal
> coordination are different surfaces. When evidence crosses the codebase
> boundary, `external-anonymization-rule.md` applies; a sanitized summary is the
> usual way to comply. Text that an AI assistant sends
> to a provider is an external destination; "access-controlled" does not cover
> it. A private internal coordination record MAY keep a necessary,
> access-appropriate reference when its governing privacy and access rules
> allow it.
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
value is `MUST` (D1 option A). Only the evidence clause carries that strength.
The privacy minimization sentence and the destination paragraph are advisory or
restate existing rules.

## Semantic examples (review evidence, not a validator)

| Case | Result |
|---|---|
| MR description says "approved in chat, message 412" | Forbidden: chat as approval evidence. Cite the decision record ID. |
| Issue says "scope agreed in the team thread" | Forbidden: chat as scope evidence. Cite the plan and its Gate 1 record. |
| MR note quotes a credential or token copied from a chat | Forbidden: chat as evidence. A credential in forge text is also unsafe; this rule adds no separate ban, and `security-hardening-rule.md` already keeps credentials out of committed text. |
| Internal MR note cites a private thread link, with no secret in the URL, as evidence of authority | Forbidden: chat as evidence. The privacy text adds no separate ban on the link. Cite a decision record ID instead. |
| Issue reports that a webhook relay drops events, with a sanitized payload that reproduces it | Allowed: the relay is the subject; the payload is minimal and sanitized. |
| MR changes a chat-integration adapter and describes its behavior | Allowed: chat is the artifact under review. |
| Public issue pastes the project's real name and a private thread link as evidence | Forbidden: chat as evidence, and a non-codename project handle crossing the boundary (`external-anonymization-rule.md`). Use the codename and a sanitized summary. |
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
| AI landing/loaders | Updated: `agf/templates/AGENTS.md` gains one line in its "Rules" section, after the directory table, covering merge request and issue publication; `agf/rules/README.md` lists the rule. Foundry-local routes: `.claude/rules/workflow/` symlink, `.claude/rules/INDEX.md` row, the `claudeMdExcludes` glob for the new symlink in `.claude/settings.json`, and the `AGENTS.md` list line. `.claude/CLAUDE.md` is unchanged; it imports `AGENTS.md`. |
| Validation/drift evidence | Not mechanically decidable; human review with the examples above. |
| Distribution | The owner, README, template and page already ship through `agf/` coverage; foundry-local loaders are project-local. |

### Strength propagation (answers `q48r` F1)

D1 option A sets the strength: `MUST NOT` on the evidence clause. The
implementation MUST apply that strength, and no other, to every surface that
expresses the behavior:

| Surface | Required handling |
|---|---|
| New owner front matter (`strength`) and text | `MUST` in the clause and the front matter. |
| Projection manifest (`reason` text for each row) | States `MUST` for the evidence clause only. |
| Day-to-day page paragraph | Uses `MUST NOT` in its own words, or only names the owner. |
| `agf/templates/AGENTS.md` line | Names the owner and uses `MUST NOT`, or stays a neutral pointer. |
| `agf/rules/README.md` index line | Neutral pointer; names no strength. |
| `agf/CHANGELOG.md` entry and migration note | States `MUST NOT` and the BREAKING classification. |
| `merge-request-description-rule.md` pointer | Neutral pointer to the owner. |
| `.claude/rules/INDEX.md` row and `.claude/rules/workflow/` symlink | Neutral foundry-local discovery pointers. |
| `.claude/settings.json` `claudeMdExcludes` | Adds `**/.claude/rules/workflow/forge-publication-evidence-rule.md` beside the other situational rules, so the rule is not loaded at startup. Neutral; no strength. |
| Foundry `AGENTS.md` route line | Keeps the stricter `ghgs` direction (no mention of the live chat surface in merge request or issue text) in the line's own words. It cites the generic owner as a floor and never replaces or weakens `ghgs`. |

A pointer that only names the owner stays neutral and needs no strength. A
surface that restates behavior must match option A or be removed. Gate 2 review
checks each row of this table against the diff.

## Downstream impact and SemVer (decision D1: option A)

A new `MUST NOT` narrows a previously implicit permission, which
`downstream-impact-rule.md` classifies as BREAKING. The framework's version
policy reserves breaking changes for a MAJOR release (`agf/framework.md`, the
versioning section: "`MAJOR`: breaking changes").

**D1 is option A: keep `MUST NOT`**, selected in governor decision
`20261101T105902Z-governor-reviewer-kqgp`. The change is BREAKING and
**ineligible for the v2.13.x maintenance line or any v2.x minor release**. It
may ship only through a separately approved major-release plan. This plan
chooses no version number, milestone or date and supplies no release authority.

The implementation stays unmerged and unshipped until that major-release
sequence exists, recorded by a binding `governor-reviewer` decision, and fresh,
release-specific acceptance covers the implementation. The implementation merge
request stays Draft or unmerged until then, because `agf/CHANGELOG.md`
Unreleased on `main` flows into the next release. Gate 2 for the implementation
MUST cite that decision.

*Not selected:* option B, weakening the clause to `SHOULD NOT` as an ADDITIVE
recommendation, was the alternative. It is not part of this plan. Making the new
privacy minimization sentence mandatory would be a further breaking change and
is not proposed.

```
Impact:
  Foundry:          Already applied by governor direction; the generic rule adds the
                    adopter-facing invariant.
  Direct adopters:  BREAKING (MAJOR only): merge request and issue text must not
                    rely on live chat as evidence of authority.
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
2. The new rule contains the clause, the allowed case, the privacy paragraphs and
   the destination paragraph, the stricter-direction sentence, the limits and the
   examples, and names no product or tool. Its front-matter `strength` is `MUST`
   (D1 option A).
3. `agov plan projection-check --base-ref origin/main <manifest>` passes without
   `--allow-planned` at the implementation head (the owner then exists).
4. The changelog entry states `MUST NOT`, the BREAKING classification and the
   migration note. Every row of the strength-propagation table is checked against
   the diff and the result is recorded in the Gate 2 handoff.
4a. The Gate 2 handoff cites the binding `governor-reviewer` decision that records
   the separately approved major-release sequence. The implementation merge
   request is not merged before that decision exists.
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
plans. The implementation MUST NOT land before the next freeze or in any v2.x
release. It stays Draft until the major-release decision in D1 exists, and is
then rechecked against that release's freeze.

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

Approved at exact head 5a33651b in `20261101T120448Z-governor-reviewer-saps`; D1 is
option A. The advisory rows record all four review rounds (rounds 1 to 4). The
approval authorizes no implementation merge, release, tag, version or date.
