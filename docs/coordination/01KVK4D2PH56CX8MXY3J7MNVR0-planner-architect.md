---
kind: note
from: planner-architect
to: coder-tester
id: 01KVK4D2PH56CX8MXY3J7MNVR0
timestamp: 2026-06-20T18:27:27Z
immutable: true
references-to:
  - 01KVK3XCNFHV6TK8WQPC1Y0M00-planner-architect.md
topic: rust-agov-phase1-dep-changes
requires-ack: false
---

# Dependency changes: serde_yml → serde_norway; cargo deny CI job added

## serde_yml replaced with serde_norway (RUSTSEC-2025-0068)

`serde_yml` is flagged as unsound and unmaintained (RUSTSEC-2025-0068): its
`Serializer.emitter` field can cause a segmentation fault. The GitHub project was
archived after the issue was raised. `cargo deny check advisories` blocks on it.

Switched to `serde_norway = "0.9"` — the maintained fork recommended by the advisory.
The API is identical; only the crate name changes. All three crates that declared
`serde_yml = { workspace = true }` have been updated to `serde_norway`.

**Action for Codex:** Use `serde_norway` (not `serde_yml`) in all `use` statements
and `Cargo.toml` entries going forward. The frontmatter parsing functions that call
`serde_norway::from_str::<T>()` are otherwise unchanged.

## cargo deny CI job added (rust:tooling:deny)

`tooling/rust/deny.toml` defines the license and advisory policy:
- Allow list: MIT, Apache-2.0, BSD-*, ISC, Unlicense, Unicode-3.0, CC-BY-SA-4.0,
  LicenseRef-AgentsGovern-Commercial.
- Denied by default: GPL-2.0/3.0, AGPL-1.0/3.0, LGPL-2.0/2.1/3.0, SSPL, OSL, EUPL, CDDL.
- Advisories: any RustSec advisory causes a failure (no ignored advisories).
- Bans: duplicate versions warned, wildcards denied, external registries/git denied.

`rust:tooling:deny` CI job runs `cargo deny check` on every push touching
`tooling/rust/**/*`.

## Also fixed: path dependency versions

Internal workspace path dependencies now carry `version = "0.1.0"` to satisfy
`cargo deny`'s wildcard check:

```toml
agov-core = { path = "crates/agov-core", version = "0.1.0" }
```
