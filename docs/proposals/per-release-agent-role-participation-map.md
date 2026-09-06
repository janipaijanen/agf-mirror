<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Proposal: Per-release agent-role participation map

Status: draft for planner-architect critique and RUW inclusion. Authored by
the contribution-auditor session on operator direction, 2026-09-06. Not a
Gate 1 plan; needs planner-architect adoption and Gate 1 before implementation.

## Outcome

Every AGF release publishes a **mechanically derived** map of which
agent/reviewer roles produced and reviewed the coordination records that led
to that release, plus which model played each role. It is derived from the
immutable coordination channel over the release's own commit range — it is
evidence, not narrative.

This makes AGF's central claim ("structural role-separated multi-agent
governance actually runs") checkable by anyone with the repository, and it is
the most direct dog-fooding artifact the project can ship.

## Problem

AGF asserts that its role model, gate chain, and multi-agent collaboration are
real. Today an evaluator cannot verify that from a release without reading the
whole coordination channel by hand. The data to prove it already exists —
every coordination record carries `from:`, `to:`, `model:`, `kind:`, and
`references-to:` — but nothing surfaces it per release. Issues #38 (evidence
for parallel-collaboration claims), #45 (human accountability as a first-class
object), and #47 (reviewer independence by session/model/vendor) all circle
this same gap.

## Design

### 1. Derivation command (mechanical, reproducible)

Add `agov usage role-map <from-ref> <to-ref>` (name TBD; extends the existing
`agov usage` and `agov coord graph-index` primitives). Given the coordination
range between two release points, it emits deterministically:

- per-role record counts broken down by `kind` (claim, completion, review,
  ack, handoff, decision-record, fast-track-waiver, lesson-proposal, note);
- the set of `model:` values observed per role (e.g. `governor-reviewer:
  human`; `planner-architect: codex-gpt-5, claude-sonnet-5`);
- a reference-edge summary (how many records cite how many others — a crude
  interaction-density signal, not a quality signal);
- a coverage line (records parsed / legacy files excluded), so the map states
  its own completeness.

Output formats: text and JSON. No record bodies, note text, names, or
restricted content — identifiers, roles, models, and counts only.

### 2. Canonical docs page

`agf/docs/site/docs/<evidence-section>/release-role-participation.md`:

- explains what the map is, exactly how it is derived, and the range rule;
- states plainly what it does **not** show: it evidences that the process ran
  (Empirically validated), never that the governance model performs better
  than an alternative (that remains Theoretical / separately argued);
- links each release's committed snapshot.

Per `marketing-tone-rule` message-integrity, this page is the single canonical
source of the numbers.

### 3. Release-page / CHANGELOG block

A short derived block quoting the canonical page, e.g.:

> Produced across N coordination records by 7 roles — planner-architect ×X,
> coder-tester ×Y, contribution-auditor ×Z, infosec-sentinel ×W,
> capability-developer ×1, governor-reviewer ×V. Models: claude-sonnet-5,
> codex-gpt-5, human. [Full map](...).

It does not restate the numbers in different words; it links.

### 4. Committed snapshot

Each release commits its map output to an explicit dated evidence path
(`agf/docs/.../snapshots/<version>.json` or similar), per the AGENTS.md
telemetry-as-evidence guidance: document source, date, and confidence limits.

## Range rule

The release's coordination window is `<previous published release tag>..<this
release tag>` on the coordination channel. Abandoned candidates (e.g.
`v2.7.0-rc.1`) are not release points. The first real map derives from the
`v2.9.0..<next tag>` window, which already carries a dense multi-role,
multi-model trail (this session alone: contribution-auditor, infosec-sentinel,
planner-architect, coder-tester, capability-developer, governor-reviewer;
models claude-sonnet-5, codex-gpt-5, human).

## Anonymization and honesty

- Public surfaces (release page, Pages) use codename only. Roles are roles,
  never people; the human governor appears as `governor-reviewer: human`,
  never by name (`external-anonymization-rule`).
- Evidence-status wording is fixed: "these roles produced/reviewed N records
  in this range." No effectiveness claim (`verifiable-claims-rule`,
  `implementation-honesty-rule`).

## Scope boundary

This proposal ships the map as an **optional additive** artifact — a command,
a docs page, a CHANGELOG block, a committed snapshot. It is **not** proposed
as a required `release-via-merge-request-rule` step. Making it mandatory later
is a separate gate-mechanic change requiring its own Gate 1 and negative test
per `gate-self-test-rule.md`.

## Affected files (after Gate 1)

- `tooling/rust/` — the new `agov usage role-map` subcommand + tests.
- `agf/docs/site/docs/<evidence-section>/release-role-participation.md` — new
  canonical page; site nav entry.
- `CHANGELOG.md` / `agf/CHANGELOG.md` — a template line for the derived block.
- `docs/reviews/rule-projection/<plan-id>.yaml` — six-row projection screen
  **if** the docs page carries normative "a release should include this"
  wording; if it is purely descriptive evidence, the projection rule does not
  fire and that is recorded as the reason.
- release snapshot evidence path.

No schema, gate, CI-blocking, or release-metadata change. `agov-mcp` gains the
subcommand only if the adapter auto-exposes `usage` subcommands.

## Test strategy

- Deterministic output for a fixed coordination-range fixture (same input →
  byte-identical map).
- Anonymization test: the output contains no human names, no record bodies,
  no credentials, for a fixture that includes `model: human` records.
- Re-derivation reproducibility: running the command twice on the same range
  produces the same result; running it on `HEAD` of a branch vs the merged
  tag produces consistent role/model sets.
- Coverage honesty: the map's own "records parsed / legacy excluded" line
  matches `agov coord graph-index`.

## Risks and controls

- **Narrative drift / cherry-picking.** Control: the map is command output over
  a defined range, not hand-assembled. The docs page publishes the command and
  range so anyone re-derives it.
- **Overclaiming.** Control: fixed evidence-status wording; the page explicitly
  disclaims effectiveness.
- **Maintenance burden.** Control: it is a command, run once per release, not
  per-release prose.
- **Range ambiguity.** Control: the explicit `<prev tag>..<this tag>` rule;
  abandoned candidates excluded by definition.
- **PII leak on a public surface.** Control: the anonymization test in CI-run
  tooling tests; roles/models/counts only by construction.

## Rollback

Remove the subcommand, the docs page and nav entry, and the CHANGELOG template
line. Delete or retain snapshots at the operator's choice. No data or schema
migration.

## Why this belongs in the current RUW

- Small and additive: one command built on existing `agov coord`/`usage`
  primitives, one page, one CHANGELOG block. It does not compete with or block
  the !2405 universal-polling work or the Propeller line.
- The data already exists; implementation is close to Level 4 in one cycle
  because there is no new data model — only aggregation of fields the channel
  already stores.
- It is the single highest-leverage evidence artifact for AGF's core public
  claim, delivered exactly where an evaluator asks "does this project use its
  own framework?" — the release page.
- The first map has an unusually strong window to derive from: the
  v2.9.0→next coordination trail is multi-role, multi-model, and human-in-loop.
- If planner/coder capacity is tight against !2405, it can be a fast-follow
  MR inside the same RUW rather than a release blocker.
