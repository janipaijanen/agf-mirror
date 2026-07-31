---
id: v249-live-tooling-surface-cleanup
status: pending-gate1
topic: v249-live-tooling-surface-cleanup
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: "The advisory review at 20260731T082657Z-contribution-auditor-srn2 checked scope completeness, historical-provenance preservation, patch compatibility, and the supported-command boundary; it reports complete coverage and no blocking findings."
  - role: infosec-sentinel
    status: not-applicable
    reason: "The proposed change removes a broken local wrapper and stale contributor/adopter guidance, preserves exclusion controls, and adds no credential, secret, network trust boundary, learning payload, or security-control implementation."
---

# Plan: v2.4.9 live tooling-surface cleanup

## Decision requested

Authorize a patch-compatible correction of issue #93: make every supported
entry point and current contributor/adopter instruction use the Rust `agov`
operator path, while preserving clearly historical and generic ecosystem
references. The implementation remains Gate 2 controlled; this plan does not
authorize a version bump, tag, release, branch deletion, or later-minor work.

## Verified defect and release character

The defect still reproduces on post-v2.4.8 `main`:

- `bin/agf` is tracked, describes itself as a live AGF CLI, and delegates every
  command to a nonexistent `tooling/elixir` Mix project;
- `tooling/rust/tests/parity/run.sh` is a tracked executable whose fixtures and
  oracle are the removed Elixir implementation;
- both current GitLab MR templates instruct contributors to run Mix tasks;
- AGENTS.md says legacy sources remain under `tooling/elixir/`, although
  `git ls-files tooling/elixir` is empty;
- current framework, contract, governance-manifest, and decommission guidance
  still presents Mix commands as supported operator instructions.

Correcting a broken command, dead test harness, and inaccurate documentation
adds no command, schema, obligation semantic, or supported behavior. The work
is therefore a v2.4.9 patch. If implementation discovers that compatibility
requires a new alias, command, or behavioral contract, stop and return to Gate
1 for minor-release classification.

## Reference classification

Implementation must classify every tracked Elixir/Mix and legacy Python-CLI
reference before editing it:

| Class | Treatment |
|---|---|
| Supported executable or active CI/test helper | Remove when it depends on deleted tooling; do not add a second wrapper around `tooling/agov`. |
| Current contributor/adopter/operator guidance | Replace with the exact supported Rust CLI spelling and validate the example. |
| Upgrade or decommission instruction intentionally identifying legacy syntax | Retain, and clarify as legacy only if the surrounding text is ambiguous. |
| Changelog, ADR, approved historical plan, coordination, or learning evidence | Preserve verbatim; history is not rewritten to resemble current guidance. |
| Generic ecosystem example, such as an adopter's own Elixir/Ecto command | Preserve when it does not claim to be AGF tooling. |
| Exclusion or negative-test fixture that prevents Elixir residue entering a release | Preserve and test; these safeguards are not live implementation claims. |

The initial active-path inventory includes AGENTS.md, both files under
`.gitlab/merge_request_templates/`, `agf/framework.md`,
`agf/docs/governance/manifest.md`, `agf/docs/contracts/`, and
`agf/docs/adopt/decommission.md`. The implementation must repeat a full tracked
search and record classifications for any additional current surface found.

## Implementation sequence

1. Freeze a tracked-reference inventory with file, line, class, decision, and
   exact replacement or preservation reason.
2. Delete the broken `bin/agf` wrapper. `tooling/agov` remains the single
   supported repository wrapper; no compatibility shim is introduced for a
   command that is already nonfunctional in the released baseline.
3. Delete the dead Elixir/Rust parity harness and any live reference to its
   nonexistent CI job or fixtures. Preserve Rust-native tests and historical
   parity evidence.
4. Convert active AGF commands to their real Rust spellings, including
   `agov validate learnings`, `agov validate infosec`, other `agov validate`
   subcommands, and `agov dist` subcommands as applicable.
5. Correct AGENTS.md and governance/contract descriptions so they identify
   Rust as the only implementation and distinguish retained local-residue
   exclusions from shipped tooling.
6. Preserve `.gitignore`, `.dockerignore`, release-manifest exclusions, and
   `check-no-elixir-artifacts.sh` unless a test proves a narrower replacement
   provides equal protection.
7. Add a deterministic regression check that fails when a supported executable,
   current MR template, or current operator-documentation allowlist contains
   `mix agov`, `tooling/elixir`, the removed Python CLI, or `bin/agf`. Explicit
   historical/generic/exclusion paths must be enumerated rather than hidden by
   a repository-wide blanket exclusion.
8. Build clean source archives and exercise the supported Rust commands from a
   clean checkout and extracted tar/zip before Gate 2.

## Required evidence

- `git ls-files tooling/elixir` returns no implementation files and `bin/agf`
  plus the dead parity harness are absent.
- The classified full-search inventory has no unclassified tracked match.
- The new negative check rejects fixtures containing stale live commands and
  accepts explicitly classified historical/generic/exclusion examples.
- Both MR templates contain executable Rust `agov` validation commands.
- `cargo test --manifest-path tooling/rust/Cargo.toml` passes.
- `agov validate`, `agov dist validate`, and release-path validation pass.
- Clean tar and zip builds contain no Elixir/Mix or legacy Python tooling
  payload and pass the existing archive self-sufficiency checks.
- Strict documentation build and `git diff --check` pass.

## Acceptance criteria

1. No supported entry point invokes removed tooling.
2. No current AGF contributor/adopter instruction presents Mix or the legacy
   Python CLI as supported; retained mentions are visibly historical, generic,
   or protective.
3. Rust `agov` remains the single operator path, with no new surface or semantic
   contract.
4. Existing residue-exclusion and release self-sufficiency guarantees remain
   blocking and green.
5. The implementation MR receives exact-head Gate 2 approval before merge.
6. Issue #93 closes only after the v2.4.9 published artifact demonstrates the
   correction; implementation merge alone may mark it Level 4 Validated, not
   Level 5 Deployable.

## Stop conditions

Stop and return to the governor if the correction needs a new command or alias,
changes validator semantics, removes historical evidence, weakens a release
exclusion, exposes a supported consumer of `bin/agf`, or requires unrelated
v2.5+ work. Do not solve issue #92 wrapper/registry parity in this patch.
