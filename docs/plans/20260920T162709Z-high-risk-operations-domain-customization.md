---
id: 20260920T162709Z-high-risk-operations-domain-customization
status: pending-gate1
topic: high-risk-operations-rule-agf-ng-domain-customization
advisory_consultation:
  - role: infosec-sentinel
    status: pending
  - role: contribution-auditor
    status: pending
affected_paths:
  - agf/rules/always/high-risk-operations-rule.md
  - .claude/rules/always/high-risk-operations-rule.md
---

<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Customize high-risk-operations-rule.md for AGF-NG's actual domain

## Intent

Named as fast-follow work in governor decision
`20260920T115342Z-governor-reviewer-3es7` (the LIGHT/NORMAL/STRICT
governance-mode classification), which built its STRICT tier partly on
`high-risk-operations-rule.md`'s named-area list — but that list currently
describes a generic SaaS application (audit log pipeline, RLS, auth flow,
tenant isolation) that does not describe AGF-NG. Verified directly: no
audit-log service, no row-level security, no OIDC login flow exists
anywhere in this repository. Until this plan, the rule's STOP-before-
editing obligation was, in practice, inert for this project's own
self-governance — the six named areas never match anything a session
actually edits here.

This plan replaces that generic list with AGF-NG's own actual high-risk
areas, grounded in this project's own incident history rather than
invented from first principles, and adds a proper `## CUSTOMIZE` section
(matching the pattern every other rule in `agf/rules/` that ships an
adopter-configurable list already uses — e.g. `i18n-supported-locales-
rule.md`'s `**Supported locales:** en`) so the shipped content is both a
real, in-force rule for AGF-NG's own governance and a worked example for
adopters of what "customize this list for your domain" concretely means.

## Grounded evidence for each area

Verified against this repository directly (not asserted from memory) and
against `agf/learnings/foundry.yaml`:

1. **Coordination integrity mechanics** —
   `tooling/rust/crates/agov-coord/**`,
   `agf/schemas/coordination-message.schema.json`,
   `.gitlab/scripts/changed-files.sh`, and the `agov:governance:validate`
   job in `.gitlab-ci.yml`. *Consequence if broken: a bug can let a
   malformed, backdated, or falsely-approved record into the append-only
   audit trail undetected, or silently disable the coordination-
   publication fast path's fail-closed guarantee* — this is the exact
   failure class MR !692 produced (cited in
   `coordination-publication-rule.md`'s own evidence header): seven
   implementation files entered `main` through a branch a title claimed
   was coordination-only, with a title-suppressed CI pipeline.

2. **Gate/role-authority validation logic** —
   `agf/schemas/gate1-plan.schema.json`'s `gate1_approved_by`/
   `advisory_consultation` contract and the Rust code that cross-checks
   it against `agf/agents/MANIFEST.yaml`. *Consequence if broken: a
   naive or broken check can be fully bypassed, falsely certifying a
   governor approval that never happened* — directly evidenced by
   learning `20260803T212420Z-hv3k`: independent red-team analysis (Grok
   and, convergently, Codex) found a proposed naive fix to this exact
   surface "fully bypassed" whenever the relevant MANIFEST is absent,
   and separately vulnerable to near-miss role-string matches and
   lifecycle-state gaps.

3. **Release, tag, and publication mechanics** — the `release:*` and
   `agov:oci:release` jobs in `.gitlab-ci.yml`, the `agov dist`/`agov
   release` command implementations, and `immutable-tags-rule.md`'s
   enforcement points. *Consequence if wrong: schema/config errors ship
   silently and, because tags are immutable once pushed, recovery costs
   multiple patch releases against an already-broken version* —
   evidenced by this project's own documented three-incident history
   (v0.25.0, v0.38.0, v0.41.0), already cited as `gate-self-test-rule.md`'s
   own motivating evidence.

4. **Agent role/authority manifest edits** — specifically the
   `played_by` and `status` fields in `agf/agents/MANIFEST.yaml` and each
   `agf/agents/*.yaml`. *Consequence if wrong: silently reassigns who may
   approve Gate 1/Gate 2, or grants a suspended or dormant role's
   authority to the wrong actor* — narrower and more load-bearing than
   `downstream-impact-rule.md`'s blanket `agf/agents/*.yaml` trigger,
   which requires only classification and a migration note, not a
   stop-and-confirm pause before the edit starts.

5. **Coordination/plan schema version changes** — breaking field changes
   (not additive ones) to `agf/schemas/*.json` themselves. *Consequence
   if wrong: these are load-bearing contracts already validated against
   by historical immutable coordination records; an incompatible field
   change can retroactively make historically-valid append-only records
   look invalid, with no way to fix them after the fact* (mirrors the
   original rule's "migration scripts... irreversible once applied"
   framing, applied to schema versioning instead of a database).

6. **Publication-hold / release-freeze enforcement** — the
   `publication_hold` mechanics defined in
   `coordination-publication-rule.md` and any code path that reads or
   enforces them. *Consequence if bypassed: a coordination record could
   merge during a declared release-freeze window when it must not* —
   the direct analog of the original rule's "time-lock or state-lock
   logic" item, already present in this project's own schema vocabulary
   rather than invented.

Two of six areas (1 and 4) do not yet have a documented incident at the
severity of `hv3k` or the iron-gate three-incident history — flagged
honestly per `verifiable-claims-rule.md` rather than overclaimed; their
inclusion rests on structural analogy to the original rule's audit-log
and RLS items (both "silent corruption of a trust boundary with delayed
detection"), not on a recorded incident. If a reviewer judges that
insufficient for STRICT-tier inclusion, downgrading either to NORMAL
(still requires full Gate 1, just not the STOP-before-starting pause)
is a reasonable Gate 1 finding this plan does not foreclose.

## Downstream-impact classification

Per `downstream-impact-rule.md`'s own Parnas test: **BREAKING**, not
PREVENTIVE-treated-as-ADDITIVE. Before this change, the rule's STOP-
before-editing obligation matched nothing real in this codebase, so it
required nothing in practice for any of the six areas listed above —
editing them went straight to ordinary Gate 1 (or, for LIGHT-eligible
edits, the fast-track waiver). After this change, editing any of the six
newly-named areas requires stopping, writing a plan file, and waiting for
confirmation *before* that ordinary Gate 1 process even starts — a real
restriction on previously-unencumbered (if formally ungoverned) practice.

```
Impact:
  Foundry:          Yes -- six named areas newly require STOP-and-plan-
                     first before Gate 1 begins, and a targeted
                     verification pass after editing (cargo test scoped to
                     the affected crate; agov coord validate / agov plan
                     validate as applicable; a fresh read of
                     gate-self-test-rule.md's negative tests if gate
                     mechanics are also touched).
  Direct adopters:  BREAKING for any adopter who has NOT yet customized
                     this rule for their own domain (the shipped default
                     changes from generic SaaS-app language to this
                     project's own AGF-NG areas, which will not match a
                     typical adopter's codebase either) -- this argues
                     for the CUSTOMIZE section being the primary shipped
                     artifact, with AGF-NG's own list marked as a worked
                     example, not the default every adopter inherits
                     unexamined.
  Downstream:       No change to any adopter-produced artifact shape.
                     Agent behavior toward an adopter's own users is
                     unaffected -- this rule governs agent behavior
                     toward the adopter's own codebase, not toward the
                     adopter's end users.
  Migration:        None for historical commits -- this is a
                     going-forward procedural obligation, not a
                     retroactive judgment on prior edits to these areas
                     (per downstream-impact-rule.md's own "classify,
                     don't silently rewrite history" discipline, and
                     documentation-rule-projection-rule.md's baseline-
                     audit-scope precedent for the same principle).
  Evidence:         20260803T212420Z-hv3k (gate-authority validation
                     logic); the v0.25.0/v0.38.0/v0.41.0 three-incident
                     history already documented in
                     gate-self-test-rule.md (release/tag mechanics).
```

## Proposed content (draft — subject to Gate 1 review)

Replace `high-risk-operations-rule.md`'s single flat STOP list with:

```markdown
# High-Risk Operations: Stop and Plan First
#
# Framework principle: Human review escalation, boundary enforcement
# See: framework.md — Principle 5 (Escalation over Improvisation), Pattern 3 (Escalation)

- STOP before editing any of the following high-risk areas:
  - **Coordination integrity mechanics** — the Rust validator/schema
    logic implementing `agov coord validate` and `agov coord
    publication-check`, and the coordination-message schema itself
    (`tooling/rust/crates/agov-coord/**`,
    `agf/schemas/coordination-message.schema.json`,
    `.gitlab/scripts/changed-files.sh`,
    `agov:governance:validate` in `.gitlab-ci.yml`). *Consequence if
    broken: a malformed, backdated, or falsely-approved record can enter
    the append-only audit trail undetected, or the coordination-
    publication fast path's fail-closed guarantee can be silently
    disabled — the exact failure class MR !692 produced.*
  - **Gate/role-authority validation logic** — the code and schema
    contract implementing `gate1_approved_by`/`advisory_consultation`
    cross-checks against `agf/agents/MANIFEST.yaml`
    (`agf/schemas/gate1-plan.schema.json` and its Rust validators).
    *Consequence if misconfigured: a naive or broken check can be fully
    bypassed, falsely certifying a governor approval that never
    happened (learning `20260803T212420Z-hv3k`) — especially in a
    MANIFEST-absent workspace.*
  - **Release, tag, and publication mechanics** — the `release:*` and
    `agov:oci:release` CI jobs, the `agov dist`/`agov release` command
    implementations, and `immutable-tags-rule.md`'s enforcement points.
    *Consequence if wrong: errors ship silently and, because tags are
    immutable once pushed, recovery costs multiple patch releases
    against an already-broken version (this project's own v0.25.0,
    v0.38.0, v0.41.0 incidents).*
  - **Agent role/authority manifest edits** — the `played_by` and
    `status` fields specifically in `agf/agents/MANIFEST.yaml` and each
    `agf/agents/*.yaml`. *Consequence if wrong: silently reassigns who
    may approve Gate 1/Gate 2, or grants a suspended or dormant role's
    authority to the wrong actor.*
  - **Coordination/plan schema version changes** — breaking field
    changes to `agf/schemas/*.json`. *Consequence if wrong: these are
    load-bearing contracts already validated against by historical
    immutable records; an incompatible change can retroactively make
    historically-valid append-only records look invalid, with no way to
    fix them after the fact.*
  - **Publication-hold / release-freeze enforcement** — the
    `publication_hold` mechanics in `coordination-publication-rule.md`
    and any code path that reads or enforces them. *Consequence if
    bypassed: a coordination record could merge during a declared
    release-freeze window when it must not.*

- For any of the above: propose the change with a plan file and wait for
  confirmation. Do not implement silently.

- After editing any of the above: run this project's applicable
  verification before proposing a commit — `cargo test` scoped to the
  affected crate(s) under `tooling/rust/`, `agov coord validate` /
  `agov plan validate` on any touched schema-conforming files, and, if
  the edit is also gate mechanics per `gate-self-test-rule.md`, a fresh
  run of that gate's own negative test.

## CUSTOMIZE

The list above is AGF-NG's own configured high-risk-area list —
grounded in this project's incident history (learning
`20260803T212420Z-hv3k`; the v0.25.0/v0.38.0/v0.41.0 release incidents)
rather than invented, and kept here as a worked example. **Adopters
building a different kind of project MUST replace this list with their
own domain's high-risk areas** — the original generic SaaS-application
example this rule shipped with (audit log pipeline, access control,
authentication flow, row-level security/tenant isolation, migration
scripts, time-lock/state-lock logic) remains a reasonable starting point
for a typical multi-tenant web application, but a governance-framework
project like AGF-NG needed its own list because none of those generic
areas exist in this codebase.

Adopters MAY also adjust the verify-after-editing step to their own test
runner (`pytest tests/security/ -v`, `npm test -- --grep security`, etc.)
in place of the `cargo test` command shown above.
```

## Affected files

- `agf/rules/always/high-risk-operations-rule.md` (canonical; content
  replaced per above)
- `.claude/rules/always/high-risk-operations-rule.md` (byte-identical to
  canonical except front matter, confirmed by diff before drafting this
  plan; receives the same replacement)

No schema, validator, CI, or agent-definition file changes — this is
rule content only. Confirmed no gate mechanics touched, so
`gate-self-test-rule.md`'s negative-test requirement does not apply to
this MR itself (it applies going forward to anyone editing the newly-
named gate/coordination/release-mechanics areas this rule now points
at).

## Rule-projection screen

Not applicable per `documentation-rule-projection-rule.md`'s own schema
(`changed_pages_or_runbooks` requires at least one entry): this plan
changes no Pages or runbook page. `simplicity-rule.md`'s existing STRICT-
tier reference to `high-risk-operations-rule.md`'s "named area" already
cross-references this rule by pointer rather than by copied content, so
no follow-on edit to `simplicity-rule.md` is required by this plan — its
STRICT trigger for "touches any high-risk-operations-rule.md named area"
picks up the new list automatically once this plan merges.

## Test strategy

No code under test — rule-content-only change. Verification is:
- `agov plan validate` on this plan file.
- Manual grep confirming none of the six newly-named areas' example
  paths are stale by the time this plan reaches Gate 2 (file layout may
  shift; re-verify each named path resolves).
- Read-through confirming the CUSTOMIZE section accurately describes
  both AGF-NG's own configuration and the original generic example it
  replaces, so adopters relying on the prior generic list are not
  silently left without guidance.

## Risks

- **Overclaiming evidence risk**: areas 1 and 4 (coordination integrity,
  role-manifest edits) don't have an incident at `hv3k`'s or iron-gate's
  documented severity. Addressed above by explicit disclosure rather
  than presenting all six areas as equally evidenced.
- **Path-drift risk**: naming specific file/crate paths in a rule
  (rather than only conceptual areas, as the original generic list did)
  makes the rule more concrete but also more likely to go stale as the
  codebase reorganizes. Accepted trade-off — a concrete, actionable rule
  that needs occasional path updates is more useful than a permanently
  path-agnostic one nobody can apply.
- **Scope-creep risk**: this plan does not reopen the deferred
  contractual/compliance/legal STRICT trigger from the governance-mode
  decision (`3es7`). That remains explicitly out of scope here too —
  a third plan, not folded into this one, if the governor wants it
  pursued.

## Rollback

Pure rule-content change on a feature branch behind Gate 1 and Gate 2;
revert-by-new-commit if the governor finds the new list worse than the
generic one it replaces. No runtime, schema, or data effect to unwind.
