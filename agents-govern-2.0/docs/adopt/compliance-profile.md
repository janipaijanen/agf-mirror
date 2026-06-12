<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Compliance profile

Which verifiability layers an adopter enforces. The framework
defines [five layers of verifiability](../contracts/integration-contract.md#five-layer-verifiability-model);
not every adopter needs all of them. A *compliance profile* names
the subset an adopter has chosen to enforce, so reviewers,
auditors, and downstream consumers can read the conformance
claim against an explicit standard.

This page is the canonical home for the three-tier model. Action
plan item [D5] (issue #68) from the
[2026-05-04 ChatGPT engagement](../plans/2026-05-04-chatgpt-engagement-action-plan.md).

## How this differs from adoption intensity and domain profile

Three orthogonal concepts the framework uses, each answering a
different question:

| Concept | Question it answers | Examples | Page |
|---|---|---|---|
| **Adoption intensity** | *How many gates apply?* | Minimal / Standard / Regulated / Safety-critical | [Adoption intensity guide](intensity-guide.md) |
| **Domain profile** | *What domain-specific concerns layer on top of the core?* | `research-lab`, future `air-gapped`, future `learning-systems` | [Domain profiles](profiles.md) |
| **Compliance profile** (this page) | *Which verifiability layers are enforced?* | Artefact-only / Process-aware / Full | (here) |

A project can sit anywhere on each axis independently. A
solo-developer exploration might run **Minimal** intensity at the
**Artefact-only** tier with no domain profile; a regulated medical
deployment might run **Safety-critical** intensity at the **Full**
tier with a future health-systems profile.

The orthogonality matters because adopters and external reviewers
have repeatedly conflated *"how many gates"* with *"how strictly
those gates are verifiable"*. Naming the axes separately keeps
each decision explicit.

## The three tiers

The compliance vocabulary used on this page (and in the planned
[`framework.yaml`](../contracts/integration-contract.md#capability-reporting)
target-tier declaration). Defined here before any later table
references them:

- **Artefact-only.** The adopter enforces *layer 1* of the
  five-layer model: schema and cross-field validation on
  governance artefacts (agent definitions, learning records,
  rule files, etc.). The Continuous Integration (CI) engine runs
  validators and blocks merges on non-zero exit. Nothing else is
  mechanically enforced.
- **Process-aware.** Artefact-only plus *layers 2a and 2b*: the
  CI engine records that gates ran (pipeline status), and the
  governance artefacts record approvals (review comments,
  reviewer signoff, escalation closure notes). A reader can
  verify both *that the gate ran* and *that the decision was
  recorded*.
- **Full.** Process-aware plus *layer 3*: every artefact carries
  a binding identifier (Universally Unique Identifier (UUID) v7
  correlation Identifier (ID), commit trailers, sidecar metadata
  per [B1]–[B3], shipped v1.1.44–v1.1.45) so a reader can link any
  artefact back to the execution trace that produced it.

Layer 4 (runtime behaviour) is the explicit hard boundary — out
of scope for every tier. The framework does not enforce runtime
behaviour at any tier. See the
[boundary refinement](../position/overview.md#boundary-refinement-2026-05-04)
for the framing.

## Tier-to-layer mapping

The table below summarises which layers each tier enforces. Read
top-to-bottom: each tier inherits everything the tier above it
enforced. The right-most column names the issue series whose
work, when shipped, makes the tier mechanically achievable.

| Tier | Layer 1: Artefact | Layer 2a: Gate execution | Layer 2b: Decision recording | Layer 3: Binding | Layer 4: Runtime |
|---|---|---|---|---|---|
| **Artefact-only** | ✅ Enforced | — | — | — | (out of scope) |
| **Process-aware** | ✅ Enforced | ✅ Enforced | ✅ Enforced | — | (out of scope) |
| **Full** | ✅ Enforced | ✅ Enforced | ✅ Enforced | ✅ Enforced ([B1]–[B3], shipped v1.1.44–v1.1.45) | (out of scope) |

The framework today ships layers 1 and partial 2a/2b for
Process-aware adopters, plus the binding-contract work for Full
tier as of v1.1.45 (issues
[B1](https://gitlab.com/janipaijanen/agent-governance/-/issues/56),
[B2](https://gitlab.com/janipaijanen/agent-governance/-/issues/57),
[B3](https://gitlab.com/janipaijanen/agent-governance/-/issues/58)
all closed). The
[Binding contract page](../contracts/binding-contract.md) carries the per-leg
specification.

## Degraded modes — what each tier gives up

Lower tiers trade verifiability for setup cost. The table names
what each tier *cannot* claim relative to Full, so adopters and
external auditors can see the trade-off explicitly rather than
discovering it during review.

| Tier | What it gives up vs Full | Failure mode it cannot detect | Operational cost saved |
|---|---|---|---|
| **Artefact-only** | All of layers 2a, 2b, and 3 | "The validator passed but did the CI engine actually run it?"; "Was an approval recorded for this Merge Request (MR)?"; "Which artefact came from which agent?" | No CI-engine instrumentation; no review-comment schema; no binding identifiers in trailers / sidecars |
| **Process-aware** | Layer 3 only | "Which agent produced this artefact?"; "Can I trace this learning record back to the conversation that produced it?" | No UUID v7 generation; no commit-trailer enforcement; no sidecar metadata files |
| **Full** | Nothing at the layers in scope | (catches all in-scope failure modes) | (full setup cost) |

**What no tier catches.** Layer 4 (runtime behaviour — *did the
agent actually obey its jurisdiction at run time?*) is out of
scope at every tier by design. Adopters who need runtime
enforcement compose the framework's artefact-time discipline
with a runtime-layer tool (sandboxing, capability-restricted
execution environments). The framework's contract is to produce
verifiable runtime artefacts, not to enforce runtime behaviour.

## How to choose

Most adopters should target **Process-aware** as a baseline and
**Full** when the binding contract is needed for downstream audit.
The reasoning:

- **Artefact-only is rarely sufficient for team adoption.** It
  catches schema violations but not "did the gate even run on
  this MR?" — a high-frequency drift mode in the framework's own
  evidence corpus (see
  [`learnings/iron-trellis.yaml`](https://gitlab.com/janipaijanen/agent-governance/-/blob/main/learnings/iron-trellis.yaml)
  v0.58.0 entry on the missing pre-merge gate).
- **Full is mechanically achievable as of v1.1.45.** The
  binding-contract work ([B1]–[B3]) shipped across v1.1.44 and
  v1.1.45. Adopters declaring the Full target use
  `agov-validate-binding` plus sidecar metadata on non-commit
  artefacts; see [Binding contract](../contracts/binding-contract.md) for
  the per-leg specification.
- **Process-aware is the cheaper baseline.** All the layers it
  enforces are mechanically achievable today using the shipped
  validators plus the framework's own
  [`.gitlab-ci.yml`](https://gitlab.com/janipaijanen/agent-governance/-/blob/main/.gitlab-ci.yml)
  reference implementation, without the per-task correlation-ID
  generation overhead the Full tier requires.

A short decision shortcut:

- Solo developer, throwaway exploration → **Artefact-only** is
  fine. Schema validation alone catches most copy-paste
  divergence.
- Team project, regulated industry, audit-bound → target
  **Full**. The binding contract is shipped; correlation IDs and
  trailers / sidecars give the audit trail downstream auditors
  expect.
- Anything in between → **Process-aware** is the recommended
  default; promote to Full when binding evidence is needed.

## How tiers are declared

**Shipped v1.1.47** ([P1]): adopters declare their target tier in
`framework.yaml` at the repository root:

```yaml
# Adopter declares which tier their project targets.
compliance_target: process-aware  # one of: artefact-only | process-aware | full
```

`agov-validate-framework-config` validates the file against
[`schemas/framework.schema.json`](https://gitlab.com/janipaijanen/agent-governance/-/blob/main/schemas/framework.schema.json)
(Draft 7, `additionalProperties: false`); `agov --capabilities` reads
the declaration to compute the actionable list.

`agov --capabilities` schema and live output:
[Capability reporting](../contracts/integration-contract.md#capability-reporting).
With v1.1.47 closing the action plan, every shipped capability flips
to `true` and `missing_for_full_compliance` is empty — adopters
targeting any tier can compare against a complete capability map.

## The framework's own tier

The `agents-govern` project declares
**Process-aware** in its
[`framework.yaml`](https://gitlab.com/janipaijanen/agent-governance/-/blob/main/framework.yaml)
(`compliance_target: process-aware`, shipped v1.1.47 with [P1]).
Layer 3 (binding) is mechanically supported as of v1.1.45 but the
framework's own commits are not yet retroactively `Agent-ID:`-
trailer-tagged; promotion to **Full** lands when the framework
starts emitting trailers on its own agent-active branches.

This is recorded explicitly per the
[self-conformance rule](https://gitlab.com/janipaijanen/agent-governance/-/blob/main/.claude/rules/always/self-conformance-rule.md):
a framework that publishes a compliance vocabulary should declare
its own tier in that vocabulary, and the declaration should live in
the same machine-readable surface adopters consume.

## See also

- [Integration contract](../contracts/integration-contract.md) — canonical
  home for the five-layer verifiability model and the validator
  command surface.
- [Adoption intensity guide](intensity-guide.md) — orthogonal
  axis: how many gates apply.
- [Domain profiles](profiles.md) — orthogonal axis:
  domain-specific gate stacks.
- [Position overview's boundary refinement](../position/overview.md#boundary-refinement-2026-05-04)
  — the *no runtime behaviour enforcement; runtime artefact
  production requirements stand* statement that anchors the
  layer 3 / layer 4 split.
- [Reviewer reality map](../start/reviewer-reality-map.md) —
  reviewer-facing summary of what is verifiable today.
