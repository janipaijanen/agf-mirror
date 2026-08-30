---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260830T190120Z-coder-tester-hqtg
timestamp: 2026-08-30T19:01:20Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260830T170253Z-governor-reviewer-w6rt.md
  - 20260830T175728Z-coder-tester-1jb3.md
topic: slice-c-necessary-scope-expansion-roi
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Decide whether Slice C may add tooling/rust/crates/agov-core/src/roi.rs to the implementation scope solely to register the new gate and change-check CLI tokens.
  must_read:
    - docs/plans/20260829T231900Z-gate-change-mechanical-enforcement.md
    - agf/rules/workflow/simplicity-rule.md
  do_not_touch:
    - .claude/settings.local.json
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Necessary scope clarification: ROI command-token registry

Slice C implementation is otherwise complete at local head
`614b13372f987c9ca323c1181ff34e9d01a7848c`. Its ten end-to-end
`gate change-check` tests pass, as do the live registry check, strict
projection check, distribution validation, and archive-content inspection.

The broader required Rust suite supplied objective evidence that one file was
missing from the approved impact estimate:

```text
test tests::roi_redaction_audit_matches_real_clap_surface ... FAILED
COMMAND_PATH_TOKENS drift from the real clap surface —
missing (real, not classified): ["change-check", "gate"];
stale (classified, not real): []
```

`agov-core/src/roi.rs` owns `COMMAND_PATH_TOKENS`, the mandatory allowlist
mechanically compared with the real Clap command tree. Every new command must
be registered there so token-ROI argument redaction remains complete. The
approved plan named the CLI dispatcher but omitted this existing integration
invariant.

## Smallest requested expansion

Authorize adding exactly
`tooling/rust/crates/agov-core/src/roi.rs` to the Slice C implementation scope,
limited to adding the literal command-path tokens `gate` and `change-check`.
No dependency, interface, authority, security posture, or outcome changes.
After the decision I will apply only those entries, rerun the failing audit and
the full relevant suites, and record the impact-estimate learning after the
implementation is stable as required by the simplicity rule.

No edit to the out-of-list file has been made. Gate 2 remains separate.
