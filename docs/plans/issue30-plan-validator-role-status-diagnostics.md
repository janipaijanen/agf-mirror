---
id: issue30-plan-validator-role-status-diagnostics
status: approved
topic: issue30-plan-validator-role-status-diagnostics
gate1_approved_by: governor-reviewer
gate1_approved_at: 2026-07-24T10:10:00Z
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: Diagnostic wording and tests do not change credential or data-flow boundaries.
  - role: contribution-auditor
    status: not-applicable
    reason: The governor approved the narrow issue #30 scope referenced by MR !462.
---

# Gate 1 plan: issue #30 role-status diagnostics

Preserve manifest-authoritative advisory-role validation while distinguishing
roles absent from the manifest from roles present with a non-active status.

Implementation scope:

- retain role statuses when loading `agf/agents/MANIFEST.yaml`;
- keep active roles valid and the two mandatory advisors available during
  manifest fallback;
- report absent roles as unknown;
- report known non-active roles with their authoritative status;
- add regression tests for active, suspended, skipped, unknown, and missing-
  manifest behavior;
- clarify the schema-shape versus manifest-semantic boundary in operator docs.

No role status, schema shape, release metadata, or release artifact changes are
authorized.
