---
id: 20261030T054200Z-pages-reader-journey-patch
status: approved
gate1_approved_by: 20261030T060244Z-governor-reviewer-gnqj
gate1_approved_at: "2026-10-30T06:02:44Z"
topic: public-pages-reader-journey-and-documentation-corrections
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: Independent factual/security review CLEAR after two wording corrections and legacy-overlap reconciliation; exact reviewed content and reviewer provenance in handoff grjh.
  - role: contribution-auditor
    status: consulted
    reason: Independent contribution review CLEAR after legacy-overlap reconciliation; exact reviewed content and reviewer provenance in handoff grjh.
affected_paths:
  - agf/docs/build/mkdocs.yml
  - agf/docs/build/hooks/check_commissioning_links.py
  - agf/docs/site/index.md
  - agf/docs/site/docs/index.md
  - agf/docs/site/use-cases.md
  - agf/docs/site/docs/start/should-you-adopt.md
  - agf/docs/site/docs/start/commission.md
  - agf/docs/site/docs/start/adopter-faq.md
  - agf/docs/site/docs/adopt/install-and-layouts.md
  - agf/docs/site/docs/adopt/third-party-plugin-coexistence.md
  - agf/docs/site/docs/concepts/position.md
  - docs/reviews/rule-projection/20261030T054200Z-pages-reader-journey-patch.yaml
depends_on:
  - 20261027T052118Z-gitlab-pages-homepage-modernization
  - 20261027T052500Z-tooling-coexistence-discoverability
  - 20261021T220000Z-retire-windows-support
  - 20260809T063148Z-pages-rework-discovery-and-ia
  - agents-govern-2.0-pedagogical-docs-restructure
  - get-started-commissioning-ux-v1
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Proposed v2.13.1: a clearer public documentation journey

## Decision requested and release boundary

The governor requested a proposal to rework the public GitLab Pages site's
order, language and contents as a patch release. Recommend candidate **v2.13.1**,
with plain English and a first-time adopter as the primary audience. Experienced
operators and assistant readers retain direct access to references.

Gate 1 was approved by governor decision `20261030T055817Z-governor-reviewer-pksk`
and narrowly amended by `20261030T060244Z-governor-reviewer-gnqj`.
Implement only the bounded documentation slice below. This is not merge,
deployment or release approval; do not bump versions or tag from this plan; a separate release-publication
plan/freezing decision must admit its reviewed exact head, reconcile changelogs
and version surfaces, and authorize the normal release process. Do not promise a
publication date before independent rendered review and CI evidence exist.

This qualifies as a patch because it corrects documentation defects and improves
navigation without changing a command, schema, installation behavior, governance
contract, supported platform or dependency. Any discovered need to change those
contracts returns to a separate plan and version assessment.

## Baseline and observed problems

Baseline: main `4e0dde87323f5a7964f623f9437fa00c318ad442`, after v2.13.0
publication and public roadmap synchronization. The fresh anonymous homepage
fetch on 2026-10-30 matched this checkout; an older search-engine snapshot did
not. Browser rendering was unavailable in this session, so these are source and
content findings, not completed visual acceptance.

- Homepage route cards already exist. Preserve the v2.12 presentation work;
  align their reading order and wording rather than redesigning the visual theme.
- Top navigation puts roadmap and the AI evaluation dossier before the learning
  path. Start leads with a dense concept map before the decision path. Home and the docs index
  offer different sequences; the index omits commissioning, coexistence and MCP.
- Commission and Install repeat artifact discovery, verification and setup.
  Should-you-adopt and Use cases repeat the Adopt/Wait/Skip decision.
- Commission introduces the framework archive through an optional-tooling bullet,
  says "No macOS and Windows tooling are unsupported", and prescribes
  `agov validate adopter-docs` as a general adopter commissioning check.
- Position's per-tool notes prescribe copying rules into assistant directories,
  describe skills as Claude-only, and claim runtimes work identically. These
  conflict with the canonical loader guidance and overstate the evidence.
- "No runtime" and "probably compatible" need precise boundaries: AGF is not an
  agent runtime; optional local tooling exists; third-party coexistence is not
  certified integration or an endorsement.

The two 2026-10-27 predecessor plans in `depends_on` supplied the landed homepage and
coexistence baseline. This proposal deliberately revises their public copy/nav
surfaces after that delivery; it does not reopen their CSS work or extend their
authority. Recheck current claims, target and open MRs before implementation;
resolve any new overlap explicitly before mutation.

The collision check also identifies `20261021T220000Z-retire-windows-support`:
its landed platform decision remains the support boundary. This patch corrects
the resulting Commission sentence, and does not revive Windows support.

Two older pending plans have no `affected_paths`, so the mechanical collision
check cannot assess them: `20260809T063148Z-pages-rework-discovery-and-ia` and
`agents-govern-2.0-pedagogical-docs-restructure`. Their overlapping, unexecuted
Pages work is acknowledged here, not treated as an approved prerequisite. This
fresh source audit, independent IA/factual consultations and concrete navigation
proposal carry forward their operational-before-theory and progressive-disclosure
intent. Their broader page rewrites, new figures and changes outside this plan's
allowlist are not inherited. Request an explicit governor disposition replacing
that overlapping unexecuted scope with this plan at Gate 1; old plan statuses
remain pending until that decision is recorded. Any historical plan metadata
reconciliation is a separately scoped lifecycle change, not implicit authority
to edit additional product files or a claim that those plans were implemented.

The landed `get-started-commissioning-ux-v1` plan (file
`20260806T122121Z-get-started-commissioning-ux-v1.md`) supplies the Commission
baseline. Preserve its operational-first-screen invariant, one canonical
commissioning prompt, new/existing-project branching, artifact verification and
completion contract. The first screen must show the practical setup path before
gate theory or taxonomy. That landed baseline is acknowledged in `depends_on`;
it is not new work to redo. These manual dispositions supplement the incomplete
mechanical coverage of historical plans.

## Proposed order

Use six top-level destinations. Keep every current navigation leaf reachable
exactly once in the navigation, preserve page URLs and existing deep-link anchors,
and allow contextual cross-links. The implementation records a before/after leaf
inventory so omissions cannot be hidden by a successful build.

| Destination | Pages in proposed reading order |
| --- | --- |
| Home | Existing homepage: what AGF does, who it helps, next step; short evidence and licensing pointers. |
| Start here | Documentation overview; Should you adopt?; Use cases; Costs and project changes (Downstream impact); Commission AGF and optional agov; Installation details (Install and layouts); Assistant loader configuration; Your first governed change; Adopter FAQ. |
| Use AGF | Operations runbook; Day-to-day operation; Session continuity; Poll and coordinate; Run AGF in real situations; Review and release; Upgrade from v1; Exit and decommission. |
| Tools and customization | Install and use agov (Tooling and token savings); AGF's MCP server; Third-party assistant tooling; Choose rules and roles (Applicability catalog); Customizing the framework. |
| Reference | Roles and gates; AGF and agent runtimes; Coordination protocol; Learning loop and evidence; Framework map; Validation contracts; Environment variables; Release artifacts; Capability and evidence labels; Coordination graph index; Review findings; Tooling conformance; Release information; Disclaimer. |
| About AGF | Evaluate AGF; Single-page AI evaluation dossier; Roadmap; Licensing; Commercial licensing; Contribute; Influences and acknowledgements; Philosophical lenses; Evolution. |

The primary journey is **Decide → Set up → First governed change → Operate**.
Home cards and the docs index must use the same order. Detailed concepts and
technical references remain linked at the point of use; they are not mandatory
pre-reading before a visitor can understand the product.

## Proposed language and page ownership

Use short, concrete sentences, expand unfamiliar abbreviations at first use,
and explain Gate 1/Gate 2 before relying on those names. Distinguish what the
human decides from what an assistant or validator does. Preserve required
qualifications and link supporting detail rather than stripping it for brevity.

Illustrative homepage copy, subject to implementation review:

> **Govern AI-assisted changes in your repository.**
>
> AGF gives your assistants shared rules, named responsibilities and review
> checkpoints. For non-trivial changes, a human approves the plan before work
> starts and decides whether the tested result may merge. Plans, approvals and
> handoffs stay in Git. Optional local agov tooling helps validate records and
> produce compact status summaries.

Primary action: **Decide whether AGF fits**. Secondary action: **Set up AGF**.
Keep a direct **Already using AGF? Open the runbook** link. Qualify the existing
strap as no AGF-hosted service or agent runtime, with optional local tooling;
retain evidence/maturity and license visibility, without leading with jargon.
This wording is explanatory, not a revision of gate exceptions or authority.

| Page owner | Bounded change |
| --- | --- |
| Home and docs index | Align the journey, improve the introductory copy, expose official MCP and third-party tooling separately. Preserve existing layout/CSS and public feature-summary boundaries; do not weaken their factual content. |
| Should you adopt? | Own the Adopt/Wait/Skip decision and its evidence limits. |
| Use cases | Own concrete scenarios and evidence; replace duplicated decision prose with a pointer, preserving any unique qualification. |
| Commission | Own one first-time setup path. Separate framework and optional tooling artifacts, correct platform wording, retain checksum/provenance verification and collision handling. Use `agov version`, `agov session resume`, and general `agov validate` when optional tooling is installed; treat warnings explicitly, not as a promise of empty output. Explain the manual route when tooling is absent. |
| Install and layouts | Own detailed layout, prerequisites, file/evidence reference and troubleshooting. Link Commission for the ordered quick path; retain unique warnings and details. |
| AGF and agent runtimes | Explain the boundary, point to canonical loader setup instead of duplicating per-tool recipes, remove obsolete Claude-only skill claims. Describe observed Foundry use separately from conceptual library mappings; no "works identically" or invented adopter evidence. |
| Third-party tooling and FAQ | Give a concise orientation for combining tools while keeping AGF authority above tool instructions. Distinguish the official local agov-mcp adapter from external MCP servers and other assistant plugins. Link dated Ponytail/Caveman evidence rather than advertise names on the homepage. |

Before deduplicating text, inventory unique warnings, facts, commands and anchors;
record their retained destination in review evidence. If a unique section cannot
fit the destination within this allowlist, retain it and link it instead of
expanding scope. The operations runbook, day-to-day and real-situations pages
already have distinct jobs; change their navigation order, not their bodies.

## Include and exclude criteria

Include a change only when it fixes a demonstrated misleading/stale instruction,
removes a duplicated onboarding step without losing information, improves a
reader's next action, or makes an existing capability/evidence boundary clearer.
The exact twelve-path allowlist is in front matter. The projection manifest
records normative owners, explanation, loader/runbook and distribution impacts.
No other product source is authorized by this plan.

The sole acceptance-hook change is `check_commissioning_links.py`: require
primary hero action exactly `docs/start/should-you-adopt/index.html` and secondary
hero action exactly `docs/start/commission/index.html`. Each action independently
requires its expected action class, an internal relative link, target existence
and exact target equality. In the same file, exercise both actions with a
positive fixture and missing-anchor, external-target, missing-target and
wrong-but-existing-target negative controls. Update obsolete comments and result
messages. Retain all other Commission relative/external-link, live GET, release
archive checksum/provenance, tutorial-gating and prerequisite checks and tests.
This aligns the documentation guard to the approved journey, without changing
CI policy or runtime behavior.

Preserve third-party evidence dates, telemetry disclosures, non-endorsement and
version-specific limits. Coexistence stays Level 2 Surface-defined/Theoretical;
agov-mcp remains Level 4 Validated with its documented `dir` limitation. Do not
refresh vendor versions or make new vendor claims without separately scoped
primary-source research. Assistant loader guidance remains the canonical owner.

Exclude runtime/CLI/API/schema/rule changes, loader installation behavior, CI or
release automation, telemetry changes, new platform support, integrations,
dependencies, rebranding, CSS redesign, new art, translation, wholesale technical
reference rewrites, root README edits, and the separate inherited SPDX repair
proposal `20261030T000640Z-coder-tester-tgrk`. Root README has local user edits in
the main workspace and is outside this task. Existing published artifacts stay
immutable. Release metadata/changelog edits belong to the later publication plan.

## Sequence, collaboration and acceptance

1. Gate 1 reviews this exact proposal and its include/exclude boundary. The
   information-architecture and factual/security consultations are independent
   inputs; their combined scope intentionally includes narrow home, position,
   FAQ and coexistence corrections beyond the IA-only recommendation.
2. After approval, the planner assigns one documentation implementer and one
   independent web-UX reviewer, with acknowledged claims before editing. Ask
   Claude for the next-actions stake when available (recorded availability was
   2026-10-30T13:00Z); that consultation is not an approval or a hidden dependency
   on an unavailable session. No extra background session or timer is needed.
3. Implement on a branch from the checked target. Publish the exact diff and a
   leaf/anchor and deduplication inventory. Have an independent factual reviewer
   compare platform, validator, loader and compatibility wording against current
   canonical sources. Check all claims and commands against the supported path.
4. Run the CTA hook self-tests, existing strict MkDocs build, real rendered CTA
   hook, other site link/content hooks, projection and
   applicable adopter-doc validation, plus whitespace checks. Preserve guard
   assertions. If an existing test assumes an old label or reading order and
   requires a fixture/code edit outside the allowlist, amend the plan first;
   do not weaken or bypass the test to obtain green.
5. Independent web-UX review must cover desktop and narrow mobile navigation,
   keyboard focus/order, link labels, readability and finding the first-change
   path. Record tested routes, viewport sizes and findings. Lack of browser
   access is a concrete blocker to this acceptance step, not a visual pass.
6. Gate 2 requires that exact-head CI passes, all current pages/anchors remain
   reachable, no unique warning disappears, primary journey wording agrees
   across home/nav/index, and factual plus rendered review findings are resolved.
   The projection record and documentation-estate sweep must show no accidental
   change to normative authority or claims beyond the evidence.
7. Only then freeze the v2.13.1 candidate through the normal release-publication
   plan. Confirm version availability and packaging, changelogs, deployment
   authorization and public verification there. A working branch or rendered
   preview is not a released capability.

Release success is a visitor finding the decision, setup, first-change and daily
operation paths without contradictory instructions, while existing users retain
their links and technical reference. No speculative performance or conversion
claim is part of this patch.
