---
provenance: verified-agf-v2
---
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

AGF v2.0 supports exactly one layout: the framework lives inside the
project as the visible `agf/` directory. Governance-artifact placement is
not directory-based (there is no separate "governance root" outside
`agf/`) — it is ownership-based, per ADOPTING.md's "Confirm the layout"
table:

| Artifact | Path | Ownership |
|---|---|---|
| Learning records | `agf/learnings/<codename>.yaml` | Adopter-owned |
| Agent definitions / role roster | `agf/agents/*.yaml` | Adopter-owned |
| ADRs and governance docs | `agf/docs/adr/` | Adopter-owned |
| Framework rules | `agf/rules/` | Upstream-owned |
| Working plans | `docs/plans/` | Adopter-owned |
| Coordination channel | `docs/coordination/` | Adopter-owned |
| `GOVERNANCE.md` | Project root | Adopter-owned |

Rules:
- Create `learnings/`, `agents/`, and `docs/adr/` content under `agf/`, at
  the paths above — never at the project root and never under an invented
  alternate directory.
- Framework rules stay under `agf/rules/`. Tool-specific root files (e.g.
  a `CLAUDE.md`/`AGENTS.md` loader) may point assistants at those rules,
  but must not duplicate the AGF rule tree.
- `GOVERNANCE.md` lives at the project root, not under `agf/` — it is the
  adopter's own document, not shipped framework content.
- Some paths under `agf/` are adopter-owned despite living inside the
  upstream-managed tree (learnings, agent definitions, ADRs) — do not
  assume everything under `agf/` is safe to overwrite on a framework
  upgrade. See issue #79 (upgrade-safe adopter role authority) for the
  tooling contract that makes this distinction mechanically enforced
  rather than documentation-only.
