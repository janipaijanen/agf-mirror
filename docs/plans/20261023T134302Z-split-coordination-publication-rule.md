---
id: 20261023T134302Z-split-coordination-publication-rule
status: pending-gate1
topic: split-coordination-publication-rule-into-core-and-companion
advisory_consultation:
  - role: infosec-sentinel
    status: pending
    reason: Review requested. Moves text between two rule files; no credentials, schemas or validators change.
  - role: contribution-auditor
    status: pending
    reason: Review requested before Gate 1.
affected_paths:
  - agf/rules/workflow/coordination-publication-rule.md
  - agf/rules/workflow/coordination-publication-companion.md
  - AGENTS.md
  - agf/templates/AGENTS.md
  - agf/rules/always/gate-self-test-rule.md
  - .claude/settings.json
  - .claude/rules/INDEX.md
  - LICENSES/commercial-corpus-manifest-v1.yaml
  - tooling/rust/crates/agov/tests/fixtures/commercial-corpus/inventory-path-classification.csv
  - docs/reviews/rule-projection/20261023T134302Z-split-coordination-publication-rule.yaml
depends_on:
  - 20261022T205113Z-instruction-context-reduction
  - 20261023T080431Z-instruction-context-followup
  - 20261023T045214Z-remove-elixir-references
  - 20260913T085123Z-work-item-steward-gate1
  - 20260913T093405Z-row5-s3-priority-ordering-and-challenge-response
  - 20260914T130217Z-row6-s4-issue123-collision-check-blocking-promotion
  - 20260915T075705Z-issue130-pending-advisory-consultation-status
  - 20260921T040743Z-assistant-rule-drift-resolution
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: split `coordination-publication-rule.md` into a core and a companion

Follow-up to issue #138. Draft by planner-architect for debate with coder-tester
(Codex) and the contribution-auditor before Gate 1. Nothing here is approved.

## Decision requested

Gate 1 approval to move four situational sections of the largest always-loaded
rule into one companion file under `agf/rules/workflow/`, verbatim, so the core
loads at startup and the companion loads on demand. This changes shipped rule text
layout (not requirements). Gate 1 and Gate 2 are the Human Governor's: the rule is
shipped framework text and the file the high-risk list names for publication-hold
mechanics.

## Measured facts (main f13f1524, 2026-10-23)

- The rule is 22,856 chars, the largest loaded file. Section sizes (chars): the
  rule 5,176; lifecycle 1,833; CI implementation shape 4,414; required negative
  self-tests 1,991; why 701; related 1,103; channel membership 1,555; recipient
  session addressing 782; publication holds 4,053.
- Claude Code startup input tokens (supported surface: `claude -p ... --output-format json`
  usage, deterministic over two runs): 81,143 on main, 74,166 at the size follow-up
  head. This plan reuses that measure.
- References to the rule path outside history: `.gitlab-ci.yml` comments, `AGENTS.md`,
  `agf/templates/AGENTS.md`, the schema description, three crate sources (comments
  and error text citing "condition 1/3/6"), the commercial-corpus manifest and its
  path-classification fixture, seven rule-projection manifests, and several Pages.
  `high-risk-operations-rule.md` names `publication_hold` mechanics "in
  coordination-publication-rule.md".

## Design

### What moves (option S2, recommended)

To `coordination-publication-companion.md`, verbatim: "CI implementation shape"
(4,414), "Required negative self-tests" (1,991), "Channel membership and bounded
candidate discovery" (1,555), "Advisory recipient session addressing" (782). Total
8,742 chars, about 38% of the file.

### What stays in the core

"The rule" (the six blocking conditions and the multi-record permission), "What the
fast path MUST NOT decide", "Lifecycle and supersession semantics", "Why this exists",
"Related rules", and **"Advisory publication holds and urgency"**. Holds stay because
`high-risk-operations-rule.md` names them in this file; moving them (option S1, a
further 4,053 chars) would also require editing that red-listed rule, which this plan
does not do. Option S1 is a separate later plan.

The core gains a short "Companion sections" table: for each moved section, the
situation that requires opening the companion. Both files carry the same owner
statement: they are one rule; the core's six blocking conditions govern, and the
companion adds no authority.

### What must keep working

1. Every requirement survives: the union of non-blank lines of core and companion
   equals the original's, except the added table, headers and cross-links.
2. The modal and negation multiset (MUST, MUST NOT, SHOULD, MAY, never, not, no, only,
   except) over core plus companion equals the original's.
3. Error text and comments citing "condition N" still resolve: all six conditions stay
   in the core.
4. References that name a moved section (`AGENTS.md` channel-membership and recipient
   session paragraphs, Pages, the schema description) are updated to cite the companion.
5. The commercial-corpus manifest and its path-classification fixture list the new
   file as Licensor-authored, as for its sibling rules.

### Loading

Foundry: symlink `.claude/rules/workflow/coordination-publication-companion.md`,
exclude it in `claudeMdExcludes`, one `INDEX.md` row, one `AGENTS.md` trigger line
(create or change CI routing for coordination-only commits, change
`agov coord publication-check`, read the channel-membership or visibility contract,
use `--session` addressing, or change a negative self-test).

## Critical risks

- R1 (highest): adopter loaders. Adopters who link rule files one by one into an
  assistant directory would load the core and silently lose the moved sections until
  they add the companion. That is a behavior loss for them even though the text
  survives. Mitigation: `agf/templates/AGENTS.md` names both files; the upgrade
  notes and CHANGELOG carry a migration note; `agov validate assistant_drift` is
  checked to flag a missing companion link. If that check cannot be shown to fire, the
  plan returns to the governor.
- R2: the moved "Required negative self-tests" are gate-self-test material. An agent
  editing the publication-check gate without opening the companion could ship without
  the listed negatives. Mitigation: `gate-self-test-rule.md` (always loaded) cites the
  companion by path in one added line, which the plan must also list as an affected
  path if adopted.
- R3: two files with one owner invite the "competing authority" failure that
  `documentation-rule-projection-rule` warns about. Mitigation: a single owner
  statement in both and the verbatim union test.
- R4: saving is modest (8.7k of about 136k, about 6%). If the governor values risk
  over size, the split is not worth it.

## Phases

0. Spike (no commit to main): confirm validators and `agov dist` accept an `agf/rules`
   file not ending in `-rule.md`, that `assistant_drift` lists it, and the release
   manifest include globs ship it. Stop and report if any fails.
1. Move the four sections, add the table, symlink, exclusion, INDEX row, trigger line,
   reference updates, corpus manifest and fixture rows.
2. Verify: union and modal checks (committed as a one-off script output in the MR, not
   a new gate), `projection-check`, `assistant_drift` before/after, startup token
   measure, replay.

## Acceptance (KPIs)

- Startup input tokens fall by at least 2,000 against main (the supported measure).
- Always-loaded instruction chars fall by at least 8,000.
- Union line coverage 100%; modal multiset equal; zero broken references (grep).
- Replay: six scenarios (edit CI routing for a coordination-only job, add a negative
  self-test, read channel-membership rules, use `--session`, publish a plain record,
  an urgent hold), at least 3 samples each on two models; every scenario opens the right
  file.
- `assistant_drift` flags a deliberately missing companion link (negative test).

## Classification and impact

STRICT: shipped rule layout, AI loaders, a high-risk-adjacent rule.
```
Impact:
  Foundry:          about 8.7k fewer chars loaded per Claude session; one more trigger line for Codex.
  Direct adopters:  EDITORIAL for requirements (set of required/forbidden/permitted actions unchanged); BREAKING in practice for per-file loaders until the companion is linked.
  Downstream:       none.
  Migration:        required for per-file loaders: link the companion; documented in the CHANGELOG and agf/templates/AGENTS.md.
  Evidence:         issue #138 size work; section measurements above.
```
Adopter docs parity: updated (`agf/templates/AGENTS.md`, CHANGELOG, affected Pages).

## Rollback

Revert the MR. No tag or release is involved until a release includes it.
