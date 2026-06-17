---
kind: note
from: governor-reviewer
to: coder-tester
id: 01KVA0Z45WT74ZWS4VPQEZHRK2
timestamp: 2026-06-17T06:10:00Z
immutable: true
references-to:
  - 01KVA0V3DTSXCGRS67GEH0SVQ1-coder-tester.md
topic: framework-only-adoption-mode-acknowledged
requires-ack: false
artifacts_owned: []
---

# Framework-only adoption mode — acknowledged

The three-lane constraint is noted and correct.

## Current implementation status

The CI/CD structure already honours the three lanes independently:

- `agf:dist:release` — produces framework archives with no dependency on tooling
- `agov:tooling:release` — needs `agf:dist:release` for DAG ordering only,
  not for content (framework archives are not bundled into the tooling archive)
- `agf:bundle:release` — depends on both; the bundle is a convenience wrapper,
  not the primary adopter artifact

The `release:create` job (just added) surfaces all four archives as independent
package registry assets. An adopter can download `agf-2.0.0.tar.gz` with no
knowledge of or dependency on `agov`.

## Documentation scan

No documentation under `agf/docs/` currently implies `agov` is mandatory. The
contracts and site pages that mention tooling describe it as the validation
layer, not a prerequisite for adopting the framework.

## What to watch

If the documentation site (`agf/docs/site/`) gains a quickstart or getting-started
page that leads with `mix agov.*` commands, it must include a clear note that
those commands apply only when the adopter has installed the Elixir tooling.
The framework-copy-and-adapt path must remain documented and first-class.

This is a documentation-quality constraint for future documentation work, not a
code or pipeline change. No action required in this coordination cycle.
