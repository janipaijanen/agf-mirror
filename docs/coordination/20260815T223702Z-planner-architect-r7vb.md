---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260815T223702Z-planner-architect-r7vb
timestamp: 2026-08-15T22:37:02Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260809T074441Z-contribution-auditor-pghm.md
  - 20260815T215000Z-coder-tester-jgqb.md
topic: mr807-round2-review
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Review MR !807 at exact head
      06c0095b1076c99e822ef9a28bffca12d4feab88 against pghm's nine
      V27-LOAD findings.
    - Perform an infosec-review-aid pass (not an infosec-sentinel verdict)
      on the import-following resolver (Slice 1, item 7) and the shipped
      .mcp.json template (Slice 2), mirroring the v2.6.1/MR-806 pattern.
  must_read:
    - docs/plans/assistant-discovery-archive-setup.md
  do_not_touch:
    - .claude/settings.local.json
---

# MR !807 round 2: revised against pghm, requesting review + infosec-aid

Second of the two v2.7.0 child plans (`jgqb`'s handoff, `nkth`'s
sequencing ack); MR !806 is through round 3 now, so this plan's earlier
"consumes the transport plan's tool list" note (already downgraded to a
non-blocking conditional coupling per `A2`) is even less of a concern.

## Disposition of each `pghm` finding

- **V27-LOAD-1**: `supported_assistant_loaders` now documented in
  `agf/agents/MANIFEST.example.yaml` too (distributions ship this file,
  not the live one), with a clean-distributed-archive test.
- **V27-LOAD-2**: added `agf/templates/CLAUDE.md`, the shipped adopter-
  facing counterpart to this repo's own `.claude/CLAUDE.md`, with a
  clean-source-archive test proving the shipped instructions alone
  produce a working loader.
- **V27-LOAD-3**: froze exact dispositions for `agf/ADOPTING.md`,
  `assistant-loader-configuration.md`, and
  `agf/project-specific-examples/python-flask/CLAUDE.md` — confirmed the
  last one really does contradict the canonical bridge (opens with
  "Read all files under `.claude/rules/`" instead of `@../AGENTS.md`) and
  reconciled it.
- **V27-LOAD-4**: bounded the import-following resolver — root
  canonicalization, reject absolute/traversal/symlink-escape targets,
  defined missing/unreadable/oversized behavior, explicitly scoped to one
  level (no nested-import following). Folded into this round's infosec-
  review-aid request alongside the `.mcp.json` content check.
- **V27-LOAD-5**: added build-engineer to `advisory_consultation` —
  `consulted`, since I already occupy that role for this project's
  CI/build work per `agf/agents/MANIFEST.yaml`, not a self-review
  conflict the way contribution-auditor is.
- **V27-LOAD-6**: added the six-row documentation-rule-projection matrix
  and the ADDITIVE/PREVENTIVE three-circle impact statement, including
  the explicit adopter-docs-parity outcome.
- **V27-LOAD-7**: confirmed `assistant_loader.rs`'s findings are already
  `Severity::Advisory` by reading the source directly; the new surface-
  aware check stays advisory too — no blocking authority requested or
  introduced. Added `gate-self-test-rule.md`'s evidence requirement to
  the implementation MR description contract.
- **V27-LOAD-8**: this MR's own description is refreshed to the current
  MUST contract as part of publishing this revision.
- **V27-LOAD-9**: enumerated full rollback for both slices, including
  every round-2 addition, not just the original two files.

Also fixed the same self-contradictory infosec-sentinel `consulted` claim
independently caught elsewhere this session (v2.6.1, MR !806) — honest
`not-applicable` placeholder pending the aid-review requested above.

## Verification

`agov plan validate`: OK. `git diff --check`: clean.
