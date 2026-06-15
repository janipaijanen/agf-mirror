---
provenance: verified-agf-v2
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Audit Log Completeness: Every CUD Operation Produces an Audit Trail
#
# Framework principle: Verification over Assumption, evidence-first
# See: framework.md — Section 3 (Cross-Cutting Concerns)

> **Provisional status.** Single-adopter evidence (silver-birch,
> 2026-04-18, framework v0.33.0 — issues #33 and #36). The CUD-without-
> audit-trail gap is structurally invisible without a dedicated agent
> or checklist asking the question, and the silver-birch project found
> it only after ~6 weeks of adoption when a project-originated
> data-quality agent surfaced the gap (~65 missing audit calls across
> 33 controllers). Adopters hitting this rule: please file a learning
> record so we can promote the rule from provisional to recommended,
> or refine the checklist if your stack's failure modes differ.

**Scope:** Every controller, handler, RPC endpoint, GraphQL mutation,
or other entry-point that performs Create, Update, or Delete on
persisted data.

## The rule

Every successful CUD operation on persisted data MUST produce an audit
trail entry. The audit entry MUST include, at minimum:

- **Who** — actor identity (user, service, system).
- **What** — entity type and identifier of the affected record.
- **When** — timestamp.
- **Action** — created / updated / deleted (and ideally the diff payload).
- **Scope** — tenant / project / organization / user-account, whichever
  bounding scope the action belongs to.

Audit logging is part of the success path, not the error path. Audit
calls inside `catch` / `rescue` / error branches create false trails
and MUST NOT be used to "cover" failed operations.

## Required audit-call placement

Audit calls go inside the success branch, after the persistence
operation has succeeded:

<!-- CUSTOMIZE: replace with your stack's success-branch idiom -->

    case Repo.insert(changeset) do
      {:ok, entity} ->
        Audit.log_<scope>_audit(scope_id, "create", actor_id, %{entity: entity})
        {:ok, entity}
      {:error, changeset} ->
        # No audit call here — operation did not complete.
        {:error, changeset}
    end

The same shape applies to update and delete: log only after `{:ok, ...}`
(or your stack's success indicator).

## Scope-specific audit functions

Audit calls SHOULD use a scope-specific function name that encodes
the bounding scope, not a generic catch-all. This forces the caller
to think about which scope the action belongs to:

<!-- CUSTOMIZE: replace with your project's route-to-scope mapping -->

| Route type | Audit function name pattern |
|---|---|
| Project-scoped (`/projects/:id/...`) | `log_project_audit/4` |
| Org-scoped (`/orgs/:id/...`) | `log_org_audit/4` |
| User-scoped (`/users/:id/...`) | `log_user_audit/4` |
| Settings/admin (`/admin/...`, `/settings/...`) | `log_settings_audit/4` |

A generic `log_audit/N` function with a free-form scope argument is
discouraged — it accepts wrong-scope calls without complaint and the
gap surfaces only when a regulator or incident response asks for a
trail by scope.

## Verification — two complementary checks

1. **Per-change (Gate 2 checklist).** For each CUD action in the
   diff, verify a named audit call is present in the success branch
   (after the success match or successful insert). Audit calls in
   error branches are forbidden.

2. **Per-milestone (cross-cutting review).** Grep all CUD entry-points
   and confirm every one has at least one audit call. The exact
   command varies per stack, but the principle is universal:
   *list all entry-points performing create/update/delete; intersect
   with entry-points calling the audit module; the difference is the
   gap.*

   Example sketch (adapt to your stack):

       # List entry-points with CUD operations
       rg --files-with-matches "(Repo\.insert|Repo\.update|Repo\.delete)" lib/your_app_web/

       # List entry-points calling the audit module
       rg --files-with-matches "Audit\.log_" lib/your_app_web/

       # Set difference is the gap

## Why this exists

CUD-without-audit is **structurally invisible**: code with full RBAC,
authentication, CSRF, and input validation can still ship zero audit
logging because no automated check fails. The gap accumulates silently
and is only caught when an explicit cross-cutting agent or checklist
asks "where's the audit call?"

For projects bound by GDPR, SOX, HIPAA, PCI-DSS, or similar regimes,
this gap is a compliance failure waiting to happen. For all other
projects, it's a forensic and incident-response gap (no answer to
"who changed this and when").

## CUSTOMIZE

Adopters MUST customize:

- **Audit context module name** (e.g., `<Project>.Audit`,
  `audit_service`, `AuditLogger`).
- **Audit function names** per scope category for your project.
- **Required fields** — the minimum-set above is a baseline; many
  projects add IP address, user-agent, request ID, change-source
  classification (UI / API / job / migration).
- **Routes-to-scope mapping** — which routes use which audit
  function in your project.

Adopters MAY customize:

- Optional fields (diff payload format, source classification).
- Audit log table partitioning strategy.
- Retention policy hooks (some regimes require min retention; others
  require max retention with deletion verification).

## Related rules

- `always/security-hardening-rule.md` — generic security gates;
  audit logging is one specific gate inside the broader category.
- `workflow/systematic-code-review-rule.md` — Gate 2 checklist
  that should reference audit-completeness when CUD changes are
  in the diff.
- (Project-specific) `domain/audit-log-rule.md` — adopters may
  add a project-specific rule capturing their schema, retention
  policy, and access controls. This generic rule is the structural
  invariant; the project-specific one is the implementation.
