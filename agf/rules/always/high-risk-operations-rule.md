---
provenance: verified-agf-v2
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# High-Risk Operations: Stop and Plan First
#
# Framework principle: Human review escalation, boundary enforcement
# See: framework.md — Principle 5 (Escalation over Improvisation), Pattern 3 (Escalation)

- STOP before editing any of the following high-risk areas:
  - **Audit log pipeline** — the service(s) that record audit entries
  - **Access control / authorization** — the policy or permissions module
  - **Authentication flow** — OIDC, session, or login handling
  - **Row-level security or tenant isolation** — RLS policies, tenant scoping logic
  - **Migration scripts** — especially those touching security policies or constraints
  - **Time-lock or state-lock logic** — any code that enforces read-only periods

<!-- CUSTOMIZE: add project-specific high-risk areas (e.g. billing, payment processing) -->

- For any of the above: propose the change with a plan file and wait for confirmation. Do not implement silently.

- After editing any of the above: run the full security test suite before proposing a commit.

<!-- CUSTOMIZE: adapt the verify command to your test runner -->
```bash
# Example: run security tests
# uv run pytest tests/security/ -v
# npm test -- --grep security
# go test ./tests/security/...
```
