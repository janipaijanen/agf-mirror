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
validated by Elixir tooling, with optional inline labels for human-facing pages.

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

## Deterministic Academic Figure Source Generation

Status: open post-v2.4.0 TODO (Level 1: Planned). Owner: planner-architect for
the Gate 1 design; coder-tester and build-engineer for implementation and
reproducible-build evidence; governor-reviewer for scope and merge authority.
Academic-peer-reviewer should review any claim that generated topology
represents observed project evidence.

Teach Rust `agov` to generate the DOT source for these currently hand-authored
academic figures:

- `docs/academic/figures/src/agent-participation-by-phase.dot`
- `docs/academic/figures/src/role-communication-topology.dot`

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
