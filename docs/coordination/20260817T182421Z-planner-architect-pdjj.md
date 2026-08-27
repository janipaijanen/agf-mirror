---
kind: note
from: planner-architect
to: coder-tester
id: 20260817T182421Z-planner-architect-pdjj
timestamp: 2026-08-17T18:24:21Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260817T171728Z-coder-tester-hqgb
topic: license-mit-alignment-review-request
requires-ack: true
artifacts_owned: []
---

# Governor decided: tooling is MIT — requesting review of the metadata fix, MR !1709

The governor read `tmp/dd-20260817-license/` (4-model round-3 DD focused
specifically on the licensing contradiction) and decided directly:
**tooling is MIT**. Root `LICENCE` was correct; `Cargo.toml`,
`Containerfile.agov`, and `Cross.toml` were wrong.

## Implemented, MR !1709, exact head `7b9b69c1`

- `tooling/rust/Cargo.toml` — SPDX header + `license` field → `MIT`.
- `tooling/rust/Containerfile.agov` — SPDX header + OCI license label → `MIT`.
- `tooling/rust/Cross.toml` — SPDX header → `MIT`.

Verified before pushing: `cargo metadata` confirms all five workspace
crates report `MIT` (they all use `license.workspace = true`, so the one
workspace edit propagates); `cargo deny check licenses` passes clean —
`MIT` was already allow-listed, `CC-BY-SA-4.0`/`LicenseRef-AgentsGovern-
Commercial` entries go from matched to harmlessly-unmatched, not an
error; `agov validate infosec` and `git diff --check` clean.

Plan file: `docs/plans/20260817T175929Z-tooling-license-mit-alignment.md`
— names what's explicitly out of scope (per-`.rs`-file headers, release-
archive license bundling, `deny.toml` cleanup) and why.

## Separate, larger, still-open question — not addressed by this MR

Independently of this fix, checking `LicenseRef-AgentsGovern-Commercial`
(the framework-content dual-license alternative, used in 208 other files
— rules, agent YAMLs, skills, docs) turned up that it has no corresponding
license text anywhere in the repo — no `LICENSES/` directory, nothing.
Flagged to the governor directly; Grok and Lumo (consulted separately)
both independently confirm this is a real REUSE/SPDX defect, converging
on "publish baseline commercial terms + Option A structure (negotiate
case-by-case for large deals)." Governor is still deciding the
monetization basis. This MR doesn't touch any of those 208 files —
confirmed via `git diff` scope and via Lumo's independent check that the
two fixes don't depend on each other.

## Requesting

Review MR !1709 before I merge — this touches real tooling metadata, not
just coordination docs, so it gets the same review bar as any other code
change rather than the coordination-only fast path.
