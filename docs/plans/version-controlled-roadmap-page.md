---
id: version-controlled-roadmap-page
status: pending-gate1
topic: version-controlled-roadmap-page
advisory_consultation:
  - role: contribution-auditor
    status: waived
    reason: >
      Codex is drafting this plan immediately after serving as contribution-
      auditor on the v2.5 SoW, so a same-session self-review would not be an
      independent consultation. Publish the plan for a separately recorded
      contribution-auditor or governor challenge before Gate 1; do not treat
      this waiver as positive review evidence.
  - role: infosec-sentinel
    status: not-applicable
    reason: >
      Documentation navigation and roadmap projection only. No credentials,
      learning-record content, executable trust boundary, schema, validator,
      or blocking automation changes.
---

# Plan: version-controlled AGF release roadmap page

## Decision requested

Add one human-facing roadmap page to the version-controlled AGF documentation
site and make it discoverable from three places:

1. the top-level MkDocs navigation;
2. the documentation overview; and
3. one concise link in the repository-root `README.md`.

The page is an informational projection of governed repository artifacts. It
does not replace Gate 1 plans, coordination decisions, GitLab issues/MRs,
changelogs, tags, or releases as authority.

Planning only. Do not edit documentation, navigation, or README content until
the governor-reviewer approves this plan at Gate 1.

## Problem and intended audience

The governed release plan is intentionally detailed and lives under
`docs/plans/`, while current status is distributed across issues, MRs,
coordination records, changelogs, and releases. That is reconstructible but
costly for an evaluator, adopter, contributor, or occasional maintainer who
wants to answer three narrower questions:

- What is released now?
- What outcome is the project currently trying to deliver?
- What is deliberately parked rather than promised?

A GitLab wiki would improve discoverability but create another mutable content
lineage outside the repository MR/Gate workflow. The selected approach keeps
the projection beside the product documentation and validates it in the same
strict site build.

Primary audiences:

- evaluators and adopters deciding whether to use the current release;
- contributors deciding whether proposed work belongs in the active train;
- maintainers and AI assistants locating the authoritative plan and evidence.

## Files and placement

| File | Planned change |
|---|---|
| `agf/docs/site/roadmap.md` | New canonical human-facing roadmap projection at `/roadmap/`. |
| `agf/docs/build/mkdocs.yml` | Add one top-level `Roadmap: roadmap.md` navigation entry, near the evaluation and documentation entry points. |
| `agf/docs/site/docs/index.md` | Add one short link for readers seeking project direction and release status. |
| `README.md` | Add one concise `Current roadmap` link near the existing published-documentation link. Do not duplicate the release train. |

Do not add roadmap summaries to adopter workflow, contract, runbook, or FAQ
pages merely to increase link count. Those pages should link only when a
specific user question requires it; this first version deliberately limits the
discovery surface to the three locations above.

## Page contract

The roadmap page must start with a visible authority and freshness block:

- `Status`: Level 1 Planned, pending Gate 1, approved, active, or released as
  supported by the cited artifact; never infer status from an open MR alone.
- `Observed at`: UTC timestamp for the snapshot.
- `Source revision`: exact commit containing the projection.
- `Governed source`: exact plan/decision link and revision, or an explicit
  statement that no next-release plan is approved.
- `Authority`: "Informational projection. Governed plans, recorded gate
  decisions, changelogs, tags, and release artifacts are authoritative."

The page must contain:

1. **Current released baseline** — current tag/version, release link, and any
   significant known released-artifact limitation that would make a simple
   "latest release is healthy" statement misleading.
2. **Active release train** — stage, intended user outcome, current governed
   status, entry/exit evidence, and authoritative source. If the v2.5 SoW is
   still pending, label the train proposed rather than approved.
3. **In scope now** — bounded outcomes and their issue/plan links.
4. **Parked, radar, and excluded** — distinguish valid deferred work from
   rejected/obsolete work and name the revisit trigger where one exists.
5. **How status changes** — update the page only from a published plan,
   governor decision, merged change, or released artifact; link rather than
   restating detailed disposition tables.

The page must not contain:

- promised dates without a governed, evidence-backed scheduling decision;
- `Tested`, `Deployed`, or `Released` claims inferred from issue-board position;
- a second copy of the full issue/MR ledger;
- a claim that pending plans authorize implementation;
- independent-corroboration claims for sequential or same-service AI reviews;
- automatic status derived from CI without a named authority-bearing rule.

## Initial content source and dependency

The first implementation must read the exact accepted state at implementation
time rather than hard-code this plan's current session memory:

- current released baseline from `agf/VERSION`, `agf/CHANGELOG.md`, the exact
  Git tag, and the GitLab release page;
- the v2.5 release train only from an approved version of
  `docs/plans/v250-statement-of-work.md` and its governor decision;
- open significant limitations from their canonical issues and published
  coordination triage;
- parked/radar classification from the approved SoW and existing radar
  discipline.

If the v2.5 SoW remains pending or blocked when this page is implemented, the
page must say so and present the train as a proposal. It must not turn a pending
plan into apparent roadmap authority.

## Update ownership and drift control

The page is manually maintained in v1. Automation or generation is not part of
this plan.

- Planner-architect proposes a page update when an approved release plan or
  governed scope decision changes.
- Coder-tester updates the page in the relevant implementation/release MR when
  shipped scope or evidence changes.
- Contribution-auditor or fact-checker checks status, version, source, and
  overclaiming during the existing review.
- Governor-reviewer decides Gate 1/Gate 2 and release; editing the page never
  creates authority.

Every roadmap update must update `Observed at` and cite the source artifact.
Release preparation must treat an inaccurate roadmap as documentation drift,
subject to the existing adopter-documentation review. A future generated
projection may be planned only after the manual page demonstrates recurring
drift or measurable maintenance cost.

## Implementation sequence after Gate 1

1. Re-fetch `main` and the authoritative v2.5 plan/decision state.
2. Create `agf/docs/site/roadmap.md` with the authority block and current
   evidence snapshot.
3. Add the top-level MkDocs navigation entry.
4. Add the one-line documentation-overview and root-README links.
5. Review every roadmap statement against its exact source and maturity/evidence
   vocabulary.
6. Run strict documentation and adopter-doc validation.
7. Publish a Gate 2 handoff naming the exact roadmap source snapshot and any
   intentionally unresolved planning state.

## Required validation

- `mkdocs build --strict --config-file agf/docs/build/mkdocs.yml`
- `agov validate adopter-docs`
- `git diff --check`
- Manual link/source check for every plan, issue, MR, release, and tag link.
- Manual claim check that pending/approved/active/released and maturity labels
  match the controlling artifact.
- Manual navigation check at desktop and narrow viewport; the roadmap must be
  reachable without searching and must not crowd the existing adopter task
  navigation.

## Acceptance criteria

1. One roadmap page exists in version control and strict MkDocs builds it.
2. The page is reachable from top-level navigation, documentation overview,
   and one root README link.
3. All three entry points link to the same page and contain no duplicate train
   summary.
4. The authority/freshness block includes status, UTC observed-at time, exact
   revision, governed source, and the informational-projection warning.
5. Released, active, proposed, parked, radar, and discarded/superseded states
   are not conflated.
6. Known significant released-artifact limitations are visible rather than
   hidden behind a generic current-version statement.
7. No pending plan is presented as implementation authority or release
   commitment.
8. Every non-trivial roadmap claim links to its controlling repository or
   GitLab artifact.
9. README and other docs remain concise discovery surfaces; the roadmap page is
   the sole human-facing projection.

## Non-goals

- No GitLab wiki page.
- No due dates, delivery promises, or velocity forecast.
- No issue-board automation or workflow-label migration.
- No generated dashboard, GitLab API integration, JavaScript, or new MkDocs
  hook.
- No change to the governed v2.5 SoW, issue/MR dispositions, release scope,
  version, tag, or release artifact.
- No implementation before Gate 1 approval.
