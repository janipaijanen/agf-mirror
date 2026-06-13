<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Documentation Audience
#
# Framework principle: Audience scope boundary
# See: framework.md — Pattern 7 (Documentation Review Pipeline)

- **docs/plans/**: Design decisions, not user docs; always timestamped from OS.

- **README.md** (repo root): Target **developers**. Focus on development experience: make targets, quick start, CI locally, schema, migrations, testing, project layout. Point to docs for deployment and operations.

- **Public documentation site** (content under `docs/`): Target **business users** (directors, managers, employees) and **system administrators** who deploy and maintain the solution. Focus on deployment, QA verification, operations, data entry, configuration, backup. Convey that the solution is mature: schema migrations, audit logging, data quality checks, backup procedures are built in.
