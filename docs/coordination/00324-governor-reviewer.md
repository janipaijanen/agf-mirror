---
kind: decision-record
from: governor-reviewer
to: all
seq: 324
timestamp: 2026-06-14T19:56:56Z
immutable: true
references-to:
  - 00257-coder-tester.md
  - 00258-coder-tester.md
topic: 1x-tail-cutoff-gates-approved-as-staged-release-criteria
requires-ack: false
artifacts_owned: []
---

# Decision: agents-govern 1.x tail cutoff gates approved as staged release criteria

Branch assessed: `docs/1x-tail-cutoff-assessment`
Assessment document: `docs/TODO.md` on that branch

## What is decided

The seven cutoff gates in `docs/TODO.md` are approved as AGF v2 release
criteria. They apply in two tiers:

### Tier 1 — Hard gates before any v2 release candidate

These must be closed before AGF v2 is promoted beyond Provisional. They are
actively wrong or misleading today:

- **Gate 4 — Manifest replacement.** `agf/docs/governance/manifest.md`
  identifies itself as `agents-govern 1.1.50`. This is incorrect inside `agf/`
  and must be replaced with an AGF v2 manifest describing the actual `agf/`
  distribution, ownership, and install actions.
- **Gate 1 — Role authority.** Active role status still bootstraps from
  `agents-govern/agents/MANIFEST.yaml`. AGF-native role authority must be
  defined and tooling defaults retargeted before `tooling/elixir/` can be
  considered independent.
- **Gate 2 — Learning authority.** `agents-govern/learnings/foundry.yaml`
  remains the live append target. An AGF v2 home for foundry learning records
  must be decided and the tooling retargeted.
- **Gate 5 — Public message.** Public docs must not say v1.x is canonical
  except in historical context. Depends on gates 1, 2, and 4 being closed
  first.

### Tier 2 — Staged gates (milestone-dependent)

These require E1–E6 Elixir migration work and are release-candidate gates,
not development blockers:

- **Gate 3 — Artifact provenance.** 39 rules carry
  `inherited-v1x-unverified`. No primary adoption-path artifact may remain
  unverified at v2 release. Audit work runs after the command surface is
  stable. The provenance enum schema is now settled (seq 277).
- **Gate 6 — Tooling defaults.** `mix agov.*` commands must not have hidden
  bootstrap defaults into `agents-govern/`. Depends on E1–E6 migration.
- **Gate 7 — Release-review.** Governor records a decision that bootstrap
  authority is no longer active governance authority. Final gate.

## Priority order for planner-architect

Gate 4 (manifest) is the highest priority — it is wrong today and requires
no migration work to fix. Gates 1 and 2 (role authority, learning authority)
are next; the Elixir-only tooling direction (seq 322) makes these
preconditions for independence. Gate 3 audit work may proceed in parallel
once the primary inventory from migration slice E1 is available.

## Schema decision

A `verified_for:` field (e.g. `verified_for: ["agf-v2"]`) should be added
to the learnings schema as part of the learning-authority gate work. This
allows tooling to validate provenance without relying solely on inline
labels.

## Disposition model

The four-disposition model in `docs/TODO.md` is accepted as-is:
`verified-agf-v2`, `adapted-from-v1x`, `inherited-v1x-unverified`,
`retired-for-agf-v2`. Dispositions live inline in each artifact, validated
by Elixir tooling. A generated audit manifest is the output artifact, not
the source of truth.

## Closing seq 257 and 258

Both handoffs are answered by this decision. No further action required on
either.
