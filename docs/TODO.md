# TODO: AGF v2 Verification Audit

Status: active TODO. Owner: governor-reviewer for final disposition; planner-architect
and contribution-auditor should critique the policy before it becomes a release
gate.

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

Status: open TODO. Owner: planner-architect for assessment shape; coder-tester
for implementation evidence; governor-reviewer for urgency and release impact.

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

Status: open TODO. Owner: coder-tester for investigation; planner-architect for
modeling guidance; governor-reviewer for whether the result affects public
claims.

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

Status: open any-agent TODO. Owner: planner-architect for the information
architecture; coder-tester for command/skill inventory and executable evidence;
governor-reviewer for authority and release treatment.

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

## Historical Branch-Only Coordination Recovery and Prevention

Status: open post-consolidation TODO. Owner: planner-architect for the forensic
assessment and prevention design; coder-tester/build-engineer for `agov` and CI
controls; contribution-auditor for independent root-cause review;
governor-reviewer for the disposition of each historical record.

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

Status: open post-v2.4.0 TODO (Level 1: Planned). Owner: planner-architect for
the Gate 1 design; coder-tester and build-engineer for implementation and
reproducible-build evidence; governor-reviewer for scope and merge authority.
Academic-peer-reviewer should review any claim that generated topology
represents observed project evidence.

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

Status: open post-v2.5.0 TODO (Level 1: Planned). Owner: planner-architect for
the comparison and Gate 1 design; contribution-auditor for adversarial review;
fact-checker for primary-source fidelity; governor-reviewer for scope and any
binding adoption decision. Academic-peer-reviewer should review any formalized
uncertainty vocabulary before it becomes a framework claim.

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
