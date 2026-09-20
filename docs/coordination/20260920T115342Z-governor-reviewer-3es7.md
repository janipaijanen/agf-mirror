---
kind: decision-record
from: governor-reviewer
to: all
id: 20260920T115342Z-governor-reviewer-3es7
timestamp: 2026-09-20T11:53:42Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260920T113823Z-planner-architect-7sb5.md
requires-ack: false
topic: governance-mode-classification-gate1-approved
artifacts_owned:
  - docs/plans/20260920T113336Z-governance-mode-classification.md
---

<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

<!--
Filed into the coordination channel by the claude-sonnet-5 session at the
governor's explicit request ("proceed as proposed and include the
statements above", 2026-09-20, live session), in the same transcription
pattern as q862/8274/rhgk: the YAML front matter and this header were
added by the assistant for schema validity; the decision itself is the
governor's.
-->

# Gate 1 approved: LIGHT/NORMAL/STRICT governance-mode classification plan

The governor-reviewer approved Gate 1 for the plan
`docs/plans/20260920T113336Z-governance-mode-classification.md`
(MR !3074, `plan/governance-mode-classification`) in the active session,
at exact head `b6d674e2c03d0688309603b95186605fac276a27`, whose canonical
MR pipeline (id `2865239755`) passed.

**Resolution of the plan's one flagged open question:** the governor
adopted option 2 (defer). The contractual/compliance/legal/externally-
binding-deliverable STRICT trigger — the governor's own "contractual ERP
requirements baseline" example — is **not** part of this plan's STRICT
list. It is routed to a named fast-follow plan that first customizes
`high-risk-operations-rule.md` for AGF-NG's actual domain. This plan's
STRICT list is exactly `high-risk-operations-rule.md`'s current areas
plus `downstream-impact-rule.md`'s BREAKING trigger plus
`gate-self-test-rule.md`'s gate-mechanics carve-out — nothing more.

**Basis for this resolution:** the governor asked the session what its
own agentic roles (planner-architect, contribution-auditor, build-
engineer, coder-tester) would propose on this exact question. All four
converged on deferring, for four independent reasons (branch-sizing
discipline; this project's own evidence-before-MUST pattern for
provisional rules; the observation that mechanical CI enforcement, not
rule text, is where real "machine speed" gets delivered; and keeping
mechanically-checkable criteria separate from subjective ones). The
governor then directed "proceed as proposed and include the statements
above" — adopting that synthesis and directing it be recorded verbatim in
the plan, which the immediately-preceding plan commit (`b6d674e2`) does.

**advisory_consultation disposition, both accepted as non-blocking for
this Gate 1:**
- `infosec-sentinel`: `not-applicable` — no learning-record or credential
  surface in this diff.
- `contribution-auditor`: `waived` — same-actor limitation disclosed (this
  session authored both the plan and the contribution-auditor-lens
  statement within it; Codex, the usual different-model player, is
  dormant until ~2026-09-23). The governor was told this explicitly before
  directing "proceed as proposed." A durable independent contribution-
  auditor re-review by a different model remains available once Codex
  returns and does not require reopening this Gate 1 decision unless it
  surfaces a blocking finding.

**Named fast-follow plans** (not scheduled or Gate-1-approved by this
decision — future work items only):
1. Customize `high-risk-operations-rule.md` for AGF-NG's actual domain
   and reconsider the contractual/compliance STRICT trigger there, with a
   Provisional-status evidence header per this codebase's existing
   pattern.
2. Wire LIGHT/NORMAL/STRICT mode into CI path-scoping per
   `ci-trigger-scoping-rule.md`'s pattern, as its own gate-mechanics
   change with its own Gate 1 and `gate-self-test-rule.md` negative test.

Implementation of the approved plan (editing
`agf/rules/workflow/simplicity-rule.md` and its `.claude/` loader
projection) may now proceed under the ordinary coder-tester workflow.
