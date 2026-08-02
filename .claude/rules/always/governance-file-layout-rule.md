<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Governance File Layout
#
# Framework principle: Artifact Ownership
# See: ADOPTING.md — "Confirm the layout"
# Evidence: sage-review created root-level governance dirs twice by
#   following ADOPTING.md paths literally in a subdirectory layout.
# Corrected 2026-08-01: this rule previously offered a choice between two
#   layouts ("Layout A: framework-as-project" / "Layout B: framework-as-
#   subdirectory") via a commented-out CUSTOMIZE block. That choice is not
#   real — AGF v2.0 has supported exactly one layout since v2.0.0, stated
#   explicitly in ADOPTING.md ("AGF v2.0 supports one adoption layout") and
#   in install-and-layouts.md ("AGF v2.0 uses one supported layout"). The
#   dual-option block was stale leftover framing, not a live adopter
#   choice, and is removed below.
# Revised 2026-08-02 (Codex review, MR !916 on a related plan): the
#   ownership table's `agf/agents/*.yaml` row states the same manifest
#   path issue #79's own Gate 1 plan (docs/plans/v250-rc1-contract-core.md)
#   decides. That plan is the authority for this row, not this rule
#   independently. All other rows restate long-standing convention already
#   published in ADOPTING.md, unaffected by #79.

AGF v2.0 supports exactly one layout: the framework lives inside the
project as the visible `agf/` directory. Governance-artifact placement is
not directory-based (there is no separate "governance root" outside
`agf/`) — it is ownership-based, per ADOPTING.md's "Confirm the layout"
table:

| Artifact | Path | Ownership |
|---|---|---|
| Learning records | `agf/learnings/<codename>.yaml` | Adopter-owned |
| Agent definitions / role roster | `agf/agents/*.yaml` | Adopter-owned pending #79\* |
| ADRs and governance docs | `agf/docs/adr/` | Adopter-owned |
| Framework rules | `agf/rules/` | Upstream-owned |
| Working plans | `docs/plans/` | Adopter-owned |
| Coordination channel | `docs/coordination/` | Adopter-owned |
| `GOVERNANCE.md` | Project root | Adopter-owned |

\* This row's canonical path, precedence, and migration behavior are
decided by issue #79's own Gate 1 plan
(`docs/plans/v250-rc1-contract-core.md`), not asserted independently by
this rule — this row restates that plan's decision, not the other way
around. If that plan's decision changes before Gate 1, update this row to
match; do not treat this row as settling the question first.

Rules:
- Create `learnings/`, `agents/`, and `docs/adr/` content under `agf/`, at
  the paths above — never at the project root and never under an invented
  alternate directory.
- `.claude/rules/` stays at the project root (it is a tool config, not a
  governance artifact).
- `GOVERNANCE.md` lives at the project root, not under `agf/` — it is the
  adopter's own document, not shipped framework content.
- Some paths under `agf/` are adopter-owned despite living inside the
  upstream-managed tree (learnings, agent definitions, ADRs) — do not
  assume everything under `agf/` is safe to overwrite on a framework
  upgrade. See issue #79 (upgrade-safe adopter role authority) for the
  tooling contract that makes this distinction mechanically enforced
  rather than documentation-only.
