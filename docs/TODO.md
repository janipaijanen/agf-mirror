# TODO: AGF v2 Verification Audit

Status: `close-superseded` (Queue Convergence B2, 2026-09-09). The staged
provenance mechanism and release audits replaced this proposed blanket gate:
`agf/schemas/agent-role.schema.json` and `agf/schemas/rule.schema.json` carry
the current provenance vocabulary, while `docs/audits/rc1-readiness-audit.md`
records the high-risk-path audit and its explicitly deferred residual risk.
Future targeted provenance findings need their own current issue or plan; this
TODO is no longer a parallel backlog owner.

## Problem

AGF v2 is inheriting material from agents-govern v1.x: learnings, rules,
documentation, examples, claims, and operating habits. Some of that material is
probably still valid. Some may only be valid for the old v1.x layout, tooling, or
bootstrap process.

The in-development AGF must not present inherited material as verified AGF v2
behavior until it has been checked against AGF v2 itself.

## Proposed Rule

Every learning, rule, documentation claim, validator claim, and adoption
instruction in AGF v2 must carry one of these dispositions before release:

| Disposition | Meaning | Release treatment |
|---|---|---|
| `verified-agf-v2` | Reviewed against current AGF v2 layout, schemas, tooling, docs, and operating model | May be presented as AGF v2 guidance |
| `inherited-v1x-unverified` | Originates in v1.x; may still be valid for AGF v2, but has not been verified under AGF v2 | Must be labelled as inherited/unverified, not advertised as AGF v2 fact |
| `adapted-from-v1x` | Originates in v1.x and has been revised for AGF v2 | Needs review evidence before promotion to `verified-agf-v2` |
| `retired-for-agf-v2` | v1.x material is obsolete, misleading, or not worth carrying forward | Remove from AGF v2 public/distribution docs |

Suggested public label for unverified inherited material:

> From agents-govern v1.x. May be valid for AGF v2, but not currently verified
> under the AGF v2 layout/tooling. Adopters may verify and report evidence
> upstream.

## Scope

Review at least:

- `agf/rules/**/*.md`
- `agf/agents/*.yaml`
- `agf/schemas/*`
- `agf/docs/**/*.md`
- `agf/framework.md`
- `agf/ADOPTING.md`
- public documentation under `agf/docs/site/`
- figures and figure descriptions under `agf/docs/figures/` and
  `agf/docs/site/reference/figures.md`
- any README, manifest, release, or CI guidance that claims AGF v2 behavior

## Release Gate Candidate

Before AGF v2 is promoted beyond Provisional:

1. Inventory inherited v1.x material.
2. Mark each item with one of the dispositions above.
3. Verify anything public-facing that claims current AGF v2 behavior.
4. Retire or clearly label inherited/unverified material.
5. Record verification evidence in coordination, a review file, or structured
   metadata.
6. Ensure public docs do not require adopters to guess whether a claim is v1.x
   history or AGF v2 guidance.

## Debate Points For Claude / Planner-Architect

- Should this become a hard release gate, or a staged quality gate with
  high-risk pages/rules first?
- Should the disposition live inline in each artifact, in a manifest, or in a
  generated audit report?
- Do learnings need a schema field such as `verified_for: ["agf-v2"]`, or is
  repository-level provenance enough?
- Should adopters be invited to verify inherited material through upstream
  learning records, issue reports, or both?
- What is the minimum acceptable label for inherited v1.x content so that it is
  honest without making the docs unreadable?

## Rust Implementation Health Assessment

Status: `close-superseded` (Queue Convergence B2, 2026-09-09). Decision-record
`20260623T203039Z-planner-architect-mrjh` completed the requested assessment:
full modularisation was deferred, readable-ID extraction was made the targeted
prerequisite, and message-loading extraction was made opportunistic. The
readable-ID implementation now lives in
`tooling/rust/crates/agov-coord/src/readable_id.rs`; a new structural concern
needs fresh evidence and a new work item rather than reopening this assessment.

Assess the current Rust `agov` implementation before AGF v2 release hardening,
with special attention to whether refactoring/modularization has become urgent.
The question is not "can we keep adding features?" but "are command contracts,
validators, graph tooling, version provenance, and downstream documentation
still landing in a structure that future AI and human maintainers can reason
about safely?"

Minimum assessment scope:

- command boundaries and whether CLI subcommands have clear module ownership;
- repeated parsing/normalization logic across coordination, plans, learnings,
  and graph-index code;
- test architecture for stable stdout JSON, exit codes, fixtures, and failure
  behavior;
- whether version provenance should be centralized before more commands depend
  on it;
- where machine-readable command contracts should live and how they should be
  tested;
- whether current coupling will make the three downstream guidance layers
  harder to keep synchronized: human manuals, AI manuals/cold-start guidance,
  and machine-readable command contracts.

Candidate output:

1. A short implementation-health review that names concrete refactoring risks.
2. A decision on urgency: defer, opportunistic cleanup, or release-blocking
   modularization.
3. If refactoring is needed, a Gate 1 plan that sequences it before contract
   sprawl makes the cost higher.

## Codex Opinion

Make this a staged release gate, not an immediate blocker for all development.
The highest-risk artifacts should be verified first: adoption/exit docs,
downstream-impact docs, rules that alter project behavior, validators, and any
claim that says "AGF v2 supports". Low-risk background material can carry
`inherited-v1x-unverified` temporarily, but should not appear in the primary
adoption path without verification.

The cleanest long-term mechanism is probably a small audit manifest generated or
validated by Rust `agov` tooling, with optional inline labels for human-facing
pages.

## Token ROI Zero-Savings Workflow Assessment

Status: `close-superseded` (Queue Convergence B2, 2026-09-09). MR !2383
(`fix/roi-workflow-baselines`) shipped the missing workflow baselines, tests,
and unreleased changelog evidence; its implementation commits are `672d1b88`
and `bc438bd5`. Issue #116 remains the distinct direct-CLI-versus-MCP
measurement question and is the canonical home for that comparison.

Assess why some workflow rows report zero saved tokens in:

```bash
./tooling/agov token roi report --log /home/devi/src/AGF-NG/tmp/codex-agov-token-roi.jsonl
```

Known example from the Codex snapshot: workflows such as `coord-status`,
`coord-brief`, and `coord-stale` may show measured output tokens but `0 saved`.
Determine whether this is correct because those workflows intentionally have no
configured baseline, or whether the ROI model/reporting logic is silently
dropping useful savings evidence.

Minimum assessment scope:

- list workflows that report non-zero `approx_tokens` and zero
  `approx_saved_*`;
- identify whether each workflow lacks a baseline by design, lacks a baseline
  accidentally, or should be excluded from savings totals;
- check whether command-level grouping hides workflow-level zero-savings
  explanations;
- decide whether reports need a confidence/uncertainty note for zero-savings
  rows;
- update fixture expectations if the model changes.

Candidate output:

1. A short note explaining which zero rows are expected and why.
2. A proposed baseline policy for routine coordination commands.
3. Tests that protect the intended zero-savings behavior or the corrected
   savings behavior.

## First-Class AGF Skills and Commands: Usage and Ordering

Status: `close-superseded` (Queue Convergence B2, 2026-09-09). MR !2369
shipped the AGF development starter, and the current supported lifecycle is
split deliberately across `docs/reference/agf-development-starter.md`,
`docs/reference/session-resume.md`,
`agf/docs/site/docs/operate/session-continuity.md`,
`agf/docs/site/docs/adopt/assistant-loader-configuration.md`, and
`agf/skills/README.md`. New skills or cross-surface drift require their own
bounded issue or conformance plan.

Treat AGF-related skills and commands as first-class operator interfaces, not
incidental shortcuts. Document their purpose, prerequisites, authority
boundary, side effects, and order of use. This includes `compact-agf` and the
other project skills, the `agov` CLI/MCP command surfaces, and any human-facing
aliases that invoke the same governed workflow.

Minimum documentation scope:

- inventory each supported skill and command, including whether it is a skill,
  slash command, CLI command, MCP tool, or compatibility alias;
- state when to invoke it, when not to invoke it, which role normally owns the
  action, and whether it is read-only, advisory, blocking, or authority-bearing;
- document prerequisites, expected artifacts, validation requirements,
  downstream obligations, safe stopping conditions, and recovery behavior;
- define canonical lifecycle ordering, including cold start, role-specific
  resume, task execution, validation/review, durable closeout,
  `compact-agf`, and `/compact` or session replacement;
- assess an assistant-neutral portable session-transit skill, informed by the
  AIHero `/handoff` source recorded in `docs/sources.md`, for work that must move
  between Claude, Codex, another harness, another repository, a colleague, or a
  forked side task. Keep it distinct from authoritative AGF coordination: the
  transit document should reference durable artifacts instead of copying them,
  identify role/branch/exact SHA/validation/next action, label observed versus
  inferred or unverified claims, screen sensitive content, and require a normal
  validated and published coordination record whenever it creates a downstream
  obligation. Do not default delayed or cross-machine handoffs to an ephemeral
  OS temporary directory;
- distinguish first-class supported paths from legacy/reference surfaces and
  prevent documentation from silently recommending fallback implementations;
- keep human manuals, AI cold-start/manual guidance, skill descriptions, MCP
  schemas, and machine-readable command contracts synchronized;
- document cost-aware use: targeted output, single-owner polling, bounded
  subagents, and when a cheaper model is safe for routine skill execution;
- add validation or contract tests where ordering, prerequisites, or output can
  be checked mechanically instead of relying only on prompt memory.

Candidate output:

1. A concise operator journey showing the normal order of first-class skills
   and commands from cold start through closeout and compaction.
2. A reference table for every supported surface with triggers, authority,
   inputs, outputs, side effects, and next step.
3. Cross-surface conformance checks that detect missing, contradictory, stale,
   or differently ordered instructions.
4. Usage telemetry guidance that measures quality-adjusted cost without
   treating context occupancy as billing-grade evidence.
5. A Gate 1 implementation recommendation for either an adopter-facing
   `handoff-agf` skill or no new feature, including its boundary with
   `compact-agf`, `agov session resume`, and coordination `kind: handoff`.

## Historical Branch-Only Coordination Recovery and Prevention

Status: `close-superseded` (Queue Convergence B2, 2026-09-09). MR !1010
delivered the approved forensic inventory and prevention design in
`docs/plans/historical-branch-only-coordination-recovery.md` and
`docs/reviews/historical-branch-only-coordination-inventory.yaml`. The later
Queue Convergence B4 carrier archive and CP-C visibility/collision lane own the
remaining publication and tooling work; this narrative TODO must not compete
with those governed owners.

MR !994 consolidated 79 immutable coordination records from the active
2026-08-01 through 2026-08-03 backlog. The broader visibility scan also exposed
older branch-only coordination records predating that bounded packet. Those
older records still need a deliberate recovery and disposition pass; their
continued presence on historical branches must not be mistaken for publication
to `main`, acknowledgement, supersession, or harmless archival state.

Minimum recovery and assessment scope:

- fetch and scan all relevant remote-tracking refs before inventorying; include
  local branches only as a secondary recovery source, not as proof that another
  agent could see a record;
- enumerate every coordination record absent from the current target branch,
  recording its filename/id, blob hash, originating commit, branch, MR or other
  publication route, sender, recipient, kind, `requires-ack` state, references,
  and whether an identical or conflicting blob exists elsewhere;
- determine why each record was created and then left branch-only, using Git,
  MR, pipeline, discussion, bridge-note, and coordination evidence rather than
  guessing. Candidate causes to test include skipped or missing pipelines,
  abandoned/conflicted/closed MRs, records bundled with implementation work,
  stale target branches, missing bridge notes, validation failures, duplicate
  publication attempts, branch deletion or replacement, and manual workflow
  interruption;
- determine whether the record created a downstream obligation that is still
  live, was acknowledged elsewhere, became moot, or was superseded by a later
  binding record. Do not infer closure merely from age or from the associated
  implementation having merged;
- assign an explicit governed disposition to every record: preserve unchanged
  on the target branch, preserve through a byte-identical consolidation,
  superseded with the replacing record named, duplicate with the canonical blob
  named, invalid with validation evidence, or intentionally excluded by a
  governor decision. Never silently discard unique historical evidence;
- separately assess the operational cost already incurred: invisible
  obligations, repeated reviews, duplicate CI, misleading `agov` resume/status
  results, governor reconstruction time, and any decisions made without the
  expected record being visible.

Prevention design must address introduction and detection, not only periodic
cleanup:

- define a mechanically checkable close/merge invariant: an MR that introduces
  a coordination record cannot be treated as disposed until the identical blob
  is visible on its target branch or an explicit governor disposition names the
  replacement/exclusion;
- make coordination-only publication failures visible and actionable when a
  pipeline is skipped, absent, canceled, or no longer attached to the exact
  head; do not let a successful bridge note substitute for publishing the
  canonical record;
- extend the supported `agov` visibility/poll path to scan fetched
  remote-tracking branches deterministically, continue past legacy sequence
  collisions with explicit diagnostics, and report newly introduced
  branch-only records separately from acknowledged historical debt;
- add a CI or repository check that detects coordination records present in an
  MR/source branch but absent from the intended target, with tests for
  coordination-only MRs, implementation MRs carrying a handoff, superseded
  records, byte-identical consolidation, conflicting blobs, deleted branches,
  skipped pipelines, and target-branch advancement;
- require bridge notes to identify the record, exact source commit, branch/MR,
  and intended publication path so recovery does not depend on session memory;
- define one owner and cursor for recurring remote visibility polling, plus a
  bounded escalation age for records that remain branch-only after their first
  visible publication attempt;
- ensure MR-close and worktree/branch-cleanup guidance checks for unpublished
  coordination before declaring the work complete.

Candidate outputs:

1. A machine-readable inventory and human-readable root-cause report for all
   pre-2026-08-01 branch-only records, with one governed disposition per record.
2. A recovery MR or ordered set of MRs that preserves every record authorized
   for publication unchanged and proves filename/blob collision handling.
3. A Gate 1 prevention plan covering `agov`, CI, polling, MR closeout, and
   operator documentation without giving tooling binding gate authority it
   does not have.
4. Fixture-driven tests demonstrating that a newly introduced branch-only
   coordination record becomes visible before the originating task can be
   reported as durably complete.

## Deterministic Academic Figure Source Generation

Status: `park-radar` (Queue Convergence B2, 2026-09-09; Level 1: Planned).
The authoritative academic targets moved to the separate `agf-promotional`
repository, while foundry's generic `agov coord graph` remains only a partial
precursor. Review by **2027-01-15**, or earlier if `agf-promotional` requests a
generator contract or a second academic figure update exposes repeatable
manual-source drift. Promotion requires a cross-repository Gate 1 plan naming
the target owner, evidence window, and reproducibility contract.

Teach Rust `agov` to generate the DOT source for these currently hand-authored
academic figures. The "Governing the Governors" paper and its figures
relocated to `agf-promotional` (`docs/academic/`); the target files are now:

- `docs/academic/figures/src/agent-participation-by-phase.dot` (in `agf-promotional`)
- `docs/academic/figures/src/role-communication-topology.dot` (in `agf-promotional`)

This is source generation, not merely rendering existing DOT with Graphviz.
The existing `agov coord graph --format dot` command is only a partial
precursor: it counts valid coordination messages by `(from, to, kind)` and
emits a basic live graph. It does not currently reproduce either academic
figure, declared-but-unobserved communication channels, governed role-to-phase
mappings, human/governor authority relationships, evidence-window semantics,
or the academic visual contract.

Minimum design boundary:

- derive observed communication edges from validated coordination records;
- derive role identity and current status from `agf/agents/MANIFEST.yaml`;
- represent declared-but-unobserved channels through an explicit governed
  configuration rather than inventing them from absence or model inference;
- represent participation by phase through an explicit governed role-to-phase
  mapping, because message counts alone cannot prove phase participation;
- require an explicit evidence window or snapshot identifier for any figure
  presented as an observed case study;
- distinguish observed durable traffic, declared/required channels, historical
  aliases, human authority, AI-operated roles, and automated checks without
  inferring actor or outcome from coordination message kind;
- reuse the approved semantic palette and colour-independent shape/label
  contract, while keeping participant, message-kind, responsibility, and
  outcome axes distinct; and
- include versioned, closed-allowlist provenance metadata so a future reader
  can trace each generated figure back to the generator version, repository
  source commit, explicit evidence window/snapshot, governed configuration,
  and repository-relative input files (with stable content hashes where
  applicable); carry the DOT-to-SVG/PDF subject and SHA-256 linkage in an
  embedded record or adjacent provenance sidecar chosen at Gate 1; prohibit
  absolute paths, host/user/session identifiers, raw environment maps, and
  ambient timestamps that would leak local state or break reproducibility; and
- emit deterministic DOT with stable node/edge ordering, escaping, labels, and
  no ambient timestamps or machine-specific paths.

Candidate command surface should be decided at Gate 1 rather than fixed here.
It may extend `agov coord graph` or introduce a focused academic/figure
subcommand, but it must keep the generic coordination graph useful and avoid
hard-coding foundry-only role names into adopter-facing behavior.

Minimum acceptance evidence:

1. Fixture-driven exact-output tests for both figure types, including empty,
   observed-only, declared-only, mixed, inactive-role, alias, and unknown-role
   cases.
2. Negative tests proving message kind is not treated as actor identity,
   automation status, or approval outcome.
3. Regeneration checks proving identical inputs produce byte-identical DOT.
4. Graphviz SVG/PDF rendering checks with source/output synchronization and
   desktop, narrow, light, dark/fixed-background, print/greyscale, and
   screen-reader evidence.
5. A documented migration that replaces the hand-authored academic DOT files
   only after semantic review; until then, those files remain the authoritative
   manually maintained sources.
6. Provenance validation proving a reader can resolve the generator and input
   revision from the published artifact set, with negative cases for missing,
   mismatched, dangling, duplicated, unknown-key, and checksum-inconsistent
   metadata.
7. Privacy fixtures proving provenance cannot persist absolute/home paths,
   host, user, assistant, model-session, conversation, request, or raw
   environment identifiers.

This TODO does not expand or block v2.4.0. Workstream D should correct and
regenerate the existing sources under its approved release plan; generator
implementation requires its own Gate 1 plan after the release priority clears.

## Post-v2.5.0 Intelligence-Analysis Product Discipline Assessment

Status: `park-radar` (Queue Convergence B2, 2026-09-09; Level 1: Planned).
No release, issue, or approved child plan currently owns this assessment.
Review by **2027-01-15**, or earlier after two material review defects show the
same observation/assumption/judgment ambiguity, or an adopter asks for a
formal analytic-product contract. Promotion remains a proposal through the
issue board and Gate 1; it is not latent v2.11 scope.

Assess whether mature public intelligence-analysis product discipline can
improve AGF's plans, reviews, coordination, and decision products without
importing intelligence-community authority, institutional assumptions, or
compliance claims. Start from Batch 15 of `docs/sources.md`: Luntta's secondary
design synthesis and simulated Aurora prototype are discovery/design artifacts;
ODNI ICD 203/206/208, ICS 206-01, and the UK government guidance/doctrine are
the primary sources for any substantive comparison.

Minimum assessment scope:

- identify which AGF artifact types should explicitly state their intended
  audience, decision/question served, time sensitivity, authority boundary,
  and next action; distinguish a product being valid from it being useful and
  visible in time;
- test whether current review/coordination prose reliably distinguishes
  observation, source information, assumption, inference/judgment,
  alternative explanation, recommendation, and binding decision;
- compare probability/likelihood, analytical confidence, AGF evidence status,
  and capability maturity as separate axes. Do not create a composite score or
  use confidence language without defining what its foundation is;
- assess whether review artifacts should record plausible alternatives,
  contrary evidence, key assumptions, information gaps, and indicators or new
  evidence that would change the judgment;
- audit source provenance beyond locators: currency, completeness, quality,
  independence, access, validation, technical limits, possible bias/deception,
  and which judgments each source can actually support;
- audit AI-assistance provenance across coordination, plans, reviews, generated
  artifacts, and tool telemetry. Compare model/system identity, version,
  settings, prompt/parameters, performance/training-data limits, and date of
  use against AGF's privacy, secret, reproducibility, and data-minimization
  boundaries; do not persist sensitive prompts or ambient session metadata;
- assess whether corrections and renewed reviews explain what evidence or
  reasoning changed, rather than only replacing the verdict;
- examine discoverability, recipient visibility, timeliness, feedback, and
  consistency across alternate, summarized, sanitized, or exported versions of
  the same facts, judgments, uncertainty, and decision status;
- use Aurora only as a synthetic interface/protocol case study. Preserve its
  stated limits (simulated data, heuristic confidence, deterministic
  hypotheses, incomplete provenance, no operational user validation) and do
  not present it as evidence that the mechanisms work in production;
- map proposals onto existing AGF fields and rules before adding schema. Prefer
  documentation guidance or derived views where they suffice, and preserve the
  distinction between advisory analytic quality and governor authority.

Candidate outputs:

1. A crosswalk from each primary analytic standard/guidance concept to current
   AGF support, gap, non-applicability, and evidence status.
2. A small synthetic review packet demonstrating observation/assumption/
   judgment/alternative/uncertainty/decision separation, including a corrected
   judgment and a sanitized view whose meaning remains congruent.
3. A decision on whether improvements belong in writing guidance, coordination
   or review schemas, `agov` derived views/validation, or no AGF change.
4. If structural changes remain justified, a separate Gate 1 plan with
   negative tests against false precision, composite confidence scores,
   unsupported source-quality claims, provenance leakage, and tooling that
   accidentally acquires binding authority.

This work must not expand or block v2.5.0. It begins only after the release's
contracted work and publication path are complete.

## Pages + README Visual Direction (v2.5.1) — Draft, Not Confirmed Final

Status: `close-superseded` (Queue Convergence B2, 2026-09-09). The bounded plan
landed in MR !1082; README slice 1 landed in MR !1193 and Pages theme/hero
slice 2 in MR !1205. Governor decision
`20260806T105215Z-governor-reviewer-djmq` accepted slice 2 and explicitly left
navigation slice 3 unauthorized.
Any navigation revival requires its own fresh scoped Gate 1 plan, so this
pre-implementation draft is not a live work owner.

Four persona-briefed drafting passes (technical writing, developer
experience/information architecture, marketing, UX/visual design) proposed a
coordinated redesign of the public Pages site and README: palette, typography,
flattened navigation, a real hero component, and a README restructure that
relocates evidence/maturity hedging out of the pitch without deleting any
claim. Inspired by a reference site's visual register (near-monochrome
palette, confident minimal copy, flat navigation), adapted to AGF's own
governed diagram palette rather than copied wholesale.

Full synthesis, decisions, risks, and test strategy:
`docs/plans/20260803T192952Z-pages-readme-visual-refresh-v251.md`.
Visual mockup: `docs/plans/assets/20260803T192952Z-pages-readme-visual-refresh-v251-mockup.html`.
Drafting branch: `plan/pages-readme-visual-refresh-v251`.

No implementation has started. `README.md`, `agf/docs/build/mkdocs.yml`, and
the site CSS remain untouched.

Open before implementation proceeds:

- Confirm hero copy variant (plan recommends "Plans reviewed before code.
  Code reviewed before merge."; two alternates are in the plan).
- Resolve the dark-mode accent contrast gap with a real validated palette
  token added to `reference/palette.md` and passed through
  `check_palette_contrast.py` — the mockup's dark-mode accent is illustrative
  only, not a proposed final value.
- Decide whether `.claude/rules/docs/documentation-audience-rule.md`'s
  "Public documentation site" row should be customized for this project — the
  plan flags a mismatch (rule says business users; AGF's own site targets
  developers/technical evaluators) but does not resolve it.
- Sanity-check the proposed 7-tab navigation flattening against real adopter
  feedback if any exists, rather than treating the drafting pass alone as
  validation.
- General re-review: the human governor has not yet confirmed this is the
  final direction versus a strong starting point to iterate on further.

Candidate next step once confirmed: implementation as two independently
reviewable MRs (README restructure; Pages theme/nav/hero), sized so each
answers one yes/no review question, per `branch-and-worktree-rule.md`.

## Philosophical-lenses page: self-application gap, not yet actioned

Status: `awaiting-governor` (Queue Convergence B2, 2026-09-09). Exact question:
should the still-unimplemented, Gate-1-approved
`docs/plans/philosophical-lenses-learning-additions.md` remain authorized for a
fresh future implementation, or be retired under the current documentation and
source-evidence expectations? The plan, decision
`20260721T002215Z-governor-reviewer-7mg5`, and handoff
`20260721T075914Z-planner-architect-qkte` establish that the work is real but
do not assign it to v2.11; only the governor can change release scope or retire
that approval.

A review of `agf/docs/site/docs/concepts/philosophical-lenses.md` against its
actual source text (2026-08-03, same session as the v2.5.0-rc.1 three-track
evaluation) found the page's evidence-status discipline is genuinely careful
(each of the six lenses correctly scopes its own citation, e.g. the
Bainbridge lens explicitly disclaims corroboration from literature it didn't
cite), but every lens ends in a generic "design question" that the page never
answers against AGF's own concrete mechanisms — despite the project's
self-application motto ("the framework that cannot govern itself has no
business governing anything else").

This is no longer a hypothetical gap. The three-track evaluation
(`docs/coordination/20260803T204257Z-planner-architect-sjqm.md`, learning
`20260803T205236Z-gt3x`) already answers several of the page's own design
questions for AGF specifically:

- The **legibility critique** ("does producing the evidence the system wants
  distort the work, and can a record be gamed while looking legitimate")
  is exactly what the mutation-served-as-authentic finding demonstrates,
  observed in this system, not hypothesized.
- **Accountability sinks** ("is there a specific named person who could have
  decided otherwise") has an honest current answer: `gate1_approved_by` can
  be empty or a non-governor role and still validate as `approved`.

Recommendation: add a concrete "applied to AGF" callout per lens (or at
least for legibility and accountability sinks, the two with direct evidence
already on record), citing the relevant coordination/learning record, rather
than leaving all six design questions purely generic. Low effort — the
evidence already exists and is already cited elsewhere in the repo; this is
a linking/citation task, not new research.

Not captured as a `learnings/` entry: this is a documentation-content gap on
an already-published page, not operational evidence from running the
framework's tooling (contrast with `t8mz`, which is the adjacent but
distinct "no self-reported enforcement profile" finding). Recorded here in
TODO.md for lack of a more specific tracked home.
