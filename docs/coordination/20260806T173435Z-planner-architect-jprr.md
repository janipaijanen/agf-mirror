---
artifacts_owned:
- docs/plans/20260806T122121Z-get-started-commissioning-ux-v1.md
from: planner-architect
handoff:
  do_not_touch:
  - .claude/settings.local.json
  - the slice-3 navigation restructure (all pages except one new mkdocs.yml
    entry for the new commissioning page)
  must_read:
  - docs/plans/20260806T122121Z-get-started-commissioning-ux-v1.md
  - docs/coordination/20260806T162915Z-governor-reviewer-bqdk.md
  next_agent: governor-reviewer
  status: ready
  task:
  - "Renewed Gate 1 review of the plan revision resolving GS-PLAN-R1 through R5."
id: 20260806T173435Z-planner-architect-jprr
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260806T162915Z-governor-reviewer-bqdk.md
- 20260806T122415Z-planner-architect-tsj7.md
requires-ack: true
timestamp: 2026-08-06T17:34:35Z
to: governor-reviewer
topic: get-started-commissioning-ux-plan-r2
---

# Plan revised: all five `bqdk` findings resolved

`docs/plans/20260806T122121Z-get-started-commissioning-ux-v1.md` revised in
place (still `status: pending-gate1`). `agov plan validate` passes
(`severity: blocking`, OK). Finding-by-finding resolution:

- **GS-PLAN-R1** (bind scripted-check paths): named three new files exactly —
  `agf/docs/build/hooks/check_commissioning_first_screen.py` (DOM-order +
  prompt-before-manual-command assertions), `check_commissioning_links.py`
  (link integrity, external URLs HTTP-verified end-to-end), and
  `tooling/tests/test_commissioning_postconditions.py` (fixture
  postconditions), plus two new fixture directories under
  `tooling/fixtures/commissioning/`. No "location TBD" language remains.

- **GS-PLAN-R2** (canonical prompt ownership): decided —
  `commission.md` is the single canonical prompt source;
  `install-and-layouts.md`'s existing Step 2 is replaced with a pointer and
  the page renumbered. No shared-include mechanism introduced (out of scope
  for this plan); no implementation-time choice left open.

- **GS-PLAN-R3** (AGOV installation procedure): fully specified against the
  live GitLab Releases API for `v2.5.0-rc.2`, not invented — exact asset
  naming (`agov-<version>-<platform>.tar.gz` + `.sha256` +
  `-provenance.json`), the honest platform limitation (Linux x86_64/aarch64
  only today, no macOS/Windows binary published), `sha256sum -c`
  verification, two documented installation destinations
  (`~/.local/bin/agov` or project-local `tooling/agov`), PATH exposure, and
  the exact verification commands (`agov version`, `agov session resume`,
  both confirmed against this repository's actual built binary).

- **GS-PLAN-R4** (reproducible acceptance): split into Class A
  (deterministic, CI-repeatable — fixture postconditions against two
  committed before/after fixture pairs, link integrity, rendered
  first-screen DOM order, tutorial-gating) and Class B (one-time AI-execution
  evidence — a named coding assistant runs the actual prompt against
  fixture-seeded scratch repositories, captured as Gate 2 evidence, not
  re-run in CI). The two classes are no longer conflated.

- **GS-PLAN-R5** (existing-install ownership semantics): added an
  "Ownership authority" section citing `agf/docs/governance/manifest.md`'s
  Contents table and `.claude/rules/always/governance-file-layout-rule.md`'s
  ownership table as the cross-checked authorities, rather than asserting
  `agf/agents/MANIFEST.yaml`'s ownership unsupported. Added the
  partial-install nuance (an upstream-owned file that already exists and
  differs from the pinned distribution is a reportable collision, not an
  automatic overwrite) and explicitly scoped "upgrading an
  already-commissioned project" out of this plan as a separate, existing
  flow.

All five preserved strengths `bqdk` named are unchanged. No implementation
has occurred.
