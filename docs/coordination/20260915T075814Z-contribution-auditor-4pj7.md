---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260915T075814Z-contribution-auditor-4pj7
timestamp: 2026-09-15T07:58:14Z
model: claude-sonnet-5
immutable: true
references-to: []
topic: issue130-pending-advisory-consultation-status-gate1-audit
requires-ack: false
artifacts_owned: []
---

Self-performed contribution-auditor consultation for the plan
(`20260915T075705Z-issue130-pending-advisory-consultation-status.md`),
before Gate 1 request, per this project's established practice for a
plan introducing gate mechanics.

# Contribution review: issue #130 Gate 1 draft

## Finding, corrected in the same draft before Gate 1 request

**The first draft's Rust snippet assumed an iteration source
(`consultation_entries`) that doesn't exist in the real function.**
`validate_semantics_with_manifest` only extracts role *names* into
`roles: Vec<String>` today, not `(role, status)` pairs. A literal
implementation of the original snippet would not compile. **Fixed**: the
snippet now iterates `data.get("advisory_consultation")` directly a
second time, matching the function's own existing style of one focused
loop per concern, and is buildable as written.

**The first draft also deferred the required six-surface rule-projection
manifest to "implementation time"**, even though this plan changes
`AGENTS.md` (an AI landing/loader surface) and is exactly the class of
change `documentation-rule-projection-rule.md` requires the screen for
*at Gate 1*, not deferred -- Row 6 S4's own plan included its manifest
at Gate 1, and this one should too. **Fixed**: the manifest now exists
(`docs/reviews/rule-projection/20260915T075705Z-...yaml`), validated via
`agov plan projection-check --allow-planned`.

## No other blocking finding

- **Scope discipline:** correctly limited to a schema enum addition plus
  one semantic validator check; explicitly declines to touch which roles
  are required, `waived`'s existing behavior, or any retroactive
  migration of existing plans.
- **Normative-owner reasoning holds up:** the plan's claim that
  `agov plan validate`'s blocking authority over plan-frontmatter
  semantics is already established (unlike Row 6 S4's brand-new CI gate)
  is correct -- `required_roles()` and `role_statuses()` already block
  today without their own dedicated rule text, and this slice adds a
  check of the same kind inside the same already-authorized command.
- **Test strategy is concrete and covers the actual failure mode**
  (`status: approved` + a `pending` row) rather than only a schema-shape
  test, and explicitly includes the negative/positive/backward-compat
  triad `gate-self-test-rule.md` requires.
- **Evidence citation is accurate:** verified `20260907T062346Z-contribution-auditor-a8pj`
  does contain finding A1 as described, and that this session's own Row
  6 S4 drafting independently hit the identical gap (`pending` rejected
  by the schema), corroborating the issue's premise with fresh evidence
  rather than relying on the original report alone.

## What this review does not do

Not a Gate 1 approval and grants no governor authority. Does not
independently verify the eventual Rust implementation, since none exists
yet -- that is Gate 2 evidence.
