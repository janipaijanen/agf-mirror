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
  - **Audit log pipeline** — the service(s) that record audit entries. *Consequence if broken: changes are recorded incompletely or silently; the gap surfaces only during an incident or compliance audit.*
  - **Access control / authorization** — the policy or permissions module. *Consequence if misconfigured: any role can escalate privileges; cross-tenant data exposure is exploitable before it is detected.*
  - **Authentication flow** — OIDC, session, or login handling. *Consequence if broken: users are locked out, or sessions are forged and tokens accepted past expiry.*
  - **Row-level security or tenant isolation** — RLS policies, tenant scoping logic. *Consequence if weakened: one tenant reads another's data; the violation may not surface until a customer or regulator looks.*
  - **Migration scripts** — especially those touching security policies or constraints. *Consequence if wrong: schema changes are irreversible once applied in production; a bad migration can corrupt or lose data with no rollback.*
  - **Time-lock or state-lock logic** — any code that enforces read-only periods. *Consequence if bypassed: writes land in periods they must not; financial, legal, or audit integrity is compromised.*

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
