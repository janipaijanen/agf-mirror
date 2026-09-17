---
kind: note
from: planner-architect
to: governor-reviewer
id: 20260910T050007Z-planner-architect-jh3g
timestamp: 2026-09-10T05:00:07Z
model: claude-sonnet-5
immutable: true
topic: pmm-docs-estate-capability-settled-as-v211-precondition-and-role
references-to:
  - 20260910T041405Z-planner-architect-asjp.md
  - 20260910T041359Z-planner-architect-et3p.md
  - 20260910T041223Z-qvzj.md
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

<!--
Filed by the claude-sonnet-5 planner-architect session as a bridge for a
binding governor decision delivered in-session (2026-09-10), per the
off-channel-directions-must-be-bridged discipline (htar / #122). The
decision text below is the governor's; the planner added the routing that
follows it. Governor: please ack or correct this record so it is binding
on-channel.
-->

# Bridge: governor settles the documentation-estate ("PMM") capability as a v2.11 precondition and a full role

## Governor decision (2026-09-10, delivered in-session — bridged here)

> "PMM as precondition, equal to coder-tester, planner-architect etc."

This settles the two open points in `asjp`:

1. **v2.11 scope**: the documentation-estate capability **IS a v2.11 IN
   workstream and a hard release-cut precondition** — `asjp` step S4's
   parent-scope amendment is authorized. Not the lighter changelog-only
   coupling.
2. **Shape**: it is a **standing MANIFEST agent role, of equal standing to
   `coder-tester`, `planner-architect`, and the other active roles** — not
   a rule / checklist. This is a direct foundry governor decision, the same
   authority basis by which the Improvement Propeller was activated; it
   overrides, for foundry, the rule-first calibration that learning `qvzj`
   recommends. `qvzj` remains a `scope: framework` proposal — its rule-vs-
   agent calibration still stands as guidance for adopters generally; the
   governor has made foundry's own call directly.

## Routing

### S1 (capability-developer) — narrowed to the agent contract

The shape question is now closed. Capability-developer's assessment
(`et3p`) narrows to the **agent contract**: mandate; phase and maturity at
introduction; the touchpoint manifest reconciled with
`docs/reviews/rule-projection/`; event-driven cadence classes; the
role's communication channels and gate interactions; the evidence
triggers; and — load-bearing — the **explicit boundaries** against
`documentation-writer` (authoring, currently skipped), `web-ux-reviewer`
(visual/UX/accessibility), `marketing-tone-rule` (tone + message-
integrity), and `fact-checker` (claim veracity). It owns **content
currency and cross-touchpoint consistency**, not authoring, visuals,
tone, or fact-checking.

### S2 (planner-architect / Claude) — after S1 returns

Draft the agent definition (`agf/agents/<name>.yaml`) plus its Gate 1
plan and decomposition, for governor Gate 1. Proposed working name:
**`documentation-estate-steward`** (the source proposal called it
"Product Marketing Touchpoint Keeper"; the governor refers to it as
"PMM"). Governor names it at Gate 1.

### S4 (planner-architect / Claude) — v2.11 parent-scope amendment

Amend `docs/plans/20260909T021841Z-v211-release-scope.md` to add the role
as an IN row and add "the steward's first estate-wide documentation
assessment is complete and every finding is dispositioned (fixed /
deferred with reason / referred)" as a named release-cut precondition,
the same shape as the `#129` withdrawal and `j8bq` amendments. Own
governor Gate 1 decision.

### S5 / S6

Fold the "estate assessment precedes freezing the metadata allowlist +
CHANGELOG scope" precondition into the v2.11 release-publication child
plan (MR !2646). Then, per the 2026-09-10 directive recorded in `dst6`,
the **Claude planner-architect session coordinates the v2.11 / RUW release
drive** to a governor Gate 2 and the separate tag decision, in
collaboration with Codex.

## Sequencing note

This adds a role introduction + implementation + a first full estate
assessment ahead of the v2.11 cut, alongside the IN workstreams that still
have no Gate 1 plan (coordination-semantics, #123/CP-C, `qbka`, CP-B-2).
v2.11 moves further out; that is the accepted consequence of making the
capability a hard precondition.
