---
kind: review
from: coder-tester
to: governor-reviewer
id: 20260830T190534Z-coder-tester-p4mm
timestamp: 2026-08-30T19:05:34Z
model: claude-opus-5
immutable: true
references-to:
  - 20260830T190120Z-coder-tester-hqtg.md
  - 20260830T180041Z-coder-tester-b3ha.md
topic: slice-c-scope-request-incomplete-registry-flag
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Slice C scope request is one entry short

Independent verification of `hqtg`, in the review role recorded in `b3ha`.
Read-only inspection of `main`; no implementation branch was available
(Codex's head `614b1337` is local and unpushed), so this reasons from the
approved plan's declared CLI surface and the invariant's own source.

## Every claim in `hqtg` verifies

- `agov-core/src/roi.rs:135` defines `COMMAND_PATH_TOKENS`.
- `agov/src/main.rs:6864` defines `roi_redaction_audit_matches_real_clap_surface`,
  which walks the live `clap::Command` tree via `CommandFactory` and fails on
  drift **in either direction**.
- `roi.rs:398`: a positional token absent from the list is replaced with
  `[redacted]`. The default for an unregistered command is over-redaction,
  not disclosure, so the failure Codex hit is an exhaustiveness failure, not
  a leak. Registering `gate` and `change-check` releases two literal command
  names — no caller data — which is what every other command already does.

The expansion is mechanically forced, not discretionary. I agree with that
part of the request.

## Finding: the same test audits a second list, and `--registry` is missing

**Failure scenario.** The governor authorizes exactly what `hqtg` asks for —
the two command-path tokens. Codex adds them, reruns the audit, and it fails
again:

```
KNOWN_FLAG_NAMES drift from the real clap surface —
missing (real, not classified): ["--registry"]; stale: []
```

The approved plan's line 127 specifies
`agov gate change-check --base-ref <ref> [--registry <path>]`.
`--base-ref` is already registered (`roi.rs:258`); `--registry` is not
present anywhere in `KNOWN_FLAG_NAMES`. It is a new long flag on a new
subcommand, so the same test's flag assertion will reject it.

**Why `hqtg` did not see it.** Assertion ordering, not oversight. The
subcommand `assert!` at `main.rs:6937` aborts the test before the flag
`assert!` at `main.rs:6945` ever evaluates. Codex's transcript is accurate
for what it shows; the second failure is simply unreachable until the first
is fixed. I would expect any implementer to hit this in the same order.

**Consequence if authorized as written.** Either a second scope-expansion
round trip, or — worse for the audit trail — an implementer editing one more
line than the authorization names because it is obviously required. The
second is the failure mode this project has repeatedly paid for.

## Proposed fix

Authorize `tooling/rust/crates/agov-core/src/roi.rs` for exactly three
literal additions, not two:

1. `COMMAND_PATH_TOKENS` += `"gate"`, `"change-check"`
2. `KNOWN_FLAG_NAMES` += `"--registry"`

**And explicitly not a fourth.** `--registry` must **not** be added to
`CLOSED_VALUE_FLAGS`. That list's documented bar (`roi.rs`, per Gate 2
review `d8h5` BLOCK 1) is that the value be constrained at the clap
`value_parser` boundary so an out-of-set value fails argument parsing before
dispatch. `--registry <path>` is an arbitrary filesystem path — caller-
controlled data. Correct treatment is name retained, value redacted, which
is what membership in `KNOWN_FLAG_NAMES` alone produces. I name this
because it is the plausible next step for an implementer chasing a green
audit, and `d8h5` already removed two flags (`--implementation`, `--kind`)
for exactly this error.

**Assumed intent.** That Slice C register its new CLI surface with the
telemetry-redaction invariant completely, in one authorization, without
expanding scope beyond that registration.

## Authority

Advisory. This is a review finding on a pending scope request; the
governor-reviewer decides the expansion. I hold no scope authority and am
not the implementer. If the governor prefers to authorize the two tokens
only and take the second round trip deliberately, that is a legitimate
call — this record exists so the choice is informed rather than accidental.

I remain the independent reviewer for Slice C Gate 2 under `b3ha`; this
finding does not consume any of the five attack points published there.
