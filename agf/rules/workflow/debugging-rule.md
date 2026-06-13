---
provenance: inherited-v1x-unverified
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Structured Debugging: Diagnose Before Fixing
#
# Framework principle: Quality Gate — root-cause analysis
# See: framework.md — Principle 5 (Escalation over Improvisation)

## The Anti-Pattern to Refuse

Trial-and-error debugging: changing code until the error goes away.
This produces fixes that mask symptoms, break other things, and leave
the root cause in place.

## The Four-Phase Process

### Phase 1 -- Reproduce the failure precisely

Before looking at code, establish a clean reproduction:
- What is the exact error message, stack trace, or wrong output?
- What inputs / request / state trigger it?
- Is it deterministic or intermittent?
- Does it reproduce in isolation (unit test) or only in a full stack?

**Write a failing test that reproduces the bug before touching production code.**
If you cannot write a test that reproduces it, you do not understand it yet.

### Phase 2 -- Locate the fault boundary

Narrow down where the contract breaks. Work inward from the symptom:
- Is the input to the failing function wrong? -> fault is upstream
- Is the function receiving correct input and producing wrong output? -> fault is here
- Is the output correct but interpreted wrongly? -> fault is downstream

Read the module docstring and any existing tests before reading the implementation.
The tests describe the intended contract -- compare that to the actual behaviour.

### Phase 3 -- Understand the root cause

Do not propose a fix until you can state the root cause in one sentence:
> "The bug is that X does Y when it should do Z, because [reason]."

If you cannot complete that sentence, return to Phase 2.

### Phase 4 -- Fix, verify, prevent recurrence

1. Apply the minimum change that fixes the root cause
2. Run the failing test -- it must now pass (GREEN)
3. Run the full test suite -- no regressions
4. Add an assertion or guard that makes this class of bug impossible to reintroduce
   (input validation, type annotation, DB constraint, helper function)
5. Add a regression note to the test name or docstring explaining what it prevents

## When given a screenshot or error output

1. Read the full stack trace from bottom to top (innermost frame first)
2. Identify the file and line in your project code (not library internals)
3. State your hypothesis about the root cause before opening the file
4. Open the file and verify or refute the hypothesis
5. Only then write code

## Never do these

- Change code to "see if it helps" without a hypothesis
- Swallow the exception with a catch-all to make the error disappear
- Add a null-check without understanding why the value is null
- Copy a fix from another place in the code without understanding why it worked there
