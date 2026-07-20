---
id: external-reviewer-fit-assessment-proposals
status: pending-gate1
topic: docs-site-external-reviewer-proposals
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: docs-only additive content; no security-relevant surface
  - role: contribution-auditor
    status: not-applicable
    reason: internal docs polish, not an external contribution requiring audit
---

# Plan note: external reviewer's 8 fit-assessment proposals (issue #17)

**Date:** 2026-07-20T07:55:03Z
**Status:** fast-track — one combined note covering all 8 sub-items, per
simplicity-rule ("docs-only, additive, no migration" tier); most are already
done.

## Mapping proposals to real files (checked before planning)

The external review used page titles/URLs that don't exactly match this
repo's file paths. Each proposal below is mapped to the actual file under
`agf/docs/site/`, and its **current status was verified by reading the live
file**, not assumed from the original review.

**Note on `agf/docs/site/contribute.md`:** this file has an unmerged
in-flight change on branch `docs/append-only-audit-trail-policy` (commit
`49dc956`, "docs: state append-only audit trail policy vs GDPR erasure")
adding a new "Audit trail policy" section after "Anonymization requirement".
None of the 8 proposals below touch `contribute.md`, so there is no
collision — noted here only so a future implementer re-checks that file's
current state before editing anything near it, since this note was written
against `main` before that branch merges.

| # | Proposal | Target file / section | Status found |
|---|---|---|---|
| 1 | Clarify v2.1-docs-label vs framework-content-version 2.2.4 relationship | `agf/docs/site/index.md`, the `> **Status:**` callout near the top | **Already done.** The callout explicitly distinguishes "framework content version" (on the Release information page) from page-level "Released (v2.x documentation)" labels, and states the labels "are not the current release identifier." |
| 2 | Compact visual summary of free/marked/contracted customization zones | `agf/docs/site/docs/advanced/customizing-the-framework.md`, "The three zones" | **Already done.** A `\| Zone \| Examples \| Upgrade effect \| Review posture \|` table follows the prose description, immediately after "The three zones" heading. |
| 3 | Compact copyable Gate 2 review checklist | `agf/docs/site/docs/operate/review-and-release.md`, under "Gate 2 in practice" | **Already done.** A "### Gate 2 review checklist" subsection exists with exactly the checklist items the review proposed (approved plan/fast-track waiver; branch matches sized work; named tests green; visible outputs inspected; no security-boundary changes; findings recorded; decision record references branch/conditions). |
| 4 | Compact daily-operation checklist | `agf/docs/site/docs/operate/day-to-day.md`, near "The daily shape" | **Already done.** A "### Daily checklist" subsection exists with the exact items proposed (read/acknowledge coordination entries; claim before acting; choose fast-track or full gate; verify branch before commit; externalize state before compact/switch/handoff; file completion/learning record). |
| 5 | Matrix of shipped / contract-target / blocking-capable / advisory-by-default validator surfaces | `agf/docs/site/docs/advanced/contracts-and-validators.md`, near "What is under contract" | **Already done.** A "### Validator surface summary" table exists with columns Surface / Shipped now / Blocking-capable / Advisory by default / Remaining target. |
| 6 | One miniature end-to-end coordination-thread example | `agf/docs/site/docs/concepts/coordination-protocol.md`, "How agents communicate" | **Already done.** A "## Miniature example thread" section exists: a complete four-file thread (Gate 1 plan handoff → governor decision-record → further steps) with realistic file contents. |
| 7 | Compact "first-week" checklist | `agf/docs/site/docs/start/adopter-faq.md`, "What should our first week look like?" | **Already done.** A "**First-week checklist:**" list exists there with all the proposed items (read decision/cost/exit pages; install `agf/` and configure loader; verify assistant loaded AGF; run one governed change; add first CI validator; prove it fires on a malformed file; review friction honestly). |
| 8 | "Configuration drift review" step added to upgrades | `agf/docs/site/docs/operate/review-and-release.md`, "Upgrading the framework"; and/or `agf/docs/site/docs/advanced/customizing-the-framework.md`, "Sending customizations upstream" | **NOT done.** "Upgrading the framework" describes the mechanical steps (branch, replace, diff-review, Gate 2, merge, pin version, read changelog) but has no explicit step to list local customizations, confirm which remain intentional, check contract drift, or decide whether local rules/roles/learnings should be upstreamed. |

## Conclusion

7 of 8 proposals are already implemented on `main`, evidently from a prior
docs-polish pass not attributed to this issue number. Only proposal 8
requires new work, and it is genuinely small: one additional paragraph or
short list appended to the existing "Upgrading the framework" section.

## Remaining work: proposal 8 only

**Affected file:** `agf/docs/site/docs/operate/review-and-release.md`,
"Upgrading the framework" section (currently lines 91-101).

**Content to add** (a "Configuration drift review" step, appended after the
existing paragraph, before "## Audit cadence"):

- List local customizations (marked-zone edits, project-domain rules added).
- Confirm which customizations remain intentional versus accidental drift.
- Check contract-zone drift: has any `agf/`-owned file been locally edited
  in a way that now conflicts with the new upstream version?
- Decide whether any local rule, role, or learning should be proposed
  upstream (see `contribute.md`).

This directly reuses existing canonical phrasing already established
elsewhere on the site (the three-zone vocabulary from
`customizing-the-framework.md`, the upstream-contribution path from
`contribute.md`) rather than inventing new terminology — per this repo's
`marketing-tone-rule.md` message-integrity requirement (state a claim once,
canonically; link, don't restate).

**Audience/tone check:** `review-and-release.md` targets administrators and
governors (public docs site audience per `documentation-audience-rule.md`);
the added content is operational/practical in tone, consistent with the
existing section, with no marketing language, per `marketing-tone-rule.md`.

## Migration needed

No.

## Test strategy

Docs-only; verification is a read-through confirming:
- The new content doesn't restate the zone table or contribute.md content
  verbatim (it should link/reference, not duplicate — per message-integrity
  rule).
- No new unqualified comparative claims introduced (verifiable-claims-rule).
- Markdown renders correctly in the existing docs build (`mkdocs build`
  locally, if available, or visual check of the built site).

## Risks

None of note — additive docs paragraph, no code or schema touched.

## Rollback

Remove the added paragraph/list from "Upgrading the framework".
