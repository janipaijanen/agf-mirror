---
kind: decision-record
from: governor-reviewer
to: all
id: 20260827T192914Z-governor-reviewer-hsj8
timestamp: 2026-08-27T19:29:14Z
model: human-directed-claude-sonnet-5
immutable: true
references-to:
  - 20260827T190630Z-planner-architect-n78m.md
  - 20260827T184034Z-coder-tester-qghk.md
  - 20260827T190624Z-planner-architect-gs47.md
  - 20260827T124803Z-governor-reviewer-rpam.md
topic: v270-rc1-gate1-approval
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Binding Gate 1 approval: v2.7.0-rc.1 release-publication plan

This record transcribes the governor-reviewer's in-session decision,
"Gate 1 approved", given in response to the decision request
`20260827T190630Z-planner-architect-n78m.md`.

## Approved

**Gate 1 is approved** for
`docs/plans/20260827T160105Z-v270-release-publication.md` (revision r4)
at exact head `2ea2f5745186e2730da9138be740a0779632e093`, MR !2155.

This authorizes the **release candidate only**: construction of
`v2.7.0-rc.1` by frozen-scope reconstruction from the peeled `v2.6.1`
commit `f780ac0ea37061e0ad3ee6fb1f00c6958b996fc0`, its publication, its
publication-verification, and the 48-hour verified hold, carrying exactly
the frozen scope `8hgm` names (issues #86, #87, #88, #90).

Stable `v2.7.0` promotion is **not** authorized by this approval. It
requires a separate binding exact-candidate record naming the accepted
RC's tag object, peeled commit, and tree, plus its own Gate 1-approved
publication plan, both produced after the hold completes.

Supported by independent review `qghk` (coder-tester, gpt-5): CLEAR for
Gate 1, no r5 plan round required, with the plan's inventory hashes and a
clean-room cherry-pick replay independently reproduced.

## Dispositions applied, and the interpretation behind them

The in-session decision was given as an unqualified approval. The two
subsidiary questions in `n78m` were not separately answered, so the
following dispositions apply. **Either may be corrected by a superseding
governor record before construction begins**; neither creates anything
immutable on its own.

1. **The two execution clarifications are NOT bound at Gate 1.** They
   therefore stand as `qghk`'s own stated default: **mandatory Gate 2
   corrections, required before any immutable tag or release object is
   created.** This is the safer route here, because the `#86` changelog
   figure is still contested — planner-architect reads seven added coord
   tools including `convergence` (`gs47`, evidenced from the registry
   diff and `98c491d0`), `qghk` said six. Binding a contested figure at
   Gate 1 would be worse than resolving it at Gate 2. The second
   clarification (replace r4's unsupported "five independent executions"
   with only executions holding a command/result record, and require
   Gate 2 to perform and record its own exact-base replay) is
   uncontested and equally binding at Gate 2.

2. **Both `advisory_consultation` dispositions are ratified.** The plan's
   own frontmatter states that the governor "ratifies or rejects this
   substitution as part of the Gate 1 decision", so approving the plan at
   this head ratifies them:
   - **contribution-auditor** — no separate pass; the independent
     coder-tester reviews (`rshs`, `rn1g`, `12ak`, `qghk`; codex-gpt-5,
     a different model and session from planner-architect) are accepted
     as the substitute independent check, on the same basis accepted for
     v2.6.0 (`kpkk`, `33br`).
   - **infosec-sentinel** — the plan-specific semantic-consultation
     waiver is granted for this exact plan head. The configured CI
     actor's mechanical checks (secret scanning, dependency CVEs, SPDX)
     remain mandatory throughout and are not waived.

## What this approval does not authorize

No release branch, protected ref, tag, GitLab Release, generic package,
OCI artifact, protection change, or namespace action is authorized by
this record beyond what the approved plan's own ordered procedure
specifies, and every step in that procedure remains subject to its
stated stop conditions and to binding exact-head Gate 2 approval of the
release MR.

Unchanged and still outside this approval: the rule-proportionality
amendment `qghk` scoped (a separate, later, separately-governed work
order), and the release-roadmap location and clustering proposal
`qsp1`, which is still with coder-tester for review.

## Next action

Construction passes to **coder-tester**, per the approved plan's staffing
section. Planner-architect updates the plan's `status`,
`gate1_approved_by`, and `gate1_approved_at` in this same review cycle,
citing this record, per `AGENTS.md`'s requirement that plan approval
metadata not be deferred.
