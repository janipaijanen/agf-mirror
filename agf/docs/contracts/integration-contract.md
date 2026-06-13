<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Integration Contract

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
│                 │         │  .claude/rules/*.md          │
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
external tooling), the **Python validators are the contract of
record**. Other artefacts are generated, consumer-facing, and may
lag the validators.

| Artefact | Status | Authoritative? |
|---|---|---|
| `src/agents_govern/validate_*.py` | Shipped | **Yes** — contract of record |
| Per-artefact JavaScript Object Notation (JSON) Schema (`schemas/*.schema.json`) | **Shipped v1.1.46** ([C1]) | No — consumer-facing |
| Cross-artefact invariant manifest (`schemas/invariants.yaml`) | **Shipped v1.1.46** ([C2]) | No — declarative summary |

**Why validators win the tie.** Cross-field invariants (manifest ↔
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
| 3. Binding (artefact ↔ execution-trace correlation) | **Yes (v1.1.45)** | Mechanically supported via UUID v7 correlation IDs (B1) + `Agent-ID:` commit trailers (B2) + `<artefact>.agov.json` sidecars / filename-suffix fallback (B3) | In-framework — [B1]–[B3] all shipped |
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
  identifiers, commit trailers, and (for non-commit artefacts)
  sidecar metadata. [B1] (UUID v7 generator) and [B2] (`Agent-ID:`
  trailer parser / branch classifier / `agov-validate-binding`)
  shipped in v1.1.44; [B3] (`<artefact>.agov.json` sidecar +
  filename-suffix fallback for environments stripping non-image
  files) shipped in v1.1.45. The umbrella `binding` capability is
  now `true` in the [capability report](#capability-reporting). See
  [Binding contract](binding-contract.md) for the full
  specification.
- **Layer 4** is the explicit hard boundary. The framework does not
  enforce runtime behaviour — see [Position overview's boundary
  refinement](position/overview.md#boundary-refinement-2026-05-04).

## Validator command surface

The framework ships these validators. Each is a Python entry point
exposed in `pyproject.toml [project.scripts]`.

| Command | Validates | Default scope |
|---|---|---|
| `agov-validate-agents` | `agents/*.yaml` and `agents/MANIFEST.yaml` for structural correctness, communication graph closure, ownership overlap detection | Whole `agents/` directory |
| `agov-validate-learnings` | `learnings/*.yaml` for schema, append-only invariant, anonymisation policy | Whole `learnings/` directory |
| `agov-validate-infosec` | Learning records for information leaks (credentials, organisation identifiers, etc.) before publication | Per-file or whole directory |
| `agov-validate-acronyms` | Markdown files in the repository for first-use acronym expansion (per [.claude/rules/docs/abbreviation-rule.md](https://gitlab.com/janipaijanen/agent-governance/-/blob/main/.claude/rules/docs/abbreviation-rule.md)) | Per-file (Pull Request (PR) discipline) or whole repo (drain) |
| `agov-validate-adoption` | The framework's own adoption discipline (ADOPTING.md steps, self-conformance) | Whole repo |
| `agov-validate-contributions` | Contribution metadata and release-archive integrity | Whole repo |
| `agov-validate-readme-stats` | The README.md "X projects, Y learnings" claim against actual corpus state | Top-level README |
| `agov-validate-binding` | Layer 3 binding contract — UUID v7 correlation identifiers (B1), `Agent-ID:` commit trailers (B2), and `<artefact>.agov.json` sidecar metadata + filename-suffix fallback (B3). Reads `framework.yaml` `binding:` section when present (added v1.1.47); see [Binding contract](binding-contract.md) | Whole repo (commit history + artefact sidecars) |
| `agov-validate-framework-config` | `framework.yaml` validator — schema check (additionalProperties: false per round-5 §7), contract_version equals VERSION, compliance_target enum. Opt-in: passes when no `framework.yaml` exists. See [`framework.yaml`](https://gitlab.com/janipaijanen/agent-governance/-/blob/main/framework.yaml) reference | Repo root |
| `agov` | Umbrella entry point: `--version` / `--contract-version` (alias) / `--contract-changelog` (A5) / `--capabilities` (A6 JSON capability report) — see *Contract versioning* and *Capability reporting* below | n/a (introspection only) |
| `agov-validate` | Aggregate runner for the default validator set; emits highest-precedence exit code with `--json` support — see *Orchestrator* below | Whole repo |
| `agov-pre-tag-check` | All validators above plus pre-release-only checks (VERSION / CHANGELOG / tag / working-tree / graph generation) — see *Orchestrator* below | Whole repo |
| `agov-validate-tag-push` | Pre-push hook validating tag-vs-VERSION consistency | Single tag |
| `agov-bump` | Atomically bump VERSION + CHANGELOG skeleton + 12 agent YAMLs + README citation | Whole repo |
| `agov-dist` | Build the release archive | Whole repo |
| `agov-health` | Self-check the framework's own state | Whole repo |

**Canonical CI invocation today** is per-validator (each validator
is its own CI job in the framework's own
[`.gitlab-ci.yml`](https://gitlab.com/janipaijanen/agent-governance/-/blob/main/.gitlab-ci.yml)).
A single-entry-point orchestrator (`agov-validate`) ships as of
v1.1.37 — see *Orchestrator* below.

## Path-based revalidation triggers

Trigger set: any change under one of the path globs below should
invoke the relevant validator(s) on the affected files.

| Path glob | Triggers |
|---|---|
| `agents/**` | `agov-validate-agents`, `agov-validate-readme-stats` |
| `agents/MANIFEST.yaml` | `agov-validate-agents` |
| `learnings/**` | `agov-validate-learnings`, `agov-validate-infosec`, `agov-validate-contributions`, `agov-validate-readme-stats` |
| `.claude/rules/**` | `agov-validate-readme-stats` (count of rule templates) — no full coherence validator yet (planned: `agov-validate-coherence`) |
| `rules/**` | `agov-validate-readme-stats` — same as above |
| `skills/**` | `agov-validate-readme-stats` |
| `framework.yaml` | `agov-validate-framework-config` (shipped v1.1.47, [P1]) |
| `*.md` (changed in PR / staged) | `agov-validate-acronyms` |
| `README.md` | `agov-validate-readme-stats` |

`agov-validate-readme-stats` appears against several corpus paths
because the README's stat claims (project / learning / rule / agent
/ skill counts) drift whenever the underlying corpus changes, not
only when the README is edited. Triggering on the corpus paths
catches the drift at MR time instead of waiting for `agov-pre-tag-check`
at release.

**Pre-merge enforcement: required.** Non-zero exit blocks the merge
via branch protection rules on `main`. The framework's own
[GitLab CI configuration](https://gitlab.com/janipaijanen/agent-governance/-/blob/main/.gitlab-ci.yml)
is the reference implementation.

**Post-merge enforcement: recommended.** A second pass on `main`
catches drift if a path-triggered job was misconfigured pre-merge.
Failure on post-merge opens an issue automatically; does not
revert.

## Exit codes

**Taxonomy** (defined in
[`src/agents_govern/_exit_codes.py`](https://gitlab.com/janipaijanen/agent-governance/-/blob/main/src/agents_govern/_exit_codes.py)
as of v1.1.36, available for validator and orchestrator use):

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

**Today** individual validators emit human-readable text on standard
output and exit with a code. The orchestrator (`agov-validate`,
shipped v1.1.37) additionally emits a single JavaScript Object
Notation (JSON) aggregate with the `--json` flag. The JSON shape is
part of the strict-contract surface:

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
security-relevant validators (e.g., `agov-validate-infosec`) since
SARIF is the GitLab / GitHub native security-finding format.

## Orchestrator

**Today** there are two orchestrator entry points:

- [`agov-validate`](https://gitlab.com/janipaijanen/agent-governance/-/blob/main/src/agents_govern/orchestrator.py)
  (shipped v1.1.37) — the Continuous Integration (CI)-friendly
  aggregate runner. Runs the default validator set (every
  no-argument-required validator), captures per-validator stdout,
  and emits the highest-precedence exit code per the taxonomy
  above. `--json` switches output from per-validator text blocks
  to the strict-contract JSON shape documented in *Output formats*.
- [`agov-pre-tag-check`](https://gitlab.com/janipaijanen/agent-governance/-/blob/main/src/agents_govern/pre_tag_check.py)
  — the pre-release composite. Runs the same per-validator
  aggregation as `agov-validate` (via the shared
  `orchestrator.run_validator` helper) plus pre-release-only
  checks (VERSION / CHANGELOG / tag existence / working-tree /
  README citation / self-conformance / communication graph
  generation). Exits with the highest non-zero code among the
  composite checks.

Both entry points consume the same shared helper at
`src/agents_govern/orchestrator.py`, so the aggregation logic has a
single implementation. Individual validators remain callable
directly for targeted Pull Request (PR) jobs.

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

Command Line Interface (CLI) surface — *shipped v1.1.43*:

- `agov --version` — framework version (= contract version).
- `agov --contract-version` — alias.
- `agov --contract-changelog` — prints the framework's
  [`CHANGELOG.md`](https://gitlab.com/janipaijanen/agent-governance/-/blob/main/CHANGELOG.md);
  filter for entries referencing validator command names, exit
  codes, output formats, etc. to see contract-affecting changes.

## Capability reporting

**Shipped v1.1.43** ([A6]): `agov --capabilities` emits a JSON report.
Today's actual output shape (run `agov --capabilities` for the live
version):

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

The `binding` flag flipped to `true` in v1.1.45 when [B3] joined
the already-shipped [B1] and [B2]; the umbrella now covers the
full three-leg Layer 3 contract per
[Binding contract](binding-contract.md). The `tension_recording`
flag flipped to `true` in v1.1.47 with [P2] (mandatory
`category: tension` recording for cross-layer conflicts; canonical
home at
[Concept — Tension recording](concept/principles.md#tension-recording)).
With both flips landed, `missing_for_full_compliance` is empty —
the strict-contract surface is complete at v1.1.47.

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

**Layer 3** of the verifiability model. Three issues close it; all
three shipped across v1.1.44 and v1.1.45:

- [B1] **Correlation Identifier (ID) convention** — Universally
  Unique Identifier (UUID) v7, lifecycle owned by Governor-Reviewer,
  propagation in commit trailers and log fields and sidecar
  metadata. **Shipped v1.1.44** as
  [`src/agents_govern/correlation_id.py`](https://gitlab.com/janipaijanen/agent-governance/-/blob/main/src/agents_govern/correlation_id.py).
- [B2] **Commit trailer schema** — `Agent-ID: <agent-name>:<version>:<correlation-id>`,
  branch-level enforcement policy. **Shipped v1.1.44** as
  [`src/agents_govern/validate_binding.py`](https://gitlab.com/janipaijanen/agent-governance/-/blob/main/src/agents_govern/validate_binding.py)
  with the `agov-validate-binding` Command Line Interface (CLI)
  entry point.
- [B3] **Sidecar metadata + filename fallback** — `<artefact>.agov.json`
  with `correlation_id`, `agent_id`, `produced_at`, `artefact_sha256`;
  filename-suffix fallback for environments stripping non-image
  files. **Shipped v1.1.45** as additional helpers in
  `validate_binding.py` (`Sidecar`, `parse_sidecar`,
  `compute_artefact_sha256`, `read_sidecar`, `verify_sidecar`,
  `parse_filename_fallback`).

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
| Validator surface | **Shipped** | — |
| Path-based revalidation triggers | **Shipped** (in framework's own `.gitlab-ci.yml`) | [A2] |
| Pull-based architecture commitment | **Shipped** (this page) | [A1] |
| Schema-of-record hierarchy | **Shipped** (this page) | [A1] |
| Five-layer verifiability model | **Shipped** (this page; canonical home) | [A1], [D4] |
| Exit code taxonomy (5-code) | **Shipped** (defined v1.1.36; validators adopt 2/3/4 opportunistically) | [A4] |
| `agov-validate` orchestrator | **Shipped** (v1.1.37; shared helper consumed by `agov-pre-tag-check`) | [A3] |
| JSON aggregate output | **Shipped** (v1.1.37 `--json`); SARIF still planned | [A3] |
| Contract versioning + stability promise | **Shipped** (v1.1.41 docs at [`contract-stability.md`](contract-stability.md); v1.1.43 CLI flags `agov --version` / `--contract-version` / `--contract-changelog`) | [A5] |
| `agov --capabilities` report | **Shipped** (v1.1.43; JSON shape under *Capability reporting* above) | [A6] |
| Compliance profile (3 tiers) | **Shipped** (v1.1.38; canonical home at [`adopt/compliance-profile.md`](../adopt/compliance-profile.md)) | [D5] |
| Binding contract — correlation identifier + trailer | **Shipped v1.1.44** ([Binding contract](binding-contract.md); validator `agov-validate-binding`) | [B1], [B2] |
| Binding contract — sidecar metadata + filename fallback | **Shipped v1.1.45** (`Sidecar`, `verify_sidecar`, `parse_filename_fallback`) | [B3] |
| Per-artefact JSON Schema | **Shipped v1.1.46** ([`schemas/`](https://gitlab.com/janipaijanen/agent-governance/-/tree/main/schemas)) | [C1] |
| Cross-artefact invariant manifest | **Shipped v1.1.46** ([`schemas/invariants.yaml`](https://gitlab.com/janipaijanen/agent-governance/-/blob/main/schemas/invariants.yaml)) | [C2] |
| `framework.yaml` (precedence + binding config) | **Shipped v1.1.47** ([`framework.yaml`](https://gitlab.com/janipaijanen/agent-governance/-/blob/main/framework.yaml); validator `agov-validate-framework-config`) | [P1] |
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
