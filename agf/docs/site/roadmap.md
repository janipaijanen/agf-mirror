# Roadmap

*Audience: evaluators, adopters, contributors, and maintainers or AI
assistants locating the authoritative plan and evidence.*

| Field | Value |
|---|---|
| Status | Released baseline confirmed; next release train **proposed**, pending Gate 1 |
| Observed at | 2026-07-28T20:17:29Z |
| Source revision | Authored against `main` at `4ed2e57c93428a32fb6d1159bfecc907ecc3dd64`. This page's own revision is whatever this file's Git/GitLab history shows as its latest commit -- do not treat the source-revision value above as this page's own commit. |
| Governed source | [MR !755](https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/755) (`docs/plans/v250-statement-of-work.md`) is open and unapproved. No next-release plan is approved as of the observed-at timestamp above. |
| Authority | Informational projection. Governed plans, recorded gate decisions, changelogs, tags, and release artifacts are authoritative. This page is neither. |

## Current released baseline

**Latest release: `v2.4.5`** — [release page](https://gitlab.com/janipaijanen/agents-govern-framework/-/releases/v2.4.5), tag `v2.4.5`.

A known significant limitation is open against this release:
[issue #81](https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/81)
documents that the shipped `v2.4.5` archive's operations-runbook test fails
when run from a freshly extracted `agf-2.4.5.tar.gz` (`Ran 7 tests / FAILED
(errors=4)`, all four from fixtures absent from the release manifest). Do not
read "latest release" as "no defects are open against it."

## Active release train

No next-release train is approved. [MR !755](https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/755)
proposes a `docs/plans/v250-statement-of-work.md` release train (a portfolio
reset, a recovery patch, and a small number of release candidates) as v2.5.0's
plan, and is currently under active, multi-role review that has not reached a
Gate 1 decision. Read the plan and its coordination trail directly rather
than relying on a summary here — the proposal has already changed shape once
during review, and a paraphrase would go stale faster than this page is
updated. This page will state the approved train, sourced from the governor's
Gate 1 decision, once one exists.

## In scope now

- **This roadmap page.** [MR !758](https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/758)
  ([plan](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/main/docs/plans/version-controlled-roadmap-page.md)),
  Gate 1 approved; this file is that implementation.
- **Issue-board workflow** ([issue #75](https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/75)) —
  proposed, Gate 1 plan open in [MR !745](https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/745),
  not yet approved.
- **Coordination-visibility performance** ([issue #76](https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/76)) —
  proposed, Gate 1 plan open in [MR !746](https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/746),
  not yet approved.

Stabilization and contract-repair work feeding a possible v2.5.0 (for example,
issue #81 above) is release-blocking maintenance, not shipped adopter-facing
product capability, until it lands in a released tag.

## Parked, radar, and excluded

- **v2.5.0 discovery portfolio** ([issue #57](https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/57)) —
  Level 1 Planned discovery surface. The issue states plainly that listing a
  candidate there does not assign it to v2.5.0; each candidate theme is
  explicitly adopted, split, deferred, or rejected by the planner before it
  becomes scope. Not being in "In scope now" above does not mean rejected —
  see the issue for the current disposition of each theme.
- **Trust-chain and governance-tooling themes** (the open ideas referenced
  from issue #57, roughly issues #32–#49) — under discussion; several have a
  named revisit trigger (for example, a source-reviewed threat-model plan, or
  confirmed adopter demand) recorded in the coordination trail rather than
  here. Follow the issue links directly instead of trusting a copied summary.

This page intentionally does not reproduce the full issue/MR disposition
ledger. It exists in the coordination channel and stays current there; a
second copy here would drift.

## How status changes

This page is manually maintained. It changes only when:

- a published Gate 1 plan or governor decision changes what is proposed or
  approved;
- a merged change or released artifact changes what is delivered;
- a coder-tester implementation or release MR updates it directly to match
  shipped scope.

Every update refreshes the `Observed at` field above and cites its source
artifact. An inaccurate roadmap is documentation drift, not a cosmetic issue,
and is handled through the project's normal adopter-documentation review.
