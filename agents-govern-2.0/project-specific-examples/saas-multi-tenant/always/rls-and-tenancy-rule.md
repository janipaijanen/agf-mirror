<!-- SPDX-License-Identifier: MIT -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Row-Level Security and Multi-Tenant Isolation
#
# Framework principle: Security Guardian checklist
# See: framework.md -- Tier 3 (Cross-Cutting Agents), Section 8 (Environment-Aware Policies)
# Evidence: copper-rapids and silver-birch (validated across Python/Flask and Elixir/Phoenix)
#
# Stack: Any multi-tenant application with database-level row isolation

<!-- CUSTOMIZE: Replace the technology references below with your stack -->

- Every query in a request context runs under RLS. Never write raw SQL
  that bypasses it except in migrations marked with an explicit owner role.
- Never use the application session from a migration file to query business
  data -- RLS policies are not applied to the migration owner role.
- Every new route must check the relevant permission before touching data.
- The tenant boundary is the root organisation. A query that returns results
  across multiple root organisations without explicit super-admin intent is
  a data leak.
- When writing a new API endpoint, verify authentication token, user, and
  tenant before any database access.

## Verify with

<!-- CUSTOMIZE: Replace with your project's tenant isolation test commands -->

```bash
# Run tenant isolation tests
pytest tests/security/test_tenant_isolation.py -v
pytest tests/security/test_rls_edge_cases.py -v
```
