---
id: 20261023T140050Z-rule-trigger-recall-corpus
status: pending-gate1
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
Gate 1 was approved in principle by the Human Governor in a live session on
2026-10-23 (bridge `20261023T140946Z-planner-architect-gmpg`). The audit found one WARN,
resolved below, so the plan stays `pending-gate1` until a governor-reviewer record
confirms this exact head.

## Decision requested

Gate 1 approval to add a durable, foundry-local evaluation surface under
`docs/reference/rule-trigger-recall/`: scenarios that test whether an agent opens
the right rule file when a situation arises, after rules were moved off the
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

Fixed: client version, model id, `claude -p ... --model <id>` with no tools, clean
session per trial, working copy at the exact head under test. Per scenario and
model: N trials with randomized paraphrase where the corpus provides one. Record the
raw answers as a dated local file or an MR artifact; the corpus and protocol, not the
transcripts, are committed. A scenario passes when the answer's path is in `accept`.

Two gates, named honestly: a **smoke gate** (any trigger edit): zero misses on every
`high` scenario at N=3; and a **claim gate** (before saying "recall >= 95%"): at least
20 trials per rule and model with fixed versions, reported as the lower bound of a
95% Wilson interval, plus the false-trigger rate on `negative` scenarios. Never run
paid model calls in blocking CI.

**Cost bound.** The claim gate is at least 20 trials x 13 rules x 2 models = 520 paid
model calls per run, before paraphrases, and about 1,040 if each rule keeps four scenario
kinds at N=20. It runs only when the governor asks for a recall claim, never on a trigger
edit and never automatically. The smoke gate is 13 rules x 4 scenarios x 3 trials x 1-2
models, roughly 156-312 calls, and runs on every trigger edit that touches a covered rule.

### Result schema (`result.schema.json`)

One JSON object per trial: `scenario_id`, `model`, `client_version`, `head_sha`,
`answer`, `pass`, `date_utc`. Foundry-local; not under `agf/schemas`.

## Acceptance

- Corpus parses; every `rule` path resolves on main; every scenario id is unique.
- The protocol runs end to end by hand on the current head, with its result file
  attached to the MR: current recall, the lower bound, and the false-trigger rate.
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
