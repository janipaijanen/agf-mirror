---
id: 20261023T140050Z-rule-trigger-recall-corpus
status: approved
gate1_approved_by: 20261023T144257Z-governor-reviewer-zc0p
gate1_approved_at: 2026-10-23T14:42:57Z
topic: rule-trigger-recall-corpus-and-protocol
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >
      `agov validate infosec` passes; the plan adds prompts and paths only. Summarized in
      contribution-auditor record 20261023T140948Z-contribution-auditor-shqm.
  - role: contribution-auditor
    status: consulted
    reason: >
      Audit by the AI side of the role (the plan's own author), record
      20261023T140948Z-contribution-auditor-shqm: no blocking finding, one WARN (claim-gate
      cost bound, fixed in this revision) and two INFO (classification, hold-out provenance,
      both fixed). The governor may pull or redo it.
affected_paths:
  - docs/reference/rule-trigger-recall/corpus.yaml
  - docs/reference/rule-trigger-recall/protocol.md
  - docs/reference/rule-trigger-recall/result.schema.json
  - docs/reviews/rule-projection/20261023T140050Z-rule-trigger-recall-corpus.yaml
depends_on:
  - 20261022T205113Z-instruction-context-reduction
  - 20261023T080431Z-instruction-context-followup
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: a rule-trigger recall corpus and manual protocol

Follow-up to issue #138. Scope agreed with coder-tester (Codex) in the 2026-10-23
experiment debate (ACS seq 184 and 186): a committed declarative scenario corpus, a
short manual protocol and a result schema; **no runner**. Draft by planner-architect.
Gate 1 was approved by the Human Governor for this exact content (head
`c676b2fe`) in governor-reviewer decision record
`20261023T144257Z-governor-reviewer-zc0p`, after the audit corrections required by `251g`.
Gate 2 and merge of the implementation stay with the Human Governor.

## Decision requested

Gate 1 approval to add a durable, foundry-local evaluation surface under
`docs/reference/rule-trigger-recall/`: scenarios that test whether an agent selects
the right rule file (route selection) and, in a smaller check, actually opens it, when a situation arises, after rules were moved off the
always-load path. This is documentation and data; it changes no rule, loader or
tooling. It is a Gate 1 item because it creates a durable evaluation surface that
later MRs will cite as acceptance evidence.

## Why

The size work moved 31 rules behind trigger lines. Today's evidence that the triggers
work is ad hoc: about 30 haiku/sonnet samples, 2 misses (challenge rule, improvement
rule), both fixed by rewording. A change to any trigger line can silently regress
recall, and three samples cannot support a 95% claim (one miss in six is 83.3%).

## Design

### Corpus (`corpus.yaml`)

Declarative list; each scenario has: `id`, `rule` (expected repo path or `NONE`),
`kind` (`direct`, `paraphrase`, `near-boundary`, `negative`), `consequence`
(`high` for security, gate, release, delegation and coordination-integrity rules;
`low` otherwise), `prompt`, `accept` (list of acceptable paths), and `author` (`planner-architect` or
`coder-tester`; hold-out scenarios carry `author: coder-tester` so the separation is
auditable). Four scenarios
per rule for the first slice of 13 high-consequence or recently-missed rules: release,
deputy-governor, documentation-rule-projection, deprecation, ci-trigger-scoping,
merge-request-workflow, immutable-tags, coordination-challenge-response,
coordination-id, feature-regression-guard, debugging, governance-file-layout,
agent-improvement-proposal. About 52 scenarios. A `negative` scenario names a
situation where the rule must **not** be chosen (false-trigger control).

Hold-out: one quarter of the scenarios are authored by Codex and marked `holdout:
true`; their text is not used when editing trigger lines, to limit overfitting
(Goodhart).

### Protocol (`protocol.md`)

**What is measured.** Two separable properties, each named for what it tests:

- **Route selection** (the default): `claude -p ... --model <id>` with no tools. The agent
  is asked which rule file it would read first. This tests whether the always-loaded
  trigger text points to the right file. It does **not** test that the agent opens it;
  every claim, pass criterion and result file says "route selection".
- **Actual open** (a small separate check): enable only the Read tool and record the first
  Read target from `--output-format stream-json`. A trial passes when that observed path is
  in `accept`. Run on a smaller sample because it costs more per call; a claim about
  "opens the rule" may use only this check.

Fixed for both: client version, model id, clean session per trial, working copy at the exact
head under test. Record the raw answers or stream as a dated local file or an MR artifact;
the corpus and protocol, not the transcripts, are committed. A scenario passes when the
answer's (or observed) path is in `accept`.

**Units and denominators.** The unit is one `(rule, model)` pair. Positive recall counts
the `direct`, `paraphrase` and `near-boundary` trials of that rule; the false-trigger rate
counts the `negative` trials of that rule separately, with its own denominator. They are
never pooled.

**Actual-open sample (fixed, small).** 5 rules (release-via-merge-request, deputy-governor,
merge-request-workflow, immutable-tags, coordination-challenge-response) x 3 scenarios
(one `direct`, one `paraphrase`, one `negative`) x 3 trials x 1 model = **45 calls** per
run, only the Read tool enabled, a read-only working copy. A positive trial passes when
`first_read_target` is in `accept`; a negative trial passes when it is not. Reported as
smoke results, not as a rate with a bound.

**Smoke gate** (any trigger edit that touches a covered rule): zero misses on every `high`
scenario at N=3 per scenario. It is a regression screen, not a statistical claim.
**Implementation acceptance is smoke-only.** The claim gate runs only if the governor
separately requests it.

**Claim gate** (before saying "route-selection recall is at least 95%"): the lower bound of
a 95% Wilson interval per unit must reach 0.95. With zero failures that needs **n >= 73**
trials per unit (73/73 gives 0.9500; 72/72 gives 0.9493; 20/20 gives only 0.8389 and 60/60
only 0.9398). One allowed failure needs n >= 110, two need 142, three need 173. The same
n >= 73 applies to the negative-trial denominator for a false-trigger rate of at most 5%.
Never run paid model calls in blocking CI.

**Cost bound.** Smoke gate: 13 rules x 4 scenarios x 3 trials = 156 calls per model, 312
for two models, per run. Claim gate, zero-failure design: 13 rules x 73 trials x 2 models =
1,898 positive calls plus 1,898 negative calls = **3,796 paid calls per claim run**; with one
allowed failure per unit, 13 x 110 x 2 x 2 = 5,720. The claim gate runs only when the governor
asks, never on a trigger edit and never automatically.

### Result schema (`result.schema.json`)

One JSON object per trial. Required: `scenario_id`, `rule`, `kind`, `model`,
`client_version`, `head_sha`, `measurement_mode` (`route-selection` or `actual-open`),
`tools_enabled` (array; empty for route selection, `["Read"]` for actual open), `answer`
(path the agent named, or `NONE`), `pass`, `date_utc`. `first_read_target` (string or null)
is **required when `measurement_mode` is `actual-open`** and forbidden otherwise: the path
of the first Read tool call taken from `--output-format stream-json`, `null` if the agent
made no Read call (which fails the trial). The schema makes route-selection and actual-open
results impossible to confuse. Foundry-local; not under `agf/schemas`.

## Acceptance

- Corpus parses; every `rule` path resolves on main; every scenario id is unique.
- An actual-open protocol smoke on the current head: 3 scenarios x 1 trial, one `direct`,
  one `paraphrase`, one `negative`, with a result file in which every record validates
  against `result.schema.json`, `first_read_target` is populated from stream-json, and a
  deliberately malformed record (`actual-open` without `first_read_target`) fails
  validation.
- The route-selection protocol runs end to end by hand on the current head, with its result file
  attached to the MR: current route-selection recall, the lower bound, and the false-trigger rate, reported as
  smoke-gate results only; a claim-gate run is out of scope unless the governor requests it.
- Codex authors the hold-out quarter and signs the protocol as reproducible.

## Classification and impact

STRICT for review purposes: documentation and a data file under `docs/reference/`, but later
MRs will cite the corpus as acceptance evidence, so it is reviewed like gate evidence.
No rule, loader or tooling change.
```
Impact:
  Foundry:          adds a durable evaluation surface; no behavior change.
  Direct adopters:  none; not under agf/.
  Downstream:       none.
  Migration:        not required.
  Evidence:         issue #138; Codex debate seq 184 and 186; today's 2 misses in ~30 samples.
```
Adopter docs parity: not applicable.

## Risks and rollback

- R1: Goodhart: triggers get tuned to the corpus. Mitigation: hold-out quarter authored
  by another agent, not read when editing triggers.
- R2: the corpus rots as rules change. Mitigation: path-resolution check in the MR,
  re-run the smoke gate after any trigger edit.
- R3: small samples dress up as statistics. Mitigation: the two named gates.
- R4: model-version drift changes results. Mitigation: record model id and client
  version; do not compare runs across versions without a re-baseline.

A runner is explicitly deferred until the corpus has been used twice and its input
contract is stable. Rollback: delete the directory; nothing depends on it.
