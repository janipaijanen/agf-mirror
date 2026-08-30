---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20260830T191046Z-governor-reviewer-jqad
timestamp: 2026-08-30T19:10:46Z
model: human-directed-claude-opus-5
immutable: true
references-to:
  - 20260830T190120Z-coder-tester-hqtg.md
  - 20260830T190534Z-coder-tester-p4mm.md
  - 20260830T170253Z-governor-reviewer-w6rt.md
  - 20260830T175728Z-coder-tester-1jb3.md
topic: slice-c-roi-scope-approved-three-literals
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Slice C scope expansion approved, at the amended three-literal shape

Binding Gate 1 scope decision. Given by the governor-reviewer in session
("!2257 request is approved. proceed as proposed") and transcribed here by
Claude; the decision is the governor's, the transcription is mine.

## Decision

`tooling/rust/crates/agov-core/src/roi.rs` enters the Slice C implementation
scope of `docs/plans/20260829T231900Z-gate-change-mechanical-enforcement.md`.

"As proposed" resolves to the amended shape in `p4mm`, not the two-token
shape originally requested in `hqtg`. The authorization is therefore three
literal additions:

1. `COMMAND_PATH_TOKENS` += `"gate"`
2. `COMMAND_PATH_TOKENS` += `"change-check"`
3. `KNOWN_FLAG_NAMES` += `"--registry"`

## Explicitly not authorized

- **No `CLOSED_VALUE_FLAGS` change.** `--registry <path>` is a caller-
  controlled filesystem path with no clap `value_parser` constraint, and so
  fails that list's documented membership bar (Gate 2 review `d8h5`,
  BLOCK 1). Correct treatment is name retained, value redacted — which
  membership in `KNOWN_FLAG_NAMES` alone already produces.
- **No other edit to `roi.rs`** beyond the three literals above.
- **No other file** enters scope under this decision.

## Basis

The expansion is mechanically forced, not discretionary:
`roi_redaction_audit_matches_real_clap_surface`
(`tooling/rust/crates/agov/src/main.rs:6864`) walks the live `clap::Command`
tree and fails on drift in either direction against both lists. A new
command that is not registered cannot pass the required suite.

The failure is one of exhaustiveness, not disclosure: an unregistered
positional token is replaced with `[redacted]`
(`tooling/rust/crates/agov-core/src/roi.rs:398`), so the unregistered state
over-redacts rather than leaks. Registering two command names and one flag
name releases no caller data.

`hqtg`'s claims were independently verified in `p4mm` before this decision
was made. `p4mm` also established that the two-token request was one entry
short — `--base-ref` is already registered (`roi.rs:258`), `--registry` is
not — and that `hqtg` could not have observed it, because the subcommand
assertion (`main.rs:6937`) aborts the test before the flag assertion
(`main.rs:6945`) evaluates.

## Exhaustiveness of the three

Checked against the plan's declared command signature
(`docs/plans/20260829T231900Z-gate-change-mechanical-enforcement.md`, the
Implementation boundary table): `agov gate change-check --base-ref <ref>
[--registry <path>]`. Two subcommand tokens, two flags, of which `--base-ref`
is already registered. Three is therefore the complete set for the surface
the plan authorizes, not merely one more than was asked for.

The audit's flag walk also collects clap aliases via `get_all_aliases()`, so
any hidden long alias on `--registry` would need registering too; the plan
declares none.

## Bound on this authorization

This authorizes exactly the three literals named above. If implementation
surfaces a fourth entry that the audit also demands, stop and request it —
do not extend under this decision on the grounds that it is obviously
required. That inference is precisely the failure mode this narrow
authorization exists to prevent.

## Resolution and remaining gates

Resolves the pending `requires-ack` obligations in `hqtg` and `p4mm`.

Gate 2 is untouched by this decision and remains a separate governor-
reviewer decision on the exact implementation head. Claude remains the
independent Slice C reviewer under `b3ha`; the five attack points published
there are unaffected and unconsumed.
