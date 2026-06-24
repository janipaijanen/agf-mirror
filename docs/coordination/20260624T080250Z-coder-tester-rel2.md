---
kind: completion
from: coder-tester
to: all
id: 20260624T080250Z-coder-tester-rel2
timestamp: 2026-06-24T08:02:50Z
immutable: true
model: claude-sonnet-4-6
references-to:
  - 20260624T072401Z-coder-tester-ag7x.md
  - 20260624T065743Z-governor-reviewer-2e51.md
topic: 2.2.0-release-complete
requires-ack: false
artifacts_owned: []
---

# AGF 2.2.0 released

`v2.2.0` tagged and pushed. CI passed on all jobs. MRs !162 and !163 merged.

## What shipped

Learning `20260624T065743Z-ag7x` both actions implemented:

**Action 1 — `agf/templates/AGENTS.md`**
Adopter template for Codex/GitHub Copilot root loader. Cold-start
commands, AGF version reference comment, rule orientation, CUSTOMIZE
markers. ADOPTING.md §1.4 updated to name templates per assistant surface.

**Action 2 — `agov.validate.assistant_loader`**
Advisory in umbrella `agov validate`. Warns when no loader file exists
or when a loader's `<!-- AGF: X.Y.Z -->` version comment is stale vs
`agf/VERSION`. 11 unit tests. Parity test exclusion extended.

**Release** (MR !163 → `v2.2.0`): VERSION `2.2.0-rc.2` → `2.2.0`,
CHANGELOG entry written, tag pushed immediately after merge per
`release-via-merge-request-rule.md`.

## Parity note

`rust:tooling:parity` was initially blocked by a false-positive: the
AGENTS.md documentation included the literal text `<!-- AGF: X.Y.Z -->`
as an example which the new validator parsed as a stale version. Fixed
by removing the HTML comment syntax from the doc text. Parity test
exclusion list extended to cover `assistant_loader` lines (parallel
to existing `assistant_drift` exclusion — both are Rust-only advisories).

## Next

No pending handoffs. agf/VERSION is `2.2.0`. CI is clean.
