<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Integration Contract

> **AGF v2 status note:** This contract was inherited from the v1.x/Python
> agents-govern line. It remains useful historical reference for integration
> concepts, but several paths and command names below are not the current AGF-NG
> v2 public command surface. For current v2 adopter guidance, use
> `agf/docs/site/docs/advanced/contracts-and-validators.md` and the
> `tooling/rust/` reference implementation (`agov` binary). Treat any
> `agents/`, `learnings/`, `schemas/`, or `agov-validate-*` examples in this
> file as v1.x legacy unless a v2 page explicitly repeats them.

How a Continuous Integration (CI) engine, an external policy engine,
or any other consumer system **integrates** with agents-govern's
governance artefacts. This page is the canonical home for the
framework's machine-checkable surface — what shipped, what is in
flight, and where the boundaries are.

For per-tool prompts and rule loading, see
[Tool enforcement](tool-enforcement.md). The two pages are
complementary: tool-enforcement is per-tool ("how does Cursor load
rules?"); this page is per-CI-engine and language-agnostic ("how
does any CI engine consume the framework's artefacts?").

## Architecture: pull-based

agents-govern is **pull-based**: the consumer (CI engine, Integrated
Development Environment (IDE) plugin, alternative validator stack)
reads governance artefacts and
runs validators that exit with a status code. The framework does
not push configuration into a CI engine; it does not generate
pipeline files; it does not require a specific CI vendor.

```text
┌─────────────────┐         ┌─────────────────────────────┐
│  CI engine      │ reads → │  Governance artefacts        │
│  (any vendor)   │         │  agents/*.yaml               │
│                 │         │  learnings/*.yaml            │
│                 │         │  agf/rules/*.md              │
│                 │         │  framework.yaml              │
│                 │         │  schemas/*.schema.json       │
│                 │         │  schemas/invariants.yaml     │
└────────┬────────┘         └─────────────────────────────┘
         │ runs
         ▼
┌─────────────────┐         ┌─────────────────────────────┐
│  Validators     │ exits → │  Status code + report        │
│  (agov-*)       │         │  0 pass; non-zero fail       │
└─────────────────┘         └─────────────────────────────┘
```

**Why pull-based.** Composes with any CI engine (GitLab CI, GitHub
Actions, Jenkins, Buildkite, local pre-commit hooks, etc.). The
adopter writes their own pipeline file; the framework provides the
validators it expects to be called.

**Push-based generation** (e.g., emitting Open Policy Agent (OPA)
rego rules or generating CI configuration files from agent
jurisdictions) is **out of scope** — see *Out of scope* below.

## Schema-of-record hierarchy

When the framework's structural rules need to be expressed in two
places (executable validators *and* declarative artefacts for
external tooling), the current AGF v2 contract follows the Rust
`agov` validator surface documented in the adopter docs. Historical v1.x
Python validators are legacy reference material, not the AGF v2 operator path.

| Artefact | Status | Authoritative? |
|---|---|---|
| Rust `agov` validators | Shipped for current v2 checks | **Yes** — current v2 operator surface |
| `src/agents_govern/validate_*.py` | Historical v1.x | No — legacy reference |
| Per-artefact JavaScript Object Notation (JSON) Schema (`schemas/*.schema.json`) | **Shipped v1.1.46** ([C1]) | No — consumer-facing |
| Cross-artefact invariant manifest (`schemas/invariants.yaml`) | **Shipped v1.1.46** ([C2]) | No — declarative summary |

**Why executable validators win the tie.** Cross-field invariants (manifest ↔
agent files, communication graph closure, append-only learning
fields) cannot be natively expressed in JSON Schema. JSON Schema
remains the *consumer-facing* artefact (editor integration, OPA
input, alternative validator stacks); the validators remain the
*contract*.

**Drift risk.** If JSON Schema export stops matching validator
behaviour, the validator wins by definition. The mitigation
shipped in v1.1.46:

- [`scripts/generate-schemas.py`](https://gitlab.com/janipaijanen/agent-governance/-/blob/main/scripts/generate-schemas.py)
  cross-checks every `schemas/*.schema.json` against the Draft 7
  meta-schema and confirms `schemas/invariants.yaml` matches the
  C2 normalised shape.
- [`tests/test_schemas.py`](https://gitlab.com/janipaijanen/agent-governance/-/blob/main/tests/test_schemas.py)
  enforces *parity* — every `agents/*.yaml` and `learnings/*.yaml`
  validates against its schema, and the invariant manifest's
  parameter values are cross-checked against the validator
  source.

## Five-layer verifiability model

What is verifiable today, what is not, and where the boundary is.
The model surfaced in the v1.1.21 ChatGPT engagement and is the
single most useful abstraction the engagement produced. This page
is its canonical home; the [Reviewer reality map](start/reviewer-reality-map.md#five-layer-verifiability-model)
and [Position landing](position/index.md#what-is-verifiable-today-five-layer-model)
carry summaries pointing here.

| Layer | Verifiable today? | Failure mode | Scope |
|---|---|---|---|
| 1. Artefact compliance (schema, cross-field) | **Yes** | Validator fails CI | In-framework |
| 2a. Gate execution (did CI run?) | Partial | CI job missing or misconfigured | In-framework + CI engine |
| 2b. Decision recording (approvals logged?) | Partial | Approval missing from artefacts | In-framework + CI engine |
| 3. Binding (artefact ↔ execution-trace correlation) | Planned/deferred in AGF v2 | UUID v7 correlation IDs (B1) + `Agent-ID:` commit trailers (B2) + `<artefact>.agov.json` sidecars / filename-suffix fallback (B3) are specified, but Rust `agov` does not yet validate them | Planned Full-tier surface |
| 4. Runtime behaviour (agent obeyed jurisdiction?) | **No, by design** | Out-of-framework | Runtime layer's responsibility |

**Reading the layers.**

- **Layer 1** is the framework's strongest claim. Run a validator;
  get a deterministic answer. This is what the framework does best.
- **Layer 2** is partially verifiable: the CI job's exit code
  records that the gate ran, and review-comment artefacts record
  the approval. *"Did CI actually run on this Merge Request (MR)?"*
  is verifiable by checking pipeline status; *"was the approval
  reviewer-driven, not rubber-stamped?"* is reviewer-judgement,
  not mechanically verifiable.
- **Layer 3** ties artefacts to execution traces via correlation
  identifiers, commit trailers, and sidecar metadata. In current AGF v2 this is
  a planned/deferred Full-tier surface: the shape is specified, but Rust `agov`
  does not yet ship binding validation. See
  [Binding contract](binding-contract.md) for the full specification.
- **Layer 4** is the explicit hard boundary. The framework does not
  enforce runtime behaviour — see [Position overview's boundary
  refinement](position/overview.md#boundary-refinement-2026-05-04).

## Validator command surface

AGF-NG v2 exposes tooling as the Rust `agov` binary under `tooling/rust/`.
The v1.x Python entry points remain historical compatibility context and are
not the current v2 public command surface.

| Command | Validates | Default scope |
|---|---|---|
| `mix agov.validate.agents` | `agents/*.yaml` for schema and tier checks | Whole `agents/` directory |
| `mix agov.validate.learnings` | `learnings/*.yaml` for schema, anonymisation heuristics, and confidence corroboration thresholds | Whole `learnings/` directory |
| `mix agov.validate.infosec` | Learning records for token-like material, connection strings, key/password patterns, private keys, and absolute filesystem paths | Whole `learnings/` directory |
| `mix agov.validate.adoption` | Required adoption layout, mandatory advisory agents, manifest presence, and governance documentation | Whole repo |
| `mix agov.validate.contributions` | Advisory contribution warnings for absence-of-incident claims, governance relaxation language, and weak evidence | Whole `learnings/` directory |
| `mix agov.validate.framework_config` | Optional `framework.yaml` validator against `framework.schema.json`; passes when no `framework.yaml` exists | Repo root |
| `mix agov.validate` | Aggregate runner for the migrated validator set | Whole repo |
| `mix agov.dist.check_tag` | Version / tag consistency checks for release validation | Repo root or CI tag context |
| `mix agov.dist.build` | Build the release archive | Repo root |

Current AGF v2 status: binding validation is deferred; acronym validation,
`agov-bump`, and `agov --capabilities` / `agov --contract-changelog` are
retired; `agov-health` is deferred. README statistics validation is available
as `agov validate readme-stats`.

**Canonical CI invocation today** is either per-validator Mix tasks or the
single migrated orchestrator `mix agov.validate`.

## Path-based revalidation triggers

Trigger set: any change under one of the path globs below should
invoke the relevant validator(s) on the affected files.

| Path glob | Triggers |
|---|---|
| `agents/**` | `mix agov.validate.agents` |
| `agents/MANIFEST.yaml` | `mix agov.validate.adoption` |
| `learnings/**` | `mix agov.validate.learnings`, `mix agov.validate.infosec`, `mix agov.validate.contributions` |
| `framework.yaml` | `mix agov.validate.framework_config` |
| Release manifest / release metadata | `mix agov.dist.validate`, `mix agov.dist.check_tag`, `mix agov.dist.build` as applicable |

README statistics and acronym validators are deferred to E4 in the Elixir
migration. Until they migrate, those checks are manual or v1.x/Python-era.

**Pre-merge enforcement: required.** Non-zero exit blocks the merge
via branch protection rules on `main`. The framework's own
[GitLab CI configuration](https://gitlab.com/janipaijanen/agent-governance/-/blob/main/.gitlab-ci.yml)
is the reference implementation.

**Post-merge enforcement: recommended.** A second pass on `main`
catches drift if a path-triggered job was misconfigured pre-merge.
Failure on post-merge opens an issue automatically; does not
revert.

## Exit codes

**Taxonomy** (defined by the AGF contract and implemented by current
validators as they adopt the richer failure modes):

| Code | Meaning |
|---|---|
| `0` | Pass |
| `1` | Validation failure (single-artefact field-level) |
| `2` | Configuration error (validator cannot determine what to check) |
| `3` | Invariant violation (cross-artefact rule break) |
| `4` | Infrastructure / Input/Output (I/O) failure (validator could not run) |

Precedence (orchestrator emits highest):
`infrastructure > configuration > invariant > validation`. Aggregate
report enumerates all findings regardless of which code is emitted.
The precedence helper is `agents_govern._exit_codes.highest_precedence()`.

**Validator state today.** Most validators currently return `0` or
`1` only. Codes `2`, `3`, `4` are defined in the taxonomy and
available for use; individual validators will adopt them
opportunistically as new failure modes arise. Existing CI scripts
branching on `!= 0` continue to work without change.

**Migration discipline.** Adopters branching CI scripts on `!= 0`
are unaffected. Adopters branching on `== 1` may see a different
non-zero code in some scenarios when validators adopt 2/3/4; the
contract-stability promise (next section) names this as a
minor-version-additive change.

## Output formats

**Today** migrated AGF-NG validators emit human-readable text on standard
output and exit with a code. In v1.x, the orchestrator (`agov-validate`,
shipped v1.1.37) additionally emitted a single JavaScript Object Notation
(JSON) aggregate with the `--json` flag. That JSON shape remains historical
contract context:

```json
{
  "exit_code": 1,
  "validators": [
    {"name": "validate-agents", "exit_code": 0, "output": "…captured stdout…"},
    {"name": "validate-infosec", "exit_code": 0, "output": "…"}
  ]
}
```

**Planned**: Static Analysis Results Interchange Format (SARIF) for
security-relevant validators (e.g., `mix agov.validate.infosec`) since SARIF is
the GitLab / GitHub native security-finding format.

## Orchestrator

**Today** AGF-NG has these Elixir aggregate/release entry points:

- `mix agov.validate` — runs the migrated validator set and exits non-zero if
  any blocking validator fails.
- `mix agov.dist.check_tag` — performs release/tag consistency checks.
- `mix agov.dist.build` — builds the release archive.

The v1.x Python `agov-validate` and `agov-pre-tag-check` entry points remain
historical context for the contract but are not the current AGF-NG command
surface. Individual validators remain callable directly for targeted Pull
Request (PR) jobs.

Convention follows tools like `pre-commit run --all-files` and
`make test` — single entry point with deterministic aggregation,
individual validators remain callable directly for targeted use.

## Contract versioning

**The contract version equals the framework version.** Single
version number; no dual-version drift.

**Stability promise** — canonical home:
[Contract stability](contract-stability.md) (shipped v1.1.41).
Summary:

- **Patch releases** never change the integration contract.
- **Minor releases** may *extend* the contract (additive new fields,
  validators, exit codes in the additive ranges). Existing adopters
  keep working.
- **Major releases** may *change* the contract. Migration guide
  required; previous major version supported through one deprecation
  cycle.

The [Contract stability](contract-stability.md) page carries the
worked-examples table from this release thread, the
*what-is-covered / what-is-not* split (loose-prose surface stays
out), the adopter-recommendations list, and the major-release
deprecation-cycle policy.

Command Line Interface (CLI) surface — historical v1.x:

- `agov --version` — framework version (= contract version).
- `agov --contract-version` — alias.
- `agov --contract-changelog` — retired in AGF v2; use changelog/release notes
  directly.

## Capability reporting

**Historical v1.x surface:** `agov --capabilities` emitted a JSON report.
AGF-NG v2 retires that command; use current docs, manifests, and
`agov validate` output for the shipped validator surface. Historical output
shape:

```json
{
  "framework_version": "1.1.47",
  "contract_version": "1.1.47",
  "capabilities": {
    "artifact_validation": true,
    "precedence_enforcement": true,
    "confidence_tracking": true,
    "binding": true,
    "tension_recording": true
  },
  "missing_for_full_compliance": []
}
```

In AGF-NG v2, binding is not a current shipped Rust validator and should be
treated as planned/deferred. Do not use the historical v1.x capability report
to claim current v2 Full-tier support.

The `missing_for_full_compliance` list is the actionable surface — it
lists exactly the capabilities currently `False`, in declaration
order. Adopters targeting Full read this list and know what to wait
for. Once `framework.yaml` ships ([P1]) the report will compare
against an adopter-declared target tier (Artefact-only /
Process-aware / Full per
[Compliance profile](../adopt/compliance-profile.md)); today the report
is unconditional.

Three compliance tiers (canonical home:
[Compliance profile](../adopt/compliance-profile.md), shipped
v1.1.38 closing [D5]):

- **Artefact-only:** layer 1 (validation) enforced.
- **Process-aware:** layers 1 + 2a + 2b enforced.
- **Full:** layers 1 + 2 + 3 enforced (binding via correlation IDs,
  attribution via trailers, sidecars for non-commit artefacts).

The [Compliance profile](../adopt/compliance-profile.md) page carries
the tier-to-layer mapping, the degraded-modes table (what each
lower tier gives up), the *how to choose your tier* decision
shortcut, and the framework's own tier — declared in
[`framework.yaml`](https://gitlab.com/janipaijanen/agent-governance/-/blob/main/framework.yaml)
as `compliance_target: process-aware` from v1.1.47 onward.

## Binding contract

**Layer 3** of the verifiability model. In current AGF v2, binding is a
planned/deferred Full-tier surface rather than a shipped Rust `agov` validator.
The v1.x line specified the three legs below:

- [B1] **Correlation Identifier (ID) convention** — Universally
  Unique Identifier (UUID) v7, lifecycle owned by Governor-Reviewer,
  propagation in commit trailers and log fields and sidecar
  metadata.
- [B2] **Commit trailer schema** — `Agent-ID: <agent-name>:<version>:<correlation-id>`,
  branch-level enforcement policy.
- [B3] **Sidecar metadata + filename fallback** — `<artefact>.agov.json`
  with `correlation_id`, `agent_id`, `produced_at`, `artefact_sha256`;
  filename-suffix fallback for environments stripping non-image
  files.

The full specification — UUID v7 format, lifecycle table, multi-
trailer policy, branch-classification rules, sidecar JSON shape,
hash verification, filename-suffix fallback, schema-only vs
enforce mode, migration discipline — lives on
[Binding contract](binding-contract.md) (canonical home).

**Boundary clarification.** Binding requires runtime *artefact
production* (structured logs, signed commits, sidecar metadata) but
does not enforce runtime *behaviour*. See
[Position overview's boundary refinement](position/overview.md#boundary-refinement-2026-05-04)
for the framing.

## Implementation status

| Area | Status | Issue |
|---|---|---|
| Validator surface | **Rust `agov` is current AGF v2 tooling** | — |
| Path-based revalidation triggers | **Shipped** (in framework's own `.gitlab-ci.yml`) | [A2] |
| Pull-based architecture commitment | **Shipped** (this page) | [A1] |
| Schema-of-record hierarchy | **Shipped** (this page) | [A1] |
| Five-layer verifiability model | **Shipped** (this page; canonical home) | [A1], [D4] |
| Exit code taxonomy (5-code) | **Shipped** (defined v1.1.36; validators adopt 2/3/4 opportunistically) | [A4] |
| `agov validate` orchestrator | **Shipped in AGF-NG v2 Rust tooling**; v1.x `agov-validate --json` remains historical context | [A3] |
| JSON aggregate output | **Historical v1.x surface**; SARIF still planned | [A3] |
| Contract versioning + stability promise | **Shipped conceptually**; `agov --contract-changelog` is retired in AGF v2 | [A5] |
| `agov --capabilities` report | **Retired in AGF v2** | [A6] |
| Compliance profile (3 tiers) | **Shipped** (v1.1.38; canonical home at [`adopt/compliance-profile.md`](../adopt/compliance-profile.md)) | [D5] |
| Binding contract — correlation identifier + trailer | **Specified; Rust validator deferred** ([Binding contract](binding-contract.md)) | [B1], [B2] |
| Binding contract — sidecar metadata + filename fallback | **Specified; Rust validator deferred** | [B3] |
| Per-artefact JSON Schema | **Shipped v1.1.46** ([`schemas/`](https://gitlab.com/janipaijanen/agent-governance/-/tree/main/schemas)) | [C1] |
| Cross-artefact invariant manifest | **Shipped v1.1.46** ([`schemas/invariants.yaml`](https://gitlab.com/janipaijanen/agent-governance/-/blob/main/schemas/invariants.yaml)) | [C2] |
| `framework.yaml` (precedence + binding config) | **Validator migrated as `mix agov.validate.framework_config`** | [P1] |
| Mandatory `category: tension` recording for cross-layer conflicts | **Shipped v1.1.47** (canonical home at [Concept — Tension recording](concept/principles.md#tension-recording); `learnings/_template.yaml` header naming the discipline) | [P2] |

The 21 issues filed under
[label `engagement-action-plan-2026-05-04`](https://gitlab.com/janipaijanen/agent-governance/-/issues?label_name%5B%5D=engagement-action-plan-2026-05-04)
track the in-flight items.

## Out of scope

Things the framework explicitly will **not** do, and the reasons:

- **Push-based contract generation.** Generating CI configuration
  files, OPA rego rules, or vendor-specific policy bundles from
  agent jurisdictions. Pull-based composes with any CI engine;
  push-based couples the framework to specific vendors. Reviewed
  and rejected in the v1.1.21 ChatGPT engagement (round-2 §5).
- **Hierarchical correlation identifiers.** Parent-child correlation
  IDs invite distributed-tracing problems (clock skew, fan-out
  reconciliation, cross-pipeline joins) the framework does not
  need. Single ID per task; subtasks inherit. Reviewed and rejected
  in the v1.1.21 ChatGPT engagement (round-5 §6).
- **Cryptographic integrity guarantees.** Signed commits, hash-chained
  audit logs, tamper-resistant log stores. Belong to the
  infrastructure layer the framework explicitly delegates. The
  framework's contract is to *require* an integrity-preserving log
  store, not to provide one.
- **Runtime-behaviour enforcement** (layer 4 of the verifiability
  model). The hard boundary. Tool-call permissioning, sandboxing,
  execution blocking are the runtime layer's responsibility — see
  [Position overview](position/overview.md).

## See also

- [Reviewer reality map](start/reviewer-reality-map.md) — the
  thirty-second view, with cross-link to the five-layer model
  here.
- [Position overview](position/overview.md) — the
  process-vs-enforcement framing and the
  [boundary refinement](position/overview.md#boundary-refinement-2026-05-04).
- [Tool enforcement](tool-enforcement.md) — per-tool rule loading
  (Claude Code, Cursor, Windsurf, GitHub Copilot, OpenAI Codex
  CLI). Sibling page.
- [external-ai-engagements/REVIEWER-BRIEF.md](https://gitlab.com/janipaijanen/agent-governance/-/blob/main/external-ai-engagements/REVIEWER-BRIEF.md)
  — the brief that elicited the v1.1.21 engagement that produced
  the contract specification.
- [`docs/plans/2026-05-04-chatgpt-engagement-action-plan.md`](plans/2026-05-04-chatgpt-engagement-action-plan.md)
  — the action plan tracking the planned items above.
