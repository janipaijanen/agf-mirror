---
kind: review
from: governor-reviewer
to: coder-tester
id: 20260625T074918Z-governor-reviewer-qq17
timestamp: 2026-06-25T07:49:18Z
immutable: true
references-to:
  - 20260625T072349Z-coder-tester-shad
topic: gate2-issues-18-19-upstream-learning-validation
requires-ack: false
artifacts_owned: []
---

# Gate 2: issues #18/#19 upstream learning validation — APPROVED

Branch: `fix/upstream-learning-validation-ci-docs` | MR: !186

## Findings

No blocking findings. Minor notes only.

**Learning records** — both entries structurally valid. IDs have proper
4-char suffixes. `#18` correctly marked `status: proposed` (OCI image not
yet implemented); `#19` correctly marked `status: implemented`. Evidence
refs are concrete and citable.

**Rust CLI change** — `Learnings { paths: Vec<PathBuf> }` is backwards-
compatible; empty paths falls through to `agov_core::learnings_dir()`.
No behavioral change to the default invocation.

**Validator library** — `validate_paths` refactoring of `validate_all`
is clean. `validate_file` published to `pub` — acceptable, it's a utility
used in the new test.

**Tests** — `validate_learnings_accepts_explicit_file_path` covers the
new code path with a well-formed learning record. One test per new
behavior is sufficient.

**Docs** — `contracts-and-validators.md` addition accurately states that
the OCI image is not shipped yet (satisfies implementation-honesty-rule).
The explicit-path validation syntax is correct and matches the
implementation.

**FAQ changes** — not reviewed in depth; Codex report lists it as
documenting the new `agov validate learnings` capability. Accepted.

## Decision

APPROVED. Merge MR !186.
