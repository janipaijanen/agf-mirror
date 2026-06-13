<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Feature Regression Guard
#
# Framework principle: Quality Gate — test-as-contract, regression prevention
# See: framework.md — Pattern 1 (Pipeline), ADOPTING.md Step 3.5

**Scope:** Every commit that modifies a route, service, template conditional,
permission check, or feature-flag behaviour.

## The rule

A commit must not cause an existing test to fail silently or be deleted
without replacement. Tested behaviour is a contract -- it represents what
the product does for users today.

## What counts as a regression

- An existing test that passed before the commit now fails.
- An existing test is deleted or weakened (assertion removed, skip added)
  without a replacement test that covers the new intended behaviour.
- A feature-flag conditional is changed so that a view, route, or UI
  element that was previously visible to a role is now hidden (or vice
  versa) without updating the corresponding test.
- A template conditional block that controlled visibility is removed or
  altered, changing what users of a specific role see.

## What to do

### When modifying a tested feature

1. Run the existing tests for the feature **before** making changes.
   Confirm they pass (GREEN baseline).
2. Make the change.
3. Run the same tests. If any fail:
   - If the failure reflects the **intended** new behaviour: update the
     test in the **same commit** to assert the new contract.
   - If the failure is **unintended**: your change has a bug. Fix the
     code, not the test.
4. Never delete a test without writing a replacement that covers the
   new behaviour. A test deleted without replacement is a **BLOCK**.

### When modifying an untested feature

1. **Before touching the code**, write a characterisation test that
   captures the feature's current behaviour (what it returns, who can
   see it, what state it produces). This test must pass against the
   current code.
2. Write a plan note (in the commit message or a plan file if >1 module)
   stating: what the feature currently does, what it will do after the
   change, and which roles are affected.
3. Make the change.
4. Update or replace the characterisation test to assert the new
   intended behaviour.

## BLOCK conditions

- Test deleted without replacement.
- Test weakened (assertion removed, skip added) without documented
  justification in the commit message.
- Route or template visibility changed for a role without a test verifying
  the new behaviour.
