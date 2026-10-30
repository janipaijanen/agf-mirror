---
id: 20261030T072047Z-v2131-readme-journey-followup
status: approved
gate1_approved_by: 20261030T073453Z-governor-reviewer-gs85
gate1_approved_at: "2026-10-30T07:34:53Z"
topic: v2131-readme-journey-and-coexistence-projection
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: Independent contribution review cleared exact corrected scope and authority/evidence boundaries; recorded in 8p5m.
  - role: infosec-sentinel
    status: consulted
    reason: Independent factual/security review cleared the corrected authority wording and bounded scope; recorded in 8p5m.
affected_paths:
  - README.md
  - docs/plans/20261030T072047Z-v2131-readme-journey-followup.md
  - docs/reviews/rule-projection/20261030T072047Z-v2131-readme-journey-followup.yaml
depends_on:
  - 20261030T054200Z-pages-reader-journey-patch
  - 20260921T040743Z-assistant-rule-drift-resolution
  - 20261027T052500Z-tooling-coexistence-discoverability
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Correct two README projections after the Pages journey change

## Decision and baseline

Gate 1 8p5m approves exactly two existing README paragraphs; gs85 corrects the
collision evidence and reapproves that unchanged scope with explicit dependencies. No README
implementation begins until that binding approval and this approved plan are
published. Gate 2 is separate. No release/tag/deployment
permission or extension of the governor delegation is requested.

Baseline: main `bb407c35969ccb251a32f5fe32179e58600223a7`, the verified
!3394 merge. Its canonical Pages journey is already implemented under pksk/gnqj.
A full-estate read-only release assessment found two cross-touchpoint drifts:

1. README's Quick install follow-up calls Install the location of the full
   commissioning prompt. Commission is now the ordered setup/prompt owner;
   Install remains its detailed layout/customization/validation reference.
2. README's third-party-tools paragraph still says “probably compatible”, while
   the canonical coexistence page describes a proposed pattern with Level 2
   Surface-defined / Theoretical evidence and no established compatibility.

Both links resolve, but the projections no longer match their canonical owners.
The release-content freeze remains open until the two findings are resolved and
verified. No diagnosis of runtime or actual compatibility is implied.

## Existing-plan overlap disposition

The tracked-plan collision check identified both README overlaps above. The
assistant-rule-drift plan's loader/rule/count work is preserved; this child
changes no loader, rule, count or historical disposition and does not declare
that plan complete. The coexistence-discoverability plan supplied the existing
README paragraph and its then-permitted “probably compatible” wording. This
child deliberately replaces only that README projection with the exact narrower
proposed/Theoretical wording below, matching the already-reviewed canonical
Pages correction. Decision gs85 explicitly approves that replacement under the
current delegation; no unrelated part of the older plan is reopened or closed.

The pre-commit changed-only scan did not include the untracked new plan in its
eligible comparison pairs. Its zero-overlap result was insufficient evidence,
not proof of no overlap. Pipeline 3007476950/job 17600154237 caught both missing
dependencies; gs85 preserves 8p5m's historical record and appends the correction.
Run the changed-plan gate against staged or committed plan bytes before future
publication. The 204 historical plans without affected_paths remain unassessed.

## Exact proposed copy

Replace only the existing two-line paragraph beginning “Full mechanics,
prerequisites, and the longer commissioning prompt:” with:

> For ordered setup, prerequisites, and the full commissioning prompt, use
> [Commission AGF](../../agf/docs/site/docs/start/commission.md). For layout,
> customization, and validation details, see
> [Install the `agf/` directory](../../agf/docs/site/docs/adopt/install-and-layouts.md).

The rendered plan links above are relative to this plan. In README the exact
hrefs must be `agf/docs/site/docs/start/commission.md` and
`agf/docs/site/docs/adopt/install-and-layouts.md`. Preserve the following first-
change and template links, numbered setup steps, headings and all other bytes.

Replace only the paragraph under “Using AGF with other assistant tooling” with:

> Assistant tooling may help inside an approved change. It cannot approve AGF
> gates or override applicable rules or human governor decisions. Required
> coordination records remain part of the evidence. This is a proposed pattern:
> Level 2 — Surface-defined, with Theoretical evidence;
> compatibility has not been established. Review installers, hooks, and data
> flows before enabling another tool. Read the
> [dated evidence and limits](../../agf/docs/site/docs/adopt/third-party-plugin-coexistence.md).

Use the README-relative href
`agf/docs/site/docs/adopt/third-party-plugin-coexistence.md` in implementation.
The linked canonical page retains the vendor versions, dates, telemetry,
non-endorsement and inspection limits. This summary creates no new tool claim,
trust approval, instruction precedence or integration capability.

## Boundaries and ownership

Only README's two named paragraphs may change. Do not edit its governed
public-feature-summary block, other copy, counts, code examples, citation,
version/license text, CSS or headings. The two audit paths are this plan and its
six-surface projection record. No Pages/body, rule, schema, runtime, dependency,
CI, loader, header-repair or release-metadata change is authorized.

Use an isolated existing checkout based on the verified main target. Never edit
or incorporate the user's dirty root README at `/home/dev/s`. Do not overwrite
unrelated changes or copy that checkout's working bytes. Root takes this child implementation after the original implementation session
completed the draft and acknowledged the handover; root supplies binding
decisions/publication under delegated authority, and existing reviewers supply
contribution/security review. No new sessions or timers.

The parent Pages scope remains historically twelve paths. This child is a
separate one-product-file correction, not a retrospective expansion of pksk/gnqj.
Pending release-publication plan `20261030T064744Z-v2131-release-publication`
must acknowledge this dependency and pin its content freeze only after the
child's verified Gate 2 merge. The later metadata MR stays confined to its own
enumerated 29 metadata paths; this correction is not hidden in that MR or in
an audit-only report carrier.

## Validation and gates

Before implementation, reobserve target/open claims and run changed-plan
collision checks. Resolve any live overlap through its actual owner and governor
record; this plan grants no competing claim.

After approval:

1. Make only the two literal paragraph substitutions. Prove the complete diff
   contains README plus the explicitly authorized plan/projection lifecycle
   evidence; every other README byte, including the feature-summary block,
   remains unchanged.
2. Resolve all three README-relative targets on disk and inspect the rendered
   README excerpts/links. Run the existing tooling cross-surface and feature-
   summary tests. Do not weaken assertions or alter outside-scope fixtures.
3. Run strict projection and applicable adopter-doc validation, `git diff
   --check`, and required exact-head CI. Preserve inherited advisory findings
   as such; no header compliance or warning-free claim is implied.
4. Independent reviewers compare both exact paragraphs with Commission,
   Install and coexistence; verify the new summary remains proposed/
   Theoretical and does not establish compatibility. Root records Gate 2 for
   the exact head and merges through the normal MR path, then verifies actual
   parents/tree. No merge is authorized by Gate 1; publish the approved plan before implementation.
5. Refresh the estate finding dispositions at that verified merge and extend
   the proposed release content-freeze SHA accordingly. No publication or
   latest-published status changes occur in this child.
