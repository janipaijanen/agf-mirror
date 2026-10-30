---
id: 20261030T163253Z-pages-structure-and-voice
status: pending-gate1
topic: public-pages-information-architecture-and-reader-voice
affected_paths:
  - agf/docs/site/**
  - agf/docs/build/mkdocs.yml
  - agf/docs/build/hooks/**
  - docs/reviews/pages-structure-and-voice/**
  - docs/reviews/rule-projection/20261030T163253Z-pages-structure-and-voice.yaml
depends_on:
  - 20260916T002012Z-roadmap-refresh-post-v2101-issue130
  - 20261021T220000Z-retire-windows-support
  - 20261023T045214Z-remove-elixir-references
  - 20261023T083302Z-remove-elixir-leftovers
  - 20261023T155559Z-agov-strict-args-ref-list
  - 20261024T070435Z-timer-reality-polling-alignment
  - 20261025T192020Z-supersedes-adopter-docs
  - 20261027T045944Z-record-only-publication-termination
  - 20261027T052118Z-gitlab-pages-homepage-modernization
  - 20261027T052500Z-tooling-coexistence-discoverability
  - 20261027T230355Z-v212-release-freeze-corrections
  - 20261027T235807Z-v212-release-publication
  - 20261029T204751Z-v213-release-freeze-preparation
  - 20261029T220500Z-v213-release-publication
  - 20260809T063148Z-pages-rework-discovery-and-ia
  - 20261030T054200Z-pages-reader-journey-patch
  - 20261030T064744Z-v2131-release-publication
  - 20261030T072047Z-v2131-readme-journey-followup
  - 20261030T160040Z-forge-evidence-rule
advisory_consultation:
  - role: infosec-sentinel
    status: pending
    reason: Review requested for preservation of privacy, trust and authority boundaries during consolidation.
  - role: contribution-auditor
    status: pending
    reason: Review requested for scope, provenance, compatibility and preservation of governance meaning.
  - role: web-ux-reviewer
    status: pending
    reason: Independent baseline source review received; concrete plan review requested. Rendered implementation review remains required.
  - role: documentation-estate-steward
    status: pending
    reason: Claude supplied initial navigation, consolidation, audience and terminology recommendations; concrete plan review requested.
  - role: fact-checker
    status: pending
    reason: Review requested for example wording, human accountability versus delegated role authority, and evidence labels.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: make public AGF documentation understandable before it becomes technical

Governance mode: STRICT. This is a planning proposal, not implementation or
release authority. The human requested a substantive rethink of structure and
voice for first-time visitors, CTOs, business owners and AI leads, in
collaboration with Claude. Claim: `20261030T163253Z-planner-architect-3sgb`.

## Problem and intended result

The present site makes readers learn the framework's organization before they
can decide whether it helps them. The earlier journey patch improved the
homepage route but preserved the full navigation inventory. This plan does
not treat that preservation as a constraint on the new reading experience.
It preserves URLs and governance meaning, not the current menu or repetition.

Baseline: main `4076884d4d59ece494a685417bb24f50a9df8993`, checked against the
live site on 2026-10-30. The navigation has Home plus five groups and 46 page
entries in total. Inspection found:

- "Start here" combines an adoption decision with installation and loader
  reference. A visitor must choose among nine entries before knowing why.
- Home and Documentation repeat benefits, next steps and tooling explanation.
- "Should you adopt?" reaches a long activation/inventory discussion before
  its adopt/wait/skip decision. That is the wrong order for a business reader.
- "Evaluate" is an expert review protocol, not an adoption decision guide.
- The operating hub leads with recovery and specialist procedures rather than
  an ordinary change. Setup competes with an installation-reference sequence.
- "Two gates, one repo", "governor" and role identifiers arrive before their
  meaning. "You" sometimes means a human and sometimes an assistant executing
  commands. The solo-project explanation exists but is easy to miss.

These are source-based design findings, supported by a live-page check, not
visitor-study results. The proposed improvement is a testable design hypothesis.

## One site, five purposes

Keep Home as the product introduction. Use five primary sections after Home;
each starts with a short, useful landing page. Do not add four parallel persona
sites. Audience links point into the same canonical explanations.

| Section | Landing page | Question answered and immediate destinations |
|---|---|---|
| Decide | `docs/start/should-you-adopt.md` | Is this useful here? Fit and non-fit; responsibilities; cost and risk; evidence and limits; licensing; examples |
| Get started | `docs/start/commission.md` | How do I try it? One setup sequence, then one small governed change; optional tool/configuration detail when needed |
| Work with AGF | `docs/operate/runbook.md` | What do I do next? Propose and implement an ordinary change; approve/review; handle a block; recover/coordinate; upgrade/leave |
| Reference | `docs/index.md` | Where is the exact detail? Concepts, installation/tool reference, commands/contracts, glossary and evidence vocabulary |
| About | `docs/about.md` (new, short index) | What is the project's status and basis? Releases/roadmap, terms, contributing, independent review resources and background |

Section landing links must work in desktop and mobile navigation. Use the
existing Material theme, not a custom navigation application. Keep at most
three levels of navigation. Specialist detail belongs beneath a named task or
reference subgroup, not in a flat list of equal-looking first steps.

The small About index is the only new reader page proposed. Do not create
another glossary, overview or persona landing page: reuse the concept page,
Decide sections and existing indexes. No new dependency is needed. Use Material's built-in `navigation.indexes`
feature for clickable section landing pages, and verify rendered keyboard/mobile
behavior. If the installed theme cannot provide it, use an explicit first
"Overview" entry in each group instead; do not add a plugin or build a custom
navigation system.

### Home: explain, demonstrate, then route

In this order:

1. What AGF does, in plain language: repository files that define how people
   and assistants plan, approve and record software changes. Explain that the
   reader keeps their existing assistant and tools.
2. One concrete, explicitly illustrative change: the assistant proposes a
   small feature; the accountable approver approves the plan; the assistant
   implements and tests; the approver reviews the result and decides on merge.
   Distinguish automated evidence from approval. Introduce the two gate names
   here, after their purpose is understood.
3. What the reader takes on: a named accountable person, review attention,
   maintained records and existing engineering checks. Link costs, evidence
   limits and exit beside these commitments, not behind promotional claims.
4. Two main actions: **Decide whether AGF fits** and **Set up AGF**, retaining
   the existing exact CTA destinations and classes. A smaller existing-user
   link goes to Work with AGF.
5. Short audience links to Decide's sections: "Understand the basics",
   "Cost and accountability", "Technical fit", "Assistant workflows".
   Supporting role labels may name business owners, CTOs and AI leads, but
   labels describe the question rather than require a job title.

The first screen must contain the concise explanation and primary action;
the numbered order describes the full narrative, not a requirement to push
both actions below all explanatory sections. Put a short primary action beside
the introduction and keep deeper examples below it.

Keep version/source available through the existing header. Reduce the long
provenance notice to a concise accurate link; retain release-versus-development
distinction on Release information. Move the motto and detailed maturity/tool
taxonomy out of the introductory screen. This is an explanatory rewrite of
Home, not merely a sidebar relabel.

### Audience routes and decisions

| Reader | Route | They should be able to decide or explain |
|---|---|---|
| First-time visitor | Home → Decide | What AGF provides, who runs the assistant, who approves, whether to continue |
| Business owner | Home → Decide: cost and accountability | Required human attention, control, limits, licensing and exit; no CLI knowledge required |
| CTO | Home → Decide: technical fit → exact reference | Fit with existing repository/CI, authority and review boundaries, rollout obligations and evidence limits |
| AI lead | Home → Decide: assistant workflows → tool/configuration reference | Framework versus runtime, assigning responsibilities, optional CLI/MCP, third-party coexistence limits |
| Solo maintainer | Home → Get started → first change | They may be the named human approver; the assistant prepares work; both decisions remain explicit |
| Existing user | Home → Work with AGF → selected task | How to advance ordinary work or recover a blocked session without repeating onboarding |

### Navigation ownership and AI entry

Every page has one navigation owner. Tooling, MCP, third-party tools,
customization, layouts and loader configuration live under Reference, grouped
as Concepts, Tools and configuration, and Contracts and artifacts. Get started
links the relevant reference at the point of need; it does not list another
copy of those pages in its sidebar. About owns expert framework review and
project background. Contextual links are intentional, duplicated ownership is
not. Each reference subgroup begins with one sentence saying what to find there.

`agf/docs/site/llms.txt` is in scope explicitly: update its reading paths and
labels to the final structure, preserve valid canonical destinations and
identity/evidence boundaries, and check it as an AI-lead entry point. Do not
leave an old AI-facing map alongside the new human-facing navigation.

## Content ownership and disposition

All paths below are relative to `agf/docs/site/`. This is the implementation
map: regroup and rewrite introductions; retain unique technical material.
Before any consolidation, read the whole source page and record where each
unique instruction, caveat, evidence statement and fragment goes.

| Existing page(s) | New purpose and disposition |
|---|---|
| `index.md` | Single product introduction, example and audience routes as above |
| `docs/index.md` | Compact Reference directory; remove duplicated product pitch and four-step onboarding prose |
| `docs/start/should-you-adopt.md` | Decide landing; fit/non-fit, responsibilities, costs, limits and audience sections first; detailed inventory later |
| `use-cases.md` | Keep detailed examples and evidence; short selected summaries in Decide link here, without copying the whole page |
| `docs/adopt/downstream-impact.md` | Canonical costs, responsibilities, file/CI impact and privacy; plain-language summary before implementation inventory |
| `docs/start/adopter-faq.md` | Short answers linking to their owners; no second complete specification of roles/setup/workflow |
| `docs/start/commission.md` | Sole ordered first-time setup tutorial; title Set up AGF; preserve verification, collision safeguards and framework-only route |
| `docs/start/first-governed-change.md` | First real change after setup; explicit human decisions, assistant actions and check evidence |
| `docs/adopt/install-and-layouts.md` | Installation reference linked at the relevant setup step; remove competing tutorial order without losing unique safeguards |
| `docs/adopt/assistant-loader-configuration.md` | Assistant configuration reference; Get started links it at verification time |
| `docs/operate/runbook.md` | Work landing, ordinary change first; short task selection rather than a competing operating specification |
| `docs/operate/day-to-day.md` | Canonical routine; human and assistant responsibilities visible where they differ |
| `docs/operate/run-agf-in-real-situations.md` | Worked examples linked beside the task they illustrate; prompts remain direction, not authority |
| `docs/operate/review-and-release.md` | Review, approval and release procedure; retain exact-evidence and authority requirements |
| `docs/operate/session-continuity.md`, `docs/operate/poll-and-coordinate.md` | Recovery and coordination subgroup; keep their distinct canonical procedures |
| `docs/adopt/upgrade-from-v1.md`, `docs/adopt/exit-and-decommission.md` | Upgrade and leave subgroup; retain all removal/preservation caveats; exit also linked from Decide |
| `docs/concepts/roles-and-gates.md` | How approvals work; canonical plain-language responsibility table/glossary first, role IDs/delegation/rotation detail later |
| `docs/concepts/position.md` | Technical fit: framework versus runtimes and existing assistants; linked from Decide and Reference |
| `docs/concepts/coordination-protocol.md` | Decision-record protocol reference; define coordination before procedural detail |
| `docs/concepts/evidence-and-learnings.md` | Evidence and learning reference; Decide links its evidence limits, operators link its learning procedure |
| `docs/adopt/tooling-and-tokens.md`, `docs/adopt/mcp-server.md`, `docs/adopt/third-party-plugin-coexistence.md` | Tools subgroup: optional CLI, MCP, third-party tools. Preserve dated evidence and compatibility limits |
| `docs/adopt/applicability-catalog.md`, `docs/advanced/customizing-the-framework.md` | Customization reference; distinguish shipped inventory, activation, optional pruning and unshipped starter proposals |
| `docs/start/framework-map.md`, `docs/advanced/contracts-and-validators.md` | Framework and validation reference; not prerequisites for understanding Home |
| `reference/environment-variables.md`, `reference/release-artifacts.md`, `reference/coord-graph-index.md`, `reference/review-findings.md`, `reference/tooling-conformance.md` | Technical reference subgroup; precise identifiers retained |
| `reference/maturity-vocabulary.md`, generated `reference/release-info.md`, `reference/disclaimer.md` | Evidence vocabulary, exact build provenance and limitations remain readily reachable |
| `docs/adopt/licensing.md`, `docs/adopt/commercial-licensing.md` | One clear licensing entry linked from Decide/About; retain separate canonical detail and unchanged terms, not an editorial legal rewrite |
| `evaluate.md`, `ai-evaluation-dossier.md` | About → Review the framework / Full documentation for reviewers; explicitly distinguish independent review from adoption; retain evaluator safeguards |
| `roadmap.md`, `contribute.md`, `acknowledgements.md`, `docs/concepts/philosophical-lenses.md`, `future/evolution.md` | About: current status and contributions first, background/research nested; preserve nonbinding future labels |
| `reference/artifact-registry.md`, `reference/figures.md`, `reference/palette.md` | Preserve existing auxiliary reference routes and links; no new promotion into introductory navigation |
| `llms.txt` | AI-facing reading paths updated to match the final canonical pages, URLs and evidence limits |
| `content-plan.md` | Refresh the non-public planning map to match this ownership model; keep it excluded from publication |

### Voice contract for this editorial change

- "You" addresses the named human reader. "Your assistant" addresses delegated
  execution. In assistant-ready prompts, explicitly label the quoted block as
  instructions for the assistant; do not silently change the addressee.
- Introduce responsibilities before role identifiers. Example: "Choose the
  person accountable for approvals—you, if this is a solo project. AGF calls
  this person the human governor. Record the assignment explicitly."
- Explain plan approval before "Gate 1", and approval of the completed change
  before "Gate 2". Define the decision log before "coordination channel".
- Use exact role IDs in tables, configuration, commands and links where they
  are required. The readable summary does not rename roles or grant authority.
- A role is a responsibility, not necessarily another employee, model or
  installed assistant. Explain solo/team examples without implying that an
  implementing assistant may approve its own work or that advisory reviewers
  acquire binding authority. Human accountability and bounded role delegation
  must remain distinct and faithful to the canonical rules.
- On pages that can be entered directly, provide a brief first-use explanation
  or descriptive link. A definition on Home alone is insufficient.
- Lead with the reader's question, outcome and next action. Put commands,
  schema detail and specialist exceptions after the necessary explanation.
  Keep materially relevant limits beside the claim they qualify.
- No invented ROI, compliance promise, production enforcement, validated
  integration or maturity claim. Preserve the existing evidence vocabulary;
  reduce repetition by linking precise reference, not by deleting caveats.

### Agent roles, active participants and adding a specialist

The human's follow-up asks explicitly for agent roles, self-approval boundaries,
which roles are active by default, which are shipped but inactive, and how to
bring another role into the work. These are required outcomes, not an appendix.

Use **agent roles** when describing responsibilities and definitions; use
**assistant**, **person** or **session** for the participant playing a role.
Do not promise that each role is a separately running autonomous agent.
The canonical role page must provide a compact table separating:

- shipped role definitions;
- the documented solo trial (`coder-tester` plus human `governor-reviewer`);
- the standard governed project (also `planner-architect`);
- specialist roles, their purpose and activation trigger;
- how the reader locates and interprets their own project's authorized, assigned
  active roster; never publish foundry's roster as the adopter default.

The shipped `agf/agents/MANIFEST.example.yaml` deliberately marks all roles
skipped. It is a template, not the project's live roster. Consequently do not
label any shipped role automatically active or promise an always-running
process. Explain which responsibilities the selected operating model requires,
then how the human assigns and records them in the project's live manifest.
Do not confuse all shipped rules being available with all roles being active.

Show a short request example for adding a needed specialist to a change:
"Ask for a security review of this plan." Explain the distinct subsequent
steps: the assistant proposes the needed role and scope; the authorized human
approves activation/assignment; the project records the player, responsibilities,
limits and review handoff; the specialist supplies findings; the authorized
approver decides. Existing active roles may be consulted within their current
scope without pretending every consultation creates a new role. New role design
or authority changes still follow the canonical evolution/approval process.

Explain self-approval precisely instead of asserting a new universal slogan.
Implementation does not confer approval authority; advisory findings do not
approve gates. Check the apparent tension between the public role page's
"No agent both implements and approves" and `agf/ADOPTING.md` section 1.1's
Phase 1 human conflict guidance. Also reconcile human accountability with
bounded deputy-role delegation. Fact-checker review must identify the canonical
scope and show a correct solo example. If resolving the apparent conflict needs
a policy decision, publish that finding and stop the affected wording; do not
silently strengthen or weaken policy in a documentation patch.

## Compatibility, boundaries and patch eligibility

This is conditionally patch-eligible content: editorial structure and clarity only.
The exact final diff plus factual and projection reviews must demonstrate that
classification; a whole-estate rewrite is not automatically a patch.
No new framework obligation, authority, command, schema, dependency, commercial
term, supported platform or default changes. No release version/date is selected
by this plan. A later release plan must satisfy its own checklist; it cannot
inherit v2.13.1-specific exceptions or rewrite released artifacts.

Prefer regrouping navigation over moving files. There is no redirect plugin
in the current build and this plan adds none. A moved section must leave a real
static compatibility page/section at its old path, retaining old anchors and
meaningful destination links; preserve necessary context, not an empty stub. Existing published page URLs
must keep working. If consolidation moves a section, retain its existing anchor
at the old URL with an explicit destination; do not assume a redirect preserves
fragments. Inventory generated HTML IDs and internal links before rewriting,
including dossier links and legacy explicit anchors. A preserved HTTP 200 with
lost meaning or an unhelpful blank stub is not sufficient.

Do not edit generated Release information or the dossier as independent prose.
Preserve their build hooks and source provenance. Keep the exact shared feature-summary block in `docs/index.md` byte-identical
to root README, with the same markers and opener. Place it as a bounded secondary
section after the Reference task directory; it is an explicit exception to the
consolidation of repeated product explanation. Explain human governor in plain
language before that block so a direct entrant understands its terms. Run
`python3 tooling/tests/test_public_feature_summary.py` unchanged. Relocating or
rewriting the shared block or changing its test requires a plan amendment;
this scope does not include root README or test changes.
Retain the existing hero CTA targets, artifact verification, single Step 6
first-change link and other commissioning guards. Hook edits are limited to
documented navigation/anchor and content-source changes, never weakening safety
or verification to make the rewrite pass.

No normative rules, role definitions, root README, root loaders, changelogs,
release metadata, unrelated styling redesign, new figures or runtime code are
included. Minimal existing CSS changes are allowed only if needed for the
restructured navigation/content to remain readable and accessible.

### Required projection evidence

Moving or consolidating normative operating guidance is governed by
`agf/rules/workflow/documentation-rule-projection-rule.md`, even when the
intended meaning is unchanged. This plan includes the initial canonical
`docs/reviews/rule-projection/20261030T163253Z-pages-structure-and-voice.yaml`
for existing source paths; implementation must update it to name every final
changed/new Pages/runbook file and all six surface dispositions:
canonical normative owners, adopter explanation, runbook situation map, AI
landing/loaders, validation/drift evidence, and distribution. Release/migration classification
is additionally documented in this plan.
Record owner-by-owner semantic preservation. Existing owners and unchanged
loader paths may be explicitly unchanged with reasons; do not invent a new
normative owner or silently modify an excluded loader. Any necessary extra
projection change requires a plan amendment and refreshed overlap review.

The strict implementation check is `agov plan projection-check --base-ref
<accepted-base> <manifest>` (without planned-owner relaxation). Preserve its
full changed-page coverage. Alongside the manifest, keep the section/anchor
map and factual comparisons as evidence, not substitute rules. If the final
diff changes obligations, authority, defaults, support contracts, licensing,
commands/schemas or evidence strength, stop and amend/reclassify before Gate 2
and release planning. The projection check cannot establish semantic equality;
that remains a reviewer responsibility.

## Work allocation and sequencing

1. Codex prepares this plan; Claude challenges the information architecture
   and content ownership as documentation-estate-steward. Independent reviewers
   assess the plan. Keep pending-gate1 until all consultations are disposed and
   a binding governor record exists.
2. After approval, implement six bounded, independently reviewable MRs in the
   order below. This plan coordinates the whole outcome; it is not permission
   for a single 46-page rewrite MR. A page audit may conclude "unchanged"; do
   not rewrite every page merely because it is inventoried. Each MR has a
   precise claim, its own exact-head checks and Gate 2, and a reversible diff.

   | Order | Reader question / slice | Proposed owner and paths |
   |---|---|---|
   | 1 | Who does what? | Claude: roles-and-gates, position, role/activation sections; resolve factual ambiguities before other prose depends on them |
   | 2 | Should I adopt? | Claude: Decide, downstream impact, FAQ and selected use-case summaries |
   | 3 | How do I start? | Claude: Commission, first change, layouts and loader explanation; preserve one setup path |
   | 4 | How do I work? | Claude: operating hub and procedures, upgrade/exit introductions; coordinate the reserved forge-rule section before any shared-page edit |
   | 5 | What is AGF? | Codex: Home narrative and first-screen actions, using the reviewed role explanation and completed destinations |
   | 6 | Where do I find detail? | Codex: navigation, Reference/About indexes, llms.txt, content-plan, minimal remaining reference introductions and final compatibility map |

   Source paths retain their URLs, so each content slice works under the old
   navigation. Existing CI automatically publishes qualifying main changes to
   Pages: each slice can be public before the final integrated version. Its
   Gate 2 must explicitly cover that automatic deployment. Verify affected live
   pages, links and source SHA after each deployment, holding dependent slices
   until acceptance completes. Switch navigation last, after destination content
   exists. Freeze
   completed shared pages; writers agree exact handoffs before any overlap.
   Claude performs the final documentation-estate sweep independently of Codex's
   navigation work. Each author gets an independent implementation reviewer.
   Rollback a failing slice through its normal governed revert; later dependent
   slices stop. No content or navigation-only merge chooses a release version.
3. Before rewriting, run the baseline question script below on the existing
   built site. Record screenshots, routes and answer evidence. For each slice,
   rerun affected scenarios and preserve unaffected checks; run the complete
   script on the integrated final site. These are advisory usability measures,
   not a new mechanically binding prose rule.
4. Both writers inspect whole pages before consolidation. Keep the final
   page/section/anchor map and an advisory sentence-preservation report under
   `docs/reviews/pages-structure-and-voice/`. Extract baseline sentences
   containing MUST, never, only, approve or authority as review candidates;
   map them to their preserved final meaning/location or explain the editorial
   disposition. This catches likely drift but is neither a complete semantic
   inventory nor a new automated blocker. Review unflagged safeguards too.
5. Obtain distinct-model rendered web-UX review and factual review per slice and
   for the integrated candidate. Rework findings, run required exact-head CI and
   request Gate 2. Peer agreement and green builds never supply gate authority.
6. Only after all required content and navigation slices are accepted may
   release planning select a patch version. Intermediate Pages deployments follow
   existing main CI as above; final integrated site verification and release
   publication are distinct acceptance events, neither substituting for the other. If one slice changes a normative contract, stop it for
   reclassification rather than carrying it into the documentation patch.

### Baseline and comparison question script

Use the same questions on the current and candidate built site. The reviewer
must not be an author of the evaluated slice. Record pass/fail, route, correct
answer source, observed ambiguity and whether prior project knowledge helped.
A desk review is not an unbiased first-time-user study; record that limitation.
Use a fresh independent reviewer where available rather than treating familiar
maintainers as representative users. No invented timings or conversion claims.

| Audience | Four questions, scored separately |
|---|---|
| First-time visitor | What does AGF supply? What runs the AI? Who approves work? Where do I start deciding or trying it? |
| Business owner | What human attention is required? What costs/risks remain? What terms apply? Can we leave while preserving our work/history? |
| CTO | Where do approvals sit relative to CI? Which responsibilities must be assigned? What changes in the repository and rollout? What does the evidence not establish? |
| AI lead | How do role definitions differ from running assistants? What is shipped versus active? How do I request an inactive specialist? Which CLI/MCP/third-party tooling claims are actually supported? |

Record 16 individual baseline and final outcomes, not just an aggregate score.
Each baseline failure must have a traced correction or explicit accepted
residual; the final walkthrough must resolve all essential understanding and
routing questions above. Explain improvements through evidence, not a menu
count alone. Add the separate solo/active-specialist scenarios in acceptance.

Collision scan on the committed plan found 16 declared-path predecessor
intersections, all currently marked approved. They are explicitly listed in
`depends_on`; their shipped functionality and safeguards remain preserved while
this plan changes their documentation organization. This is an overlap
acknowledgement, not a declaration that their lifecycle records are closed.
The scan also reported 204 plans without declared paths, so it is not a proof
of complete collision coverage. The older pending discovery plan
`20260809T063148Z-pages-rework-discovery-and-ia` was inspected manually and
covers the same IA question. This concrete proposal replaces that discovery
work in practical scope; request a binding supersession/lifecycle disposition
at Gate 1 rather than running both plans or silently changing its status.

Predecessor plans for the shipped reader journey/release are historical context,
not an instruction to repeat their structure. The active forge-evidence plan
also proposes `docs/operate/day-to-day.md`; this plan acknowledges that overlap,
reserves its publication-evidence section for that owner and requires an explicit
sequence before either implementation touches the shared file. No unapproved
forge-rule prose is imported into this patch. Recheck all live plans/MRs and
claims at Gate 1 and before implementation; add explicit dispositions for any
other detected overlap rather than broadly waiving collisions.

## Acceptance evidence

The implementation handoff must include:

1. A complete baseline-to-final route/content/anchor map, including pages absent
   from primary navigation. Every unique safeguard and evidence qualification
   has a preserved home. No duplicate setup workflow or competing owner remains.
2. Six documented task walkthroughs matching the audience table. From Home,
   each audience reaches its relevant landing/section within two purposeful
   link selections. A business reader can assess costs, responsibility, terms,
   limits and exit before encountering commands or schema syntax.
3. An independent comprehension walkthrough: after reading the introductory
   screen, the reviewer can explain what AGF supplies, what the assistant does,
   who approves and what to do next. Record actual route and points of confusion.
   This is an expert walkthrough, not evidence of measured human usability or
   a guarantee that every visitor understands within a fixed time.
4. Explicit role comprehension scenarios, answered against reconciled canonical
   owners: distinguish a shipped definition, skipped example and assigned active
   role from a running process; identify the solo model's preparer and approver
   at both gates; explain the applicable self-approval boundary; request an
   inactive specialist through approval/assignment; distinguish that from
   consulting an active specialist or designing a new role. A keyword match or
   presence of a table is not evidence of understanding.
5. A first-use terminology and addressee sweep of all public source pages, not
   just Home; ambiguous uses of "you", unexplained governor/gate terms and
   incorrect human/AI authority statements resolved or explicitly justified.
6. Strict MkDocs build; existing docs, link, commissioning, figure/accessibility
   and provenance checks appropriate to changed paths; generated dossier and
   release-information inspection. Preserve the current checks' material intent.
7. Rendered desktop (1440×900) and mobile (390×844) checks in light/dark modes: usable
   section landings, keyboard focus/order, headings, contrast, no horizontal
   overflow and no missing destinations. Capture the first-screen primary CTA
   location: the reader sees a concise explanation and an identifiable next
   action without scrolling past a large figure, long notice or role catalog.
   The independent reviewer must inspect
   rendered output; source-only critique does not close this criterion.
8. Factual checks of roles, optional tooling, supported installation, current
   status, delegation, privacy, licensing and integration evidence against
   canonical owners. A change to their meaning is outside this patch plan.
9. Before merge, exact-head pipeline/jobs and resolved findings; after deployment,
   public checks of Home, all section landings and representative old deep links,
   with build source SHA and UTC timestamp. Record any blocked check honestly.

## Planning review status

Initial Claude consultation recommended five purpose-based groups, demotion of
expert material, clearer first-use roles and consolidation of duplicated entry
paths. The independent UX source review independently identified late role
explanations, duplicated orientation, premature inventory and exceptional
workflows dominating the operations hub. This draft accepts those findings but
rejects blind licensing merges, a new glossary page and preservation of Home
without a deeper rewrite. Concrete-head review is still pending.

Concrete-head review of b968a1fb requested revision: the contribution auditor
required canonical six-surface projection evidence and conditional patch
classification; web-UX requested role comprehension scenarios, explicit
first-screen CTA checks and a shared-summary contract decision; Claude
requested bounded slices, a baseline, unique navigation owners, static anchor
compatibility, explicit section-index mechanics, llms.txt coverage and stronger
authority-preservation evidence. The revision incorporates these. InfoSec
cleared the original plan's boundaries but must review the final delta.
