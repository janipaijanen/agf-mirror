<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
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
    immutable once pushed, recovery costs a new patch version rather
    than a fix to the broken one — exactly what `agf:dist:release`'s
    tag-vs-VERSION guard and `release:namespace-check` already exist to
    prevent.*
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
grounded in this project's own verified incident and enforcement
history (learning `20260803T212420Z-hv3k`; the tag-vs-VERSION and
namespace-absence guards already in `.gitlab-ci.yml`) rather than
invented, and kept here as a worked example. **Adopters
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
