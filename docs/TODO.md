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
