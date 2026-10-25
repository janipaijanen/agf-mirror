---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20261025T192135Z-planner-architect-2gpj
timestamp: 2026-10-25T19:21:35Z
model: claude-sonnet-5-5
immutable: true
references-to:
  - 20261025T191011Z-planner-architect-k6sp.md
topic: supersedes-adopter-docs-gate1
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Gate 1 decision on plan supersedes-adopter-docs (MR !3217, exact head d2563a4a810afb380ad4a722c5d096261279bf06)
  must_read:
    - docs/plans/20261025T192020Z-supersedes-adopter-docs.md
    - docs/reviews/rule-projection/20261025T192020Z-supersedes-adopter-docs.yaml
  do_not_touch:
    - .claude/settings.local.json
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 1 handoff: document the coordination `supersedes` field

Requested by codex-foundry (ACS seq 487, advisory room, no authority). It resolves the `Adopter docs: GAP` found in the Gate 2 handoff `20261025T191011Z-planner-architect-k6sp` for MR !3215, which stays unchanged.

## What is ready

- Plan `20261025T192020Z-supersedes-adopter-docs`, status `pending-gate1`, branch `plan/supersedes-adopter-docs`, MR !3217, exact head `d2563a4a810afb380ad4a722c5d096261279bf06`.
- It chooses documentation over a waiver: a contract paragraph in `coordination-publication-rule.md`, an explanation and example in `coordination-protocol.md`, and one changelog line after !3215 merges. Schema, tooling and tests are not touched.
- Projection manifest included; six rows disposed (runbook row: not applicable, with reason).

## Evidence

- `agov plan validate` OK; `agov plan projection-check --allow-planned` OK; collision-check 0 unacknowledged; `agov validate infosec` OK; `git diff --check` clean. Local binary: main checkout release build of 2026-10-22.
- Finding behind it: the schema description points to rule text that never names the field; the conditions exist only in `supersession.rs`.
- Exact-head MR pipeline: see MR !3217 (the push pipeline was skipped by design).

## Open items for the decision

- Advisory rows `infosec-sentinel` and `contribution-auditor` are `pending`; Gate 1 needs both resolved in the approved plan.
- Wording fidelity against `supersession.rs` is a human review item.

## Not done

No rule, page, schema or changelog edit; no release, tag or deploy. This handoff is not an approval.
