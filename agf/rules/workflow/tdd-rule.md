---
provenance: verified-agf-v2
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Test-Driven Development (TDD)
#
# Framework principle: Quality Gate — test-first design
# See: framework.md — Pattern 1 (Pipeline), ADOPTING.md Step 3

## The Iron Rule

**No production code without a failing test first.**

If you have written code before a test: stop, write the test, confirm it fails (RED),
restore the code, confirm it passes (GREEN), then refactor.

## Where to put tests

<!-- CUSTOMIZE: adapt locations and markers to your project -->

| Type | Location | DB needed? |
|------|----------|-----------|
| Pure logic: helpers, services, schemas, calculations | tests/unit/ | No |
| HTTP routes, full request/response, DB queries | tests/integration/ | Yes |
| Browser UI, auth flows | tests/e2e/ + @pytest.mark.e2e | Yes + auth provider |
| Security regressions (SSRF, RBAC, tenant isolation) | tests/security/ | Yes |

## RED -- Write a meaningful test

A meaningful test:
- Name: `test_<what>_<when>_<expected_outcome>`
- Tests one behaviour, not one function
- Uses the smallest fixture that exercises the behaviour
- Fails for the RIGHT reason (not ImportError, not wrong assertion)
- Would catch a real regression if the code broke

A test that always passes (or always skips) is worse than no test.

## GREEN -- Write the minimum code

Write only enough to make the test pass.
No extra logic. No untested edge cases. No "while I'm here" additions.

## REFACTOR -- Clean while green

Remove duplication, clarify names, simplify logic.
Tests must stay green throughout. Do not change behaviour.

## Verify

<!-- CUSTOMIZE: adapt test runner commands to your project -->
```bash
# Run unit tests (no DB required)
# Run integration tests (requires live DB)
# Run security regression tests
```

Coverage floor: set a minimum and never lower it.

## Anti-patterns -- refuse these

- `pass` or `assert True` in a test body
- Testing private methods or internal state instead of behaviour
- Mocking so aggressively the test proves nothing about real code
- Skipping RED ("I know it fails, let's just write the code")
- Writing tests after the fact solely to hit a coverage number
- `pytest.skip()` in CI (acceptable locally, not acceptable in CI validation)
