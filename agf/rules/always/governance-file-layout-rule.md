---
provenance: inherited-v1x-unverified
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Governance File Layout
#
# Framework principle: Artifact Ownership
# See: ADOPTING.md — "Where to put files"
# Evidence: sage-review created root-level governance dirs twice by
#   following ADOPTING.md paths literally in a subdirectory layout.

Before creating or modifying governance files (`learnings/`, `agents/`,
`docs/adr/`, `GOVERNANCE.md`), determine which layout this project uses:

<!-- CUSTOMIZE: uncomment the layout that matches your project -->

<!-- Layout A: framework-as-project (framework repo IS the project) -->
<!-- GOVERNANCE_ROOT: . -->

<!-- Layout B: framework-as-subdirectory -->
<!-- GOVERNANCE_ROOT: agents-govern -->

Rules:
- Governance artifacts go under GOVERNANCE_ROOT, not the project root.
- `.claude/rules/` stays at the project root (it is a tool config, not a governance artifact).
- If unsure which layout is in use, check: does `GOVERNANCE.md` exist at the project root or in a subdirectory? Follow the existing pattern.
- Never create `learnings/`, `agents/`, or `docs/adr/` at the project root if a `GOVERNANCE.md` exists in a subdirectory.
