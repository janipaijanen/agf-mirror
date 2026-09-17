<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Operating-model token/footprint efficiency — preliminary assessment

**Status:** preliminary, early-started under explicit governor authorization
("since we are now paused, you could progress it now") during a pause
where both AI sessions were token-constrained. This is **not** the full
assessment `pnqk` describes — that one needs v2.11 published/verified and
the queue-closeout executed, so the workflow being measured is in a
stable, finished state rather than mid-flight. This document is evidence
and candidate directions gathered from what is already observable, offered
as an input to that later assessment, not a substitute for it.

**Evidence status:** the measurements below are **empirically informed**
(drawn from this project's own single session/history) unless marked
otherwise — not independently corroborated across adopters or sessions.
The proposals in "Candidate directions" are **theoretical** — reasoned
from the evidence, not yet tried.

## A. Token-economy evidence gathered now

| Observation | Measurement | Source |
|---|---|---|
| Cold-start fixed cost | `AGENTS.md` alone is 581 lines; `.claude/rules/` totals 4,745 lines; `agf/rules/` totals 5,967 lines | `wc -l` on this checkout, 2026-09-11 |
| Coordination-channel growth | 3,789 `docs/coordination/*.md` files exist; average ~70 lines/record | `find` + `wc -l`, 2026-09-11 |
| Multi-round review overhead (case study) | The QC-1 disposition ledger alone went through **5 independent-review rounds** (`6pms` → `gsat` → `qsmq` → a Claude review `pnmm` → `a585`) before converging, each round a full coordination-record read/write/re-verify cycle | this session, 2026-09-10 |
| Bridge-record overhead (case study) | The "conversational governor decision → must be transcribed into an on-channel record before it's binding" pattern (`POLL-G1`) produced roughly 10 additional coordination records in one session purely to make already-made decisions durable | this session, 2026-09-10/11 |
| Existing zero-token mechanisms | `agov coord validate`, `plan validate`, `publication-check`, `projection-check` are already deterministic Rust — no LLM call needed for what they check | `tooling/rust/crates/agov*` |

**Reading:** a meaningful share of this session's token cost was **governance ceremony** (re-stating an already-made decision in the required record shape, re-reading full files to extract one changed line, running a full review round to catch a formatting/consistency defect a mechanical check could have caught) rather than judgment. The `agov` layer already carries the purely mechanical checks; the ceremony that remains is largely in the *coordination and review protocol itself*, not in individual task execution.

## B. Footprint/model-size evidence and reasoning

No experiment has been run; this is reasoning from task shape, not a
measured result.

Coordination and governance tasks observed this session fall into three
capability tiers:

1. **Mechanical / deterministic** — schema validation, publication
   range-checks, hash verification, command-inventory diffing. Already
   zero-LLM (`agov`). No model-size question here at all.
2. **Templated drafting** — filling a coordination-record shape from a
   known trigger (an ack, a routing note, a classification, a status
   transition), where correctness is **objectively, mechanically
   checkable** (`agov coord validate` / `plan validate` either accept the
   YAML frontmatter and schema or reject it). This is the natural first
   target for a smaller/quantized model: the safety net already exists
   independent of model quality — a malformed draft never reaches the
   channel, it just fails validation and gets redone.
3. **Judgment-heavy** — Gate 1/2 review substance, cross-cutting
   consistency findings (the DES-1..4 estate-sweep findings, the R56-E3
   sibling-discovery gap, the `p2rk` two-plan schema-field collision),
   architecture decisions, drafting a plan's risk/rollback reasoning.
   These required multi-file context, inference across documents that
   don't share an obvious lexical link, and correcting course after a
   reviewer's counter-argument — the profile large-context frontier
   models are suited for. No evidence yet that a small/quantized model
   holds up here; this is the open question, not a settled negative.

## Candidate directions (theoretical, unvalidated)

**Token economy:**
- Tiered loader: a short "core" `AGENTS.md` loaded always, with
  role-specific and rule-category appendices loaded on demand rather than
  one ~600-line file plus ~10,000 lines of rules referenced by every
  session regardless of role.
- Batch coordination records more aggressively by default (already
  practiced ad hoc this session after governor feedback) rather than
  one-record-one-MR as the default assumption.
- A bounded review-round ceiling: after N independent-review rounds on
  the same artifact with only bounded/mechanical findings remaining,
  escalate to a governor decision rather than another AI round. `agov
  coord convergence` (Level 4 Validated per the v2.8.0 CHANGELOG) already
  exists for exactly this and may be under-used — worth checking whether
  it was available to apply to the QC-1 review loop.

**Footprint minimization:**
- Route capability-tier-2 tasks (templated, mechanically verifiable
  drafting) through a smaller/local/quantized model **behind the same
  `agov`-mediated interface**, with `agov coord validate` / `plan
  validate` as the existing, zero-additional-cost safety net.
- Keep capability-tier-3 tasks (judgment, cross-cutting review, Gate 1/2
  substance) on frontier models until there is direct evidence a smaller
  model matches reliability — this is a claim to test, not assume either
  way, per `implementation-honesty-rule.md`.

## Proposed first concrete, bounded experiment

Does not require v2.11 or the queue-closeout to be done — it can start
whenever the governor authorizes it, independent of the RUW trigger:

**A quantized/small-model pilot scoped to tier-2 drafting only** —
mechanical coordination-record drafting (acks, routing notes,
classification records) where `agov coord validate` is the objective
accept/reject gate. Measure: draft-acceptance rate on first pass, tokens
consumed per accepted record versus a frontier-model baseline, and any
governance-relevant error class a validator doesn't catch (e.g. a
factually wrong routing target that is still schema-valid). This is safe
because a malformed draft cannot reach the channel; it is measurable
because `agov` already gives a pass/fail signal per attempt.

## What the full (triggered) assessment still needs

- A stable, finished rows-4–7 workflow shape (not mid-slice-implementation)
  to classify a realistic task-type distribution across a full release
  cycle, not one session's activity.
- `documentation-estate-steward`'s own touchpoint-manifest method could be
  reused to inventory the *coordination* estate's growth rate, not just
  the documentation estate's currency.
- Real numbers from the tier-2 pilot above, if run, rather than the
  task-shape reasoning in section B.

## Explicitly not concluded

This document does not conclude that any current role, rule, or gate
should be removed, weakened, or replaced. It identifies where ceremony
cost is concentrated and names one safe, bounded experiment. Any actual
change to gate mechanics, role definitions, or the coordination protocol
remains a full Gate 1 decision under `gate-self-test-rule` and the
existing calibration rules, exactly as today.
