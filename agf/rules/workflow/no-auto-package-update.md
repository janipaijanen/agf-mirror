---
provenance: verified-agf-v2
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# No Automatic Package Updates
#
# Framework principle: Supply chain trust boundary
# See: framework.md — ADOPTING.md Step 3.2 (Security checklist)

- **Never** run package-upgrading commands without explicit user request.
- **Forbidden:** any command that upgrades dependencies beyond what the lockfile specifies.
- Use the package manager's install/sync command only to install per lockfile.
- Add new dependencies via the project manifest + lock (without `--upgrade`).
- Propose changes and ask approval before running any command that could update versions.
