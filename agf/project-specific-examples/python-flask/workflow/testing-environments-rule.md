<!-- SPDX-License-Identifier: MIT -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Testing Environments and Test Tier Mapping
#
# Example: Python + Flask + pytest + SQLAlchemy + Alembic
# Copy and adapt for your project.

> **Before writing any test:** Read the existing tests for the module under test first.
> Understand the current contract before adding to it.

## The three tiers

Every test belongs to exactly one tier. The tier determines where it runs, what
infrastructure it requires, and which pytest mark (if any) to apply.

| Tier | Mark | Runs in | DB | Auth provider | When |
|---|---|---|---|---|---|
| **Unit** | (none) | local + CI | no (or optional skip) | no | every commit |
| **Integration** | (none) | CI (`APP_ENV=test` container) | **yes** | no | every commit via CI |
| **E2E / Identity** | `e2e`, `e2e_identity` | dedicated E2E environment | **yes** | **yes** | pre-release / nightly |

---

## Tier 1 -- Unit tests (`tests/unit/`)

**What they cover:** Pure logic -- helpers, services, schema validation, utility
functions, Pydantic schemas, etc.

**Rules:**
- Must not require a running DB. If the module under test imports ORM models,
  wrap the import and call in `with app.app_context():` using the `app` fixture.
- Must not import from route modules without an app context.
- Must never call `db.session` or ORM `.query` directly.
- Allowed to `patch` any service or model at the boundary.
- Must pass in < 5 seconds per file without a database.

**DB-optional pattern** (when a unit test needs the `app` fixture for context but not DB):
```python
def test_something(app):
    with app.app_context():
        from app.services.my_service import my_function
        result = my_function(some_input)
    assert result == expected
```

---

## Tier 2 -- Integration tests (`tests/integration/`, `tests/security/`)

**What they cover:** Full HTTP request/response cycle against a live database
(via the framework's test client). DB is seeded with test fixtures.
All migrations run before the session starts.

**Rules:**
- Always use the test client + a DB seed fixture. Never skip DB setup.
- If a DB is not available, use `pytest.skip("Database not available")` gracefully --
  but in CI a live DB is always present and tests must not be skipped.
- Must not import browser automation tools or start a live HTTP server.
- Test one HTTP concern per test: status code, response body, redirect, flash message,
  audit log entry, or DB state.
- Security regression tests (`tests/security/`) follow the same rules as integration tests.
- Tests should be hermetic: each test restores any config changes it makes (use `try/finally`
  or a fixture with teardown).

---

## Tier 3 -- E2E and identity tests (`tests/e2e/`)

**What they cover:**
- **`@pytest.mark.e2e`**: Browser-driven tests (e.g. Playwright) against a live running app.
  Tests UI flows: form submission, navigation, downloads.
- **`@pytest.mark.e2e_identity`**: Full auth flow via your identity provider. Covers:
  - Login / logout
  - Role-based access (admin vs editor vs viewer)
  - Group revocation / session staleness
  - Session timeout enforcement
  - Auth callback error handling

**Rules:**
- `e2e` tests: require `DATABASE_URL` + a running app instance.
- `e2e_identity` tests: additionally require a running auth provider with seeded credentials.
- All `e2e_identity` tests must clean up auth provider state after themselves.
- Session and revocation tests must use realistic sleep/poll rather than mocking
  time -- the goal is to verify actual session expiry behaviour.

---

## Writing new tests -- decision tree

1. **No HTTP, no DB, pure logic?** -> `tests/unit/` -- no mark needed.
2. **HTTP request/response, needs a seeded DB?** -> `tests/integration/` -- use test client + DB fixture.
3. **Browser UI flow?** -> `tests/e2e/` -- mark `@pytest.mark.e2e`.
4. **Requires auth provider, role assignment, or session test?** -> `tests/e2e/` -- mark `@pytest.mark.e2e_identity`.
5. **Security regression (SSRF, zip bomb, MIME, access control)?** -> `tests/security/` -- same fixtures as integration.

Never write a test that:
- Silently passes when the DB is absent (skipping when unavailable is acceptable for local;
  it must not silently pass in CI by accidentally testing nothing).
- Mixes browser automation and the framework test client in the same test.
- Relies on test order (use fixtures to establish state, not shared mutable globals).

---

## Test DB hygiene

- The test DB should always be a **fresh ephemeral DB**. Migrations run from scratch on every CI run.
- Never share state between tests via the DB without a fixture. Use session-scoped seeds
  for read-only fixtures; use per-test DB operations for mutating tests.
- Integration tests that mutate DB state (create, update, delete) must roll back or
  use transactions that are safe to repeat (idempotent) since the session-scoped DB
  is shared across the test session.
