---
id: 20260905T144657Z-roi-workflow-baseline-repair
status: approved
gate1_approved_by: governor-reviewer
gate1_approved_at: 2026-09-05T15:45:55Z
topic: roi-workflow-baseline-repair
candidate_release: v2.9.0
advisory_consultation:
  - role: infosec-sentinel
    status: waived
    reason: >-
      The repair changes only fixed numeric baseline selection for existing
      local aggregate telemetry. It adds no collected field, raw prompt,
      credential, network request, or persistence destination.
  - role: contribution-auditor
    status: consulted
    reason: >-
      Claude-backed contribution-auditor review
      `20260905T154555Z-contribution-auditor-3tbq` covered MR !2372 at exact
      head `38d027dcc2280c1bfbfba92103c52144dea1a3d8` and returned COMPLETE/CLEAR
      with zero blocking findings. Its two advisory wording observations do
      not change the bounded implementation scope.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Restore documented ROI workflow baselines

## Problem

`docs/reference/tooling-token-roi.md` states that `cold-start`,
`session-resume:<role>`, and `next-handoff:<role>` receive automatic savings
baselines. The Rust `workflow_baseline()` function currently maps only
validation workflows. Successful resume and handoff calls are therefore
recorded with output sizes but without `baseline_source` or saved-token fields,
causing `agov token roi report` to show zero savings for those workflows.

MCP does not repair this discrepancy: it adds transport attribution to the
same child `agov` record, while baseline selection remains inside `agov-core`.

## Scope

1. In `tooling/rust/crates/agov-core/src/roi.rs`, restore the documented fixed
   mappings:
   - `cold-start` and every `session-resume:<role>` workflow: 3,000–5,000
     manual tokens, source `docs/reference/tooling-token-roi.md#live-baseline`;
   - every `next-handoff:<role>` workflow: 1,000–3,000 manual tokens, same
     source.
2. Preserve all validation mappings and their
   `docs/reference/tooling-token-roi.md#validation-baseline` source.
3. Add regression tests covering exact and prefix mappings, source selection,
   savings subtraction, and an unknown workflow remaining unestimated.
4. Add the repair to the pending v2.9.0 entries in `CHANGELOG.md` and
   `agf/CHANGELOG.md` after the feature MR merges and the release branch is
   regenerated from the new `main` tip.

No historical JSONL records are rewritten. No existing zero-valued report is
retroactively recalculated; only successful calls recorded after the repaired
binary is built gain these automatic fields.

## Work order

1. Implement and test on `fix/roi-workflow-baselines`, branched from the exact
   approved plan base.
2. Run targeted `agov-core` tests and the complete Rust workspace suite.
3. Open an MR to `main` and obtain binding Gate 2 approval.
4. Merge the repair before continuing release construction. Re-fetch and
   re-freeze `main`, re-run the v2.9.0 version-inventory proof, and rebuild the
   release metadata branch from that new base. Do not carry code changes only
   on the release branch.

## Acceptance criteria

1. A new `cold-start` record receives a 3,000–5,000 baseline and the
   `#live-baseline` source.
2. Arbitrary valid role suffixes on `session-resume:` receive the same mapping.
3. Arbitrary valid role suffixes on `next-handoff:` receive a 1,000–3,000
   mapping and the `#live-baseline` source.
4. Saved-token values continue to subtract measured output tokens and prompt
   overhead with saturation at zero.
5. `coord-validate`, `plan-validate`, `validate:umbrella`, and
   `validate:<subcommand>` retain their existing values and
   `#validation-baseline` source.
6. An unknown workflow retains no baseline or saved-token estimate.
7. Direct CLI and MCP-mediated calls use identical baseline arithmetic; MCP
   changes attribution only.
8. `cargo test --manifest-path tooling/rust/Cargo.toml --workspace` passes.

## Risks and rollback

The main risk is overstating savings by accidentally assigning a documented
baseline to unrelated workflow names. Exact `cold-start` matching and the two
bounded prefixes limit that risk; unknown-workflow regression coverage keeps
the default fail-closed. Rollback is a normal revert of the feature MR before
release publication. Published historical telemetry remains untouched either
way.
