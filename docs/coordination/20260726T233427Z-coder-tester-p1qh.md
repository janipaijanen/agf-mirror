---
kind: handoff
from: coder-tester
to: planner-architect
id: 20260726T233427Z-coder-tester-p1qh
timestamp: 2026-07-26T23:34:27Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260726T231144Z-governor-reviewer-tgj6
  - 20260726T231147Z-governor-reviewer-3ess
  - 20260726T232326Z-coder-tester-kgkh
topic: downstream-extension-namespace-v250-assessment
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Independently challenge the proposed no-shadow extension contract and reserved AGF namespace before turning it into a Gate 1 plan.
    - Propose the smallest coherent pre-v2.5.0 documentation, schema, upgrade, loader, and validation slices; keep implementation separate until Gate 1.
    - Decide whether this should revise MR !661 after its existing blockers are resolved or receive a separate Gate 1 plan; do not silently append it to a blocked umbrella plan.
  must_read:
    - agf/docs/site/docs/advanced/customizing-the-framework.md
    - agf/docs/site/docs/start/adopter-faq.md
    - agf/docs/site/docs/adopt/assistant-loader-configuration.md
    - agf/docs/site/docs/adopt/install-and-layouts.md
    - agf/skills/README.md
    - agf/skills/upgrade/SKILL.md
    - agf/schemas/rule.schema.json
    - agf/schemas/framework.schema.json
    - docs/coordination/20260726T232326Z-coder-tester-kgkh.md
  do_not_touch:
    - agf/VERSION
    - any release branch or tag
    - rust:tooling:preview:windows-package
    - rust:tooling:build:windows:preview
---

# Downstream extensions need identities, not override precedence

The human governor identified a pre-v2.5.0 documentation and contract gap:
adopters need an explicit procedure for proposing project-local rules, skills,
commands, hooks, agents, and similar entities without silently overriding AGF
entities. The governor asked whether AGF should reclaim its prefix and requested
hard argument, improvement, and coordination.

## Observed live gaps

- `customizing-the-framework.md` says project rules are free to add and
  upgrades never touch them, but does not define an adopter-owned extension
  root, a canonical identity, or conflict behavior.
- `rule.schema.json` allows only a bare kebab-case `id` and says it is unique
  "within the rules". It has no namespace or origin field.
- `skills/README.md` uses `origin: project`, but explicitly says tools do not
  interpret it. It does not prevent a project skill or wrapper from reusing an
  upstream name.
- `upgrade/SKILL.md` tells agents to skip rule files with `origin: project`,
  although the current rule schema rejects that property because
  `additionalProperties` is false. The promised preservation mechanism and
  the schema disagree.
- Loader and upgrade docs correctly tell adopters to keep local instructions
  outside copied AGF trees, but never name a standard location or state what
  happens when the local entity conflicts with an AGF entity.
- Existing learning `20260624T104500Z-rp3h` already reserves `agf-*` and
  `agov-*` filenames for framework-owned artifacts and tells adopters to use
  domain-first names. That ownership boundary should be strengthened, not
  reversed.

## Hard recommendation

**No implicit override rule.** Do not define "local wins" or "AGF wins."
Load order is not authority. A duplicate canonical identity or an undeclared
conflict must fail closed and require a governed resolution. Silent shadowing
is a configuration defect.

**Reclaim `agf` and `agov` as reserved namespaces.** This does not mean a
v2.5 mass filename rename. Paths already provide useful ownership context, and
renaming every rule would create churn. Introduce canonical logical identities
instead:

```text
agf:rule:security-hardening
agf:skill:release
agf:command:compact
blue-heron:rule:database-migration-safety
blue-heron:skill:deploy-staging
blue-heron:command:seed-sandbox
```

`agf` and `agov` are upstream-reserved. An adopter declares one stable local
namespace, normally its safe codename, and may not mint identities in the
reserved namespaces. Existing bare upstream IDs remain compatibility aliases
for v2.x; a future major version may make the namespaced identity primary.

For tool surfaces that cannot express colon-separated identities, map the
logical identity to a reserved wrapper name such as `/agf-compact`. Keep legacy
wrappers such as `/compact-agf` as documented compatibility aliases through a
deprecation window. Downstream wrappers use their own namespace or domain name
and may not use `agf-*` or `agov-*`.

## Separate three operations that current prose blurs

1. **Customization:** filling an upstream `<!-- CUSTOMIZE -->` parameter in an
   adopter-owned instantiated projection. This retains the AGF entity identity
   and is permitted only at marked fields. It is not an override.
2. **Extension:** adding a new, uniquely namespaced project entity. Extensions
   may add or tighten constraints. Both the AGF entity and the extension remain
   active; `extends:` must not mean replacement.
3. **Deviation or fork:** weakening, replacing, or contradicting AGF semantics.
   This cannot happen through a duplicate filename, ID, skill name, wrapper, or
   loader order. It requires an explicit human-approved deviation artifact,
   names the affected canonical identities, records lost guarantees and upgrade
   consequences, and may downgrade the project's conformance claim. Contracted
   authority invariants are not overridable while claiming ordinary AGF
   conformance.

## Proposed ownership layout

Keep release-owned templates under `agf/`. Define one adopter-owned default
extension root outside that tree, for example:

```text
governance/
  rules/
  skills/
  commands/
  agents/
  hooks/
```

The exact root should be configurable, but one documented default is necessary
for upgrades, loaders, validators, and examples to agree. Tool-specific files
remain thin adapters that point to either an `agf:*` or adopter-namespaced
canonical entity; they do not become an additional semantic source.

## Conflict semantics

- Duplicate canonical identity: validation error.
- Local use of reserved namespace: validation error.
- Same tool wrapper name mapping to two canonical identities: validation
  error.
- Unique local rule that merely adds a requirement: permitted after normal
  project governance.
- Local rule that conflicts with or weakens an AGF rule: stop and escalate;
  require an explicit deviation/fork decision rather than precedence.
- Upstream acceptance of a local proposal: upstream assigns an `agf:*`
  identity; the adopter deprecates its old local identity through a documented
  migration. The two must not silently coexist as competing rules.

Prose semantics cannot be proved fully by a validator, but identity collisions,
reserved namespace use, duplicate tool mappings, undeclared extension roots,
and upgrade-boundary violations can be caught mechanically. Human Gate 1 and
Gate 2 review still judge whether an alleged extension actually weakens an AGF
constraint.

## Smallest coherent v2.5.0 requirement

Treat this as two independently reviewable slices, not FAQ prose alone:

1. **Contract and guidance:** add an adopter FAQ answer, extend customization,
   loader, upgrade, and skill guidance, reserve namespaces, define the three
   operations above, and include worked rule/skill/command examples.
2. **Machine-checkable boundary:** add backwards-compatible namespace/origin
   fields or an extension registry plus a validator for collisions, reserved
   names, wrapper mappings, and ownership roots. Reconcile the current
   `origin: project` rule-schema contradiction.

If slice 2 cannot safely fit v2.5.0, the release must label slice 1 as a
surface-defined contract and explicitly track enforcement rather than claiming
the collision problem is solved. FAQ-only guidance without even a stable
identity model would document ambiguity, not resolve it.

## Questions the next plan must answer

- Is a separate optional `namespace` field less disruptive than changing
  existing `id` values, with `agf` inferred for legacy upstream artifacts?
- Should one extension registry span rules, skills, commands, agents, hooks,
  and validators, or should each schema carry identity independently?
- Which AGF invariants are contracted/non-deviable, and how is a governed fork
  represented without normalizing "override" as routine configuration?
- Which tool surfaces support `/agf-...` aliases without creating a breaking
  rename, and what deprecation window belongs to legacy names?
- How does the upgrade workflow prove that `agf/` is replaceable while the
  adopter-owned extension root is preserved?
- What negative fixtures prove collision, reserved-prefix misuse, and duplicate
  wrapper mappings fail closed?

Do not implement from this handoff. Independently challenge the model, report
any case where explicit override is actually necessary, and route a bounded
Gate 1 plan only after the governor decides the namespace direction.
