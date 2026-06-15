---
provenance: verified-agf-v2
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Testing Environments and Test Tier Mapping
#
# Framework principle: Pipeline Topology, Tier Selection Guidance
# See: framework.md -- Section 2 (Tier Selection Guidance), Pattern 1 (Pipeline)
# Evidence: silver-birch and copper-rapids (validated across Elixir and Python stacks)

> **Before writing any test:** Read the existing tests for the module under test first.
> Understand the current contract before adding to it.

## The three tiers

Every test belongs to exactly one tier. The tier determines where it runs, what
infrastructure it requires, and which tag (if any) to apply.

<!-- CUSTOMIZE: Replace the commands and tags below with your test runner's equivalents -->

| Tier | Tag | DB required | When to run |
|------|-----|-------------|-------------|
| **Unit** | (none) | No | Every commit |
| **Integration** | `integration` | Yes (live database) | Every commit in CI |
| **E2E** | `e2e` | Yes (live database) | Every commit in CI |

## Tier 1 -- Unit tests

**What they cover:** Pure logic -- business rules, calculations, validation,
helper functions.

**Rules:**
- Must not require a running database or external service.
- Must not require HTTP context or framework runtime.
- Must pass quickly in isolation.

## Tier 2 -- Integration tests

**What they cover:** Full request/response cycles, database queries, access
control checks, tenant isolation, audit log writes.

**Rules:**
- Use the framework's test helpers for auth simulation and database setup.
- Each test uses factories or fixtures -- no raw data inserts.
- Database is wrapped in a transaction rolled back after each test.
- Security regression tests (tenant isolation, RBAC) belong here.

## Tier 3 -- E2E tests

**What they cover:** Multi-step user journeys across multiple routes or
pages -- create, redirect, view, edit, submit.

**Rules:**
- Every scenario exercises at least two routes in sequence.
- Each test manages its own setup -- do not assume shared global state.

## Writing new tests -- decision tree

1. **Pure logic, no HTTP, no DB?** --> unit test, no tag.
2. **Single HTTP action or DB query, needs a seeded DB?** --> integration.
3. **Multi-step user journey?** --> e2e.
4. **Security regression (tenant isolation, RLS, RBAC)?** --> integration.

## Test DB hygiene

- Integration and E2E tests run inside a sandbox transaction -- changes are
  rolled back after each test.
- Never share mutable state between tests.
- The test database must have all migrations applied before running.
