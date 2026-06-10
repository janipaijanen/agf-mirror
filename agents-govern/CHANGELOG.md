<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Changelog

All notable changes to the agents-govern framework are documented here.
Versions follow [semantic versioning](https://semver.org/). Each entry
cites the evidence (learnings, sentinel reviews) that triggered the change.

---























































## v1.1.50 (2026-05-08)

Documentation: make the post-commissioning Quickstart step
agent-native.

### Changed

- **`docs/adopt/index.md`** — replace the manual first-learning
  copy/edit/validate command block with an LLM-directed instruction:
  gather adoption observations, validate a learning record, and
  invoke Capability-Developer only when the observations point to a
  reusable framework improvement.
- **Release metadata** — bump `framework.yaml` `contract_version`,
  `VERSION`, and agent `framework_version` fields to `1.1.50`.

## v1.1.49 (2026-05-08)

Distribution provenance and strict-contract archive parity.

The release archive now ships the strict-contract artefacts that were
introduced across v1.1.32-v1.1.47 but were missing from the packaged
`.zip`: `framework.yaml`, JSON Schemas, invariants, schema generation,
binding and integration contract docs, contract stability docs, and the
adopter compliance profile. `MANIFEST.md` now advertises those files
directly, and its Repository provenance field normalizes SSH origins to
the public HTTPS repository URL.

The README and docs landing page also sharpen the positive framing:
agents-govern is the governance layer of an agentic operating system,
while runtime execution remains deliberately delegated to external
agent runtimes. A new Borrowed Names mapping in the principles docs
spells out exactly how the operating-system analogy maps to shipped
agents-govern mechanisms, and records the runtime boundary where the
analogy stops.

### Fixed

- **`src/agents_govern/dist.py`** — add the strict-contract files to
  the distribution allow-list, copy the targeted schema-generation
  helper, refresh generated `MANIFEST.md` contents, and normalize
  repository provenance URLs.
- **`tests/test_dist_integration.py`** — assert core strict-contract
  artefacts are present in both the built dist directory and `.zip`
  archive.
- **`tests/test_dist.py`** — cover HTTPS credential stripping and SSH
  remote URL normalization.

### Changed

- **`README.md` / `docs/index.md`** — add the agentic operating-system
  governance-layer framing to the "What this is" section.
- **`docs/concept/principles.md`** — add the Borrowed Names mapping
  for the agentic operating-system phrase so the framing stays
  self-conformant.
- **`external-ai-engagements/2026-05-08-lumo-positioning.md`** —
  record Lumo's positioning critique, including the corroborated
  "bold claim → enumeration → delegation" copy pattern and the
  Borrowed Names obligation.
- **`learnings/linked-data-probe.yaml`** — redact a local sandbox
  absolute path surfaced by the InfoSec release gate.
- **`learnings/copper-rapids.yaml`**, **`learnings/pine-ridge.yaml`**,
  **`learnings/silver-index.yaml`** — reword release-gate
  contribution-audit false-positive phrases without changing the
  underlying learning content.
- **`docs/adr/20260505-lean-agile-test-fly.md`**,
  **`docs/adr/20260505-yaml-ld-provisional-artifact-shape.md`**,
  and **`docs/process/*.md`** — replace repo-root relative links
  with public source links or plain artifact names so the Pages
  strict-link gate can build the manifesto-cycle documentation.

## v1.1.48 (2026-05-08)

Documentation: reframe the
[Quickstart (15-minute path, Layout B)](docs/adopt/index.md#quickstart-15-minute-path-layout-b)
section in the Adopt index.

The fastest adoption path is to point an Integrated Development
Environment (IDE)-resident Large Language Model (LLM) — Claude
Code, OpenAI Codex Command Line Interface (CLI), Cursor, Windsurf,
GitHub Copilot Chat — at `ADOPTING.md` and let the agent perform
the commissioning steps. The previous bash-only block buried the
*real* recommended path under a wall of copy-paste commands and
implied that the manual path was the only option.

### Migration

None. The link from
[`docs/start/should-you-adopt.md`](docs/start/should-you-adopt.md)
into the Quickstart anchor still resolves (heading and slug
preserved). The manual step-by-step walkthrough remains canonical
in
[ADOPTING.md § Quickstart](ADOPTING.md#quickstart-15-minute-path-layout-b).

### Changed

- **`docs/adopt/index.md`** — Quickstart section replaced with the
  LLM-driven recommendation; manual fallback links to
  ADOPTING.md.


## v1.1.47 (2026-05-04)

**Action plan complete: 21 of 21 items closed.** Phase 5 capstone
batch from the
[2026-05-04 ChatGPT engagement action plan](docs/plans/2026-05-04-chatgpt-engagement-action-plan.md#phase-5--precedence-and-tension-recording-scale-stability):

- **P1 — `framework.yaml` precedence and conflict policy**
  (issue [#63](https://gitlab.com/janipaijanen/agent-governance/-/issues/63)):
  new `framework.yaml` at the repository root with
  `contract_version`, `compliance_target` (artefact-only / process-
  aware / full), `precedence`, `conflict_policy`, `binding`, and
  `validators` sections. New `agov-validate-framework-config`
  validator enforces the
  [`schemas/framework.schema.json`](schemas/framework.schema.json)
  shape (Draft 7, `additionalProperties: false` per round-5 §7) and
  cross-checks `contract_version` against `VERSION` per A5.
- **P2 — Mandatory `category: tension` recording** (issue
  [#70](https://gitlab.com/janipaijanen/agent-governance/-/issues/70)):
  cross-layer conflict resolutions are recorded as `category:
  tension` learning entries. Documented discipline (no schema
  change). Canonical home at
  [Concept — Tension recording](docs/concept/principles.md#tension-recording);
  `learnings/_template.yaml` header refreshed with the mandatory
  marker.

### Capability surface complete

With P1 + P2 shipping, the umbrella capability map flips its final
flag — `tension_recording: true` — and `missing_for_full_compliance`
is empty:

```json
{
  "framework_version": "1.1.47",
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

The framework's own
[`framework.yaml`](framework.yaml) declares
`compliance_target: process-aware`. Promotion to **Full** is
mechanically supported (Layer 3 binding shipped in v1.1.45) but
awaits the framework's own commits starting to carry `Agent-ID:`
trailers — the migration is prospective only per
[binding-contract.md](docs/binding-contract.md).

### Test-Driven Development

Tests written first (RED → GREEN). 12 new tests in the new
`tests/test_validate_framework_config.py` plus 2 added to
`tests/test_capabilities.py` (tension flip + empty missing list).
Suite total: 485 → 499.

### Migration

Adding `framework.yaml` is **opt-in for adopters**. Adopters
without one stay on the v1.1.46 surface unchanged; adopters who
ship `framework.yaml` get tier-aware capability reporting,
machine-readable binding configuration consumed by
`agov-validate-binding`, and the precedence policy in
declarative form.

### Added

- **`framework.yaml`** at the repository root — the framework's
  own machine-readable companion to `framework.md`.
- **`src/agents_govern/validate_framework_config.py`** —
  schema-validation + contract-version cross-check entry point.
- **`agov-validate-framework-config`** in `pyproject.toml`
  `[project.scripts]`; added to the orchestrator's default set.
- **`tests/test_validate_framework_config.py`** — 12 cases
  covering existence, schema rejection, version cross-check, and
  command-line interface (CLI) help.
- **`docs/concept/principles.md`** — *Tension recording* section
  (P2 canonical home).

### Changed

- **`src/agents_govern/capabilities.py`** — `tension_recording`
  flips to `true`; comment updated for v1.1.47 closure.
- **`src/agents_govern/orchestrator.py`** —
  `validate-framework-config` added to `default_validators()`.
- **`learnings/_template.yaml`** — header comment names the five
  categories, marks `tension` as mandatory for cross-layer
  conflicts, points at the new principles section.
- **`docs/integration-contract.md`** — validator surface table
  gains `agov-validate-framework-config`; path-trigger row for
  `framework.yaml` flips to shipped; capability-reporting example
  refreshed for v1.1.47 (empty missing list); implementation status
  table flips P1 + P2 rows.
- **`docs/adopt/compliance-profile.md`** — *How tiers are
  declared* section reframed: declarable today via `framework.yaml`;
  framework's own tier note now points at the live declaration.
- **`tests/test_capabilities.py`** — adds
  `test_tension_recording_is_true_post_p2` and
  `test_missing_for_full_compliance_is_empty_at_v1_1_47`.

### Linked issues

- Issue [#63](https://gitlab.com/janipaijanen/agent-governance/-/issues/63) (P1)
- Issue [#70](https://gitlab.com/janipaijanen/agent-governance/-/issues/70) (P2)


## v1.1.46 (2026-05-04)

Closes the Phase 3 schema-artefacts batch from the
[2026-05-04 ChatGPT engagement action plan](docs/plans/2026-05-04-chatgpt-engagement-action-plan.md#phase-3--schema-artifacts-ecosystem-integration):

- **C1 — JavaScript Object Notation (JSON) Schema export per
  artefact type** (issue [#59](https://gitlab.com/janipaijanen/agent-governance/-/issues/59)):
  new `schemas/agent.schema.json`, `schemas/learning.schema.json`,
  and `schemas/framework.schema.json` (forward-looking for
  P1). Each declares Draft 7 `$schema` and a stable `$id`. Editor
  plugins, Open Policy Agent (OPA) stacks, and ecosystem-specific
  linters consume these without re-implementing validator logic.
- **C2 — Cross-artefact invariant manifest**
  (issue [#60](https://gitlab.com/janipaijanen/agent-governance/-/issues/60)):
  new `schemas/invariants.yaml` declares the structural rules the
  Python validators enforce *across* artefacts (manifest closure,
  ownership-overlap threshold, communication symmetry, append-only
  supersession, confidence corroboration, anonymisation
  heuristics, binding-contract formats). Ten invariants in the
  initial manifest, each carrying `id`, `scope`, `type`,
  `parameters`, `severity`, `enforced_by`, `introduced_in` per the
  round-4 §1 normalised shape.

### Source-of-truth hierarchy preserved

The Python validators in `src/agents_govern/validate_*.py` remain
the **contract of record** per
[Schema-of-record hierarchy](docs/integration-contract.md#schema-of-record-hierarchy).
The schemas and the invariant manifest are *consumer-facing*
artefacts. The drift mitigation:

- [`scripts/generate-schemas.py`](scripts/generate-schemas.py)
  cross-checks every `schemas/*.schema.json` against the Draft 7
  meta-schema and confirms `invariants.yaml` matches the C2
  normalised shape. Single-command verification helper.
- `tests/test_schemas.py` enforces *parity*: every
  `agents/*.yaml` and `learnings/*.yaml` validates against its
  schema, and the invariant manifest's parameter values
  cross-check against the validator source (e.g.,
  `agents.owns_overlap_threshold.max_shared_keywords == 3`
  matches the literal `len(overlap) >= 3` in
  `validate_agents.py`). When a validator threshold changes, the
  parity test fails until `invariants.yaml` is updated.

### Test-Driven Development

Tests written first (RED → GREEN). 22 new test cases in the new
`tests/test_schemas.py` (schema validity, parity, invariant
shape, parity-against-validator). Suite total: 463 → 485.

### Migration

Purely additive. Adopters who already validate with the Python
validators see no change. Adopters using editor plugins or
external policy stacks point them at the new
`https://gitlab.com/janipaijanen/agent-governance/-/raw/main/schemas/`
URLs (or vendor a copy) and pick up the schemas without
re-implementing validation rules.

### Added

- **`schemas/agent.schema.json`** — agent definition shape.
- **`schemas/learning.schema.json`** — project learning record shape.
- **`schemas/framework.schema.json`** — `framework.yaml` shape with
  `additionalProperties: false` per round-5 §7 (forward-looking
  for [P1]).
- **`schemas/invariants.yaml`** — ten cross-artefact invariants in
  the C2 normalised shape.
- **`scripts/generate-schemas.py`** — verification helper.
- **`tests/test_schemas.py`** — schema validity + parity tests.
- **`pyproject.toml`** — `jsonschema>=4.0` added to the test
  dependency group (test-only; adopters consuming the schemas
  pick their own validation library).

### Changed

- **`docs/integration-contract.md`** — schema-of-record hierarchy
  table updates the schemas and invariants rows from *Planned* to
  *Shipped v1.1.46*; drift-risk paragraph names the verification
  helper and the parity test as the mitigation; implementation-
  status table flips both rows; pull-based architecture diagram
  drops the *(planned)* annotations.

### Linked issues

- Issue [#59](https://gitlab.com/janipaijanen/agent-governance/-/issues/59) (C1)
- Issue [#60](https://gitlab.com/janipaijanen/agent-governance/-/issues/60) (C2)


## v1.1.45 (2026-05-04)

Closes the third and final Layer 3 binding-contract item from the
[2026-05-04 ChatGPT engagement action plan](docs/plans/2026-05-04-chatgpt-engagement-action-plan.md#phase-2-binding-layer-closes-layer-3-meta-finding):

- **B3 — sidecar metadata + filename-suffix fallback**
  (issue [#58](https://gitlab.com/janipaijanen/agent-governance/-/issues/58)):
  `<artefact>.agov.json` sidecar files carrying `correlation_id`,
  `agent_id`, `produced_at`, `artefact_sha256`. The validator
  recomputes the artefact hash and compares; mismatch flags. For
  environments stripping non-image files (some container
  registries, certain Continuous Delivery (CD) systems), the
  filename-suffix fallback `<base>.<corr-id>.<ext>` carries the
  correlation identifier inside the filename instead.

### Layer 3 binding closed

With B3 joining B1 (UUID v7 correlation identifier, v1.1.44) and
B2 (`Agent-ID:` commit trailer, v1.1.44), the umbrella `binding`
capability flips to `true` in `agov --capabilities`:

```json
{
  "framework_version": "1.1.45",
  "capabilities": { "binding": true, ... },
  "missing_for_full_compliance": ["tension_recording"]
}
```

The Full
[compliance tier](docs/adopt/compliance-profile.md) is now
mechanically achievable: adopters declaring Full use
`agov-validate-binding` plus sidecar metadata on non-commit
artefacts. The framework's own repo remains at Process-aware until
[P1] ships `framework.yaml` declaring the target.

### Test-Driven Development

Tests written first (RED → GREEN). 21 new test cases extending
`tests/test_validate_binding.py`. Suite total: 442 → 463. One test
in `test_capabilities.py` renamed and inverted
(`test_binding_is_false_pre_b1_b3` → `test_binding_is_true_post_b3`)
reflecting the umbrella flip.

### Migration

**Prospective only.** Adopters opt in via the Full tier; existing
artefacts without sidecars are not retroactively validated. Until
[P1] ships `framework.yaml`, the validator stays in *schema-only*
mode: it confirms the contract is loadable and flags malformed
sidecar JSON, but a missing sidecar is not a failure.

### Added

- **`src/agents_govern/validate_binding.py`** — `Sidecar` dataclass,
  `parse_sidecar`, `sidecar_path`, `compute_artefact_sha256`,
  `read_sidecar`, `verify_sidecar`, `parse_filename_fallback`.
- **`docs/binding-contract.md`** — sidecar JSON shape table, hash
  verification semantics, filename-suffix fallback specification,
  pipeline-propagation guidance.

### Changed

- **`src/agents_govern/capabilities.py`** — `binding` capability
  flips to `true`; comment refresh noting all three legs present.
- **`docs/integration-contract.md`** — five-layer model row for
  Layer 3 flips to *Yes (v1.1.45)*; binding section enumerates all
  three legs as shipped; capability-reporting example refreshed
  for v1.1.45.
- **`docs/adopt/compliance-profile.md`** — Full tier marked
  mechanically achievable; *how to choose* shortcut updated;
  framework's own tier note reframed (binding mechanically
  supported but enforcement awaits [P1]).
- **`tests/test_capabilities.py`** —
  `test_binding_is_false_pre_b1_b3` → `test_binding_is_true_post_b3`.

### Linked issues

- Issue [#58](https://gitlab.com/janipaijanen/agent-governance/-/issues/58) (B3)


## v1.1.44 (2026-05-04)

Closes the first two of three Layer 3 binding-contract items from
the
[2026-05-04 ChatGPT engagement action plan](docs/plans/2026-05-04-chatgpt-engagement-action-plan.md#phase-2--binding-layer-closes-layer-3-meta-finding):

- **B1 — Universally Unique Identifier (UUID) v7 correlation
  identifier** (issue [#56](https://gitlab.com/janipaijanen/agent-governance/-/issues/56)):
  new `correlation_id` module supplying time-ordered, sortable
  identifiers per Request For Comments (RFC) 9562. Generated on
  task assignment by Governor-Reviewer; propagates through commit
  trailers (B2, this release) and sidecar metadata (B3, planned).
- **B2 — `Agent-ID:` commit trailer** (issue [#57](https://gitlab.com/janipaijanen/agent-governance/-/issues/57)):
  new `validate_binding` module exposing `parse_trailer`,
  `extract_trailers`, `is_agent_active_branch`, and `validate_commit`
  helpers, plus the `agov-validate-binding` Command Line Interface
  (CLI) entry point. Format:
  `Agent-ID: <agent-name>:<agent-version>:<correlation-id>`. Multi-
  trailer policy: chronologically-ordered for multi-agent
  contributions. Branch classification: `agent/*` default fallback;
  `framework.yaml` override pending [P1].

**Why batched:**

The trailer (B2) is the propagation mechanism for the correlation
identifier (B1); shipping them apart leaves the identifier with no
public surface. One Merge Request (MR), one feature seam, one
documentation page.

**B3 still pending.** The third leg of Layer 3 — sidecar metadata
`<artefact>.agov.json` with filename fallback for non-commit
artefacts — is tracked under issue
[#58](https://gitlab.com/janipaijanen/agent-governance/-/issues/58)
and will land in a separate batch. Until B3 ships, the
[capability report](docs/integration-contract.md#capability-reporting)
keeps `binding: false` because all three legs are required for a
true Layer 3 claim.

### Migration discipline

**Prospective only.** Existing commits without `Agent-ID:` trailers
are not retroactively validated. Until [P1] ships `framework.yaml`
declaring an adopter's tier, `agov-validate-binding` runs in
*schema-only* mode: it confirms the contract is loadable and flags
malformed trailers it encounters, but a *missing* trailer is not a
failure. The framework's own repository runs in this mode at
v1.1.44.

### Test-Driven Development (TDD)

Tests written first (RED → GREEN); 39 new test cases across two
files (`tests/test_correlation_id.py`,
`tests/test_validate_binding.py`). Suite total: 403 → 442.

### Added

- **`src/agents_govern/correlation_id.py`** — `generate()`,
  `is_valid()`, `extract_timestamp_ms()`. Manual UUID v7
  construction (CPython 3.13's `uuid.uuid7()` is unavailable on
  the project's `requires-python = ">=3.12"` floor).
- **`src/agents_govern/validate_binding.py`** — trailer parser,
  branch classifier, per-commit validator, top-level
  `validate_binding()` entry returning the strict-contract exit
  code, `agov-validate-binding` CLI wrapper.
- **`docs/binding-contract.md`** (new, 200+ lines) — canonical home
  for the Layer 3 binding specification (UUID v7 format, lifecycle
  table, multi-trailer policy, branch classification, schema-only
  vs enforce mode, migration discipline).
- **`agents/governor-reviewer.yaml`** — adds
  `Correlation-identifier lifecycle` to `governor_responsibilities`
  and `correlation_id_assigned` / `correlation_id_closed` to
  `accountability.logged_actions`.
- **`pyproject.toml`** — new `agov-validate-binding` script entry.
- **`src/agents_govern/orchestrator.py`** — `validate-binding`
  added to the default validator set so `agov-validate` and
  `agov-pre-tag-check` automatically pick it up.

### Changed

- **`docs/integration-contract.md`** — validator surface table
  gains `agov-validate-binding`; five-layer model row for Layer 3
  flips to *Partial (v1.1.44)*; *Binding contract* section
  cross-links the new canonical home; implementation-status table
  splits the binding row into shipped vs pending.
- **`src/agents_govern/capabilities.py`** — comment refresh noting
  B1 + B2 shipped at v1.1.44; the `binding` flag stays `False`
  until B3 lands.

### Linked issues

- Issue [#56](https://gitlab.com/janipaijanen/agent-governance/-/issues/56) (B1)
- Issue [#57](https://gitlab.com/janipaijanen/agent-governance/-/issues/57) (B2)


## v1.1.43 (2026-05-04)

Adds the `agov` umbrella Command Line Interface (CLI) entry point —
the first `agov` parent command in the framework — closing the
remaining halves of two action-plan items in one batch:

- **A5 CLI flags** (issue [#54](https://gitlab.com/janipaijanen/agent-governance/-/issues/54)):
  `agov --version` / `--contract-version` / `--contract-changelog`.
  The documentation half shipped in v1.1.41 (`docs/contract-stability.md`);
  this MR ships the CLI half.
- **A6 capability report** (issue [#55](https://gitlab.com/janipaijanen/agent-governance/-/issues/55)):
  `agov --capabilities` emits a JavaScript Object Notation (JSON)
  report listing currently-shipped strict-contract capabilities and
  the `missing_for_full_compliance` list (capabilities currently
  `False` that would have to ship for Full-tier adopters to claim
  Full per the
  [Compliance profile](docs/adopt/compliance-profile.md)).

**Why batched:**

Both items required introducing the `agov` parent-command (the
existing convention is flat `agov-validate-*` scripts; nothing
ships under an `agov` umbrella). Designing the umbrella once and
landing both flag sets in the same Merge Request (MR) avoids two
passes over the same subcommand-dispatch scaffolding. The
[contract-stability page](docs/contract-stability.md) explicitly
deferred the CLI half pending this batching decision; it lands now.

**What ships:**

- `src/agents_govern/cli.py` (new, ~115 lines) — the umbrella
  dispatcher. Pick exactly one action flag per invocation; mixing
  actions exits non-zero. Bare `agov` prints help and exits 1.
- `src/agents_govern/capabilities.py` (new, ~80 lines) — capability
  computation + JSON serialiser. Capability map at this version:
  `artifact_validation`, `precedence_enforcement`,
  `confidence_tracking` shipped; `binding`, `tension_recording`
  pending issues B1–B3 / P2.
- `tests/test_cli.py` (new, 11 tests) — covers help / version /
  contract-version alias / contract-changelog / capabilities /
  unknown-flag / multi-action-rejected / entry-point wiring.
- `tests/test_capabilities.py` (new, 10 tests) — covers schema
  shape, version derivation, capability boolean discipline,
  `missing_for_full_compliance` invariants, JSON serialisation.
- `pyproject.toml` — new `agov` script entry; existing
  `agov-validate-*` scripts unchanged.
- `docs/integration-contract.md` — *Validator command surface*
  table gains an `agov` row pointing at *Contract versioning* and
  *Capability reporting* sections; *Contract versioning* and
  *Capability reporting* both flip from Planned → Shipped with
  the actual v1.1.43 JSON output shape inlined; *Implementation
  status* table flips two rows.
- `docs/contract-stability.md` — *CLI surface* section flips from
  *Planned (follow-up MR)* to *Shipped v1.1.43* with the four
  flags listed.
- `docs/adopt/compliance-profile.md` — *How tiers are declared*
  section adds a new *What does ship today (v1.1.43)* paragraph
  explaining that `agov --capabilities` works unconditionally
  today and will gain target-tier comparison once
  `framework.yaml` (P1) ships.

**Self-conformance:**

- `agov-validate-acronyms` clean on touched markdown files.
- `mkdocs build --strict` passes (one pre-existing INFO note on
  `position/index.md` anchor unrelated).
- Full test suite: 403 passing (382 baseline + 21 new).
- Test-Driven Development (TDD) discipline honoured per the
  framework's TDD rule — tests written first, RED confirmed,
  then GREEN after implementation.
- The umbrella is additive — every existing `agov-validate-*`
  script continues to work unchanged.
- Running `agov --capabilities` against this version reports
  `binding: false` and `tension_recording: false` per the
  framework's actual implementation state (B1–B3 / P2 pending),
  honouring the self-conformance rule that the framework's own
  capability claims match its own behaviour.

## v1.1.42 (2026-05-04)

Absorbs the three concrete README recommendations from the
2026-05-04 OpenAI Codex (Command Line Interface (CLI)) follow-up
on the external Chief Technology Officer (CTO) onboarding-bounce
feedback (recorded in
[`learnings/iron-trellis.yaml`](learnings/iron-trellis.yaml) and
[`external-ai-engagements/2026-05-04-codex-cto-followup.md`](external-ai-engagements/2026-05-04-codex-cto-followup.md)).

**What ships:**

- `README.md` — three Provisional adopter-facing changes, all from
  the same source (Codex follow-up's recommendation thread on the
  prior CTO bounce entry):
    - **Routing block at top of README.** New *"Where to look
      first"* section directly after the docs-site pointer,
      sized for a ten-minute first-contact budget. Three
      reader-intent paths plus a *"Try it in 30 seconds"*
      command pointer
      (`uv run --with agents-govern agov-validate-adoption`).
      Addresses recommendation thread 1 of the prior CTO bounce
      entry plus recommendation 3 of the Codex follow-up entry
      in one stroke.
    - **Non-claims block extension.** New bullet *"Not an agent
      tool-call authorisation framework"* in the *"What this is
      not"* list, with a one-line pointer at Model Context
      Protocol (MCP) authorisation specifications as the
      appropriate external category. Disambiguates the
      *"governance" → tool-call auth* misread the CTO surfaced.
    - **`agov-validate-adoption`-as-demo pointer.** Lives inside
      the routing-block *"Try it in 30 seconds"* bullet rather
      than a standalone section — same effect, less ceremony.

**Provisional framing on every addition:**

Every new line carries an explicit Provisional clarification
matching the framework's documented *"ship Provisional, don't
defer"* discipline. The CTO bounce was single-empirical-source;
the Codex follow-up provides analytical second-opinion
corroboration but not independent empirical encounter. A second
independent empirical reader (different person, no exposure to
the CTO feedback) would promote any of these additions off
Provisional.

**Why batched:**

Three changes, one MR — same source, same rationale, same
Provisional framing. Per the framework's
*split-MRs-per-concern* discipline these are not three
structurally distinct concerns; they are one concern (absorb the
Codex follow-up's three concrete README recommendations) with
three change-points in the file.

**Self-conformance:**

- `agov-validate-acronyms` clean on the touched file
  (`README.md` only).
- `agov-validate-readme-stats` clean: all stat counts match.
- `mkdocs build --strict` not relevant (README is the GitLab
  repo README, not a docs-site page).
- Provisional framing visible (italic in-line) per the
  established pattern in the README's existing
  *"Cross-domain pilot adopters wanted (Provisional)"* note.

## v1.1.41 (2026-05-04)

Adds [`docs/contract-stability.md`](docs/contract-stability.md) —
canonical home for the integration-contract stability promise
(patch / minor / major rules, what is and isn't covered,
adopter recommendations, major-release deprecation cycle).
Closes the documentation half of action-plan item
[A5](https://gitlab.com/janipaijanen/agent-governance/-/issues/54)
from the v1.1.21 ChatGPT engagement.

**What ships:**

- `docs/contract-stability.md` (new, ~200 lines) — single-version
  discipline (framework version = contract version), the
  patch / minor / major promise table, what is covered (validator
  command names, exit-code semantics, output-format shape, trigger
  set, five-layer model) versus what is not (rule wording,
  learning-record content, doc prose, internal Python module
  structure), worked examples from the v1.1.36–v1.1.40 release
  thread showing each version's contract impact, adopter
  recommendations (pin to major; branch CI on `!= 0`), the
  major-release deprecation cycle policy, and the planned
  Command Line Interface (CLI) flag surface (`agov
  --contract-version`, `agov --contract-changelog`) deferred to a
  follow-up MR because adding an `agov` umbrella entry point is
  structurally significant.
- `docs/integration-contract.md` — *Contract versioning* section
  now points at the new page as canonical home (replacing the
  previous *planned* placeholder); *Implementation status* table
  flips Contract versioning + stability promise from Planned →
  Shipped (with CLI flags called out as still-planned).

**Why this matters:**

The engagement (round-4 §7) named contract-versioning absence as a
load-bearing gap: without an explicit stability discipline,
adopter Continuous Integration (CI) pipelines can't tell whether
upgrading from v1.1.36 to v1.1.40 is safe. The framework's actual
discipline (patch never changes the contract; minor extends;
major changes) was documented only implicitly across the
CHANGELOG and integration-contract page. This page consolidates
the discipline into one referenceable artefact.

The *what-is-not-covered* section is the same dual-surface
discipline the v1.1.35 dual-surface page named: the strict-contract
surface graduates to protocol-grade rigor; the loose-prose surface
(rule wording, learning records, doc prose) stays explicitly
out-of-contract by design.

**CLI flags deferred:**

The action plan's `agov --contract-version` / `--contract-changelog`
flags are intentionally not in this MR. The framework's existing
CLI convention is flat `agov-validate-*` scripts (no parent
command). Adding `agov --version` style flags requires a new
`agov` umbrella entry point with subcommand-dispatch design — a
structurally separate concern that deserves its own MR. The
documentation half is the load-bearing artefact (CI integrations
need the *promise*, not the convenience flag); the flags can ship
once the umbrella-entry-point design is committed.

**Self-conformance:**

- `agov-validate-acronyms` clean on touched markdown files.
- `mkdocs build --strict` passes locally (one pre-existing INFO
  note on `position/index.md` anchor unrelated to current work).
- *What-is-not-covered* section honours the dual-surface discipline
  from v1.1.35 — page-level documentation of which surface a piece
  of the framework belongs to.

## v1.1.40 (2026-05-04)

Closes the remaining gap on action-plan item
[A2](https://gitlab.com/janipaijanen/agent-governance/-/issues/51)
(path-based revalidation triggers + pre-merge enforcement) by
adding the two CI jobs the integration-contract trigger table
already promised but did not exist as Continuous Integration (CI)
jobs: `validate-readme-stats` and `validate-contributions`.

**What ships:**

- `.gitlab-ci.yml` — two new merge-request-event jobs:
    - `validate-readme-stats` triggered on changes to `README.md`,
      `learnings/**/*`, `agents/**/*`, `.claude/rules/**/*`,
      `rules-template/**/*`, and `skills/**/*` (any path that
      affects the README's stat counts). Blocking gate.
    - `validate-contributions` triggered on changes to
      `learnings/**/*`. Marked `allow_failure: true` because the
      validator's own output names its findings as *"patterns,
      not verdicts — review each finding before acting,"* and the
      v1.1.36 exit-code taxonomy does not yet have an *advisory*
      code distinct from `1` (validation failure). Surfacing the
      findings at Merge Request (MR) time without gating the
      merge is the right interim — a follow-up MR will either
      add an advisory exit code or split the validator's output
      into blocking-vs-advisory tiers, after which `allow_failure`
      can be removed.
- `docs/integration-contract.md` — the *Path-based revalidation
  triggers* table now lists `agov-validate-readme-stats` against
  every corpus path that affects the count claims (not only
  `README.md` itself), and `agov-validate-contributions` against
  `learnings/**`. A short note explains why readme-stats appears
  on multiple rows.

**Why this matters:**

Both validators ran inside `agov-pre-tag-check` already, so corpus
drift was caught at release time. A2's point is *pre-merge*
enforcement: catch the drift at the Merge Request (MR) gate
instead of letting it slip through to a release stub. Without the
CI jobs, the integration-contract trigger table was a description
of what *should* happen rather than what *did* happen — closing
the description-implementation gap is the A2 closure.

**Self-conformance:**

- `glab ci lint` clean on the modified `.gitlab-ci.yml`.
- `agov-validate-acronyms` clean on touched markdown files.
- `mkdocs build --strict` not relevant (table edit only).
- The framework's own MR pipeline now runs both new jobs on
  every relevant MR; this MR's pipeline is the first to exercise
  them.

## v1.1.39 (2026-05-04)

Adds `learnings/.session-notes/` to `.gitignore`. Bookkeeping
release; no framework behaviour change.

**Why:**

The directory is a Claude session-handoff working-doc convention
(transient state for the next session to pick up from), analogous
to the existing `learnings/.capability-analysis/` directory but
strictly local and never tracked. Without the gitignore entry,
`agov-pre-tag-check` flagged the untracked working-doc as a dirty
working tree on every release-time push, requiring a manual
move-aside-and-restore dance during v1.1.37 and v1.1.38 tag
pushes. This MR makes the convention durable.

**What ships:**

- `.gitignore` — one new section with comment + path entry for
  `learnings/.session-notes/`.

**Self-conformance:**

- `agov-validate-acronyms` clean on touched files.
- `mkdocs build --strict` not relevant (no doc surface changed).
- The directory's self-description (*"auto-generators do not pick
  it up; the docs site does not render it. Delete or supersede
  when the work it tracks is complete"*) matches the gitignore
  treatment.

## v1.1.38 (2026-05-04)

Adds [`docs/adopt/compliance-profile.md`](docs/adopt/compliance-profile.md) —
the canonical home for the three-tier compliance vocabulary
(Artefact-only / Process-aware / Full) and the degraded-modes
table showing what each lower tier gives up. Closes action-plan
item [D5](https://gitlab.com/janipaijanen/agent-governance/-/issues/68)
from the v1.1.21 ChatGPT engagement.

**What ships:**

- `docs/adopt/compliance-profile.md` (new, ~190 lines) — names
  the three tiers, defines each before any later table references
  it (per
  [classification-vocabulary-rule](https://gitlab.com/janipaijanen/agent-governance/-/blob/main/.claude/rules/docs/classification-vocabulary-rule.md)),
  maps each tier to the five-layer verifiability model, lists
  what each lower tier *gives up* relative to Full (the
  degraded-modes view the engagement specifically asked for),
  gives a *how to choose* decision shortcut, documents the
  planned `framework.yaml compliance_target:` declaration
  ([P1]), and records the framework's own current tier
  (Process-aware, pending [B1]–[B3]).
- `docs/adopt/.pages` — adds compliance-profile.md to the Adopt
  section navigation.
- `docs/adopt/index.md` — *Section contents* gains the
  compliance-profile entry alongside intensity-guide and
  domain-profiles.
- `docs/integration-contract.md` — the *Capability reporting*
  section now points at the new page as the canonical home for
  the tier vocabulary; *Implementation status* table flips
  *Compliance profile (3 tiers)* from Planned → Shipped.

**Why this matters:**

The compliance-tier vocabulary appeared in the v1.1.21 ChatGPT
engagement (round-5 §2.2 + §2.3) as the mitigation for the
*specification-versus-verifiable* meta-finding: adopters who
target *Full* but cannot achieve it today need an explicit
fallback profile they *can* claim, with the trade-off named in
writing. The integration-contract page (v1.1.32) introduced the
three tiers in passing; this page is the canonical home with the
material that was deferred — the degraded-modes table, the
how-to-choose shortcut, and the orthogonality clarification
against adoption intensity and domain profile.

The orthogonality matters because external reviewers and adopters
have repeatedly conflated *"how many gates"* (intensity), *"what
domain-specific concerns"* (profile), and *"which verifiability
layers"* (compliance tier). Naming the three axes separately on
one page lets each decision stay independent.

**Self-conformance:**

- `agov-validate-acronyms` clean on touched files.
- `uv run mkdocs build --strict` passes locally (one pre-existing
  Information (INFO)-level note on `position/index.md` anchor for
  `quickstart-15-min-layout-b`, unrelated to current work).
- Classification-vocabulary rule honoured — every label
  (Artefact-only / Process-aware / Full) is defined before its
  first appearance in any table on the page.
- Self-conformance rule honoured — the framework declares its own
  tier (Process-aware) explicitly.

## v1.1.37 (2026-05-04)

Ships the `agov-validate` orchestrator — single Continuous Integration
(CI)-friendly entry point that aggregates the default validator set and
emits the highest-precedence exit code via the v1.1.36 precedence
helper. Closes action-plan item [A3](https://gitlab.com/janipaijanen/agent-governance/-/issues/52)
from the v1.1.21 ChatGPT engagement.

**What ships:**

- `src/agents_govern/orchestrator.py` (new) — defines
  `ValidatorResult`, `AggregateResult`, `run_validator()`,
  `run_all()`, `default_validators()`, `to_json()`, and `main()`.
  The shared aggregation helper has one implementation across the
  framework's two orchestrator surfaces.
- `agov-validate` Command Line Interface (CLI) entry point in
  `pyproject.toml [project.scripts]`. Default invocation runs the
  seven validators that take no required arguments and prints
  per-validator output blocks plus an aggregate exit-code summary.
  `--json` switches to a single JavaScript Object Notation (JSON)
  object with `exit_code` and per-validator `name`/`exit_code`/
  `output` fields — the JSON shape is part of the strict-contract
  surface (see [`docs/integration-contract.md`](docs/integration-contract.md#output-formats)).
- `src/agents_govern/pre_tag_check.py` refactored to consume the
  shared `run_validator()` helper. Three previously inlined
  `redirect_stdout`+`io.StringIO` blocks (one per validator) collapse
  to single-line calls. Pre-release-only checks (VERSION, CHANGELOG,
  tag existence, working tree, README citation, self-conformance,
  communication-graph generation) stay where they are; behaviour is
  unchanged.
- `tests/test_orchestrator.py` (new, 15 tests) — covers
  `ValidatorResult` / `AggregateResult` shape, `run_validator`
  capture semantics, default-validator registry, JSON serialisation,
  and `main()` exit-code propagation. Test-driven: all tests were
  written against the orchestrator module before the implementation.
- `docs/integration-contract.md` — *Validator command surface*,
  *Output formats*, *Orchestrator*, and *Implementation status*
  sections updated to reflect the shipped state. JSON shape is
  documented inline.

**Why this matters:**

The orchestrator gap was the round-5 §3 finding from the v1.1.21
ChatGPT engagement: multiple `agov-validate-*` validators with no
single entry point lead CI pipelines to diverge in implementation and
miss validators over time. The v1.1.32 integration-contract page
named `agov validate` as planned; v1.1.36 shipped the precedence
helper. This Merge Request (MR) lifts the per-validator aggregation
pattern from `pre_tag_check.py` into a shared helper and exposes it
as `agov-validate`, so the framework now has the single CI-friendly
entry point external policy engines have asked for, *and*
`pre-tag-check` shares the same implementation rather than carrying a
parallel one.

The orchestrator is **minimum-viable**: it runs the no-argument
default validator set and emits aggregate exit codes / JSON. Future
MRs can add `--validators=`-style filtering, SARIF output, and a
`--profile pre-tag-check` named profile if and when the engagement
evidence supports the additions.

**Naming note.** The action plan and engagement transcripts use
`agov validate` (subcommand). Python's `[project.scripts]` does not
support nested subcommands; the existing convention across all
twelve `agov-*` entry points is flat. The hyphenated form
`agov-validate` honours the convention without losing the contract
the engagement asked for. A future `agov` parent command can be
added without breaking the `agov-validate` script.

**Self-conformance:**

- `agov-validate-acronyms` clean on touched markdown files.
- `agov-validate-agents` and the existing pre-tag-check pass under
  the refactor (382 tests pass; 15 new orchestrator tests + 367
  pre-existing).
- `uv run mkdocs build --strict` passes locally (one pre-existing
  Information (INFO)-level note on `position/index.md` anchor,
  unchanged by this MR).
- Test-driven: tests written first; implementation followed; no
  production code without a failing test (TDD discipline).

## v1.1.36 (2026-05-04)

<!-- TODO: fill in release notes -->

## v1.1.35 (2026-05-04)

Adds [`docs/position/dual-surface.md`](docs/position/dual-surface.md) —
a dedicated page documenting the dual-surface architecture (loose-prose
surface + strict-contract surface) the framework has by design.
Closes action-plan item [D3](https://gitlab.com/janipaijanen/agent-governance/-/issues/66)
from the v1.1.21 ChatGPT engagement.

**What ships:**

- `docs/position/dual-surface.md` (new, ~160 lines) — names the
  two surfaces, gives examples for each, explains why both exist
  by design, why formalising the prose surface would break the
  framework, and what the model implies for everyday adopters,
  ecosystem integration, and framework evolution.
- `docs/position/index.md` — *Other position comparisons* list
  gains an entry pointing at the new page.

**Why this matters:**

The dual-surface model surfaced in the v1.1.21 ChatGPT engagement
(round-3 §8 + round-4 §9) when the reviewer asked whether the
framework was *"becoming a governance protocol, not just a
framework."* The maintainer disposition refined that framing:
the strict-contract surface graduates to protocol-grade rigor
(validators, schemas, exit codes, contract versioning); the
loose-prose surface stays framework-shaped (rules, learning
records, evidence discipline, reviewer judgement) because
formalising it would break the evidence-driven evolution model.

The page records that refinement explicitly so future engagements
do not re-raise the question and adopters do not try to formalise
the surface that should stay loose.

**Self-conformance:**

- `agov-validate-acronyms` clean on touched files.
- `uv run mkdocs build --strict` passes locally (one pre-existing
  Information (INFO)-level note on `position/index.md` anchor,
  unchanged by this Merge Request (MR)).
- Negation-then-correction rule honoured — *"What this is not"*
  section uses positive-form companions, not setup-and-knockdown.

## v1.1.34 (2026-05-04)

<!-- TODO: fill in release notes -->

## v1.1.33 (2026-05-04)

<!-- TODO: fill in release notes -->

## v1.1.32 (2026-05-04)

Adds the **Integration Contract** page at
[`docs/integration-contract.md`](docs/integration-contract.md) —
the canonical home for the framework's machine-checkable surface
(how a Continuous Integration (CI) engine consumes governance
artefacts). Closes action-plan item [A1] from the v1.1.21 ChatGPT
engagement.

**What ships:**

- `docs/integration-contract.md` (new, ~330 lines) —
  pull-based architecture commitment; schema-of-record hierarchy
  (Python validators authoritative; JavaScript Object Notation
  (JSON) Schema and invariant manifest are generated artefacts);
  five-layer verifiability model with full layer descriptions
  (canonical home — moved from
  `docs/start/reviewer-reality-map.md` and `docs/position/index.md`,
  which now carry summaries pointing here); validator command
  surface (12 `agov-*` entry points documented); path-based
  revalidation triggers; current vs planned exit-code taxonomy
  (5-code with deterministic precedence is issue [A4]); current
  vs planned output formats (text today; JSON / Static Analysis
  Results Interchange Format (SARIF) planned with [A3]);
  orchestrator (current `agov-pre-tag-check`; planned `agov
  validate` under [A3]); contract versioning principle (single
  version = framework version) with stability promise under [A5];
  capability-reporting CLI surface ([A6]) with three compliance
  tiers ([D5]); binding-contract overview pointing at issue series
  [B1]–[B3]; comprehensive *Implementation status* table; explicit
  *Out of scope* section enumerating the four items the engagement
  reviewed and rejected (push-based generation, hierarchical
  correlation Identifier (IDs), cryptographic integrity guarantees,
  runtime-behaviour enforcement).
- `external-ai-engagements/REVIEWER-BRIEF.md` — common-concerns
  map gains an *Integration contract (CI engine consumption)* row
  pointing at the new page.
- `docs/start/reviewer-reality-map.md` — five-layer verifiability
  model section reduced to a thirty-second summary pointing at the
  Integration Contract page as the canonical home.
- `docs/position/index.md` — same reduction; canonical home note
  added.

**Why this matters:**

The integration-contract gap was the sharpest finding from the
v1.1.21 ChatGPT engagement (round-1 §5.1) and the round-3 §7
disposition pinned the requirements: canonical CLI invocations, exit
codes, JSON / SARIF outputs, directory structure, the five-layer
verifiability model, the schema-of-record hierarchy, and a
pull-based architecture commitment. This page lands all of that in
one place. Adopters and external policy engines can now read the
contract as a single document instead of inferring it from the
framework's source tree.

The page also documents what is *not* the framework's contract:
push-based generation, hierarchical correlation IDs, cryptographic
integrity guarantees, and runtime-behaviour enforcement. Naming
out-of-scope items explicitly prevents adopters from implementing
incompatible enforcement layers.

**Self-conformance:**

- `agov-validate-acronyms` clean across all four touched markdown
  files.
- `uv run mkdocs build --strict` passes locally (one pre-existing
  Information (INFO)-level note on `position/index.md` anchor,
  unchanged by this Merge Request (MR)).
- Five-layer model has *one* canonical home now (the integration
  contract page). Reality map and position landing carry consistent
  thirty-second summaries; no duplication of the layer descriptions.

## v1.1.31 (2026-05-04)

Ships the D-batch (D1, D2, D4, D6) from the
[2026-05-04 ChatGPT engagement action plan](docs/plans/2026-05-04-chatgpt-engagement-action-plan.md):
documentation absorbs four findings the engagement surfaced about
the framework's brief, reality-map, and position docs.

**What ships:**

- **[D1] Named mechanisms by name in brief and reality map.** Both
  documents now point at `cannot_touch:`, `_check_ownership_overlaps`,
  the maturity vocabulary, and per-learning `severity:` / `scope:` /
  `category:` fields with paths and line numbers. Reviewers no
  longer infer absence from invisibility.
- **[D2] Boundary refinement absorbed.** The hard-boundary statement
  sharpens from *"no runtime enforcement"* to *"no runtime
  *behaviour* enforcement; runtime *artefact production*
  requirements stand."* Shipped in
  `external-ai-engagements/REVIEWER-BRIEF.md`,
  `docs/start/reviewer-reality-map.md`, and
  `docs/position/overview.md` as a new *Boundary refinement
  (2026-05-04)* subsection.
- **[D4] Five-layer verifiability model placed.** The model
  (artefact / 2a gate execution / 2b decision recording / binding
  / runtime behaviour) lands in `docs/start/reviewer-reality-map.md`
  and `docs/position/index.md` with cross-links pointing at the
  forthcoming integration-contract page (issue [A1]). Layer 3
  (binding) is named explicitly as the missing middle the
  engagement-action-plan binding contract closes.
- **[D6] Mechanism-by-name index in the brief.** New positive-form
  companion to §2 *Schema check*. Reviewers see what the framework
  has by name before they can mistake absence for invisibility.

**Why this matters:**

The mechanism-discoverability gap was the cross-cutting finding in
the v1.1.21 ChatGPT engagement (round-1 §2.3 / §5.3) and was
corroborated by 2026-05-04-lumo-v2 and 2026-05-04-grok in the
engagement batch. This Merge Request (MR) lands the doc-side
absorption work the action plan filed as items D1, D2, D4, D6. The
corresponding code work (binding contract, integration contract
page, schema artefacts) remains on the action-plan slate as
separate issues; capacity-budget those in their own sessions.

D3 (dual-surface model documentation) and D5 (compliance-profile
page) are intentionally not in this MR — D3 needs a dedicated
position-docs section and D5 is a new page that benefits from the
integration-contract page (A1) landing first.

**Self-conformance:**

- `agov-validate-acronyms` clean across all four touched markdown
  files.
- `uv run mkdocs build --strict` passes locally (one pre-existing
  Information (INFO)-level note on `position/index.md` anchor,
  unchanged by this MR).

## v1.1.30 (2026-05-04)

Adds a correction entry on `learnings/iron-trellis.yaml` flagging
a self-conformance miss in the v1.1.29 brief-efficacy hypothesis
entry: that entry claimed *"ready for Provisional → Recommended
promotion"* on 4-engagement corroboration. Re-reading the
framework's own promotion path: 2+ engagements promote to a
Provisional learning record (correctly applied); 2+ *adopters*
promote to Recommended guidance. Engagement count is not adopter
count.

For brief-efficacy specifically, there is currently one adopter
(iron-trellis itself, the framework dogfooding its own brief).
Promotion to Recommended awaits a second adopter — a project
that adopts agents-govern, runs its own external-Artificial
Intelligence (AI) engagements using the brief, and produces a
learning record citing the brief's effect. That has not happened.

**What ships:**

- `learnings/iron-trellis.yaml` — new `learnings:` array entry,
  `category: gap`, `scope: framework`, `severity: minor`. The
  prior brief-efficacy entry (v1.1.29) is *not* modified
  (append-only invariant preserved); the new entry sits below
  and a reader following the chain catches the correction.

**Why this matters:**

The framework's discipline ("Theoretical additions make the
framework bigger; empirical additions make it better") is
symmetric: it gates structural additions strictly, but should
also gate Recommended-tier promotion strictly. Conflating
engagement count with adopter count weakens the discipline
asymmetrically. Catching the miss within the framework's own
learning corpus before propagation keeps the discipline intact.

When issue #61 (L1 `superseded_by:` field schema) ships, the
prior brief-efficacy entry's recommendation block can be marked
partially superseded by this correction entry via that field.

**Self-conformance:**

- `agov-validate-learnings` clean.
- `agov-validate-acronyms` clean on touched file.
- Append-only invariant honoured — prior entry unchanged; new
  entry added below.
- Same self-conformance discipline this correction names is the
  one this MR applies (the framework catching its own miss in
  real time).

## v1.1.29 (2026-05-04)

Adds two new learning entries on `learnings/iron-trellis.yaml`
derived from the 2026-05-04 engagement batch:

1. **Brief-efficacy hypothesis: 4-source corroboration** — codex-v2
   (within-day uncertainty disclosure), ChatGPT 5-round (round-1
   §7 self-disclosure load-bearing), Lumo v2 (within-vendor
   floor-raising; same vendor previously logged at fail/negative),
   and Grok (schema-grounded review claiming direct file
   inspection). Past the framework's 2+ adopter promotion
   threshold; ready for Provisional → Recommended promotion on
   the brief-efficacy hypothesis.

2. **Single-runtime evidence bias (Provisional)** — Lumo v2 Gap 2
   and Grok improvement-backlog item #2 name the same shape from
   outside the corpus. Two independent Artificial Intelligence
   (AI) families, neither Claude-based. Crosses the 2-engagement
   threshold for promotion from engagement-level
   promotion-candidate to Provisional learning record. Promotion
   to Recommended awaits adopter-experience evidence (non-Claude
   adopter learning record), not just reviewer observations.

**What ships:**

- `learnings/iron-trellis.yaml` — two new `learnings:` array
  entries, both `scope: framework`, severities `informational`
  (brief-efficacy) and `significant` (single-runtime bias).

**Why this matters:**

The 2026-05-04 engagement batch produced four Artificial
Intelligence (AI) engagements (ChatGPT 5-round, Lumo v2, Grok,
plus the prior codex-v2 from April). Cross-source corroboration
of two distinct findings (brief-efficacy, single-runtime bias)
crosses the framework's own promotion threshold cleanly. The
within-vendor floor-raising data point (Lumo v1 fail vs Lumo v2
pass) is the strongest novel observation from the batch.

Per the framework's symmetric evidence discipline, both findings
ship as Provisional. The brief-efficacy hypothesis is ready for
promotion from Provisional to Recommended (4 sources past
threshold); the single-runtime-bias finding stays Provisional
until a non-Claude adopter produces actual learning-record
evidence.

**Self-conformance:**

- `agov-validate-learnings` clean.
- `agov-validate-acronyms` clean on touched file.
- Append-only invariant honoured — existing entries unchanged;
  only new entries added.

## v1.1.28 (2026-05-04)

Adds the 2026-05-04 Grok (xAI) engagement record. Grok's first
engagement with the framework. Schema-grounded fit-assessment +
improvement-backlog format following the Reviewer Brief.

**What ships:**

- `external-ai-engagements/2026-05-04-grok.md` — full engagement
  with sentinel observations, verbatim Grok artefact, and a
  promotion-candidates list.
- `external-ai-engagements/README.md` — index row.

**Why this matters:**

Grok's improvement-backlog item #2 (broader platform mapping;
strong Claude / GitLab bias in examples) names the same shape as
[2026-05-04-lumo-v2.md](../external-ai-engagements/2026-05-04-lumo-v2.md)
Gap 2 (single-runtime evidence bias). **The pair (Lumo + Grok = two
independent Artificial Intelligence (AI) families) crosses the
2-engagement threshold** for promotion to a Provisional learning
record per the
[external-ai-engagements promotion path](../external-ai-engagements/README.md#promotion-path).
Promotion to `learnings/iron-trellis.yaml` filed as a follow-up
Merge Request (MR).

Grok also provides a within-vendor schema-fabrication-rejection
data point: claims to have examined `agents/coder-tester.yaml` and
`learnings/_template.yaml` directly, with structural claims
consistent with someone who actually read the files (no fabricated
fields). Same brief-working-as-intended pattern previously seen on
2026-04-27-codex-v2 and 2026-05-04-chatgpt-web; now on a fourth
independent AI family (codex / ChatGPT / Lumo / Grok all distinct).

**Self-conformance:**

- `agov-validate-acronyms` clean on touched files.
- Promotion threshold honoured: only the corroborated
  single-runtime-bias finding is promoted to a learning record;
  the other Grok-only findings stay on the engagement's
  promotion-candidates list.

## v1.1.27 (2026-05-04)

Adds the 2026-05-04 Lumo (v2, brief-aware) engagement record.
Marks the first within-vendor floor-raising data point in the
external-ai-engagements log: same vendor previously logged at
`fail / negative` (2026-04-27-lumo, hallucinated-conflation),
now passing under the Reviewer Brief.

**What ships:**

- `external-ai-engagements/2026-05-04-lumo-v2.md` — full engagement
  with sentinel observations, verbatim Lumo response, and a
  promotion-candidates list (four entries, all single-source on
  this engagement except where corroborated by 2026-05-04-grok in
  the next MR).
- `external-ai-engagements/README.md` — index row.

**Why this matters:**

Three durable observations from this engagement:

1. **Brief working on previously-failing vendor.** Lumo declined
   to invent fields, included an explicit uncertainty table at
   the end, and grounded claims to docs-site evidence. Same
   pattern as 2026-04-27-codex-v2 and 2026-05-04-chatgpt-web,
   now on a third independent Artificial Intelligence (AI)
   family.
2. **Concurrent-fix data point.** Lumo flagged
   `docs/start/reviewer-reality-map.md` as 404 on the docs site;
   correct at crawl time. !148 (merged ~30 minutes after the
   engagement) shipped the file. Documents the maintainer-first-
   eye-vs-cold-reader-eye gap in real time.
3. **Single-runtime evidence bias** (novel, corroborated). Gap 2
   of Lumo's review names a structural concern about the
   framework's evidence corpus being heavily Claude Code biased.
   Corroborated by 2026-05-04-grok improvement-backlog item #2;
   crosses the 2-engagement threshold for promotion to a
   Provisional learning record (filed in a follow-up Merge
   Request (MR)).

Per the
[external-ai-engagements promotion path](../external-ai-engagements/README.md#promotion-path),
the engagement-level findings remain on this file's
promotion-candidates list except where cross-source corroboration
unlocks promotion to `learnings/`.

**Self-conformance:**

- `agov-validate-acronyms` clean on touched files.
- Index row added with explicit "vs prior 2026-04-27-lumo fail"
  context to make the within-vendor improvement signal
  discoverable.

## v1.1.26 (2026-05-04)

Adds the **Reviewer reality map** — a docs-site companion to
`external-ai-engagements/REVIEWER-BRIEF.md` mapping common review
concerns to the place the framework addresses them. Strengthens the
framework's stated boundary across the top-level README, the docs
index, and the brief itself.

**What ships:**

- `docs/start/reviewer-reality-map.md` (new) — single-page map with
  a hard-boundary block, a common-concerns table (eight rows), an
  enforcement-model two-column table, and a "if you think
  something is missing" pointer list.
- `docs/start/.pages` — nav entry between *Should I adopt?* and
  *Why this exists*.
- `docs/index.md` — *External AI assistant* row in the audience
  table now leads with the reality map; new third bullet under
  *"What this is, what this is not"* names runtime enforcement as
  a non-goal explicitly.
- `external-ai-engagements/REVIEWER-BRIEF.md` — new §0
  *Thirty-second reality map* at the top, containing the same
  hard-boundary block and common-concerns table as the docs page,
  plus a link to the docs version. Acronym expansions throughout
  the file (Artificial Intelligence (AI), Software Development Kit
  (SDK), Machine Learning (ML), Yet Another Markup Language (YAML),
  Software Development Lifecycle (SDLC), Uniform Resource Locator
  (URL)).
- `README.md` — top-level acronym expansions (AI on first use,
  Continuous Integration (CI), Machine Learning (ML)) and a new
  hard-boundary paragraph in the *"What this is not"* section.

**Why this matters:**

The hard-boundary statement (*"this framework does not enforce
agent runtime behaviour. If you need containment, sandboxing,
execution blocking, or tool-call permissioning, use
infrastructure-level solutions"*) was load-bearing in the
v1.1.21–v1.1.25 ChatGPT (web) engagement: external reviewers
arriving cold need the boundary up-front so their critique targets
what the framework actually claims, not what the name suggests.
Shipping the same block in three locations (README, docs index,
brief) means readers landing via any of those entry points see the
boundary before forming an interpretation.

The reality map's common-concerns table also addresses the
*mechanism-discoverability gap* observed in the engagement
(round-1 §2.3 / §5.3): reviewers were inferring absence from
invisibility because the brief did not point at the framework's
in-repo mechanisms by name. The map's *Where to check* column now
points at `learnings/_template.yaml`, `agents/MANIFEST.yaml`,
`docs/tool-enforcement.md`, and other concrete artefacts so the
reader has a direct path. Action-plan items D1 / D6 (point at
`cannot_touch:` and `_check_ownership_overlaps` by name) refine
this further but are deferred to a separate MR after the action
plan is approved.

**Self-conformance:**

- `agov-validate-acronyms` clean on touched files (README,
  index.md, REVIEWER-BRIEF.md, reviewer-reality-map.md).
- `uv run mkdocs build --strict` passes locally.
- Negation-then-correction rule honoured — the hard-boundary
  block is positive-form (*"runtime containment belongs to the
  execution environment"*) rather than rhetorical setup-and-
  knockdown.

## v1.1.25 (2026-05-04)

Adds a proposed action plan in `docs/plans/` derived from the v1.1.21
ChatGPT (web) engagement. The plan is a *proposal awaiting human
approval*; it does not authorise implementation work.

> **Note on version number.** This release was originally drafted as
> v1.1.23. The capability-developer analysis (drafted in parallel as
> v1.1.24) merged first, so this release re-numbered to v1.1.25 to
> stay above current main. The branch name `docs/v1.1.23-action-plan`
> was kept for merge-request continuity; only the version artefacts
> moved.

**What ships:**

- `docs/plans/2026-05-04-chatgpt-engagement-action-plan.md` — new
  ~750-line action plan structured per
  [`.claude/rules/workflow/issue-assessment-rule.md`](../.claude/rules/workflow/issue-assessment-rule.md):
  Overview, Sequencing (five phases), Per-item assessment for
  fourteen items (Accepts / Pushback / Recommendation / Migration /
  Affected files), Doc-update audit (six items), Out of scope, and
  Codebase reality notes flagging three places where the
  engagement's framing diverged from current repository state.

**Why this matters:**

The plan turns the engagement's fourteen specified backlog items
into a sequenced, file-grounded slate that the human can review,
modify, and (for approved items) file as GitLab issues. Three
codebase reality notes prevent surprise during issue filing: there
is no `framework.yaml` or `schemas/` directory today (greenfield);
exit codes are binary 0/1 across all eleven validators today
(taxonomy is larger surface than implied); `pre_tag_check.py` is
already an orchestrator (item A3 is a refactor, not greenfield).

The plan **does not authorise implementation**. Per the
[issue-assessment-rule](../.claude/rules/workflow/issue-assessment-rule.md),
the human approves the slate before issue filing or any code
change. Per the
[external-ai-engagements promotion path](../external-ai-engagements/README.md#promotion-path),
none of the three meta-findings the plan grounds on (layer-3
binding gap, specification-vs-verifiable reframe,
runtime-artefact-production-required boundary) promote to learning
records on this single engagement; they remain provisional
hypotheses on the engagement file's promotion-candidate list.

**Self-conformance:**

- `agov-validate-acronyms` clean on the touched file.
- Plan headers explicitly state "Proposed — awaiting human
  approval" per the issue-assessment-rule.
- No code changes. No issues filed. No agent definitions modified.

## v1.1.24 (2026-05-04)

Adds a single-source capability gap analysis at
`learnings/.capability-analysis/2026-05-04-chatgpt-engagement.md`,
produced by role-playing the capability-developer agent's
discipline against the v1.1.21 ChatGPT (web) engagement.

**What ships:**

- `learnings/.capability-analysis/2026-05-04-chatgpt-engagement.md`
  — capability-developer working document under a dot-prefixed
  directory (so docs auto-generators do not pick it up). Sections:
  Signals identified (mapped to affected agent surfaces if
  corroborated), Corroboration check (single-source as of
  2026-05-04; second corroborating engagement required),
  Hypothetical proposals (if-corroborated only, not approved),
  Sentinel-watch patterns, and explicit Closing statement that
  no agent evolution is proposed and no learning records were
  created.

**Why this matters:**

The capability-developer agent's discipline (`agents/capability-developer.yaml`)
requires *corroborated* evidence before agent-evolution proposals.
Single-source engagement evidence does not meet that bar. The
correct output is therefore a watch-record, not a change request.
Filing this analysis demonstrates the discipline holding under
pressure — the framework refuses to change its own agent
definitions on single-source evidence even when the evidence is
specific enough to feel actionable.

The seven signals identified (layer-3 binding, runtime-artefact-
production-required boundary, `agov-validate-coherence` validator,
`agov --capabilities`, learning-record schema extensions,
cross-layer precedence, path-based revalidation triggers) are
recorded with explicit affected-agent mapping for *future*
capability-developer reviews. If a second multi-round engagement
corroborates any of them, the next reviewer can reference rather
than re-derive the analysis.

**Self-conformance:**

- No agent definitions modified. The 12 files in `agents/*.yaml`
  are untouched.
- No learning records created. The report lives at
  `learnings/.capability-analysis/`, dot-prefixed so the auto-
  generators do not pick it up. Single-source evidence does not
  earn a learning record per the framework's promotion path.
- `agov-validate-acronyms` clean.
- Promotion threshold honoured.

## v1.1.22 (2026-05-04)

Adds a project-scope learning record entry on `learnings/iron-trellis.yaml`
naming the *five-round external-Artificial Intelligence (AI) engagement
(critique → specification → execution-phase) produces mechanical-
specification slate from initial critique* pattern observed in the
v1.1.21 ChatGPT (web) engagement.

**What ships:**

- `learnings/iron-trellis.yaml` — new `learnings:` array entry dated
  2026-05-04, `category: validation`, `scope: framework`,
  `severity: informational`. Records three methodology observations:
  (1) §7 uncertainty disclosure was load-bearing for re-basing
  round-1 critiques against in-repo mechanisms the Reviewer
  Brief did not point at by name; (2) mechanism-discoverability
  gap at the brief / reality-map layer (reviewers infer absence
  from invisibility); (3) the round-by-round disposition format
  (Accepts / Pushback / Recommendation per section number) scales
  to five rounds without structural breakdown.

**Why this matters:**

The engagement methodology pattern is a project-scope observation
about the framework-as-its-own-adopter (iron-trellis). It is *not*
a learning on the fourteen framework-scope backlog items the
engagement produced — those remain single-source hypotheses on
the engagement file's promotion-candidate list, awaiting corroboration
in a second independent engagement per
[external-ai-engagements/README.md promotion path](external-ai-engagements/README.md#promotion-path).

The methodology observation itself is single-adopter evidence; the
recommendation block notes that promotion to a framework rule
("Multi-round engagement pattern" subsection in the engagement
README) requires a second multi-round engagement to corroborate
the round-by-round disposition format. Shipping Provisional per
the framework's symmetric evidence discipline.

**Self-conformance:**

- `agov-validate-learnings` clean.
- `agov-validate-acronyms` clean on the touched file.
- Promotion threshold honoured — single-source framework-scope
  hypotheses (the fourteen backlog items) stay on the engagement
  file's promotion-candidate list, not in this learning record.

## v1.1.21 (2026-05-04)

Adds the longest substantive external-Artificial Intelligence (AI)
engagement to date in
[external-ai-engagements/2026-05-04-chatgpt-web.md](external-ai-engagements/2026-05-04-chatgpt-web.md):
a five-round critique → specification → execution-phase exchange
with ChatGPT (web) that closed by mutual agreement at *"specification
sufficient for independent implementation without maintainer
interpretation."*

**What ships:**

- `external-ai-engagements/2026-05-04-chatgpt-web.md` — full engagement
  with sentinel observations, five rounds of verbatim ChatGPT artefacts,
  five maintainer dispositions, and a consolidated promotion-candidate
  list (eight entries). Frontmatter declares `signal_quality: strong`
  and `verification_status: partial round 1 (§7 self-disclaimed); strong
  rounds 2–5`.
- `external-ai-engagements/README.md` — index row added with
  five-round trajectory and round-5 execution-phase additions
  enumerated.

**Durable artefacts (single-source hypotheses pending corroboration):**

- **Three meta-findings:** layer-3 binding gap (artefact ↔
  execution-trace correlation — the missing middle between in-framework
  process compliance and out-of-framework runtime-behaviour compliance);
  specification-vs-verifiable-governance reframe;
  runtime-artefact-production-required boundary refinement (*"no
  runtime behaviour enforcement; runtime artefact production
  requirements stand"*).
- **Fourteen specified backlog items** spanning integration contract
  (pull-based, Command Line Interface (CLI) + exit codes + JavaScript
  Object Notation (JSON)/Static Analysis Results Interchange Format
  (SARIF) outputs); binding contract (Universally Unique Identifier
  (UUID) v7 correlation IDs, commit trailers, sidecars); schema
  artefacts (per-artefact JSON Schema + cross-artefact invariant
  manifest); per-entry confidence on learnings; cross-layer precedence
  canonical encoding (`framework.yaml`); `agov --capabilities`
  capability report; three-tier compliance profile; `agov validate`
  orchestrator; exit-code precedence ordering; sidecar checksum link.

**Why this matters:**

The strongest meta-finding (layer-3 binding gap) survived the §7
self-disclaimed re-base and sharpened the framework's own boundary
statement: the framework does not enforce runtime *behaviour*, but it
does require runtime *artefact production* (structured logs, signed
commits, sidecar metadata). That is a non-trivial clarification the
working position docs did not state explicitly.

Per the [external-ai-engagements promotion path](external-ai-engagements/README.md#promotion-path),
all engagement findings are recorded as single-source hypotheses;
promotion to learning records under `learnings/` requires a second
independent engagement raising the same shape.

**Self-conformance:**

- `agov-validate-acronyms` clean on touched files.
- Engagement file uses absolute GitLab Uniform Resource Locator (URL)
  cross-references in verbatim quoted material (consumable by external
  readers without local repository access).
- Promotion threshold honoured — no learning records created from
  single-source evidence.

## v1.1.20 (2026-05-03)

Adds a dedicated upgrade guide on the docs site for adopters
moving from older releases to the current version. Closes a gap
surfaced by an adopter question: cold readers landing on the
docs site looking for *"I'm on an old version, what do I do?"*
had no resolvable answer — the upgrade procedure was reachable
only through a small tip box on `/adopt/` and a single
reference in `ADOPTING.md`, with the cited "§6.4 Upgrade
cadence" anchor pointing at an empty `full-guide.md` stub.

**What ships:**

- New `docs/adopt/upgrade.md` — ~155-line guide with a
  quick-answer block, a choose-your-path table (incremental
  skill vs fresh re-adoption), what-the-skill-touches table,
  validator runs for verification, common gotchas, and a
  where-to-ask-for-help section. Names *fresh re-adoption* as
  a first-class option for deep version gaps (e.g.,
  pre-v0.40 → current), not just an exotic recovery path.
- `docs/adopt/index.md` — existing "Already adopted?" tip box
  repointed from the empty `full-guide.md §6.4` anchor to the
  new upgrade guide; deep-gap mention added so the option is
  visible from the landing tip.
- `docs/adopt/.pages` — new entry in the awesome-pages nav
  between *Full step-by-step guide* and *Adoption intensity
  guide*.
- `README.md` — pointer paragraph in the "For adopters" section
  naming the `/upgrade-agents-govern` skill, the deep-gap
  re-adoption option, and linking to the docs-site guide.

**Why this matters:**

Pre-v0.40 → current is roughly 80 versions of accumulated
breaking changes — `agov-bump` tooling (v0.58.0), Quickstart
smoke (v0.59.0), static drift catcher (v0.60.0), dual-license
stratum and `KEYS.md` (v1.0.x), five v1.1.x docs rules
(acronym lint, classification-vocabulary, glossary canonical-
source, pages-build full-scan, text-before-diagram). The
existing `/upgrade-agents-govern` skill diffs upstream-vs-local
across the whole archive and can technically run from any
version, but the adopter still needs to know whether
incremental upgrade or fresh re-adoption is the cheaper path.
The new guide names that decision explicitly with criteria.

**Self-conformance:**

- `agov-validate-acronyms` clean on all four touched files.
- New page leads with positive forms throughout
  (negation-then-correction rule).
- No diagrams added — used decision tables instead, matching
  the text-before-diagram rule's *"prefer prose if a diagram
  would be redundant."*
- Classification labels used in the new page
  (`customized: true/false`, `origin: project/upstream`) are
  defined inline at the table where they're used.

**Out of scope (separate concern):**

- `docs/adopt/full-guide.md` is still an empty stub
  (`wc -l` returns 2 — frontmatter only). Not addressed in
  this MR per split-by-concern preference; flag for follow-up
  to either populate or remove the empty file.


## v1.1.19 (2026-05-03)

First invocation of the `capability-developer` agent (Phase 4 meta
agent) since it was commissioned. Reactive analysis pass triggered
by human request per ADOPTING.md Step 7 periodic-review cadence.

**What ships:**

- New `reviews/2026-05-03_capability-developer-pass-1.md` —
  structured analysis document with 5 findings, 2 of which require
  Governor-Reviewer decisions, 3 are below-threshold tracking notes.

**What does NOT ship in this MR (by the agent's own design):**

- No agent definition modifications. The `capability-developer`
  is read-only by definition (`cannot_touch: agent definitions`).
  All proposals in the review document await Governor-Reviewer
  approval before any agent change.

**Findings summary** (full detail in the review document):

1. **Maintainer-first-eye pattern**: 6 within-project instances
   across v1.1.0 / v1.1.2 / v1.1.3 / v1.1.5 / v1.1.8 / v1.1.14.
   Significant within-project signal, BELOW the agent's 3+
   independent-projects threshold. Surfaced for Governor decision.
2. **academic-peer-reviewer ↔ academic-visual-reviewer ownership
   overlap**: standing `agov-validate-agents` warning for several
   releases. Both agents are `status: skipped` in this project's
   MANIFEST. Surfaced for Governor decision (intentional /
   accidental / dead-weight).
3. **Provisional-rule queue**: 5 rules shipped in v1.1.x carry
   explicit Provisional flags awaiting cross-adopter corroboration.
   Healthy application of symmetric evidence discipline; tracking
   index proposed as follow-up.
4. **Single-source-of-truth pattern**: 2 within-project instances
   (`abbreviations.md`, `CITATION.cff`). Below threshold; defer.
5. **Conflicting evidence**: none in this pass.

**Threshold report:**

Of the 5 evidence triggers defined in the agent's own definition,
only Trigger 5 (human request) was met. All findings are below
the autonomous-proposal threshold; they are surfaced for awareness,
not auto-proposed.

**Self-conformance:**

The pass document follows the agent's own `accountability` rules:
proposals cite specific evidence; single-project findings are
explicitly NOT treated as sufficient; affected agents listed per
finding; no artefacts modified; conflicting evidence surfaced
rather than resolved.

**Out of scope (separate MRs):**

- Acting on any of the 5 findings — each requires Governor-Reviewer
  approval first, then a separate concern-scoped MR.
- Cross-project capability analysis — requires independent adopter
  evidence the framework does not yet have.

## v1.1.18 (2026-05-03)

Adds a "Cite this repository" shields.io badge to the README,
linking to `CITATION.cff` at the repo root. Visible signal that the
project is citable; complements GitLab's native CITATION.cff
affordance.

**What ships:**

- README.md — new badge below the H1, before the descriptive lead:
  `[![Cite this repository](https://img.shields.io/badge/cite-CITATION.cff-blue)](CITATION.cff)`

**Why a static badge, not a Digital Object Identifier (DOI):**

Static badges ship today, cost nothing, give immediate visible
signal. A real DOI badge (Zenodo or similar) is a stronger
academic move (persistent identifier surviving even if GitLab
disappears) but requires DOI-issuer integration and per-release
deposits. Deferred until academic citation traction warrants the
setup; the static badge holds the conceptual slot until then.

**Out of scope:**

- DOI integration via Zenodo + GitLab. Defer until a researcher
  asks, a paper cites the framework, or a conference submission
  needs a DOI.
- License-status badge, build-status badge, version badge, etc.
  Defer per "ship one thing at a time" discipline.

## v1.1.17 (2026-05-03)

Extends Contribution Auditor with a seventh audit dimension —
**`scope_integrity`** — that flags conceptual scope drift in
incoming contributions. Provisional, single-source proposal
evidence (issue #44 / Copilot 2026-04-26 sentinel review);
underlying scope-drift risk weakly corroborated by three
external-AI engagements (ChatGPT, Lumo, Copilot) independently
raising scope misreads.

**What ships:**

- `agents/contribution-auditor.yaml` (version `0.1.0` → `0.2.0`):
  - New audit dimension `scope_integrity` with intent-aware red
    flags. Critically, mentioning out-of-scope concerns in
    *disambiguation / non-claims contexts* (e.g., README's "what
    this is not" block) is explicitly NOT flagged — only
    scope-expanding introductions are.
  - New escalation trigger routing scope-drift findings to the
    Governor-Reviewer, citing the three external-AI engagements
    as the evidence base.
  - New `scope_integrity_alert` message type on `sends_to`
    governor-reviewer.
  - New `scope_integrity_concern` logged action.
  - `framework_version: "1.1.17"` (auto-bumped).

**Why intent-aware, not keyword-match:**

A naive "flag any contribution mentioning ML / model / dataset"
check would false-positive on agents-govern's own README, which
explicitly mentions these as out-of-scope. The dimension's red
flags distinguish *introducing* out-of-scope concerns as
in-scope artefacts (problematic) from *mentioning* them in
disambiguation contexts (the established discipline).

**Why Provisional, not Recommended:**

Single-source proposal evidence per the symmetric evidence
discipline. Promotion to Recommended awaits adopter corroboration
of the dimension's value (e.g., a real contribution flagged by
the new dimension that wouldn't have been caught by the existing
six). The YAML carries a Provisional comment block above the
dimension noting the evidence base and inviting refinement.

**Closes:** issue #44.

**Related:** v0.31.0 non-claims block (the established boundary
this dimension protects); README + ADOPTING.md "what this is not"
sections.

## v1.1.16 (2026-05-03)

Extracts the citation block from `README.md` to canonical
`CITATION.cff` at the repo root, with a human-readable mirror at
`docs/reference/citation.md` on the docs site. Single source of
truth for academic citation; README and Pages quote excerpts.

**What ships:**

- **New `CITATION.cff`** — Citation File Format (CFF) v1.2.0
  machine-readable metadata at the repo root. Recognised by
  GitLab's "Cite this repository" feature and reference managers
  like Zotero / Mendeley. Single canonical source for academic
  citation; README and Pages quote excerpts.
- **New `docs/reference/citation.md`** — human-readable Pages page
  with full APA + BibTeX citation forms, granularity guidance, and
  rationale for using CFF as the canonical source. Linked from
  `docs/reference/index.md` Sections list and `docs/reference/.pages`
  nav.
- **README.md `## Citation` section reduced** to a brief APA quote
  + pointers to `CITATION.cff` and the docs-site Citation page.
  The `(Version X.Y)` excerpt remains so `agov-bump` can keep it
  synced; the BibTeX form is no longer duplicated in README (lives
  on the Pages page only).
- **`agov-bump` extension**: new `update_citation_cff()` function
  in `src/agents_govern/bump_version.py` keeps `CITATION.cff`'s
  `version: "X.Y"` field in sync with the same major.minor that
  `update_readme_citation()` writes to README. Wired into `main()`
  between README and agent YAML updates. 6 new tests in
  `tests/test_bump_version.py` (minor bump, major bump, patch
  idempotence, missing-field handling, major.minor granularity,
  pattern strictness).

**Why CFF as the canonical source:**

Same single-source-of-truth pattern as `includes/abbreviations.md`
canonicalising both site-wide hover tooltips and the auto-generated
glossary page. CFF is YAML-based, machine-readable, and recognised
by reference managers; researchers auditing the framework can drop
the URL into their reference manager and get a structured entry
without copy-paste.

**Discipline note:**

The README citation excerpt could in principle be auto-generated
from `CITATION.cff` (parallel to the glossary canonical-source
pattern). Deferred until a second adopter encounters the
manually-synced-version drift; for now, `agov-bump` keeps the two
in sync mechanically.

**Out of scope:**

- Auto-generation of the README excerpt from CITATION.cff at
  build time (would need a pre-commit / CI hook). Defer.
- Adding `preferred-citation:` block to CITATION.cff for richer
  reference-manager output. Defer until a researcher requests it.

## v1.1.15 (2026-05-03)

Maintenance pass: stats drift, broken anchors, stale CHANGELOG
TODO. Mechanical edits, no behavior change.

**What changed:**

- **README stats refresh** across 8 occurrences. The actual repo
  state is 15 codenamed projects (was 13–14 in stale references),
  209 learning records (was 194), 38 rule templates (was 36). All
  derived from `uv run agov-validate-readme-stats`; agent count
  (12) and skills count (7) were already correct.
- **docs/index.md Status section** stat refreshed (14 → 15
  projects, 194+ → 209+ learning records).
- **docs/status/maturity.md** Operational evidence row refreshed
  (14 → 15 projects, 194+ → 209+ learning entries).
- **Broken anchor `quickstart-15-minute-path-layout-b` fixed**:
  heading on `docs/adopt/index.md` was renamed at some point from
  the long form to `## Quickstart (15 min, Layout B)`, slugifying
  to a different anchor and breaking 5 referrers in `docs/index.md`,
  `docs/start/index.md`, `docs/start/should-you-adopt.md`,
  `docs/start/why-this-exists.md`, `docs/start/walkthrough.md`.
  Heading restored to long form `## Quickstart (15-minute path,
  Layout B)`. Three additional references to a non-existent
  `docs/adopt/full-guide.md` redirected to `docs/adopt/index.md`
  or to the canonical `ADOPTING.md` on GitLab.
- **CHANGELOG.md v0.30.2 (2026-04-15) release notes backfilled**
  from the unfilled `<!-- TODO: fill in release notes -->`
  placeholder. The release was a docs-only patch; backfilled with
  reference to the single commit and a note that the placeholder
  was left unfilled at original release time.
- **Citation granularity note** stale example refreshed
  (`e.g., 0.23` → `e.g., 1.1`). The example was from when the
  framework was at v0.23.x.

**Why now:**

`agov-validate-readme-stats` flagged drift; `mkdocs build --strict`
flagged broken anchors; repo-wide TODO scan surfaced the
unfilled v0.30.2 placeholder. Pre-launch (Hacker News /
LinkedIn / Bluesky) maintenance window is the right moment to
clean these up — readers landing cold on the docs site won't hit
the broken anchors or stale numbers.

**Out of scope:**

- Citation block extraction to `CITATION.cff` + `docs/reference/citation.md`
  ships separately as v1.1.16 (different concern: docs-architecture
  refactor, not drift fix).
- Acronym backlog in `tmp/*.md` external-AI engagement transcripts
  (lint exemption candidate, not blocking).
- Agent ownership overlap warning between `academic-peer-reviewer`
  and `academic-visual-reviewer` (pre-existing structural concern,
  separate analysis warranted).

## v1.1.14 (2026-05-03)

Ships the **negation-then-correction** docs rule as Provisional,
single-instance evidence per the symmetric evidence discipline.

The rule names a prose anti-pattern frequent in AI-drafted
public-facing copy: *"X is not A. Instead, B."* (Finnish: *"ei ole A.
Päinvastoin: B."*) — where clause one denies and clause two asserts,
when the reader does not need clause one to understand clause two.
Cutting the negation typically yields ~30% shorter prose with the
same content and stronger forward motion.

**What ships:**

- New rule file at `.claude/rules/docs/negation-then-correction-rule.md`
  marked **Provisional**: single-project evidence (one maintainer
  flag on a Finnish-language LinkedIn pricing draft, 2026-05-02).
  Sister to the abbreviation rule, classification-vocabulary rule,
  and text-before-diagram rule. Promotion to `rules-template/docs/`
  awaits second-adopter corroboration.
- New learning entry in `learnings/iron-trellis.yaml` (dated
  2026-05-02, severity: minor) documents the surfacing instance with
  worked example (Finnish LinkedIn pricing draft, before/after forms
  cutting ~30% of words).

**Pattern continuation:**

Fifth structural docs rule in the v1.1.x thread (after acronym lint
v1.1.0, classification-vocabulary v1.1.2, glossary canonical-source
v1.1.3, pages-build full-scan v1.1.5, text-before-diagram v1.1.8).
Same maintainer-first-eye discovery shape: a human reading the lived
artefact catches a discipline gap that AI author-side review of the
same draft does not flag.

**Why Provisional:**

One instance is enough to name the pattern but not the cross-adopter
corroboration the framework's symmetric evidence discipline requires
for `Recommended` status. A second adopter encountering the same
shape promotes the rule. Locale-independent: the regex shapes for
Finnish (`\bei ole\b[^.]+\.\s*(Päinvastoin|Sen sijaan)\b`) and
English (`\bis not\b[^.]+\.\s*(Instead|On the contrary|Rather)\b`)
detect the same pattern; same shape probably exists in every
European language.

**Methodology corroboration:**

Same maintainer-first-eye pattern as v1.1.0 (acronym sweep), v1.1.2
(maturity-vocabulary audit), v1.1.3 (glossary-vs-abbreviations
duality), v1.1.5 (pages-build full-scan), v1.1.8 (text-before-diagram).
The reader-eye stays load-bearing across docs, comms, and marketing
scopes alike.

## v1.1.13 (2026-05-03)

Pre-launch brutal-honesty audit pass on the framework's
self-classification claims. Tightens claims to match the
promotion-rule discipline the framework prescribes to adopters —
practising what we preach, per the self-conformance rule.

**What changed:**

- **README Status line** now leads with single-developer evidence
  base (14 codenamed projects, 194 learning records) and links the
  maturity self-assessment + Independent-adopters-wanted section.
  Replaces the prior "validated across 14 projects" wording, which
  conflicted with the framework's own promotion rule (validation
  requires *independent*-project evidence; all 14 are
  single-developer).
- **docs/index.md Status section** rewritten with the same
  single-developer disclosure, surfacing the maturity self-assessment
  page in the lead.
- **docs/status/maturity.md**: External AI readability demoted
  from Adequate to Provisional. The cited evidence (ChatGPT/Lumo
  misread → fixed by v0.31.0 non-claims block) is correction, not
  validation; retest after fix has not been performed. Promotion to
  Adequate awaits retest.
- **docs/status/maturity.md**: "Multi-adopter corroboration"
  renamed to "Intra-project corroboration" to disambiguate from
  "Independent adopter evidence" (the row immediately below).
  Single-developer-multi-instance corroboration is not the same as
  independent multi-adopter corroboration; the row title now
  reflects which is which.
- **framework.md** §"Empirically validated" citation rewording:
  `*Empirically validated: frost-quill, iron-gate*` is reworded to
  `*Corroborated by: ... — both single-developer.*` "Empirically
  validated" is defined upstream in the same section as requiring
  multiple *independent* projects; applying the label to two
  single-developer codenames was a definition-vs-usage gap.

**Why now:**

Pre-launch (Hacker News / LinkedIn / Bluesky) distribution drafts
are being prepared in `marketing/` (separate concern, not in this
MR). The HN audience in particular scrutinises overclaim shapes.
Better to apply the framework's own promotion-rule discipline to
its own classifications before the audit happens externally.

**What is not changed:**

The three remaining Stable rows on the maturity page (Conceptual
model, Tooling stability, Release process) stay Stable. Their tier
definition on the same page is *"structurally unchanged across
multiple releases"*, which is a temporal-stability claim distinct
from multi-project-validation. The temporal-stability claim is
defensible under single-developer use; demoting these rows would be
performative honesty rather than principled honesty.

No CC BY-SA 4.0 / dual-license changes. No agent definition content
changes (only the `framework_version` field auto-updated by
`agov-bump`). No rule-template changes (the negation-then-correction
rule shipping in v1.1.14 is a separate concern).

## v1.1.12 (2026-05-03)

Fixes the Layout-B blind spot in `agov-validate-adoption` reported by
the twin-doors adopter (issue #48, filed 2026-05-03). Reduces the
validator's scope from "is the whole adoption setup correct?" to "is
the governance surface correct?" — the smaller surface that the tool
can reliably check across both supported layouts.

**Root cause analysis:**

`src/agents_govern/_files.py:repo_root()` walks up from its own
source-file location looking for a directory containing both
`VERSION` and `framework.md`. Under Layout B (framework as a
subdirectory of an existing project), that resolves to the
framework subdirectory (`agents-govern/`), not the project root.

`validate_adoption.py` then derived four project-tooling check paths
from that root — `.claude/rules/`, `tests/`, `.pre-commit-config.yaml`,
and the `rules-template/` ↔ `.claude/rules/` sync check — and found
them missing. On a correctly-laid-out Layout-B project (`.claude/rules/`
and `tests/` at the project root per ADOPTING.md's "Choose your
layout" table), the validator emitted up to ~40 spurious warnings.
Twin-doors landed on 39.

**The deeper smell:** all seven other validators in the family share
the same `repo_root()` helper. Patching `validate-adoption` with
layout-aware heuristics would buy off the symptom without addressing
the shared blind spot — and would double the test surface for every
future check.

**The fix shape:** scope reduction, not heuristics. The validator's
defensible jurisdiction is the framework-shaped artefacts (agents/,
GOVERNANCE.md, license headers, manifest); those locations are
fixed by ADOPTING.md's layout table and the validator can check
them reliably from either layout. Project-tooling paths whose
location depends on the layout are dropped from the validator's
contract entirely.

**Changed:**

- [src/agents_govern/validate_adoption.py](src/agents_govern/validate_adoption.py):
  removed the `.claude/rules/`, `tests/`, `.pre-commit-config.yaml`,
  and `rules-template` ↔ `.claude/rules/` sync checks. Module
  docstring + `_HELP` text updated to reflect the new scope.
  Summary banner reads *"Governance-surface conformance"* (was
  *"Adoption conformance"*).
- [tests/test_validate_adoption.py](tests/test_validate_adoption.py):
  replaced `TestValidateAdoptionRulesSync` and the project-tooling
  cases inside `TestValidateAdoptionWarnings` with a
  `TestProjectToolingOutOfScope` class that asserts the absent-checks
  no longer warn. Added `TestGovernanceSurfaceOnly` that exercises
  the canonical "no .claude/, no tests/, no .pre-commit, no
  rules-template" project — must pass with zero warnings.

**Kept:**

- `agents/` (ERROR if missing) — Step 1.3
- Mandatory agents `contribution-auditor` + `infosec-sentinel` (ERROR)
- `GOVERNANCE.md` (WARN if missing) — Step 2
- SPDX license headers on every agent definition (WARN if missing)
- `agents/MANIFEST.yaml` present and conformant (WARN if missing)

**Migration / behaviour change:**

Layout-A adopters who relied on the dropped warnings as a "did I
forget to copy rules / set up tests / install pre-commit?" nudge
lose those warnings. Equivalents are two-line pre-commit additions
(`test -d .claude/rules`, `test -d tests`, `test -f .pre-commit-config.yaml`)
in the adopter's own `.pre-commit-config.yaml`. The framework
deliberately does not ship those as defaults because the same
checks misfire under Layout B and the framework cannot reliably
distinguish the two from inside its own subdirectory.

A follow-up issue will be filed proposing an architectural split
of `agov-validate-adoption` into two tools — `agov-check-surface`
(the current narrowed scope) and `agov-check-setup` (project-root
checks, intended to be invoked from the project root regardless
of layout). Until that lands, the surface-only check is the only
mechanically-enforced part of `validate-adoption`.

**Self-conformance:**

- 316 tests pass.
- This repo (Layout A) still passes `agov-validate-adoption` with
  zero errors and zero warnings on the narrowed surface.
- The `validate-adoption` job in `.gitlab-ci.yml` remains required
  per `tests/test_ci_self_conformance.py` — the job invocation
  is unchanged; only its scope narrows.

**Why this was missed before twin-doors:**

This repo IS Layout A — VERSION and framework.md sit at the same
root as `.claude/rules/`, `tests/`, and `.pre-commit-config.yaml`.
The validator's project-tooling checks always passed on the
framework's own dogfooding because there is no Layout-B path on
this repo to exercise them against. The same blind-spot pattern
the self-conformance rule was created to catch — drift caught
only when an external adopter exercises a code path the
maintainer's own setup never touches.

**Evidence trail:**

- Issue #48 (twin-doors learning, 2026-05-03)
- ADOPTING.md "Choose your layout" table — line 45 establishes
  `.claude/rules/` as "tool config, not a governance artifact"
- `.claude/rules/always/governance-file-layout-rule.md` — the
  silent-failure-mode warning the validator's now-removed cargo-cult
  attractor ("just move .claude/ into agents-govern/") would have
  triggered

## v1.1.11 (2026-05-02)

Fixes a broken nav-link that v1.1.9's glossary nav promotion to
Reference left behind. The maintainer first-eye check on the live
Pages site reported *"glossary on page reference does not open"*
— the cross-section reference in `docs/reference/.pages` was
rendered as a literal `.md` link by `mkdocs-awesome-pages-plugin`
because the gen-files-virtual target wasn't visible to the plugin
at nav-walk time, regardless of plugin order.

**Root cause analysis:**

`mkdocs-awesome-pages-plugin` resolves nav entries in `.pages`
files at nav-walk time. When the entry is a cross-section path
like `../concept/glossary.md` AND the target is a virtual file
generated by `mkdocs-gen-files`, the plugin emits the literal
path (`../../concept/glossary.md` from `/reference/`) without
running it through MkDocs's `.md` → URL rewrite. Result: a
404-bound nav-link.

Reordering plugins to run `gen-files` before `awesome-pages`
did not fix it (the plugin appears to walk the on-disk
filesystem at nav-resolve time, not the MkDocs in-memory file
index). The clean fix is to keep the file in the same section
as its `.pages` reference.

**Fixed:**

- [docs/_gen/glossary.py](docs/_gen/glossary.py): canonical glossary
  now renders to `reference/glossary.md` (was `concept/glossary.md`).
  Same content; new URL is `/reference/glossary/`.
- [docs/reference/.pages](docs/reference/.pages): replaced
  `Glossary: /agent-governance/concept/glossary/` (a brittle
  hardcoded site-prefix workaround) with `glossary.md`
  (sibling within Reference; renders cleanly).
- Body link in [docs/reference/index.md](docs/reference/index.md)
  now points at sibling `glossary.md`.
- Body link in [docs/concept/index.md](docs/concept/index.md)
  now points at `../reference/glossary.md`.
- Body link in [docs/evidence/index.md](docs/evidence/index.md)
  for *symmetric evidence discipline* now points at
  `../reference/glossary.md`.
- Glossary-link example in
  [.claude/rules/docs/abbreviation-rule.md](.claude/rules/docs/abbreviation-rule.md)
  now uses `../reference/glossary.md#acr` form.
- Cross-reference in
  [.claude/rules/docs/classification-vocabulary-rule.md](.claude/rules/docs/classification-vocabulary-rule.md)
  follows the same path update.

**Bookmark preservation:**

The generator also writes a small redirect stub at
`concept/glossary.md` (the old URL). The stub is a one-screen
"This page moved" notice with `<meta http-equiv="refresh"
content="0; url=../reference/glossary/">`. Existing bookmarks
and external links to `/concept/glossary/` continue to resolve
via instant browser-side redirect; no 404s.

**Self-conformance:**

- Strict-mode `mkdocs build` succeeds.
- `agov-validate-acronyms` clean on all **92 Pages-scope files**
  (the new `reference/glossary.md` is a virtual file; the new
  `concept/glossary.md` redirect stub is also virtual; both are
  built from the same generator source).
- 316 tests pass.
- Verified: nav-link from `/reference/` body now points at
  `glossary/` (clean same-section relative URL); old URL serves
  the redirect stub with the correct meta-refresh target.

**Why this was missed in v1.1.9:**

The v1.1.9 MR's verification was a build-time strict check + a
Python regex check that confirmed *"Glossary found inside
Reference nav section"* — but neither check inspected the
**href value** of the nav-link itself. The link's text was
correct and its presence in the right nav section was correct;
only the URL was broken. A reader-eye click on the live site
caught what the maintainer-eye build inspection did not. Same
locked-prediction-methodology pattern as the v1.1.x thread
generally — different angle, same lesson.

**Note for future MRs that touch nav across sections:** verify
the rendered href, not just the link's existence. A simple
Python check against `reference/index.html` for
`href="glossary/"` (relative-clean) would have caught this in
v1.1.9.

## v1.1.10 (2026-05-02)

Defines the three `documentation_state` tiers
(`minimal | adequate | comprehensive`) used in adopter learning
records. Closes the last item in the v1.1.x docs-quality thread.
Before this release the values were defined only as a YAML
comment in `learnings/_template.yaml` line 93 — used in 16 records,
rendered on every project's evidence page, but with no
reader-resolvable definition (the same gap the v1.1.2
classification-vocabulary rule named).

**Added:**

- New concept page
  [docs/concept/documentation-maturity.md](docs/concept/documentation-maturity.md):
  full criteria for each tier as a boolean checklist, examples
  from the existing 16 adopter records, an explicit
  disambiguation from the docs-site maturity-status `Adequate`
  vocabulary, guidance on self-assessment, and a note on
  movement between tiers (one-way in practice; what does happen
  is drift).
- Page added to Concept section nav
  ([docs/concept/.pages](docs/concept/.pages)) between Pipeline
  and the full framework reference.
- Tooltip definitions in
  [includes/abbreviations.md](includes/abbreviations.md) for the
  three values: `*[minimal]`, `*[adequate]` (lowercase, distinct
  from the maturity-status `*[Adequate]` capitalised), and
  `*[comprehensive]`. Each links to the new concept page for
  full criteria.
- Glossary picks them up automatically (build-time generator
  from v1.1.3); count is now ~123 entries.

**Changed:**

- `documentation_state` comment in
  [learnings/_template.yaml](learnings/_template.yaml) now
  points at the new concept page for criteria.
- `validate_acronyms.py` `_NON_ACRONYMS` extended with
  `TESTING` (filename used as an example in the new concept
  page; same shape as the existing `BREAKING` / `BASELINE`
  filename markers).

**Tier criteria summary:**

| Tier | Audience |
|---|---|
| `minimal` | Solo / pre-collaboration |
| `adequate` | Team-internal onboarding |
| `comprehensive` | External-contributor-readable |

Each tier requires *all* its checks plus all checks of every
lower tier. Boundary projects choose the lower tier — honest
under-claiming over tier creep.

**Self-conformance:**

- Strict-mode `mkdocs build` succeeds.
- `agov-validate-acronyms` clean on all 91 Pages-scope files.
- 316 tests pass.

**Closes the v1.1.x thread:**

| Release | Discipline |
|---|---|
| v1.1.0 | Acronyms expanded on first use (mechanical lint) |
| v1.1.1 | GPG commit-signing key documented |
| v1.1.2 | Classification-vocabulary rule (Provisional) |
| v1.1.3 | Glossary canonical-source-derived |
| v1.1.4 | Architecture overview text-first |
| v1.1.5 | Pages-build full-scan + 119-violation drain |
| v1.1.6 | Pipeline page text-first |
| v1.1.7 | Feedback-loops per-loop split |
| v1.1.8 | Text-before-diagram rule (Provisional) |
| v1.1.9 | Glossary nav promotion to Reference |
| v1.1.10 | `documentation_state` tier definitions |

Eleven releases. The thread's overall arc: a maintainer
reader-eye question on the live Pages site surfaces a
discipline gap; per-page boy-scout fix lands; pattern recurs
two-to-four times; rule formalises; mechanical enforcement
(where feasible) follows; deferred items eventually ship as
their own MRs. Three sibling docs rules
(`abbreviation-rule.md`, `classification-vocabulary-rule.md`,
`text-before-diagram-rule.md`); one new concept page
(this MR's `documentation-maturity.md`); glossary 8 → ~123
entries; one new mechanical CI gate (pages-build full-scan).

## v1.1.9 (2026-05-02)

Glossary moved from **Concept** to **Reference** in the docs site
nav. v1.1.3 deferred this nav promotion until the glossary had
substance worth promoting (the file was an 8-entry stub at the
time); after v1.1.3's generator and v1.1.5's vocabulary additions
took it to ~120 entries, the move can ship.

**Why Reference:** glossary entries serve readers across all
sections (Adopt, Status, Evidence, Position, Concept). Putting it
under Reference matches the *"look something up"* mental model;
Concept is for reading-by-topic.

**Changed:**

- [docs/concept/.pages](docs/concept/.pages): removed
  `glossary.md` from the section's nav list.
- [docs/reference/.pages](docs/reference/.pages): added
  `Glossary: ../concept/glossary.md` as the first nav entry under
  Reference (above Agents, Rules, Licensing, Commercial license,
  Project keys).
- [docs/concept/index.md](docs/concept/index.md): "What's in this
  section" no longer lists glossary; instead points to
  Reference → Glossary with one-paragraph context (cross-cutting,
  ~120 entries, build-time generated).
- [docs/reference/index.md](docs/reference/index.md): Sections
  list now leads with Glossary above Agents and Rules.

**URL unchanged:** the file still lives at
`docs/concept/glossary.md` (the build-time generator from v1.1.3
writes the virtual content to that path). The page URL stays at
`/concept/glossary/` so any external link, bookmark, or search
engine result keeps resolving. Only the nav placement changed.

**Self-conformance:**

- Strict-mode `mkdocs build` succeeds.
- `agov-validate-acronyms` clean on the touched files.
- `pages-build` full-scan lint (v1.1.5) covers them.
- Verified at build-time: the Glossary entry now renders inside
  the Reference nav section of every page (Python regex check on
  built `concept/glossary/index.html`).

**Closes a v1.1.3 deferred follow-up.** The two-step plan
(*"move now, populate later"* vs. *"populate first, move when
substantive"*) shipped its second step. The glossary is now both
prominent (top of Reference) and substantive (~120 entries).

**Deferred:**

- `documentation_state` tier definitions
  (`minimal | adequate | comprehensive`) — needs concrete criteria
  per tier (READMEs? Architecture Decision Records (ADRs)?
  test coverage thresholds?). Last item in the v1.1.x docs-quality
  thread.

## v1.1.8 (2026-05-02)

Formalises the four-instance pattern from the v1.1.x thread as a
sibling docs rule. After v1.1.4 (architecture overview),
v1.1.6 (pipeline), and v1.1.7 (feedback-loops) all applied the
same maintainer-first-eye fix — *"text before diagram"* — the
pattern is structural enough to ship as a rule.

**Added:**

- [.claude/rules/docs/text-before-diagram-rule.md](.claude/rules/docs/text-before-diagram-rule.md):
  the rule. Every diagram on a Pages-rendered page must be
  preceded by prose that names what the diagram shows, states any
  reading-direction or arrow convention, and tells the reader the
  take-away. Per-diagram prose preferred when a page contains
  multiple distinct concepts; per-page prose acceptable for
  multiple diagrams serving one unified concept.

  Exempt: auto-generated reference pages where surrounding
  rendered prose already provides the explanation; one-shot
  illustrative code blocks; decorative imagery.

  Reviewer-checked, not mechanically enforced (same activation
  mode as `classification-vocabulary-rule.md`). A future
  mechanical check is sketched in the rule body — parse
  Mermaid / structural-image references and flag those with no
  preceding prose — deferred until a second adopter corroborates
  the rule's value.
- Five new entries in [includes/abbreviations.md](includes/abbreviations.md):
  `ASCII`, `SVG`, `PNG`, `JPG` (image and encoding format
  acronyms used in the rule body and the rest of the docs).

**Self-conformance:**

- Rule file passes its own and the abbreviation lint.
- Strict-mode `mkdocs build` succeeds.
- 316 tests pass.
- `pages-build` full-scan lint (v1.1.5) covers all touched files.

**Provisional status:**

The rule ships **Provisional** per the symmetric evidence
discipline: four instances within one project (this framework's
own Pages pages), no second-adopter corroboration yet. Promotion
to `rules-template/docs/` for adopter use awaits a second adopter
encountering the same shape — diagrams shipped without orienting
prose on their own docs site.

**Evidence:**

- `learnings/iron-trellis.yaml` 2026-05-02 entry titled
  *"Diagrams shipped without preceding prose: same maintainer-eye
  pattern repeats four times across the v1.1.x thread."* Captures
  the four instances and corroborates the locked-prediction-
  methodology pattern (maintainer reader-eye catches what
  AI-driven authorship in isolation does not).

**v1.1.x thread now has four sibling docs rules:**

| Rule file | Mode | Shipped |
|---|---|---|
| `abbreviation-rule.md` | Mechanical lint | v1.1.0 |
| `classification-vocabulary-rule.md` | Reviewer-checked | v1.1.2 |
| (glossary single-source generator, not a rule) | Build-time | v1.1.3 |
| (pages-build full-scan) | Mechanical CI gate | v1.1.5 |
| `text-before-diagram-rule.md` | Reviewer-checked | **v1.1.8 (this MR)** |

The thread's overall shape: a maintainer reader-eye question on
the live Pages site surfaces a discipline gap; per-page boy-scout
fix lands; pattern recurs; rule formalises; mechanical
enforcement (where feasible) follows.

**Deferred:**

- Glossary nav promotion to Reference (substance now exists,
  ~116 entries from v1.1.3 + v1.1.5).
- `documentation_state` tier definitions
  (`minimal | adequate | comprehensive`) — needs concrete
  criteria.

## v1.1.7 (2026-05-02)

Third in the text-first-before-diagrams series (after v1.1.4
architecture overview and v1.1.6 pipeline). The
[feedback-loops page](https://janipaijanen.gitlab.io/agent-governance/evidence/feedback-loops/)
rendered all four loops as one big subgraph-of-subgraphs diagram
under "The four loops" with no per-loop prose. The maintainer
asked for each loop introduced in text first, with its own
diagram, *"each graph separately if possible"*.

**Changed:**

- [docs/evidence/feedback-loops.md](docs/evidence/feedback-loops.md):
  the "The four loops" section now has **four subsections**, one
  per loop. Each has:
  - A short prose paragraph explaining the loop's trigger,
    artefact, scope, and propagation path.
  - Its own Mermaid `flowchart TD` (split out from what was a
    subgraph of the previous mega-diagram).

  A fifth subsection — *"How the loops connect"* — preserves the
  two cross-loop links (operational → framework-evolution,
  framework-evolution → agent) that were previously implicit
  arrows between subgraphs in the mega-diagram. They are now an
  explicit small diagram with prose framing.

  The Loop details table, *What this is not*, and *Where to look
  for each loop in practice* sections are unchanged.

**Self-conformance:**

- Strict-mode `mkdocs build` succeeds.
- `agov-validate-acronyms` clean on the touched file.
- `pages-build` full-scan lint (v1.1.5) covers this page.
- 316 tests pass.

**Pattern reinforcement (now four within-project instances):**

| Release / MR | Page | Diagram type | Treatment |
|---|---|---|---|
| v1.1.4 | `/concept/framework/#architecture-overview` | Mermaid `flowchart` (LR→TD) | Five-paragraph intro before the diagram |
| v1.1.6 | `/concept/pipeline/` | Mermaid `sequenceDiagram` | Numbered-list step-by-step before the diagram |
| **v1.1.7 (this MR)** | `/evidence/feedback-loops/` | Mermaid `flowchart` × 4 + 1 connector | **Per-loop split** — four prose-then-diagram subsections + cross-loop-links subsection |

Four within-project instances justifies the "every diagram is
preceded by prose explaining what it illustrates" pattern as a
docs rule. Promotion deferred to its own MR (sibling to
`abbreviation-rule.md` and `classification-vocabulary-rule.md`),
shipped Provisional pending second-adopter corroboration of the
discipline's value.

**Deferred to follow-ups:**

- **Text-before-diagram docs rule** — formalise the four-instance
  pattern (`abbreviation-rule.md`/`classification-vocabulary-rule.md`
  sibling).
- **Glossary nav promotion** to Reference (substance now exists,
  ~116 entries from v1.1.3 + v1.1.5).
- **`documentation_state` tier definitions** — needs concrete
  criteria.

## v1.1.6 (2026-05-02)

Same shape as v1.1.4 (architecture overview text-first): the pipeline
concept page at
[/concept/pipeline/](https://janipaijanen.gitlab.io/agent-governance/concept/pipeline/)
opened with a one-sentence header followed immediately by the
sequence diagram, with no orienting prose explaining what the
diagram tries to show. A maintainer first-eye question — *"what
exactly does the pipeline try to illustrate?"* — surfaced the gap.

**Changed:**

- [docs/concept/pipeline.md](docs/concept/pipeline.md): page now
  leads with a paragraph framing the shape (seven steps, two
  gates, handovers as artifacts), a numbered list explaining each
  step and what each gate does, and a paragraph explaining why
  the diagram is rendered as a sequence diagram (handovers, not
  boxes) before the diagram itself. The diagram is unchanged.

**Self-conformance:**

- Strict-mode `mkdocs build` succeeds.
- `agov-validate-acronyms` clean on `docs/concept/pipeline.md`
  (CI expanded as Continuous Integration on first use).

**Pattern:**

Third instance of the "text-first before each diagram" maintainer
preference (after v1.1.4 framework architecture overview and the
queued feedback-loops MR). At three instances, the pattern is
worth promoting from per-page edits to a documentation rule.
Deferred to a follow-up: a `docs-rule.md` line "**every diagram
is preceded by prose explaining what it illustrates**" alongside
the abbreviation rule and classification-vocabulary rule, with
the same provisional-status discipline.

## v1.1.5 (2026-05-02)

Closes the v1.1.x abbreviation thread by drain-and-enforce: drains
the remaining ~119 acronym violations on Pages-scope files
(`docs/**/*.md` plus the 5 passthrough sources beyond `framework.md`
which v1.1.4 drained) and adds a full-scan step to the pages-build
CI job so the published docs site stays acronym-clean by mechanical
guarantee.

User-authorized as **option `b` until further notice** during
the v1.1.4 conversation thread, after a maintainer first-eye
question — *"was SDLC uncatched by the regex scanning for
abbreviations?"* — surfaced the trade-off in v1.1.0's
changed-files-only enforcement: low-traffic pages accumulate
acronym dust until someone touches them. Five-MR thread closes
here.

**Added:**

- **Pages-build full-scan acronym lint** in
  [.gitlab-ci.yml](.gitlab-ci.yml). The pages-build job now runs
  `agov-validate-acronyms` on every file rendered to Pages
  (`docs/**/*.md` plus the 6 passthrough sources at the repo
  root) before the strict mkdocs build. Strict-fail; cannot
  deploy or merge with a Pages-scope violation regardless of
  which file changed in the MR. Per-MR `validate-acronyms`
  remains changed-files-only — adopters who copy the validator
  get the lighter discipline; the framework's own published
  site gets stricter discipline.
- **Self-conformance test**
  `TestPagesBuildAcronymsFullScan` in
  [tests/test_ci_self_conformance.py](tests/test_ci_self_conformance.py)
  with 3 assertions: full-scan step exists, covers the 6
  passthrough sources, and walks `docs/`. Drift-catcher: the
  step cannot be silently removed.
- **76 new entries in `includes/abbreviations.md`**
  ([includes/abbreviations.md](includes/abbreviations.md)) —
  common technical (API/HTTP/SQL/...), security (XSS/CSRF/SSRF/
  SAST/DAST/CVE/RBAC/OIDC/...), compliance (GDPR/HIPAA/SOX/PCI/
  PII/NIST/ISO/IEC/IEEE/OECD/SANS/RMF/SDL/CMMI/CTI/CTF/...),
  responsibility-matrix (RACI/RASCI/PACSI/DACI), file-format
  (CAD/DXF/GLB/OOXML/MTEXT/SHACL), and license (MIT/GPL/SPDX)
  vocabulary. The generator picks them up automatically;
  glossary went 40 → 116 entries.

**Changed (boy-scout drain — 119 inline expansions across 31 files):**

- 28 fixes on [ADOPTING.md](ADOPTING.md): LLM, PII, SSN, GPS,
  HTTPS, TLS, API, GPT, YAML, MIT, TDD, CSS, WCAG, SPDX, ADR,
  DSAR, SQL, XSS, CVE, HTML, LTR, RTL, ORM, GPL, DPA, RPO, RTO.
- 9 fixes on [LICENSING.md](LICENSING.md): CLI, SPDX, ADR.
- 6 fixes on [LICENSE-COMMERCIAL.md](LICENSE-COMMERCIAL.md):
  SPDX, MIT, SLA, SSH, PDF, ADR.
- 6 fixes on [bibliography.md](bibliography.md): URL, RACI,
  ARPA, DACI, PACSI, RASCI.
- 68 fixes across 25 `docs/**/*.md` files: ADRs, position pages,
  evidence pages, start pages, reference pages, etc.
- [src/agents_govern/validate_acronyms.py](src/agents_govern/validate_acronyms.py):
  `_NON_ACRONYMS` extended with section-marker / shouted-English
  false-positives (FEEDBACK, LOOPS, START, HERE, UNTESTED, WARN,
  GAP, GAPS, PARTIAL, PARTIALS, CONFORMANT) and the git-ref
  code-token HEAD.

**Self-conformance:**

- 316 tests pass (was 313; +3 for the new `TestPagesBuildAcronymsFullScan`).
- Strict-mode `mkdocs build` succeeds.
- `agov-validate-acronyms` clean on **all 85 Pages-scope files**
  (76 in `docs/` + 6 passthrough sources + 3 tooling files
  outside Pages scope but touched by the MR).

**Pattern reinforcement (v1.1.x thread closed):**

| Release | Discipline shipped | Concrete fix |
|---|---|---|
| v1.1.0 | Acronyms expanded on first use per file (mechanical lint, changed-files-only) | Rule + `agov-validate-acronyms` |
| v1.1.2 | Classification vocabulary defined upstream of use (provisional, reviewer-checked) | Sister rule + maturity-page legend |
| v1.1.3 | Glossary made canonical-source-derived (one source, two artefacts) | Generator + 3 more classification systems |
| v1.1.4 | Architecture overview text-first + LR→TD; framework.md drained | 29 boy-scout fixes |
| v1.1.5 | Pages-build full-scan enforcement; ~119 boy-scout fixes; backlog drained | Two-tier enforcement model |

Five-MR structural improvement: vocabulary defined and resolved
upstream of where it appears, on every Pages deploy.

**Two-tier enforcement note:**

The framework now holds its **own published site** to a higher
bar than it imposes on **adopter MRs** by default. Same shape as
validate-learnings/validate-infosec. Adopters can opt into the
stricter discipline by adding the same full-scan step to their
own pages-build (or equivalent deploy-gate) job; the validator
is unchanged.

**Deferred:**

- feedback-loops page text-first treatment (each loop introduced
  in prose before its graphic) — own MR.
- Glossary nav promotion to Reference (substance now exists,
  ~116 entries) — own MR.
- `documentation_state` tier definitions
  (`minimal | adequate | comprehensive`) — needs concrete criteria,
  richer fix.

## v1.1.4 (2026-05-02)

Two small but visible reader-experience fixes on the framework
concept page, plus the boy-scout drain that the abbreviation rule
forced once the page was touched.

The maintainer flagged the
[Architecture Overview section](https://janipaijanen.gitlab.io/agent-governance/concept/framework/#architecture-overview)
as graphic-first with no orienting prose, and asked for the
diagram to render top-to-bottom for narrow-viewport readability.

**Changed:**

- [framework.md](framework.md): the *Architecture Overview*
  section now leads with a five-paragraph textual explanation of
  the chain (Five Problems → Agent Taxonomy → Rules and Skills →
  Development Pipeline → Evidence-Driven Evolution → feedback
  loop) before showing the Mermaid diagram. The diagram itself
  switched from `flowchart LR` to `flowchart TD` for narrow
  viewports.
- 27 boy-scout acronym expansions across `framework.md` (ADR,
  API, CLI, SLO, CRUD, RBAC, HTTP, OIDC, SAST, DAST, ARIA, RTL,
  LTR, ORM, PII, DSAR, ISO, CVE, SBOM, CSV, SHACL, RACI, LOC,
  YAML, TLS, plus `MAJOR`/`MINOR`/`PATCH` wrapped in backticks
  as semver tokens). The page hadn't been edited since v1.1.0
  shipped, so the changed-files-only lint had grandfathered all
  of these — same trade-off illustrated by the v1.1.3
  should-you-adopt drain.

**Tooling:**

- [validate_acronyms.py](src/agents_govern/validate_acronyms.py):
  extended the `_NON_ACRONYMS` allowlist with English verbs that
  appear shouted in caps in agent prompt role text (`READ`,
  `STOP`, `OWNS`, `OWN`, `WRITE`, `DELETE`, `UPDATE`, `CREATE`,
  `MODIFY`, `EXECUTE`, `BLOCK`, `MUST`, `NEVER`). These are
  not acronyms; the lint was treating them as such because the
  caps regex doesn't carry English-word context.

**Self-conformance:**

- 313 tests pass.
- Strict-mode `mkdocs build` succeeds.
- `agov-validate-acronyms` clean on framework.md.

**Why a separate small release:**

This MR lands as its own release per split-by-concern. The Pages
diagram readability concern is structurally distinct from the
v1.1.x classification-vocabulary thread, even if the boy-scout
drain shares the same lint-enforcement surface. Bundling would
muddy the evidence trail.

**Authorized follow-up:**

The user has authorized the **pages-build full-scan lint**
(option `W` from MR !125's tail discussion: drain the remaining
~150 Pages-scope violations in docs/ and the other passthrough
sources, then enable strict pages-build). That work starts as a
separate MR right after v1.1.4 ships; this MR drained framework.md
specifically so its 29 violations don't double-count in the W MR.

## v1.1.3 (2026-05-02)

Resolves a dual-source-of-truth gap surfaced during the v1.1.x
classification-vocabulary thread. The maintainer asked: *"why so
little entries in glossary? where are the abbreviations?"* — and
follow-ups about the by-category page legend and an SDLC term left
unexpanded on the should-you-adopt page. Audit found two parallel
definitional sources for framework vocabulary:

- `includes/abbreviations.md` — 31 entries, rendered as hover
  tooltips on every Pages page via `pymdownx.snippets` +
  `pymdownx.abbr`. Active and used by adopters.
- `docs/concept/glossary.md` — 8 entries, the "Coming in
  follow-up MR" notice unfilled since file creation. The two
  files overlapped on 4 terms and were not synchronized.

This release makes the abbreviations file the **single source of
truth** for framework vocabulary; the glossary page is now
generated from it at build time.

**Added:**

- [docs/_gen/glossary.py](docs/_gen/glossary.py): generator that
  parses `*[Term]: Definition` entries from
  `includes/abbreviations.md` and renders an alphabetically-
  sorted glossary at `concept/glossary.md` via
  `mkdocs-gen-files`. A-Z jump navigation, anchor-linkable
  per-entry headings, single-source-of-truth header.
- New entries in
  [includes/abbreviations.md](includes/abbreviations.md):
  - **Learning categories** (5): `gap`, `validation`, `tension`,
    `adaptation`, `overkill` — used in `learnings/*.yaml` and
    rendered on the evidence pages.
  - **Learning severities** (4): `critical`, `significant`,
    `minor`, `informational` — used on the same pages.
  - **`SDLC`**: Software Development Lifecycle — caught by a
    maintainer first-eye reading of `should-you-adopt.md` where
    the term appeared bare. Boy-scout: also expanded inline on
    first use in that page.
  - **`MVP`**: Minimum Viable Product — caught in the same
    boy-scout sweep on `should-you-adopt.md`.
- Visible **Legend** section at the top of the auto-generated
  [`/evidence/by-category/`](docs/_gen/learnings.py) page,
  defining all 5 categories and 4 severities in two small tables
  before the classification table that uses them. Closes the
  same gap pattern documented in the v1.1.2
  classification-vocabulary rule.

**Changed:**

- [includes/abbreviations.md](includes/abbreviations.md) header
  comment now declares the file as the single source of truth
  for framework vocabulary on the docs site, with both
  generated artefacts (tooltips and glossary) named explicitly.
  Adopters extending the file get both refreshed together.
- [docs/start/should-you-adopt.md](docs/start/should-you-adopt.md):
  three boy-scout fixes (`SDLC`, `YAML`, `MVP` expanded on first
  use). The page hadn't been edited since v1.1.0 shipped, so the
  changed-files-only lint never scanned it — illustrative of the
  trade-off in the grandfathering policy.

**Removed:**

- `docs/concept/glossary.md` (the 8-entry physical stub). Same
  URL is now served by the build-time generator; internal links
  pointing at `/concept/glossary/` continue to resolve.

**Glossary entry counts:**

| Before | After |
|---|---|
| 8 (stub) | 40 (generated) |

**Self-conformance:**

- New generator script follows the same pattern as the existing
  `docs/_gen/` scripts (agents.py, learnings.py, etc.).
- Strict-mode `mkdocs build` succeeds.
- `agov-validate-acronyms` clean on all touched markdown.

**Evidence:**

- `learnings/iron-trellis.yaml` 2026-05-02 entry titled
  *"Glossary made canonical-source-derived from
  includes/abbreviations.md (single source of truth, two
  rendered artefacts)."* Captures the third instance in the
  v1.1.x thread of "vocabulary used in public artefacts without
  reader-resolvable definition" — corroborating the v1.1.2
  classification-vocabulary rule from inside the same project.

**Pattern reinforcement:**

This is the third instance in the v1.1.x thread of the same
finding shape:

- v1.1.0 — acronyms, fixed mechanically with a lint
- v1.1.2 — maturity-status labels + sister rule, fixed
  reviewer-checked
- v1.1.3 — glossary-vs-abbreviations duality + 5 more
  classification systems (categories, severities, SDLC, MVP,
  documentation_state still pending), fixed structurally via a
  generator

Three within-project instances strongly corroborate the rule;
promotion to `Recommended` still awaits second-adopter evidence
per the symmetric evidence discipline.

**Deferred:**

- The `documentation_state` tier values (`minimal | adequate |
  comprehensive`) used in adopter learning records still lack
  reader-resolvable definitions. Each tier needs concrete
  criteria; deferred to its own MR.
- Glossary nav promotion (move from Concept to Reference now
  that it has substance) — deferred to its own MR.
- Periodic full-repo lint sweep (or pages-build full-scan
  variant) to drain the boy-scout backlog faster — deferred
  pending discussion.

## v1.1.2 (2026-05-02)

Sibling discipline to the v1.1.0 abbreviation rule: every
**classification vocabulary** used in a public artefact must have a
reader-resolvable definition upstream of its first use. The acronym
lint shipped in v1.1.0 catches one form of "vocabulary used without
definition" — `[A-Z]{3,8}` tokens. v1.1.2 names and applies the
discipline to a different form: named status / category / tier
labels rendered as prose words.

The trigger was a maintainer first-eye question right after v1.1.1
shipped: *"based on what we categorize documentation maturity? is
it documented somewhere on gitlab pages — before the maturity
classification or categories are presented in action?"* A short
audit found two instances of the gap:

- The [maturity status page](docs/status/maturity.md) used six
  status labels (`Stable`, `Emerging`, `Pending`, `Adequate`,
  `Theoretical`, `Provisional`) in its classification table. Five
  of the six had no upstream definition anywhere on the docs
  site; only `Provisional` was defined in
  `includes/abbreviations.md`.
- The same docs site auto-renders the `documentation_state`
  values (`minimal | adequate | comprehensive`) from learning
  records. Those tiers are defined only as a YAML comment in
  `learnings/_template.yaml` line 93 — no glossary, no rule, no
  concept page.
- The word `Adequate` is overloaded across the two systems with
  no disambiguation.

**Added:**

- [.claude/rules/docs/classification-vocabulary-rule.md](.claude/rules/docs/classification-vocabulary-rule.md):
  the rule, shipped Provisional. Reviewer-checked rather than
  mechanically enforced — the candidate set is open-ended (any
  prose word could be a label), so an automated lint like the
  acronym one is not yet feasible. Sketches a future mechanical
  check (parse `*[Label]:` entries from
  `includes/abbreviations.md`, scan rendered docs for
  emoji-prefixed status labels, flag any label not in the
  abbreviations file) deferred until a second adopter
  corroborates the rule.
- Five new entries in
  [includes/abbreviations.md](includes/abbreviations.md) for
  `Stable`, `Emerging`, `Pending`, `Adequate`, `Theoretical`.
  These render as hover tooltips on every Pages page via
  `pymdownx.abbr` + `pymdownx.snippets`. The `Adequate` entry
  explicitly disambiguates from the
  `documentation_state: adequate` tier in adopter learning
  records.
- Explicit **Legend** section at the top of
  [docs/status/maturity.md](docs/status/maturity.md) defining all
  six tiers in a small table before the classification table that
  uses them. Includes a callout disambiguating `Adequate` from
  the `documentation_state` tier.

**Changed:**

- [.claude/rules/docs/abbreviation-rule.md](.claude/rules/docs/abbreviation-rule.md):
  exemption list now includes `abbreviations.md` as a
  *definitional source file* (alongside the existing
  append-only-history exemptions for `CHANGELOG.md` and
  `LESSONS-LEARNED.md`). The lint-itself-uses-it file should not
  be flagged for using the very terms it defines.
- [src/agents_govern/validate_acronyms.py](src/agents_govern/validate_acronyms.py)
  + 2 new tests in
  [tests/test_validate_acronyms.py](tests/test_validate_acronyms.py)
  for the `abbreviations.md` exemption.

**Self-conformance:**

- New rule file passes its own and the abbreviation lint.
- Modified maturity page passes the abbreviation lint after two
  boy-scout fixes (`CLI` expanded; `UNTESTED` un-shouted).
- 314 tests pass (was 312; +2 for the new exemption).

**Evidence:**

- `learnings/iron-trellis.yaml` 2026-05-02 entry titled
  *"Classification vocabulary used in public artefacts without
  reader-resolvable definition (sibling pattern to acronym
  expansion gap)."* Captures the second-instance corroboration
  of the broader pattern (vocabulary used in public artefacts
  without upstream definition), the maintainer-pushback
  methodology that surfaced it, and a recommendation to define
  `documentation_state` tiers separately in a follow-up MR.

**Provisional status:**

The rule ships as **Provisional** per the symmetric evidence
discipline: two instances within one project (this framework's
own docs), no second-adopter corroboration yet. Promotion to
`rules-template/docs/` for adopter use awaits a second adopter
encountering the same shape (e.g., status pages on their own
site, severity tiers, escalation labels).

**Deferred to follow-up:**

The `documentation_state` tier values (`minimal | adequate |
comprehensive`) used in learning records are not addressed in
this MR. A separate MR will define them — either as a glossary
entry, an expanded template comment, or a concept page covering
codebase maturity end-to-end. Naming the deferred work in this
MR's learning entry per the user's split-by-concern preference.

## v1.1.1 (2026-05-02)

Documents the dedicated GPG commit-signing key in `KEYS.md` with
the cutoff Secure Hash Algorithm (SHA) of the first signed commit,
following through on the v1.0.4 iron-trellis learning that flagged
key reuse as an anti-pattern. The first signed commit on `main` is
`9f086f8` (the v1.1.0 candidate from MR !122, merged via merge
commit `2fa764a` on 2026-05-02). Earlier commits remain unsigned in
git history; their authenticity rests on git's content-addressed
integrity and GitLab's authorship records.

**Why a separate release:** the v1.1.0 commit was the first signed
commit but its release was scoped to the abbreviation rule and lint.
Documenting the commit-signing key was deliberately deferred to a
follow-up MR so that v1.1.0 carried one structural concern (rule +
lint) rather than two (rule + lint + key documentation). v1.1.1
captures the key documentation as its own release with its own
evidence trail.

**Added (KEYS.md):**

- `### agents-govern commit signing — secondary` table under the
  existing "Active keys" section, with algorithm, fingerprint, key
  ID, expiry, cutoff Secure Hash Algorithm (SHA), and revocation
  contact for the new commit-signing key.
- `## Commit signing` top-level section with the cutoff statement,
  `git log --show-signature` verification command, and a
  rationale explaining why the GPG commit key is **separate** from
  the SSH project-identity key (links to the iron-trellis 2026-05-02
  learning entry on cryptographic key reuse as an anti-pattern).
- "One key per signing purpose" preface to the "Active keys"
  section, naming the discipline explicitly so future additions
  (CI automation, hardware-token paralegal signing) follow the
  same pattern.

**Updated (KEYS.md):**

- The existing primary key table heading now reads `— primary
  (project identity)` to disambiguate from the new secondary
  commit-signing entry.

**Self-conformance:**

- KEYS.md passes `agov-validate-acronyms`. Acronyms expanded on
  first use: SSH, GPG (rendered as `GnuPG (GPG)` to satisfy the
  rule without invoking GNU as a separate token), PDF, EDDSA, SHA.
- v1.1.1 is the first release tagged after the GPG commit-signing
  cutoff; the tag itself will be GPG-signed via `tag.gpgsign true`.

**Evidence:**

- `learnings/iron-trellis.yaml` 2026-05-02 entry titled *"AI
  assistant defaulted to cryptographic key reuse during framework's
  own signing posture work."* — this MR closes the loop on that
  learning's recommendation that the principle be made explicit.

## v1.1.0 (2026-05-02)

Introduces a documentation-quality rule and the lint that enforces
it: every acronym in every markdown file must be expanded on first
occurrence, either via `Phrase (ACR)` form or via a markdown link
to the glossary. A regex sweep of the corpus found that 98 of 103
distinct non-English-word acronyms in the framework's markdown were
never expanded anywhere — a gap invisible to author-side review
because most readers silently look up unfamiliar acronyms or skim
past, but real friction for the framework's mixed audience
(software developers, security reviewers, compliance officers,
academics, and Artificial Intelligence (AI) agents configured for
governance).

The first draft of the rule carved out "industry-standard"
acronyms (`API`, `JSON`, `HTTP`) from the expansion requirement.
That carve-out was rejected: academic readers — part of the
framework's audience — follow the opposite convention, and a
software-developer-shaped exemption silently degrades the
experience for non-developer subgroups. The rule was rewritten
without the carve-out: every acronym, every file, every first use.

The lint follows the same grandfathering pattern as
`validate-learnings` and `validate-infosec`: the merge-request
Continuous Integration (CI) job and the local pre-commit hook lint
only files **changed** in the merge request / staged for commit,
so pre-existing violations don't block unrelated work. Editing a
file means accepting responsibility for its conformance — the
boy-scout rule. Initial whole-repo scan flagged ~557 violations
across ~122 files, expected to drain MR-by-MR as files are
touched. Run `uv run agov-validate-acronyms` with no arguments to
drain the backlog explicitly.

**Added:**

- [.claude/rules/docs/abbreviation-rule.md](.claude/rules/docs/abbreviation-rule.md):
  the rule. Spell out every acronym on first use per file.
  Allowlist contains only non-acronym tokens (filenames, Request
  For Comments (RFC) 2119 keywords, section markers, date
  placeholders, project-specific names). No carve-out for
  "broadly known" terms.
- [src/agents_govern/validate_acronyms.py](src/agents_govern/validate_acronyms.py)
  + 34 tests in
  [tests/test_validate_acronyms.py](tests/test_validate_acronyms.py):
  the lint. Walks every `*.md` (or only files passed on the
  command line for CI / pre-commit). Skips Software Package Data
  Exchange (SPDX) header lines, fenced code blocks, inline code,
  and Yet Another Markup Language (YAML) frontmatter before
  extraction.
- `validate-acronyms` job in [.gitlab-ci.yml](.gitlab-ci.yml) —
  runs on every merge request, lints only `*.md` files changed in
  the merge request. Mirrors `validate-learnings`.
- `validate-acronyms` hook in
  [.pre-commit-config.yaml](.pre-commit-config.yaml) — runs
  locally on staged `*.md` files.
- `TestValidateAcronymsInCI` class in
  [tests/test_ci_self_conformance.py](tests/test_ci_self_conformance.py)
  — drift-catcher: the CI job cannot be silently removed without
  the test failing.

**Self-conformance:**

The rule passes its own lint. Five acronyms (URL, RFC, SPDX, YAML,
Domain Name System (DNS)) flagged in the rule file's first draft
were expanded on first use before commit, demonstrating the rule
on itself.

**Evidence:**

- Iron-trellis learning (2026-05-02),
  [learnings/iron-trellis.yaml](learnings/iron-trellis.yaml),
  category `gap`, severity `significant`. Captures the regex sweep
  finding (98 of 103 unexpanded), the maintainer-pushback
  methodology that reshaped the rule's scope, the regex-not-list
  prompting pattern, and the "draft rule, learning record on
  commission" workflow as a first-class deliverable shape.

**Provisional status:**

The academic-strict stance (no industry-standard carve-out) ships
as **provisional** per the symmetric evidence discipline:
single-adopter (this framework's own docs). Promotion to
`rules-template/docs/` for adopter use awaits second-adopter
corroboration that the strict default fits their audience too.
Adopters with uniformly software-developer audiences may extend
their project-local `_NON_ACRONYMS` allowlist to add
industry-standard terms.

## v1.0.4 (2026-05-02)

Learning-record release. Captures a framework-level gap surfaced during
the v1.0.x dual-license rollout: the AI assistant defaulted to
recommending cryptographic key reuse ("one key, one identity") when
git commit signing came up after the project signing key had been
published in v1.0.3. The maintainer correctly rejected the recommendation
on key-per-purpose grounds. The framework had no explicit rule or
guidance about cryptographic key separation; the principle was
surfaced only by maintainer pushback, not by any framework gate.

**Added:**

- `learnings/iron-trellis.yaml` — new entry dated 2026-05-02:
  *"AI assistant defaulted to cryptographic key reuse during
  framework's own signing posture work."* Category: gap. Severity:
  significant. Recommends adding a framework-level rule documenting
  one-key-per-signing-purpose as the standard, with NIST/Mozilla/
  Apache guidance as corroborating reference; also flags that the
  InfoSec Sentinel agent's screening checklist should catch this at
  agent-config review time.

**Why this is a separate codename entry rather than a new project
codename:** the learning concerns the framework's own dogfooding
posture (publishing keys, adopting signing) rather than a specific
adopter project. iron-trellis is the established codename for
"framework dogfooding its own per-MR gates and posture"; the dual-
license rollout fits the same thread.

**Pending follow-up (driven by this learning):**

- Draft a `rules-template/always/key-per-purpose-rule.md` (or
  similar) that codifies the principle for adopters who add
  cryptographic signing to their own projects.
- Update the `infosec-sentinel` agent definition to include
  cryptographic key reuse in its screening checklist.
- Consider a static check (e.g., scan KEYS.md for any key listed
  under more than one purpose) to mechanically prevent regression.

**Self-conformance:**

- 274/274 tests pass.
- `agov-validate-learnings learnings/iron-trellis.yaml` passes
  (0 errors, 0 warnings).

## v1.0.3 (2026-05-02)

Publishes a dedicated cryptographic key for project identity and
future artifact signing. The key is published as a **commitment
signal** today — it is not yet wired to any production signing
workflow. Sophisticated buyers and reviewers ask "how does this
project sign artifacts and signal commitment?" before they trust
commercial terms; this release puts the answer in the repository
and on the docs site, before any signed artifact exists.

The framework's signing posture is staged: SSH-format signatures
(`ssh-keygen -Y sign`) using this key during the DRAFT period of
the commercial license, then X.509 / PAdES embedded PDF signatures
from a recognised CA before any commercial license PDF is executed
in counsel-reviewed final form. Customers receiving final license
PDFs will be able to verify validity directly in Adobe Reader
without additional tooling.

**Added:**

- [KEYS.md](KEYS.md) at repo root — human-readable index of project
  cryptographic keys: fingerprints, verification instructions,
  signing posture (SSH now, PAdES later), private-key signing
  workflow with `ssh-agent`, when-to-revisit triggers (CI
  automation, delegated signing), revocation policy, key custody
  notes.
- `.well-known/agents-govern.pub` — the published ED25519 public key
  itself. SHA256 fingerprint
  `Vh3YEwA1oolQ64uacYNFcGbyrbaOCkSqaHkHJa5EiaM`. Algorithmic data,
  not a copyrightable creative work; freely redistributable.
- [LICENSE-COMMERCIAL.md](LICENSE-COMMERCIAL.md) §9 "Signing and
  authenticity" — short section pointing at KEYS.md and stating the
  staged signing roadmap. Surfaces the commitment signal to anyone
  reading the commercial license DRAFT directly.
- `docs/reference/keys.md` — site-rendered view of KEYS.md via the
  existing `passthrough.py` pattern.
- `Project keys` entry under Reference in
  [docs/reference/.pages](docs/reference/.pages).

**Changed:**

- [docs/_gen/passthrough.py](docs/_gen/passthrough.py) — adds
  passthrough entry for KEYS.md, link rewrites for KEYS.md (site-
  relative) and SECURITY.md / `.well-known/agents-govern.pub`
  (GitLab-absolute).
- [.distignore](.distignore) — KEYS.md and `.well-known/` excluded
  from adopter dist. These are agents-govern's own project-identity
  artifacts; adopters publish their own keys for their own projects.

**Why publish a key now (before any signing actually happens):**

The commitment signal is the point. Buyers reading the commercial
license DRAFT can see the project has a published key, a
verification procedure, a revocation policy, and a documented
roadmap from interim SSH signatures to industry-standard PAdES
signatures. That's a credible posture even with no signed artifact
in existence today. When the first signed artifact is produced, the
trust infrastructure already exists — no new announcement required.

**Why the passphrase-protected key is the right choice:**

For a long-lived signing key the passphrase is the second factor
that protects against private-key file leakage (laptop theft,
backup compromise). Verification is unaffected — verifiers only
touch the public key. The operational pattern is `ssh-add` once per
session, then sign as many artifacts as needed. KEYS.md documents
this and the future triggers (CI automation, delegated signing)
that would warrant a different setup.

**Self-conformance:**

- 274/274 tests pass.
- `mkdocs build --strict` succeeds; the new `/reference/keys/` page
  renders with the SHA256 fingerprint, signing workflow, and
  verification instructions visible.
- Validators (`agov-validate-agents`, `agov-validate-adoption`,
  `agov-health`) all pass with no new warnings.

**Pending follow-up (unchanged from v1.0.0/v1.0.1/v1.0.2):**

- Counsel review of [LICENSE-COMMERCIAL.md](LICENSE-COMMERCIAL.md)
  to replace the draft with enforceable terms.
- Realignment of mkdocs build hooks and theme overrides to MIT
  (deferred maintenance MR).
- Prospect conversations to validate the easy-and-modest commercial
  tiers before publishing specific terms.
- First production-signed artifact (release tag, design-partner
  letter, or equivalent) using this key — at which point a release
  note will document the namespace and verification procedure for
  that specific artifact.


## v1.0.2 (2026-05-02)

Pricing language cleanup in [LICENSE-COMMERCIAL.md](LICENSE-COMMERCIAL.md).
The DRAFT commercial license shipped in v1.0.0 contained a working
sketch of pricing tiers with a specific euro figure (€1000/year for
the design-partner tier) and `TBD`/`Negotiated` placeholders for the
other tiers. Publishing specific monetary values in a pre-counsel
DRAFT was premature — the figures were maintainer working notes, not
considered offers, and risked being read as commitments by adopters
or courts. This release removes specific monetary values from
published documentation and replaces them with the public posture:
the framework operates a dual-license model with **easy and modest
commercial tiers**.

**Changed:**

- [LICENSE-COMMERCIAL.md](LICENSE-COMMERCIAL.md) section 3 — replaced
  the pricing tier table with prose describing the three tiers
  (Design partner, Standard, Enterprise) by what they're for and
  who they suit, not by amount. The "Annual fee" column is removed
  entirely; the explicit "€1000 / year" / `TBD` / `Negotiated`
  values are removed. The new prose ends with: *"Specific fees are
  not published in this draft. Terms are set in direct conversation
  with prospective licensees; the intent is friction-free adoption
  — terms designed to be accepted, not negotiated."*
- [CHANGELOG.md](CHANGELOG.md) v1.0.0 and v1.0.1 "Pending follow-up"
  entries that referenced "pricing tier validation" and the
  "design-partner sketch" softened to align with the new posture
  (no specific tier counts or sketch references).

**Unchanged:**

- License terms themselves are not affected. CC BY-SA 4.0 + DRAFT
  commercial license alternative remains the structure.
- Section 8 (Indemnity) retains the structural reference to "fees
  paid in the prior 12 months" — this is a liability cap mechanism
  referring to whatever fees may be agreed in counsel-reviewed
  terms, not an actual sum.
- All other licensing documents ([LICENSING.md](LICENSING.md),
  [CONTRIBUTING.md](CONTRIBUTING.md)) had no monetary references and
  remain untouched.

**Why a patch bump:** language cleanup, no terms change. The
dual-license offer's structure, contributor grant, and SPDX
expressions are all unchanged from v1.0.1.

**Self-conformance:**

- 274/274 tests pass.
- `mkdocs build --strict` succeeds; rendered LICENSE-COMMERCIAL page
  on the site contains no monetary values and renders the new
  "easy and modest commercial tiers" framing.

**Pending follow-up (unchanged from v1.0.0/v1.0.1):**

- Counsel review of `LICENSE-COMMERCIAL.md` to replace the draft
  with enforceable terms.
- Realignment of mkdocs build hooks and theme overrides to MIT
  (deferred maintenance MR).
- Prospect conversations to validate the easy-and-modest commercial
  tiers before publishing specific terms.






## v1.0.1 (2026-05-01)

Pages-discoverability follow-up to v1.0.0. The dual-license terms
shipped in v1.0.0 are now visible on the published docs site at
`/reference/licensing/` and `/reference/license-commercial/`, and the
site footer mentions the dual-license offer instead of advertising
CC BY-SA 4.0 as the only option for framework content. No license
terms change in this release; only how those terms are surfaced to
adopters reading the published documentation.

**Why this is a separate release.** v1.0.0 made the dual-license
enforceable in source files (SPDX expressions, contributor grant,
LICENSING.md / LICENSE-COMMERCIAL.md at repo root) but left the
published Pages site silent on the change — adopters reading the
docs would still see the old "CC-BY-SA-4.0 + MIT" footer and find no
licensing page in the navigation. That gap was a discoverability bug
flagged immediately after v1.0.0 was opened as an MR. Closing it as a
separate patch release keeps the source-of-truth licensing change
(v1.0.0) and its docs-site rendering (v1.0.1) as distinct evidence
trails per `feedback_split_mrs_per_concern`.

**Added:**

- `docs/reference/licensing.md` and `docs/reference/license-commercial.md`
  rendered at build time by extending [docs/_gen/passthrough.py](docs/_gen/passthrough.py)
  with two more passthrough entries. The repo-root files
  ([LICENSING.md](LICENSING.md) and [LICENSE-COMMERCIAL.md](LICENSE-COMMERCIAL.md))
  remain canonical; the site renders link-rewritten copies with a
  "Canonical source" admonition prepended.
- Navigation entries under `Reference` for both pages
  ([docs/reference/.pages](docs/reference/.pages)).

**Changed:**

- `mkdocs.yml` `copyright:` block — footer now reads
  *"Operational artifacts ... are dual-licensed: CC-BY-SA-4.0 OR a
  commercial license (see Licensing)"* and links to the new page.
  Python tooling and starter examples noted as MIT.
- [docs/_gen/passthrough.py](docs/_gen/passthrough.py) link-rewrite
  rules:
  - Site-relative: `LICENSING.md` and `LICENSE-COMMERCIAL.md` route
    to their on-site renditions; the v1.0.0 license ADR
    (`docs/adr/20260405-license-cc-by-sa-4.md`) routes site-relative
    so the LICENSE-COMMERCIAL.md DRAFT's link resolves cleanly.
  - GitLab-absolute: `LICENSE`, `CONTRIBUTING.md`, and
    `pyproject.toml` resolve to repo URLs since they are not
    rendered on the site.

**Self-conformance:**

- 274/274 tests pass.
- `mkdocs build --strict` succeeds; both new pages render with
  cross-links resolved (verified via grep on built HTML).
- Pre-existing INFO warnings about a missing anchor in
  `adopt/index.md` are unchanged and outside this MR's scope.

**Pending follow-up (unchanged from v1.0.0):**

- Counsel review of `LICENSE-COMMERCIAL.md` to replace the draft.
- Realignment of mkdocs build hooks and theme overrides to MIT
  (deferred maintenance MR).
- Three prospect conversations to validate pricing tiers.



## v1.0.0 (2026-05-01)

Dual-license enablement. The framework's operational artifacts (rules,
agents, skills, CI configuration, hooks) are now offered under
**CC BY-SA 4.0 OR a commercial license** at the licensee's choice;
the commercial license is currently a draft pending counsel review.
Python tooling stays MIT, evidence records (`learnings/`) stay
CC BY-SA 4.0 only, and example starter content
(`project-specific-examples/`) moves from CC BY-SA 4.0 to MIT so
adopters can copy it into their own (potentially proprietary)
repositories without share-alike contamination.

The major version bump reflects the externally-visible licensing
commitment: this is the first release under a stable dual-license
posture. The realignment was anticipated by the April 2026 license
ADR's "Reevaluation triggers" section, which named "future need for
dual licensing" and noted that contributor agreement infrastructure
would be required first. This release supplies that infrastructure.

**Added:**

- `LICENSING.md` — repository-wide license map (the authoritative
  path-to-license document)
- `LICENSE-COMMERCIAL.md` — commercial license terms, marked
  **DRAFT — NOT FOR EXECUTION** with banner and explicit
  lawyer-review-required header. Includes outline of intended terms,
  pricing tiers (sketch), and open questions for counsel.
- SPDX headers added to `.claude/rules/always/self-conformance-rule.md`,
  `.claude/rules/workflow/issue-assessment-rule.md`, and
  `.infosec-ack.yaml` (previously missing).

**Changed:**

- `CONTRIBUTING.md` — license clause expanded to grant the project
  maintainer the right to distribute stratum-1 contributions under
  additional licenses (the commercial alternative). Contributors retain
  copyright; the change is a license grant, not an assignment.
- SPDX headers in 78 stratum-1 files rewritten from
  `CC-BY-SA-4.0` to `CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial`.
  Touched: `agents/`, `.claude/rules/`, `rules-template/`, `skills/`,
  `profiles/`, `includes/`, `.git-hooks/`, `.gitlab/`,
  `.gitlab-ci.yml`, `.pre-commit-config.yaml`, `.infosec-ack.yaml`.
- SPDX headers in 7 `project-specific-examples/` files changed from
  `CC-BY-SA-4.0` to `MIT` (Stratum 3 — adopter-copy-friendly).
- Adopter-facing SPDX example blocks inside
  `skills/infosec-screen/SKILL.md` preserved as canonical
  single-license `CC-BY-SA-4.0` (those are template snippets adopters
  use, not the file's own header).

**Unchanged (intentional carve-outs):**

- `learnings/*.yaml` remain `CC-BY-SA-4.0` only — no commercial
  alternative. Evidence integrity requires that derivative learning
  collections stay open; selling the right to fork evidence privately
  would undermine the framework's credibility.
- All explanatory documentation (`README.md`, `ADOPTING.md`,
  `framework.md`, `GOVERNANCE.md`, `docs/`, etc.) remains
  `CC-BY-SA-4.0`. These are not part of the dual-license offer either
  way.
- Python tooling (`src/agents_govern/`, `tests/`) remains MIT — the
  validators are CI plumbing adopters embed in their own pipelines;
  adding share-alike or commercial-license complexity here would harm
  adoption with no commercial upside.

**Why a major bump:**

The change is a structurally externally-visible licensing commitment:
new SPDX expression on ~80 files, new contributor inbound grant, new
top-level licensing documents. Adopters' legal teams will want to
re-read the terms before pulling the next version. SemVer's signal
"break in compatibility / change of contract" is the right channel for
that. Per `feedback_semver_no_carry.md`, components don't carry, so
0.62.0 → 1.0.0 is a deliberate jump rather than a smaller increment.

**Pending follow-up (not in this release):**

- mkdocs build hooks (`docs/_gen/`, `docs/_hooks/`) and theme overrides
  (`overrides/partials/`) currently carry CC-BY-SA-4.0 SPDX headers
  inherited from earlier documentation-style treatment. Per
  [LICENSING.md](LICENSING.md) they are functionally Python/template
  code and should be MIT. Aligning their headers is intentionally
  deferred to a separate maintenance MR to keep this dual-license
  enablement change scoped.
- Counsel review of `LICENSE-COMMERCIAL.md` to replace the draft with
  enforceable terms.
- Pricing tier validation: at least three prospect conversations
  before enabling any tier other than the design-partner sketch.

**Self-conformance:**

- Validators (`agov-validate-agents`, `agov-validate-adoption`,
  `agov-health`) all pass with no new warnings.
- 274/274 tests pass.
- The single contributor history (one author across all commits) means
  no retroactive CLA gap exists; the new CONTRIBUTING.md grant applies
  going forward.



## v0.62.0 (2026-04-30)

Learning-record release. Captures the four findings from the
v0.58.0–v0.61.0 self-conformance gate session as structured evidence
in the project learning corpus.

**Added:**

- `learnings/iron-trellis.yaml` — new codename for "the framework
  dogfooding its own per-MR gates on its own MR flow." Four entries:
  layered self-conformance gates pay off (validation), agov-bump-first
  prevents propagation strands (adaptation), maintainer first-eye check
  catches what AI smoke is not scoped to walk (validation), static
  check beats human attention on known bug classes (validation).

**Changed:**

- `learnings/iron-gate.yaml` — appended a 2026-04-30 entry to the
  existing version-tag-alignment thread, recording the v0.58.0
  agov-bump-first incident as the fourth corroborating data point
  (after v0.25.2, v0.38.0, v0.41.1). No cross-codename references
  per anonymization policy.

**Why a separate codename for iron-trellis rather than extending an
existing record:** the four findings are framework-internal — about
the framework dogfooding its own gates — not sourced from any specific
adopter project's flow. Mixing them into an adopter-codenamed file
would conflate concerns. The agov-bump-first datum belongs in two
places (the version-tag thread for continuity, the new codename for
session-context); both records hold separate but consistent evidence.

**Why this is the right size:**

- Larger (also file a sentinel review summarizing the session) was
  rejected — sentinels want accumulated MR-flow evidence to assess
  "did the gates work in practice?" and we don't have that yet. The
  retrospective routine queued for 2026-05-14 is built specifically
  to gather it.
- Smaller (claude-side memory only) was rejected — these are
  framework-level disciplines that other adopters and contributors
  benefit from seeing in the public learning corpus, not just
  in the AI assistant's session memory.

---

## v0.61.0 (2026-04-30)

Quickstart fragment dedup. v0.60.0 added a static check that gated
the `mkdir/cp` invariant across 4 doc surfaces; v0.61.0 attacks the
underlying duplication for the most-bug-prone chunk.

The Layout B "rules copy" step (`mkdir -p .claude` + `cp -r
agents-govern/rules-template/ .claude/rules/`) is now a single
fragment at `includes/_qs-rules-copy-layout-b.md`. `docs/index.md`
and `docs/adopt/index.md` reference it via `pymdownx.snippets`
(`--8<-- "_qs-rules-copy-layout-b.md"`). Two static checks gate the
new arrangement: every fragment is self-consistent under the same
`mkdir-before-cp` invariant, and every command line in each fragment
appears verbatim in `ADOPTING.md` (the canonical, standalone-
rendered source — it ships in the dist tarball and cannot use
snippet directives).

Scope deliberately narrow: only the bug-prone chunk is deduped, not
the whole Quickstart. The 4 Quickstart surfaces target intentionally
different audiences (homepage vs adopt-section vs full guide vs
tool-comparison) at different verbosity levels — flattening them
into one would lose the depth gradient. Future fragments
(`_qs-skills-copy-layout-b.md`, etc.) can be added the same way.

**Added:**

- `includes/_qs-rules-copy-layout-b.md` — single source of truth for
  the rules-template copy step.
- `includes/README.md` — describes the fragment convention, the
  `_qs-*.md` underscore-prefix marker, and the
  ADOPTING.md-as-canonical sync invariant.
- `tests/test_docs_quickstart_consistency.py` — extended with
  `TestQuickstartFragments` (self-consistency + ADOPTING.md sync gate).

**Changed:**

- `docs/index.md` — Quickstart now uses `--8<--` to include the rules-copy
  fragment instead of inlining it.
- `docs/adopt/index.md` — same change.
- `ADOPTING.md` and `docs/tool-enforcement.md` — unchanged (canonical
  standalone / Layout A variant respectively).

**Why this is the right size:**

- Larger (deduplicate every Quickstart command into fragments,
  including install-uv, download-dist, pick-agent) was rejected —
  the surfaces have different verbosity (numbered vs unnumbered
  comments, prose interleaving) and a one-size-fits-all fragment
  loses that. The current scope tackles the bug-prone chunk only.
- Smaller (just add the static checks, no fragment) was rejected —
  v0.60.0 already shipped that. Without dedup the pattern keeps
  recurring (next contributor copy-pastes the broken form again).

**Evidence trail:** the dedup question was deferred from v0.60.0's
"Why this is the right size" section. Implementation surfaced that
the 4 Quickstarts are genuinely different content, narrowing the
dedup target to the shared rules-copy chunk only.

---

## v0.60.0 (2026-04-30)

Quickstart drift across docs. v0.59.0 fixed the `cp -> .claude/rules/`
bug in ADOPTING.md and added an end-to-end smoke for that file. The
maintainer-eye then asked "would the Quickstart on Pages work?" — and
it wouldn't have. The same bug was duplicated across three other
files (`docs/index.md`, `docs/adopt/index.md`,
`docs/tool-enforcement.md`), and the smoke test couldn't see them
because it only walked ADOPTING.md.

This release fixes the duplicates and adds a static check that
catches the bug class wherever it appears in maintained markdown.

**Added:**

- `tests/test_docs_quickstart_consistency.py` — scans `ADOPTING.md`
  and `docs/**/*.md` for code fences that `cp` into `.claude/<subdir>/`
  without a preceding `mkdir -p .claude...` in the same fence. Found
  the bug in 4 lines across 3 docs files (1 of which I had already
  identified by hand and 1 of which the static check surfaced that I
  missed). Includes regex sanity tests so the check itself is gated
  against drift.
- `tests/test_ci_self_conformance.py` — extended `TestQuickstartSmokeInCI`
  to assert the CI script also runs the new consistency test. Same
  drift-catcher pattern.

**Fixed:**

- `docs/index.md` — Pages homepage Quickstart now prepends
  `mkdir -p .claude` before the rules-template cp.
- `docs/adopt/index.md` — Adopt section Quickstart, same fix.
- `docs/tool-enforcement.md` — Layout A snippet (no `agents-govern/`
  prefix), same fix; the same fence's later `cp -r skills/ .claude/skills/`
  is also covered by the single mkdir.

**Changed:**

- `.gitlab-ci.yml` — `quickstart-smoke` job's script now runs both
  the end-to-end smoke and the docs consistency test in one
  invocation. Updated the leading comment to describe the two
  complementary concerns under one job (no separate before_script
  to maintain).

**Why this is the right size:**

- Larger (deduplicate the Quickstart across all 4 files via a
  single source of truth — e.g., MkDocs `include` plugin or
  generated-from-ADOPTING.md) was rejected for this MR. The right
  refactor needs an evaluation of the include mechanism and how it
  interacts with the existing `docs/_gen/*.py` auto-generation
  pipeline. The static check buys time: future drift now fails CI.
- Smaller (just patch the 3 known files, no static check) was
  rejected — without the gate, the same paste-pattern recurs.

**Evidence trail:** the maintainer-first-eye check (per
[feedback_locked_prediction_methodology.md memory] discipline) caught
this. The user looked at the live Pages site after v0.59.0 shipped
and asked "would the Quickstart work?" The static check then
surfaced a 4th instance the manual scan missed.

---

## v0.59.0 (2026-04-30)

Quickstart end-to-end smoke test (option #2 from the outcome-anchoring
discussion). v0.58.0 made CI assert that the framework follows its
own static contracts (`validate-adoption`, `validate-agents`); this
release covers the *user journey* — the literal commands a new
adopter is told to type in the Quickstart.

The smoke test caught a real bug on its first run: Quickstart Step 2
told adopters to `cp -r agents-govern/rules-template/ .claude/rules/`
on a fresh project, but `cp` will not create the destination's parent
(`.claude/`). Every new Layout B adopter following the docs literally
hit a `cp: cannot create directory` error. Fix included in this MR
(adds `mkdir -p .claude` to the Quickstart). This is exactly the
class of bug that option #2 was designed to catch.

**Added:**

- `tests/test_quickstart_smoke.py` — 5 tests, one per Quickstart step.
  Builds the dist, simulates a Layout B adopter project, executes the
  literal `cp` and `uv run` commands, asserts the claimed outcomes.
- `.gitlab-ci.yml` — `quickstart-smoke` job (runs on every MR).
- `tests/test_ci_self_conformance.py` — extended with
  `TestQuickstartSmokeInCI` to enforce the new job's presence
  (drift-catcher pattern from v0.58.0).

**Fixed:**

- `ADOPTING.md` Quickstart Step 2 — now prepends `mkdir -p .claude`
  before the `cp -r rules-template/ .claude/rules/` command. The
  prior form failed on every fresh project that didn't already
  have `.claude/` from another tool.

**Why this is the right size:**

- Larger (also smoke-test the full ADOPTING.md walkthrough Steps 0–7)
  was rejected — the Quickstart is the canonical first-touch path; the
  full walkthrough has subjective steps (assessment, role assignment)
  that resist mechanical assertion. Add full-walkthrough coverage when
  ADOPTING.md changes break the Quickstart smoke and we need broader
  net.
- Smaller (just the bug fix, no smoke) was rejected — without CI
  coverage, the same class of bug recurs on the next Quickstart edit.
  The fix without the gate is the v0.58.0 mistake repeating.

**Evidence trail:** the gap was identified during the conversation
that produced v0.58.0. The smoke test caught the `mkdir -p` bug on
the first execution against a fresh extracted dist — direct evidence
that user-facing command coverage was needed.

---

## v0.58.0 (2026-04-30)

Per-MR self-conformance gate. ADOPTING.md tells adopters to run
`agov-validate-adoption` and `agov-validate-agents`, and
`self-conformance-rule.md` claims our CI runs them — but until now
the only enforcement was at tag time (via `pre-tag-check`). A doc
or agent-definition change that broke either validator could land
on `main` and only surface on the next release attempt. This MR
closes that loop: both validators now run on every MR.

The CI job presence is itself enforced by a test
(`tests/test_ci_self_conformance.py`) so the gate cannot be
silently removed — deleting the job, or changing the script so it
no longer invokes the validator, fails the test suite.

**Added:**

- `.gitlab-ci.yml` — `validate-adoption` job (runs on every MR)
- `.gitlab-ci.yml` — `validate-agents` job (runs on every MR)
- `tests/test_ci_self_conformance.py` — asserts both jobs exist,
  invoke their validator, and trigger on `merge_request_event`

**Changed:**

- `.claude/rules/always/self-conformance-rule.md` — cites the new
  jobs by name; aspirational claims about CI coverage are now
  factual claims the test suite verifies.

**Why this is the right size:**

- Larger (also expand `validate-adoption` to cover more ADOPTING.md
  outcome statements, or rewrite ADOPTING.md to outcome-anchor every
  substep) was rejected — the existing validator already covers all
  major-step outputs, and substep outcome-anchoring should be driven
  by smoke-test findings (next MR), not speculation.
- Smaller (just add the CI jobs) was rejected — without the
  drift-catcher test, the next person to refactor `.gitlab-ci.yml`
  could remove the gate without anyone noticing until tag time.

**Evidence trail:** the gap was identified by the user asking
whether outcome statements in ADOPTING.md should be CI-enforced.
The validator already existed; the only missing piece was making
`self-conformance-rule.md`'s claim mechanically true per-MR.

---

## v0.57.0 (2026-04-30)

Lightweight README pointer to the v0.56.0 [Beyond SDLC Provisional
hypothesis page](https://janipaijanen.gitlab.io/agent-governance/position/beyond-sdlc/).

The beyond-sdlc.md page is otherwise hidden behind the docs-site
Position section nav — invisible to GitLab/GitHub front-door
readers, who are the population most likely to be cross-domain
pilot candidates. This MR adds a 6-line bold-led paragraph at the
foot of the README's "what this is, what this is not" section,
explicitly anchored against misread by stating "stated scope above
is unchanged".

**Added:**

- `README.md` — one paragraph immediately after the existing
  "If you want governance for ML research..." line and before the
  Version line, calling for cross-domain pilots on Provisional
  basis.

**Why this is the right size:**

- Larger touch (Size S — separate "Cross-domain applicability"
  section) was rejected as restructuring the front door
  unnecessarily.
- Smaller touch (silence + sidebar-only) was rejected as
  insufficient for pilot-recruiting; the page remains hidden.
- Size XXS preserves the hardened non-claims block as a clean
  unit, then adds the Provisional pointer with explicit
  scope-unchanged anchor.

**Self-conformance:** the framework's symmetric evidence
discipline (2+ adopter learnings before Recommended) is named
explicitly in the new paragraph. The README's stated scope and
non-claims block are unchanged. Mirrors v0.56.0's discipline.

**Out of scope:** matching update to `docs/index.md` homepage on
the docs site (which has parallel "what this is, what this is
not" content) — the user requested README-only. If symmetric
update is wanted, that's a one-MR follow-up.

## v0.56.0 (2026-04-30)

Adds [Position → Beyond SDLC: applicability hypotheses](https://janipaijanen.gitlab.io/agent-governance/position/beyond-sdlc/)
— a Provisional sidebar page recording that the framework's
mechanisms layer (gates, evidence discipline, learning records,
authority boundaries, symmetric communication topology) appears to
generalize beyond SDLC, per several AI-service endorsements
gathered under
[REVIEWER-BRIEF.md](https://gitlab.com/janipaijanen/agent-governance/-/blob/main/external-ai-engagements/REVIEWER-BRIEF.md)
protocol.

**Soft expansion, not hard.** The framework's stated scope —
homepage, README, REVIEWER-BRIEF non-claims block — is
**unchanged**: SDLC of multi-agent AI software development. That
is where the corroborated evidence is. The new page is an
explicit Provisional sidebar that:

- Distinguishes the **mechanisms layer** (solution-space-agnostic
  hypothesis) from the **role taxonomy + pipeline layer**
  (SDLC-specific by construction).
- Lists 5 hypothesised cross-domain applicabilities (AI service
  development, research workflows, ops runbooks, content
  production, security analysis) with the role-taxonomy gap
  each would face.
- States the symmetric-evidence-discipline bar for promotion from
  Provisional to Recommended: 2+ independent cross-domain pilot
  adopter learning records.
- Calls explicitly for cross-domain pilots, with concrete steps
  and a `cross-domain-pilot` issue label.
- Honours self-conformance — the framework's own evidence
  discipline binds the maintainer; expanding stated scope on
  pre-pilot endorsements would be exactly the scope drift that
  issue #44 is parked at Provisional to detect.

**Added:**

- `docs/position/beyond-sdlc.md` — the Provisional hypothesis
  page. HTML forensic comment dates the introduction and
  invites corroboration.
- `docs/position/.pages` — adds beyond-sdlc.md to nav.
- `docs/position/index.md` — links to beyond-sdlc.md from the
  "Other position comparisons" list with explicit Provisional
  flag.
- `docs/position/overview.md` — adds a row to the Quick Reference
  table for "Multi-agent processes outside SDLC", flagged
  Provisional.

**Why now (decision context):** the user reported that several AI
services, asked under REVIEWER-BRIEF protocol whether agents-govern
could have helped in their own service development, answered
"yes — in certain topics", citing the mechanisms layer as
load-bearing. This page captures that hypothesis honestly without
re-positioning the front door.

**What this is not:** a scope expansion. Stated scope unchanged
until 2+ cross-domain pilot adopters file learning records.

## v0.55.0 (2026-04-29)

Surfaces the existing `/upgrade-agents-govern` skill on the docs
site. Pure-content, single admonition. Provisional — single-source
evidence (maintainer first-eye observation that returning adopters
land on `/adopt/` and have to scroll Section 6.4 of the long
full-guide page or grep for "upgrade" to find the workflow).

**Added:**

- `docs/adopt/index.md` — second `!!! tip` admonition titled
  "Already adopted? Upgrading to a newer release". Three-line
  description of the workflow (`tmp/` → `/upgrade-agents-govern
  tmp/agents-govern-X.Y.Z.tar.gz`) with cross-link to
  full-guide.md §6.4 "Upgrade cadence" for the pin-vs-upgrade
  guidance.

**Why now:**

- The upgrade skill (`skills/upgrade/SKILL.md`) and the gated
  workflow have existed since pre-v0.40.0 but the entry point
  for *returning* adopters is currently inside the linear full
  guide. Returning adopters typically land on `/adopt/` first
  and need a one-click pointer rather than re-reading the full
  guide.
- This parallels v0.53.0's Evaluator-path callouts (which
  served the *evaluator* persona). Returning adopters are a
  distinct persona.

**Provisional comment** in the admonition flags it as
single-source evidence; it should either be corroborated by
adopter or external-AI evidence or removed in a future cleanup
pass. The next external-AI engagement could include "find the
upgrade workflow" as a discovery prompt to gather corroboration.

**No code, no theme change.** Tier B navigation chrome (footer
scope, breadcrumbs, TOC label/scope) remains parked.

## v0.54.0 (2026-04-28)

Captures the v0.52.0–v0.53.0 navigation-review thread as a single
learning entry under iron-gate. Pure-evidence release: no code, no
docs, no theme change. Documents both substantive findings and
methodology learnings for future-session retrieval.

**Added — `learnings/iron-gate.yaml` entry (2026-04-28, validation
category, framework scope):**

5-source corroboration that navigation chrome is over-applied
uniformly across linear and reference sections. Records:

- The 5-source convergence on the highest-priority finding (footer
  prev/next collides with task order; reference catalog reads as
  Gemini's "dictionary trap")
- The 2-source independent convergence (Cursor Proposal A +
  Windsurf Proposal #5) that drove the v0.53.0 Evaluator-path
  implementation
- Methodology finding: 3/5 sources fabricated feature-presence
  claims — 60% verification failure rate. Future navigation
  reviews must require a browse-confirmation artifact paste.
- Methodology finding (positive): locked-prediction discipline
  caught maintainer's misdiagnosis before any external input
  landed. Discipline retained.

**Recommended (not Provisional) on the strength of 5-source
corroboration:**

- Evaluator-path callouts on guided pages (implemented in v0.53.0)
- Browse-confirmation gate on future external-AI navigation
  reviews — update REVIEWER-BRIEF.md before next nav engagement

**Three further corroborated fixes deferred to v0.55.0+ pending
theme-level implementation:** disable footer in `reference/`,
strengthen breadcrumbs to include leaf page name, relabel right
TOC as "On this page", disable right TOC on short pages.

## v0.53.0 (2026-04-28)

Implements the strongest converged finding from the
[v0.52.0 navigation engagement](https://gitlab.com/janipaijanen/agent-governance/-/blob/main/external-ai-engagements/2026-04-28-navigation-review.md):
the over-applied footer prev/next causes task-order vs site-order
collisions. Cursor (Proposal A) and Windsurf (Proposal #5)
independently recommended explicit task-order content boxes on
guided pages — implemented here as `!!! tip "Evaluator path"`
admonitions, the lowest-cost theme-agnostic fix.

**Added — Evaluator-path callouts on the 6 pages a newcomer
traverses while deciding adoption:**

- `docs/index.md` — homepage, after the lead frame and before
  "What this is, what this is not"
- `docs/start/index.md` — section landing
- `docs/start/should-you-adopt.md` — step 1 of 4
- `docs/start/why-this-exists.md` — step 2 of 4
- `docs/start/walkthrough.md` — step 3 of 4
- `docs/adopt/index.md` — step 4 of 4

Each box explicitly states its position in the sequence and warns:
*"Footer prev/next links follow site order, not this sequence."*
This addresses Codex's "fights the section's purpose", Gemini's
"dictionary trap", Cursor's "Walkthrough Next → Concept" and
Windsurf's "missing section completion signals" simultaneously.

**Fixed — homepage Reading-paths Newcomer row.** Previously sent
newcomers to "Why this exists" first; now sends them to
"Should I adopt?" first, matching the Start sidebar order
(should-you-adopt → why-this-exists → walkthrough). Cursor uniquely
caught this within-section misalignment.

**Issue-assessment** recorded in
[`reviews/2026-04-28_nav-review-tier-a.md`](https://gitlab.com/janipaijanen/agent-governance/-/blob/main/reviews/2026-04-28_nav-review-tier-a.md)
per `issue-assessment-rule.md`.

**Out of scope (deferred to v0.54.0+):** theme-level changes —
disable footer in `reference/`, strengthen breadcrumbs to include
leaf page name, relabel right TOC as "On this page", disable right
TOC on short pages.

## v0.52.0 (2026-04-28)

Captures the 5-source navigation-experience external-AI engagement
that corroborated the maintainer's first-eye observation about
navigation friction. Pure-evidence release: no code, no docs-content
change, no theme-config change. Establishes the corroboration record
before any structural change based on it.

**Added:**

- `external-ai-engagements/2026-04-28-navigation-review.md` — batch
  engagement record. 5 sources queried (Lumo, OpenAI Codex, Google
  Gemini, Microsoft Copilot, Cursor); 5 substantive responses
  received and 1 non-response logged (Copilot, free tier cannot
  browse). Plus Windsurf added in flight.
- `external-ai-engagements/2026-04-28-windsurf-navigation.md` —
  Windsurf (Cascade) full transcript filed separately.

**Methodology findings recorded in the engagement:**

- **Locked-prediction discipline** (introduced in this engagement)
  caught the maintainer's misdiagnosis before any external response
  landed. Maintainer predicted "enable `navigation.footer`"; verification
  of [mkdocs.yml](https://gitlab.com/janipaijanen/agent-governance/-/blob/main/mkdocs.yml)
  showed it was already enabled. Discipline retained for future
  engagements.
- **Browse-verification gap:** 3/5 sources fabricated
  feature-presence claims (Lumo: "no prev/next"; Gemini: "without
  breadcrumbs"; Windsurf: "some pages have, others don't" + #1
  recommendation to enable a feature already enabled). Future
  navigation-review prompts should require a literal artifact paste
  ("rendered breadcrumb on `/reference/agents/build-engineer/`") as
  a browse-confirmation gate.

**Strong 5-source convergence (informs follow-up MRs, not this one):**

- Footer prev/next over-applied uniformly — collides with task order
  in `start/` and reads as "dictionary trap" in `reference/`
- Section-type ambiguity (linear vs catalog) needs a visible signal
- Right TOC stays; relabel as "On this page" (Cursor + Gemini)
- Breadcrumbs valuable; strengthen to include leaf page name

**Strongest converged fix (deferred to v0.53.0):** Cursor Proposal A
+ Windsurf Proposal #5 — explicit "Evaluator path" content boxes on
guided pages. Pure-content fix, lowest implementation cost.

**Fixed:**

- Windsurf engagement filename: `202604-28-windsurf-navigation.md` →
  `2026-04-28-windsurf-navigation.md` (missing dash between `2026`
  and `04`).

## v0.51.0 (2026-04-28)

Promotes `vs agent orchestrators` to the Position section's landing
page. The orchestrator-vs-governance distinction is the framework's
single most-frequently-mistaken comparison — three of the first five
external-AI reviewers got it wrong on first read — and the Concrete
Relationship diagram on that page is the load-bearing visual answer.
Lifting it to default means a visitor's first impression of the
Position section is "agents-govern is governance, runtime is
orchestrator's job, here's the diagram showing how they relate"
rather than the prior abstract Quick Reference table.

### What changed

- **`docs/position/index.md`** — now contains the former
  `vs-orchestrators.md` content (governance-vs-runtime framing,
  Concrete Relationship Mermaid diagram, per-orchestrator notes,
  Quick decision table). H1 changed to `Position — agents-govern vs
  agent orchestrators`. Added a top callout naming the diagram as
  the load-bearing element + linking to the overview for other
  comparisons.
- **`docs/position/overview.md`** — newly-named file containing the
  former `vs-orchestrators.md` content (Quick Reference table,
  Process-vs-Enforcement framing, "What this section is not"). H1
  changed to `Position — overview (other comparisons)`. The old
  table-row link `[vs orchestrators](vs-orchestrators.md)` updated
  to point at the Position landing.
- **`docs/position/.pages`** — reordered to put `index.md` first
  (the orchestrator landing), then `overview.md`, then the other
  per-category vs pages.
- **`docs/start/should-you-adopt.md`** — updated stale anchor link
  from `../position/index.md#process-vs-enforcement` to
  `../position/overview.md#process-vs-enforcement` (the section
  moved with the rename).

### URL impact

- `/position/` — was the Quick Reference table; now is the
  orchestrator landing.
- `/position/vs-orchestrators/` — **404** after this release. The
  content moved to `/position/`. External bookmarks to the old URL
  (the page existed since v0.37.0, ~36 hours) will break. No
  redirect plugin is configured; could be added in a follow-up.
- `/position/overview/` — **new URL**. Contains the Quick Reference
  table and Process-vs-Enforcement section.

### Why this is a release tag

Per project convention. Pure docs-restructuring (no rule, agent, or
content changes); two files renamed via `git mv` (history preserved),
one stale link updated, `.pages` reordered. No `dist/` impact.

### Self-test (per gate-self-test-rule, even though this isn't a gate)

Negative-style verification before commit:

```
$ NO_MKDOCS_2_WARNING=true DISABLE_MKDOCS_2_WARNING=true \
    uv run --group docs mkdocs build --strict --site-dir /tmp/build
Documentation built in 2.16 seconds   # no strict-mode warnings

$ grep -oE '<h1[^>]*>[^<]+' /tmp/build/position/index.html | head -1
<h1 id="position-agents-govern-vs-agent-orchestrators">…

$ grep -c 'Concrete relationship' /tmp/build/position/index.html
3   # heading + nav references — diagram present on landing

$ test -f /tmp/build/position/overview/index.html && echo ok
ok
```

## v0.50.0 (2026-04-28)

Sentinel review + meta-level assessment of the v0.38.0 → v0.49.0
release run. AI-drafted, presented for human Governor-Reviewer
ratification at merge time.

### What's new

- **[`reviews/2026-04-28_sentinel-review-and-meta-assessment.md`](reviews/2026-04-28_sentinel-review-and-meta-assessment.md)**
  — single-document review covering 18 release tags shipped tonight,
  identifying 8 patterns (5 closed, 3 open or noted), and assessing
  the framework's trajectory across 12 health axes. Recommends 2
  next-session actions, neither marathon-class.

### Patterns identified

Five closed:

- Tag-drift thread (4-source corroborated, gate landed)
- CI-coverage gap (1-source acted-upon)
- Gate-shipped-broken (3-source corroborated, rule landed)
- External-AI engagement corpus (4 of 7 findings shipped)
- Self-conformance check (process-vs-enforcement framing landed
  before the framework lived its own lesson)

Three open or noted:

- **agov-bump CHANGELOG-collision** — 8+ recurrences tonight, NOT
  yet addressed. The single most-recurring failure mode of the
  evening. Recommended fix scoped in the review.
- Phase-transition signals — capability-review observable, single-
  source; usability-advisor parked.
- Marathon cadence — 18 tags in 9 hours is not normal; note for
  next session to absorb before adding more.

### Two recommendations for next session

1. **Address agov-bump CHANGELOG-collision** (file an issue + small
   MR; ~1 hour scope; v0.51.0 or earlier patch).
2. **Run this review through an external AI sentinel** (independent
   corroboration via the brief-aware engagement flow established
   v0.38.0–v0.40.0; ~30 minutes; v0.51.0 or independent).

### Why this is a release tag

Per project convention. Sentinel review documents are
evidence-base content that ships with the framework's standard
release cadence. No `dist/` impact (the review is repo-only;
adopters consuming the framework see the rules and agents, not the
sentinel reviews).

### What this release does NOT do

- Does not change rules, agents, or any structural content. Pure
  reflection document.
- Does not promote any single-source observation to corroborated.
- Does not retroactively audit pre-tonight decisions.

### Self-test

Per the gate-self-test rule (v0.46.0), even though this isn't a
gate: the review's claim is "covers v0.38.0 → v0.49.0".
`git tag --sort=creatordate | grep -E 'v0\.(38|39|40|41|42|43|44|45|46|47|48|49)' | wc -l` returns 18, matching the review's
named pattern coverage.

## v0.49.0 (2026-04-28)

Adds [`profiles/tiny-project.md`](profiles/tiny-project.md) — an
adopter profile for solo / small-team / coursework / internship /
hobby projects whose context doesn't justify the full ADOPTING.md
flow but who still want some governance discipline. Final part of
the docs-consumability follow-up sequence (v0.47/v0.48/v0.49).

### Why this lands

A 3rd-year bachelor of software development student (or any
small-project adopter) following the docs runs into the same
density problem: ADOPTING.md is long, the framework asks for
3–12 agents and 30+ rules, the formal Governor-Reviewer split is
the default. A tiny project doesn't need that. The
[Should I adopt?](docs/start/should-you-adopt.md) page (v0.41.0)
correctly filters those adopters AWAY for the right reasons —
their context doesn't match the failure surface — but for adopters
who *do* fit a smaller version of the failure surface ("I want
a written audit trail for my course project"), the framework
previously offered no documented stop-short option.

This profile is the documented stop-short.

### What's new

`profiles/tiny-project.md` (85 lines) following the same skeleton
as the existing `profiles/research-lab.md`:

- **Status: UNTESTED** — single-source from the 2026-04-28
  consumability review; awaits independent adopter corroboration.
- **Who this is for / Who this is NOT for** — explicit fit/no-fit
  beyond the Should-I-Adopt filter.
- **The minimum viable adoption table** — one agent
  (`coder-tester`), three rules (`tdd-rule`,
  `branch-and-worktree-rule`, `external-anonymization-rule`), one
  learning record, self-review at two informal gates.
- **What this profile does NOT do** — explicitly preserves the
  framework's load-bearing rules (anonymisation universal,
  Should-I-Adopt filter still applies).
- **How to validate** — three signals worth recording (worked /
  outgrew / bounced); all three are valid evidence.

### Why this profile is unusual

Most profiles *extend* the core framework (research-lab adds
ML-governance layers; a future fintech profile would add
compliance-audit gates). The tiny-project profile *reduces* the
core: it documents a deliberate undershoot. The framework's
"grow by pain" discipline already implies this — the explicit
profile makes the implicit path findable.

### Sequence complete

This is part 3 of 3 docs-consumability follow-ups:

- **v0.47.0** — homepage 30-second summary
- **v0.48.0** — abbr-extension glossary tooltips site-wide
- **v0.49.0** (this release) — tiny-project adopter profile

After this lands and Pages goes live, the planned next step is a
sentinel review + meta-level assessment covering this evening's
full release run (v0.38.0 through v0.49.0).

### Why this is a release tag

Per project convention. New evidence-base content under
`profiles/`. No `dist/` archive impact (profiles are
documentation-only).

## v0.48.0 (2026-04-28)

Adds abbr-extension glossary tooltips site-wide. First-encounter
readers hovering over heavy framework vocabulary (jurisdiction,
Sentinel, primary verb, Layout A/B, learning record, MR/PR/CI/CD,
ADR, codename, Phase 1–5, Provisional/Recommended, grow by pain,
uv) get inline definitions without leaving the page.

Part 2 of 3 docs-consumability follow-ups (v0.47/v0.48/v0.49)
scoped from a sentinel-style review of how the docs land for a
3rd-year bachelor of software development student profile.

### What's new

- **`includes/abbreviations.md`** — 23 definitions covering general
  software-development acronyms, framework-specific vocabulary, and
  growth-phase / evidence-status terminology. Adopters extend with
  project-specific terms by adding lines in the same `*[TERM]:
  Definition` format.
- **`mkdocs.yml`** — `pymdownx.snippets` configured with
  `auto_append: [abbreviations.md]` so every page gets the
  abbreviations file appended at build time. Combined with the
  already-enabled `abbr` extension, this produces `<abbr title="…">`
  elements wherever the term appears in the page.

### Negative-test (per gate-self-test-rule, even though this isn't a gate)

Verified the change actually fires on its target failure mode (terms
appearing without tooltips → with tooltips after the change):

```
$ grep -roE '<abbr[^>]*>' build/ --include='*.html' | wc -l
744
$ grep -E '<abbr title="An agent.s defined scope[^"]*">jurisdiction</abbr>' build/evidence/projects/silver-index/index.html
[match]
```

744 `<abbr>` tags across all rendered pages; specific spot-check on
"jurisdiction" in the silver-index project page confirms the
tooltip wraps the term with the expected definition.

### What this release does NOT do

- Does not add a separate "Glossary" page in the nav. The Concept
  section already has a glossary; the abbr-extension is a
  complement, not a replacement.
- Does not validate the abbreviations file itself. A future tool
  could check for orphaned terms (defined but unused) or missing
  terms (used without definition) — out of scope for the rule's
  introduction.

### Why this is a release tag

Per project convention. Pure docs-rendering addition; no `dist/`
impact (the `includes/` directory is docs-build-only).

## v0.47.0 (2026-04-28)

Adds a "30-second summary" section to the docs site homepage. First
of three docs-consumability releases scoped during a sentinel-style
review of how the docs site lands for a 3rd-year bachelor of software
development student profile.

### Why this lands

A new visitor's first-fold view was previously the abstract intro
paragraph + the "What this is, what this is not" disambiguation. That
sequence works for someone who already knows what AI-governance vs
ML-governance vs agent-runtime means, but lands abstractly for a
student or junior developer. The 30-second summary front-loads the
three questions a newcomer actually asks: *what does it do, what does
it cost, when do I use it*, with an explicit *when not to* pointing
at the existing Should I adopt? filter.

### What's new

`docs/index.md` gains a `## In 30 seconds` section before
`## What this is, what this is not`. Four short paragraphs
(What it does / What it costs / When to use it / When not to use it),
no jargon, with one inline link to the Should I adopt? page for the
full fit/no-fit filter.

### Why this is a release tag

Per project convention. Pure docs-content addition; no `dist/` impact.

### Sequence

This is part 1 of 3 docs-consumability follow-ups:

- **v0.47.0** (this release) — homepage 30-second summary
- **v0.48.0** (planned) — abbr-extension glossary tooltips for heavy
  vocabulary (jurisdiction, Sentinel, primary verbs, etc.) so first
  appearances in any page get hover-definitions.
- **v0.49.0** (planned) — `profiles/tiny-project.md`, an adopter
  profile for student / coursework / single-dev-multi-AI use,
  documenting the minimum viable adoption path.

### Self-test

Negative-style verification done before commit, learned from tonight's
"Edit fails silently" pattern: rebuilt the docs site after the edit,
grepped the rendered `index.html` for the new section heading, got 3
matches (header + 2 nav references). Source file also greps clean.

## v0.46.0 (2026-04-28)

New framework rule: [`rules-template/always/gate-self-test-rule.md`](rules-template/always/gate-self-test-rule.md)
— **MUST** status. When introducing a new gate (git hook, validator,
CI check, pre-commit script, MR-blocking rule), the same MR MUST
include a deliberate negative test demonstrating the gate fires on
a known-bad input.

### Why this lands

Three documented incidents in this framework's own history shipped
a gate broken because nobody negative-tested it:

- **v0.25.0** — release skill defined version-tag alignment as
  process discipline; humans forgot. (Recovery: 3 patches.)
- **v0.38.0** — `agov-pre-tag-check` existed for 21 days unverified
  on its target failure mode; raw `git tag` bypassed it.
  (Recovery: one immutable broken tag.)
- **v0.41.0** — pre-push hook shipped with `mode 100644`
  (non-executable). Git silently skipped it. (Recovery: 3 patches.)

All three: gate ships, nobody negative-tests it, gate doesn't gate,
failure mode the gate was supposed to prevent recurs while everyone
assumes it cannot. Three-source corroboration clears the framework's
MUST threshold.

### What the rule requires

For any MR introducing a new gate:

1. **A negative test** — explicit demonstration that the gate fires
   on a known-bad input, with visible failure (error, exit code,
   blocking output) and reproducibility.
2. **Documentation in MR description** — bad input used, expected
   output, actual output.
3. **Verification artifact** — pipeline log, command transcript, or
   screenshot. "I tested locally" is insufficient.

The rule explicitly forbids passing-only tests and deferral of the
negative test to a follow-up MR.

### Closes the marathon-prevention sequence

Three follow-ups identified during the v0.41.x patch-marathon retro
have all landed:

- v0.44.0 — branch-and-worktree augmentations (#47, iron-gate)
- v0.45.0 — pages-build on MR pipelines (CI-coverage gap)
- v0.46.0 — gate-self-test rule (this release)

Together with v0.41.0's pre-push hook and v0.41.3's hook-integration
fix, the framework now has both *the gate that prevents tag-drift*
and *the discipline that ensures new gates ship correctly*.
Marathon-prevention is now structural, not maintainer-attentive.

### What this release does NOT do

- Does not implement automated enforcement (CI scan for the
  `## Negative test` heading in MR descriptions). Tracked as a
  potential follow-up if manual application proves insufficient.
- Does not retroactively re-verify prior gates. v0.41.0+ hook is
  already self-tested in transcripts. v0.45.0+ pages-build was
  meta-tested via its own MR pipeline before merge.

### Why this is a release tag

Per project convention. The rule ships in `dist/` via standard
`rules-template/` inheritance.

## v0.45.0 (2026-04-28)

CI policy: docs strict-mode build now gates every MR pipeline, not
just main pushes. Splits the existing `pages` job into:

- **`pages-build`** — runs `mkdocs build --strict` on every MR
  pipeline and on main. Catches docs/structural regressions
  pre-merge.
- **`pages`** — deploy-only, runs on main. Reuses
  `pages-build`'s artifact via `needs:`. Actual GitLab Pages
  deploy fires only here.

### Why this lands

Prior to this release, the `pages` job ran only on `$CI_COMMIT_BRANCH
== $CI_DEFAULT_BRANCH`. A strict-mode mkdocs failure could land
silently on every MR pipeline and only explode on the post-merge
main pipeline. The v0.41.4 incident burned this pattern in: a
regression that landed in v0.39.0 broke `pages` on every main
pipeline for **5 hours and 5 merges** before anyone noticed,
because no MR pipeline ever exercised the docs build.

After v0.45.0, the strict build runs on every MR. The same
regression class fails the MR's pipeline (visibly red), the
contributor sees it before merge, and the fix lands on the
feature branch instead of via a follow-up patch release.

### Cost

One extra ~30s CI build per MR pipeline. Negligible against the
cost of a v0.41.4-class follow-up patch (which was *itself* a
release tag, with all the discipline overhead that entails).

### What this release does NOT do

- Does not change deploy behavior. Pages still serves only the
  main-branch build at the canonical URL. MR builds produce an
  artifact (used by the deploy job and as an inspection artifact
  on the MR), but never serve at the public URL.
- Does not implement preview environments or per-MR deploy URLs.
  That would require GitLab Pages preview features which are out
  of scope.
- Does not retroactively re-test prior failing main pipelines. The
  v0.39.0–v0.41.3 main pipelines stay broken in CI history; the
  v0.41.4 fix is what makes the post-v0.41.4 builds clean. This
  MR just makes the next regression visible earlier.

### Why this is a release tag

Per project convention. CI policy change with cross-cutting effect
on adopter feedback loop. No `dist/` archive impact.

## v0.44.0 (2026-04-28)

Five augmentations to [`rules-template/workflow/branch-and-worktree-rule.md`](rules-template/workflow/branch-and-worktree-rule.md),
all adopter-evidence-driven by iron-gate during a 2026-04-28
session. Origin: [issue #47](https://gitlab.com/janipaijanen/agent-governance/-/issues/47),
approved on its merits — concrete evidence-supported gaps.

### What's new

1. **Sizing test (SHOULD)** — "one yes/no question". A branch
   passes the sizing test if a reviewer can answer one yes/no
   question from the diff. Reviewer judgement is the final
   arbiter; the test is guidance, not a hard rule.
2. **Multi-commit clarification (SHOULD)** — multiple commits per
   branch are fine if they together answer the same yes/no
   question. A bug fix and a refactor sharing one branch fail
   the test even when they share a file.
3. **Extended prefix list (MAY)** — adds `docs/`, `chore/`,
   `plan/`, `proposal/`, `adr/`, `infosec/`, and `release/X.Y.Z`
   to the established `feat/` `fix/` `refactor/` `migration/`
   `security/` set. Adopters may extend further per their tooling.
4. **AI-assisted branch state verification (MUST for adopters
   using AI coding assistants alongside IDE work; MAY for
   human-only workflows)** — IDE/agent branch collisions are real
   and cheap to mitigate. Discipline: always
   `git branch --show-current` before commit, push, force-push, or
   branch-rewrite operation; investigate surprises rather than
   resetting them away; use `--force-with-lease` not `--force`;
   if a wayward commit reaches origin, cherry-pick to the intended
   branch and roll back the wrong one with `--force-with-lease`.
5. **Cross-branch dependency surfacing (SHOULD)** — surface MR
   B-depends-on-MR-A in the description, never in the branch.
   Don't rebase B onto A's tip; don't wait for A to merge before
   opening B (draft/WIP is the right escape valve).

### Why a single MR for five augmentations

All five extend the same rule and review well as one thread. The
proposal's own framing: "they all extend the same rule and review
well as one thread." Per-augmentation strength calibration
(MAY/SHOULD/MUST) is documented inside the rule so adopters can
opt out per the existing per-rule customise pattern.

### What this release does NOT do

- Does not split the branch-and-worktree rule into a separate
  AI-assisted-development rule. The proposal explicitly bundles
  the AI-specifics into the existing rule.
- Does not address the v0.41.x patch-release marathon's actual
  load-bearing causes (CI-coverage gap; gates shipping broken).
  Those are scoped to follow-up MRs:
  - **Pages CI on MR pipelines** (planned v0.45.0) — gates
    docs/structural regressions before merge.
  - **Gate self-test rule** (planned v0.46.0) — discipline
    requiring a deliberate negative test when introducing a new
    gate, captured from the v0.41.0 hook-shipped-broken pattern.

Closes #47.

### Why this is a release tag

Per project convention. The augmented rule template ships in
`dist/` via standard `rules-template/` inheritance; adopters pick
it up on next framework upgrade.

## v0.43.0 (2026-04-28)

New framework rule: **codename-only at the codebase boundary** for
externally-destined content. Origin: [issue #45](https://gitlab.com/janipaijanen/agent-governance/-/issues/45),
filed by the iron-gate adopter following a 2026-04-28 near-miss
where AI-assisted drafting almost filed three issues at this
upstream tracker that would have leaked the adopter's identity.
Caught by the human user before publication; net leak = zero.

### What's new

- **[`rules-template/always/external-anonymization-rule.md`](rules-template/always/external-anonymization-rule.md)** —
  the rule itself. MUST status (with documented opt-out via
  CUSTOMIZE block for adopters with already-public projects).
  Includes a canonical boundary table covering internal vs external
  destinations, AI-assisted-development specifics, and a one-time
  retroactive audit on adoption.
- **[`LEARNINGS.md`](LEARNINGS.md)** Submission Privacy Policy
  section gains a cross-reference: the existing policy covers
  *learning record submissions* specifically; the new rule covers
  *all other externally-destined content* (issues, MRs, blog posts,
  external LLM API calls).
- **[`ADOPTING.md`](ADOPTING.md)** Step 0.1 mentions the rule and
  links to it; adopters customize the codename plus forbidden-term
  list at adoption time.

### Modifications applied to the iron-gate proposal

The proposal as filed was reviewed against [the issue-assessment-rule](.claude/rules/workflow/issue-assessment-rule.md);
four refinements landed in the final rule:

1. **Boundary table split for AI assistants** — one row for
   "local working state (no provider call)" (Internal), a separate
   row for "prompts/responses sent to a provider API" (External).
   Anthropic, OpenAI, GitHub Copilot etc. log/retain prompts on
   the provider side depending on tier; calling it "local" implies
   safety the boundary doesn't actually offer.
2. **"Refuse to draft" softened to "stop and ask the human"** —
   refusal dead-ends a session; asking creates a learnable
   boundary with the same protective effect.
3. **New "Retroactive audit on adoption" section** — addresses
   pre-rule externally-published content with an explicit
   one-time sweep, recorded in the adopter's learning record.
4. **Tooling clause strengthened** — from "recommended,
   adopter-side" to "SHOULD be implemented as framework tooling",
   with a follow-up issue ([#46](https://gitlab.com/janipaijanen/agent-governance/-/issues/46))
   tracking the `agov-validate-infosec` extension.

### Why MUST on single-source evidence

Anonymization leaks are *irreversible* — once a name is published
on a third-party tracker, blog, or LLM provider's training corpus,
you cannot unpublish it. The framework's symmetric evidence
discipline normally requires 2+ adopter corroboration before MUST,
but irreversibility is the established escape hatch (same logic
behind pre-incident security rules). The rule documents this
explicitly so future Sentinel reviews understand the calibration.

### What this release does NOT do

- Does not implement the forbidden-term scanner — that's [#46](https://gitlab.com/janipaijanen/agent-governance/-/issues/46).
- Does not retroactively audit *this* repo's externally-published
  artifacts. The rule fires going forward; the maintainer holds
  the natural-vocabulary list privately and runs the audit
  separately.

### Why this is a release tag

Per project convention. The new rule ships in `dist/` via the
standard `rules-template/` inheritance; adopters pick it up on
their next framework upgrade.

## v0.42.0 (2026-04-27)

Capability review of the current 12 agents + colorblind-safe
color-coding of the concept page's Mermaid diagram lines, mapped by
originating agent role.

### What's new

- **[`reviews/2026-04-27_capability-review.md`](reviews/2026-04-27_capability-review.md)** —
  sentinel-style inventory of the 12 current agents by tier, phase,
  primary verb, and primary `owns` field. Documents two adjacent gaps
  (no `capability-review` agent; no `usability-advisor` agent) as
  Provisional capability-developer observations, neither actioned in
  this MR pending corroboration. Justifies the color palette below.

- **`docs/concept/index.md`** Mermaid diagram now renders the dotted
  role-to-pipeline edges in agent-specific colors:

  | Agent | Color | Hex |
  | --- | --- | --- |
  | Planner-Architect | Blue | `#0072B2` |
  | Coder-Tester | Orange | `#E69F00` |
  | Governor-Reviewer | Vermillion | `#D55E00` |
  | Fact-Checker | Sky Blue | `#56B4E9` |
  | Contribution-Auditor | Reddish Purple | `#CC79A7` |
  | InfoSec Sentinel | Bluish Green | `#009E73` |

  Wong (2011) colorblind-safe palette — distinguishable across
  deuteranopia, protanopia, and tritanopia. Color is an enhancement;
  the edge labels (`owns`, `writes`, `consults at Gate 1`, `audits`,
  `screens`) carry the meaning for screen readers and users beyond
  the Wong-safe scope.

### Why "capability review" rather than spawning a new agent

The framework currently has no `capability-review` agent.
`capability-developer` (Phase 4, tier `meta`) covers the surface as
part of "Capability gap analysis reports". Per the framework's
"split when pain appears" discipline, separating review from
development needs a phase-transition signal that has not arrived.
This review accordingly stands in.

A future split signal — e.g., review work taking >50% of
capability-developer's bandwidth on a project — would justify a
Phase-5 separation. Single-source observation; not actioned.

### Why this is a release tag

Per project convention. Adds new evidence-base content
(`reviews/...`) and a substantive site rendering change (colored
diagram). No `dist/` archive impact (the review is repo-only; the
diagram lives in the docs site).

## v0.41.6 (2026-04-27)

Fixes the duplicate version badges introduced in v0.41.5. The new
override produced one static UL with `v0.41.5`, but mkdocs-material's
bundled JS *also* runs at page-load: it locates every
`[data-md-component="source"]` element, fetches GitLab API repo
info, caches it in sessionStorage, and injects its own
`<ul class="md-source__facts">` with version + stars + forks. Result:
two version badges visible in the upper-right — mine showing
`v0.41.5` and a JS-cached `v0.37.0 ★ 0 ⑂ 0` lingering from earlier
deploys.

### Fix

`overrides/partials/source.html` no longer sets
`data-md-component="source"` on the anchor. mkdocs-material's
runtime JS doesn't find a source element to enhance, so it skips its
own injection and leaves the static override's UL as the sole
version badge.

### Trade-off

Loses the JS-fetched repository facts (star count, fork count). For
this project both are 0 anyway. If they ever matter, two recovery
paths:

- Add them statically: extend the hook to fetch via GitLab API at
  build time and inject `extra.repo_stars` / `extra.repo_forks` for
  the override to render. Same single-render strategy.
- Re-enable runtime fetch: restore `data-md-component="source"` and
  remove the static UL (let JS own the rendering completely). But
  then version display goes back to the JS's GitLab-API-fetched
  latest release, which has worked fine post-v0.41.4 but caches in
  sessionStorage in ways that confused this evening's run.

### Verification

```
$ grep -c 'md-source__facts' /tmp/build/index.html
2   # one in header, one in mobile sidebar nav
$ grep -c 'data-md-component="source"' /tmp/build/index.html
0   # no JS-attachment point
```

After deploy, hard-refresh the docs site — only one version badge
should appear in the upper right.

### Why this is a release tag

Per project convention. Affects only docs site rendering; no `dist/`
archive impact.

## v0.41.5 (2026-04-27)

Adds a version badge to the docs site header. The previous v0.37.0
build had a `v0.37.0` badge in the upper-right repository info area
(visible in maintainer screenshots), but stock mkdocs-material's
`source.html` partial renders only the repo name — the facts widget
(version, stars, forks) is an Insiders-only feature. So the badge in
the prior screenshots came from somewhere transient (Insiders trial,
cached HTML, or similar) and never reproduced in subsequent builds.

This release wires up an equivalent badge using only the open-source
mkdocs-material plus a small hook + theme override. VERSION is the
single source of truth; agov-bump's existing update of VERSION is
enough — no new field to keep in sync.

### What's new

- **`docs/_hooks/inject_version.py`** — mkdocs hook that runs at
  build time, reads the repo's `VERSION` file, and injects it as
  `config.extra.framework_version`.
- **`overrides/partials/source.html`** — theme override of mkdocs-
  material's `source.html` partial. Preserves the default repo-name
  rendering and appends a `<ul class="md-source__facts">` with the
  framework version when `config.extra.framework_version` is set.
  Reuses mkdocs-material's existing `md-source__facts` /
  `md-source__fact` CSS classes so the styling is consistent with
  what an Insiders facts widget would produce.
- **`mkdocs.yml`** — adds `theme.custom_dir: overrides` and
  `hooks: [docs/_hooks/inject_version.py]`.

### How it works

1. `agov-bump` updates `VERSION` on every release (existing behavior).
2. On the next mkdocs build, the hook reads `VERSION` and populates
   `config.extra.framework_version`.
3. The theme override renders the badge using that value.

No new place for the version to drift; nothing to update by hand.

### Verification

```
$ uv run --group docs mkdocs build --strict --site-dir /tmp/test
$ grep -oE 'v0\.[0-9]+\.[0-9]+' /tmp/test/index.html
v0.41.5
```

### Why this is a release tag

Per project convention. Affects only the docs site rendering; no
`dist/` archive impact (the override and hook are docs-build-only).

## v0.41.4 (2026-04-27)

Fixes the GitLab Pages docs build, which had been failing in
strict-mode mkdocs since the v0.39.0 merge. The release **objects**
(GitLab tags + auto-created Releases) for v0.39.0 / v0.40.0 / v0.41.0
/ v0.41.2 / v0.41.3 all exist, but the **deployed docs site** never
updated — it has been stuck at v0.37.0 because every main pipeline
since then aborted before the `pages` deploy step.

### Two passthrough.py bugs fixed

1. **Site-relative rewrites assumed depth-1 destination.** The rule
   `framework.md → ../concept/framework.md` works when the source
   file is rendered into a one-level-deep destination (e.g.,
   `framework.md` → `concept/framework.md`, or `ADOPTING.md` →
   `adopt/full-guide.md`). For top-level destinations like
   `bibliography.md` (introduced in v0.39.0), the leading `../`
   takes the link out of the docs/ root into the GitLab repo
   filesystem — mkdocs strict-mode logs a warning and aborts.

   Fix: `_rewrite_links` now takes a `dest_depth` parameter; the
   site-relative rules use a `{prefix}` template filled with `../`
   repeated `dest_depth` times. Top-level destinations (depth 0) get
   `concept/framework.md`; one-deep destinations (depth 1) keep the
   `../concept/framework.md` form.

2. **Repo-only-file rewrites dropped fragments.** The rule
   `[X](README.md)` → `[X]({GITLAB}/README.md)` matched only links
   without `#anchor`. The version-semantics MR (v0.39.0) introduced
   `[README §Versioning](README.md#versioning)`; the rewrite missed
   it, so mkdocs treated `README.md#versioning` as a relative link
   from `adopt/full-guide.md` and looked for `adopt/README.md` —
   another strict-mode warning.

   Fix: every repo-only-file rewrite now optionally captures and
   preserves a `#fragment`. `README.md`, `LEARNINGS.md`,
   `LESSONS-LEARNED.md`, `GOVERNANCE.md`, `CHANGELOG.md`, and
   `DISCLAIMER.md` all get the same treatment.

### Verification

```
$ uv run --group docs mkdocs build --strict --site-dir /tmp/test
INFO    -  Cleaning site directory
INFO    -  Building documentation to directory: /tmp/test
INFO    -  Documentation built in 1.25 seconds
```

No warnings. Strict mode passes. The next main pipeline should
deploy Pages cleanly with content from v0.39.0–v0.41.4 finally
appearing on the live site.

### Why this is a release tag

Per project convention. This release ships only `docs/_gen/passthrough.py`
changes; no impact on `dist/` archive content.

## v0.41.3 (2026-04-27)

Fixes a hook integration ordering bug discovered during the v0.41.2
release: `agov-pre-tag-check` was designed for pre-tag-creation use
("about to make tag X — is everything aligned?") and its
"tag must not exist" check fired unconditionally. When invoked from
the new `.git-hooks/pre-push` hook the check ran AFTER `git tag`
created the local tag, so legitimate first-pushes of freshly-created
tags failed. v0.41.2 had to use `git push --no-verify` to ship.

### What changed

- **`agov-pre-tag-check`** gains an `--in-pre-push` flag that inverts
  the local-tag-existence check (the tag MUST exist locally; we are
  about to push it) and adds a remote-side check that fails only if
  the tag exists on `origin` pointing to a different commit (a real
  retag attempt). Default behavior (no flag) is unchanged.

- **`agov-validate-tag-push`** (the helper invoked by the pre-push
  hook) now passes `--in-pre-push` to `agov-pre-tag-check` for every
  tag it validates. The hook script `.git-hooks/pre-push` is
  unchanged — it just exec's `agov-validate-tag-push`.

- **`tests/test_validate_tag_push.py`** gains two regression tests
  ensuring the hook always passes `--in-pre-push` and reports
  failure correctly on non-zero exit. 20 tests now pass.

### Why this is a release

The hook landed in v0.41.0 with a defect: it gated against drift
correctly but couldn't gate the *legitimate* push of a freshly-
created tag (the v0.41.2 push had to bypass with `--no-verify`).
This patch closes the integration ordering gap so the hook can be
used end-to-end on every release going forward without `--no-verify`.

### Self-test discipline applied

Per the v0.41.1 incident learning ("self-test gate post-implementation"),
the new `--in-pre-push` mode was exercised against existing tags
before commit:

- `agov-pre-tag-check --in-pre-push v0.41.0` → "Tag exists locally
  (expected during pre-push); on origin matches local" ✓
- `agov-pre-tag-check --in-pre-push v0.99.99` → "Tag does not exist
  locally (expected during pre-push)" — fail (because there is no
  v0.99.99 tag for the hook to validate)

### Why this is a release tag

Per project convention. After this lands, the next tag push
(v0.41.3 itself, or any future tag) should pass through the hook
cleanly without `--no-verify` — the live end-to-end test of the
fix.

## v0.41.2 (2026-04-27)

> **Note: v0.41.1 is a duplicate-tag-of-v0.41.0** (it points to commit
> `c8a8ad2`, the v0.41.0 merge commit, not the actual v0.41.1 fix
> commit). Third incident in the version-tag-drift thread, same
> evening as v0.38.0. While the v0.41.1 patch was queued for
> auto-merge after CI, `git tag v0.41.1` ran from local main
> (still at the v0.41.0 commit, VERSION=0.41.0). The push invoked
> the v0.41.0 hook, which git skipped because that hook was committed
> with a non-executable bit (the very bug v0.41.1 was fixing). No
> gate fired. The actual executable-bit fix shipped to main as
> commit `9bc6ffa` but its v0.41.1 tag points elsewhere. Per
> framework convention tags are not deleted; v0.41.1 stays as a
> broken record. v0.41.2 is the working patch release.

This release is identical in content to what v0.41.1 *should have*
been: the executable-bit fix to `.git-hooks/pre-push`. No new
features. Adds a third entry to [`learnings/iron-gate.yaml`](learnings/iron-gate.yaml)
documenting the v0.41.1 incident and three additional release
disciplines (wait-for-merge, pull-before-tag, self-test gate
post-implementation).

### Why three patches not amend-and-retag

Each broken tag (v0.38.0, v0.41.1) is immutable per framework
convention. Recovery is always: skip the burned version, cut the
next number, document. Tag history therefore preserves the incident
shape — anyone looking at v0.41.1 can compare its commit to v0.41.0's
and see they are the same, which is the audit trail of the failure.

### Three-source corroboration on the gap

Version-tag drift now has three documented incidents (v0.25.2,
v0.38.0, v0.41.1). The pre-push hook (v0.41.0+, executable from
v0.41.2) closes the most common variants but the v0.41.1 case
exposes a meta-failure mode: a gate's own implementation can defeat
its activation. The three new disciplines in the iron-gate learning
are process-level; they are documented in `feedback_version_before_tag.md`
(maintainer's auto-memory) for future-self.

### Why this is a release tag

Per project convention. The hook is now executable in tree (mode
`100755`), so adopters cloning v0.41.2 and running `git config
core.hooksPath .git-hooks` get a functional gate.

## v0.41.1 (2026-04-27)

Fix: `.git-hooks/pre-push` was committed in v0.41.0 with mode `100644`
(non-executable). When git tried to invoke the hook on push it
emitted: *"The '.git-hooks/pre-push' hook was ignored because it's
not set as executable."* Sets the file mode to `100755` so the hook
runs as intended after activation (`git config core.hooksPath
.git-hooks`).

### Root cause

During the v0.41.0 prep, the executable bit was set in the git index
via `git update-index --chmod=+x` immediately after the initial
`git add .git-hooks/pre-push`. A subsequent `git add` for the wider
release commit re-staged the file from the working tree (which never
had the executable bit set on the maintainer's filesystem because
`chmod` was unavailable in the environment), silently overwriting
the earlier mode update. The final commit therefore had `100644`,
and the executable bit didn't survive.

### Why a patch release rather than amending v0.41.0

Tags are immutable. v0.41.0 ships with a non-executable hook script;
that's the historical record. v0.41.1 is the working release for any
adopter activating the hook. The lesson — that `git add` after
`git update-index --chmod=+x` will undo the mode change — is
captured here in the CHANGELOG for future-self reference; not yet
significant enough for a learning entry.

## v0.41.0 (2026-04-27)

Pre-push hook on tag refs — closes the v0.38.0 tag-drift gap by
gating pre-tag validation at the latest possible local point
(option 2 from [`reviews/2026-04-27_version-tag-drift-recurrence.md`](reviews/2026-04-27_version-tag-drift-recurrence.md),
the recommended candidate gate).

### What's new

- **`agov-validate-tag-push`** — new Python tool at
  `src/agents_govern/validate_tag_push.py` (registered as a
  pyproject script). Reads git pre-push stdin, extracts tag refs
  matching `vX.Y.Z`, and runs `agov-pre-tag-check` on each. Aborts
  the push (exit 1) on any failure. Skips non-tag refs and
  pre-release tags. Tested with 18 unit tests in
  `tests/test_validate_tag_push.py` (regex matching, stdin parsing,
  passing/failing scenarios).

- **`.git-hooks/pre-push`** — git pre-push hook (executable in the
  git index) that exec's `agov-validate-tag-push`. Activation is
  one-time per clone:

  ```bash
  git config core.hooksPath .git-hooks
  ```

- **`.git-hooks/README.md`** — documents the hook directory, the
  activation command, what the hook does and does not catch, and
  guidance for adopters who want the same gate in their projects.

### Why this lands as a release

Two-source corroboration on the underlying gap (v0.25.2 + v0.38.0,
both documented in `learnings/iron-gate.yaml`) clears the framework's
Recommended bar. The implementation follows the sentinel review's
preferred candidate (pre-push hook over wrapper command or CI-driven
tagging) for cost / coverage trade-off.

### What this release does NOT do

- Does not change the existing `agov-pre-tag-check` behavior — the
  hook calls it as-is.
- Does not delete the broken v0.38.0 tag (immutable per framework
  convention).
- Does not implement the wrapper command (`agov-tag`, option 1) or
  CI-driven tagging (option 3) from the sentinel review. Either can
  be layered on later if the pre-push gate proves insufficient.
- Does not validate tag pushes on `--no-verify`. That escape hatch
  exists by design; using it is an explicit acceptance of the risk.

### Why this is a release tag

Per project convention. The new tool ships in dist (Python tooling);
the hook script does not (`.git-hooks/` is not in dist, since
adopter projects activate hooks per their own discipline).

## v0.40.0 (2026-04-27)

Content bundle from the 2026-04-27 external-AI engagements: External
Review issue template, combined decision/MVP page, zero-overkill
investigation. Three substantive new artifacts plus one
[`LEARNINGS.md`](LEARNINGS.md) surface. Cumulative on v0.39.0
(housekeeping).

### What's new

- **External Review issue template** at
  `.gitlab/issue_templates/External Review.md`. Native submission
  shape for independent reviewers (human, AI, or mixed) producing
  critique / improvement-backlog / fit-assessment / self-application
  / adoption-mapping content. Sits between Bug Report (too narrow),
  Learning (requires adopter codename), and Rule Proposal (validated
  content only). Cross-references the Reviewer Brief without making
  it gating. Codex item #6 from the 2026-04-27 brief-aware
  engagement; would have caught Lumo's malformed "learning record"
  YAML submission earlier.

- **`docs/start/should-you-adopt.md`** — combined decision/MVP page.
  Three-question filter (scope / failure surface / appetite); ideal
  adopter profiles; poor-fit cases; expected effort across three
  intensity tiers (minimal / standard / rigorous); the 15-minute MVP
  path (references existing `ADOPTING.md` Quickstart rather than
  duplicating); explicit stop conditions; signs to wait. Linked from
  the Start section index and added to the section nav. Corroborated
  by four reviewers in April 2026: Codex's separate proposals #1
  (decision page) and #3 (MVP-30-min) merged into one artifact, with
  Lumo's Quick Start scaffolding intent, Cursor's incremental-
  adoption framing, and Windsurf's 12-vs-3-4-starter growth-tension
  framing in scope.

- **`reviews/2026-04-27_zero-overkill-investigation.md`** —
  Sentinel-style investigation of Windsurf's observation that 161
  categorized learnings contain zero `overkill` entries. Finds the
  question was prior-acknowledged by Sentinel Reviews 2 and 4 (in
  pine-ridge and silver-index entries dated 2026-03-30) with the
  proposed explanation: principles (behavioral) are inherently
  non-overkill; ceremony (procedural) has a skip mechanism that
  prevents overkill from accumulating as recorded learnings. One
  concrete mis-categorization candidate identified (quiet-river
  describes overkill but is filed under `adaptation`). Companion
  surface: new "Observed distribution" subsection in `LEARNINGS.md`
  under Learning Categories, making the answer findable without
  grepping. Closes the parked single-reviewer hypothesis recorded
  in v0.38.0's engagement log.

### Why this is a release tag

Per project convention. Per-artifact dist impact varies:
`.gitlab/issue_templates/` is not in dist; `docs/start/should-you-adopt.md`
is site-only; `reviews/` is not in dist; `LEARNINGS.md` is in dist as
a read-only reference and gains the new "Observed distribution"
subsection.

## v0.39.0 (2026-04-27)

Housekeeping bundle from the 2026-04-27 external-AI engagements:
bibliography, version-semantics clarification, process-vs-enforcement
framing, inline maturity badges. Cumulative on v0.38.0 (engagement log).

> **Note on the v0.38.0 tag.** During the v0.38.0 release window a
> stray `git tag v0.38.0` ran from the wrong active branch and tagged
> commit `570ab84` (a commit not on main). v0.38.0 itself shipped to
> main as `99e80a1` and is a real release; the immutable tag points
> elsewhere and stays broken. Per framework convention tags are not
> deleted. See [`reviews/2026-04-27_version-tag-drift-recurrence.md`](reviews/2026-04-27_version-tag-drift-recurrence.md)
> for the full analysis and three candidate gates to prevent
> recurrence (recommended: pre-push hook on tag refs).
> Precedent: v0.25.2 was burned analogously in April 2026 — see
> [`learnings/iron-gate.yaml`](learnings/iron-gate.yaml) entries
> "Version-tag alignment requires manual discipline" (2026-04-12) and
> "Version-tag drift recurs because agov-pre-tag-check is opt-in,
> not gated" (2026-04-27, two-source corroboration of the gap).

### What's new

- **[`bibliography.md`](bibliography.md)** at repo root, rendered to
  the docs site as a top-level nav item. Single source of truth for
  external citations. Initial URL-bearing entry: The Alignment Layer
  Substack (informed `framework.md` v1, 2026-04-09). Sources cited
  by name without captured URL (McKinsey "Limits of RACI";
  project-management.com "RACI Alternatives") listed under "Cited by
  name only" rather than fabricating URLs. Inline citations in
  `framework.md` (lines 9, 745) cross-reference the bibliography.
  `docs/_gen/passthrough.py` extended with a link rewrite so the
  cross-references resolve on the docs site.

- **Versioning section in `README.md`** before Citation, plus matching
  note in `ADOPTING.md` §1.2. Disambiguates four semantically distinct
  version numbers in this repo: framework content (`VERSION`), Python
  tooling package (`pyproject.toml`), per-agent definition
  (`agents/*.yaml version`), and agent-to-framework conformance
  (`agents/*.yaml framework_version`). Codex item #10 from the
  2026-04-27 brief-aware engagement.

- **Process vs enforcement framing** surfaced in two places.
  `README.md` "What this is not" gains a fourth bullet stating the
  framework defines boundaries but does not mechanically enforce
  them. `docs/position/index.md` gains a "Process vs enforcement"
  section detailing the four enforcement layers (tool permissions,
  branch protections, CI/pre-commit hooks, review-gate workflows)
  and a Quick Reference table row. Corroborated: Cursor 2026-04-27
  explicit critique + Codex 2026-04-27 implicit framework-weakness
  #4. The v0.38.0 tag-drift incident is itself an instance of the
  same self-conformance failure — process discipline (run
  agov-pre-tag-check) without environment enforcement (no gate)
  broke under speed.

- **Inline maturity badges** demonstration. `status: provisional`
  frontmatter added to `docs/evidence/case-studies.md` and
  `docs/evidence/meta-patterns.md`. Codex item #7 from 2026-04-27,
  single-source — comprehensive sweep deferred until corroborated.

### Process — version-tag drift recurrence (carried forward)

This release carries forward the v0.38.0 tag-drift evidence:

- New learning entry in [`learnings/iron-gate.yaml`](learnings/iron-gate.yaml)
  ("Version-tag drift recurs because agov-pre-tag-check is opt-in,
  not gated"): same thread as the original v0.25.2 incident, second
  data point clears the corroboration bar.
- New sentinel review at [`reviews/2026-04-27_version-tag-drift-recurrence.md`](reviews/2026-04-27_version-tag-drift-recurrence.md):
  full analysis, three candidate gates, recommendation for a git
  pre-push hook on tag refs (option 2 in the review).

### Why this is a release tag

Per project convention. No `dist/` archive impact; citation now
points to v0.39 per the agov-bump README sweep.

## v0.38.0 (2026-04-27)

External-AI engagement log + REVIEWER-BRIEF.md. New top-level directory
[`external-ai-engagements/`](external-ai-engagements/) recording how
external AI assistants (not adopters, not contributors) read this
framework. Maintained as evidence of external legibility and the
common failure modes — identity confusion, schema fabrication — that
appear when AIs summarize a generically-named framework.

### What's new

- Six engagement records preserved verbatim (Microsoft Copilot
  2026-04-09; Lumo, OpenAI Codex, Mistral chat, Cursor, Windsurf
  2026-04-27 with the Reviewer Brief in hand; plus the v1 round
  without the brief). The corpus shows quality variance by an order
  of magnitude across AIs reading the same docs in the same week.
- [`REVIEWER-BRIEF.md`](external-ai-engagements/REVIEWER-BRIEF.md):
  a short verification checklist the framework asks reviewers to run
  before commenting (identity, schema, scope, evidence, output label,
  uncertainty). Not gating; not required; designed to catch the
  failure modes already observed.
- [Reviewer Brief Efficacy Test](external-ai-engagements/2026-04-27-reviewer-brief-experiment.md):
  paired before/after comparison across the cohort. Brief eliminates
  identity confusion (was 1/4) and YAML fabrication (was 1/4) and
  elicits explicit transparency about non-verification (now 3/6 from
  0/4). Does not lift substantive depth where the AI's underlying
  signal is shallow.

### Findings now corroborated by 2+ independent reviewers

- **URL inconsistency** (Codex + Cursor + Windsurf, three reviewers,
  two distinct files). Resolved at the GitLab Pages settings level
  by disabling the "Use unique domain" toggle so
  `janipaijanen.gitlab.io/agent-governance/` again serves directly
  with `HTTP/2 200` instead of redirecting to the project-hash URL.
  No code references needed updating.
- **Onboarding friction / decision page** (Codex + Lumo + Cursor +
  Windsurf, four reviewers). Action deferred to v0.40.0 content
  bundle.
- **Process-without-enforcement framing** (Cursor explicit + Codex
  implicit). Action deferred to v0.39.0 housekeeping bundle.

### Parked single-reviewer hypotheses

- **Zero overkill findings is suspicious** (Windsurf only — verified
  accurate; deferred to v0.40.0 content bundle for the investigation).
- **Time-bounded Provisional rules** (Lumo only — meta-governance
  question, not actioned without a second source).
- **Agent-vs-tooling boundary clarity** (Copilot v2 only — could not
  verify in session; re-test on a future Copilot engagement).

### Why this is a release tag

Per project convention every meaningful merge to main is a tag,
including bookkeeping/learnings/sentinel-only changes that don't
ship in dist. The engagement log is evidence-base content, not
shipped content; the dist archive is unaffected.

## v0.37.0 (2026-04-27)

GitLab Pages content fill-in batch — three independent improvements
to the docs site shipped together. No framework rule, agent, or
governance changes; pure rendering improvements on existing content.

### What changed on the site

- **Site-rendered `framework.md` and `ADOPTING.md`** (MR !76 / MR-α).
  New generator `docs/_gen/passthrough.py` reads both files from the
  repo root, rewrites cross-links per a known mapping (mutual links →
  on-site paths; repo-only files like `DISCLAIMER.md`, `LEARNINGS.md`,
  `profiles/` → absolute GitLab URLs), prepends a "Canonical source"
  callout, and writes them to `docs/concept/framework.md` and
  `docs/adopt/full-guide.md` respectively. Eight previously-external
  GitLab-URL references on the site are now on-site links.

- **Per-agent Mermaid communication subgraph** (MR !77 / MR-β).
  Each agent reference page (auto-generated from `agents/*.yaml`)
  now opens its Communication section with a Mermaid flowchart of
  that agent's immediate communication neighbourhood — solid
  bidirectional arrows for symmetric peers, dashed arrows with
  `|sends|` labels for asymmetric ones, central node highlighted in
  the site's primary palette. Sends-To and Receives-From bullet
  lists below now show nested `message_types` per peer.

- **Walkthrough narrative + orchestrator positioning** (MR !78 / MR-γ).
  Two skeleton "Coming in follow-up MR" pages replaced with real
  content:
  - `docs/start/walkthrough.md`: a complete pipeline iteration
    end-to-end (small CLI feature, Standard adoption intensity, all
    8 steps with concrete artifacts, including a TDD-pattern
    implementation snippet).
  - `docs/position/vs-orchestrators.md`: concrete relationship
    diagram + per-orchestrator notes (Claude Code primary;
    Cursor/Windsurf evidence gap; LangGraph/CrewAI/AutoGen as
    different category) + quick-decision table for adopters.

### Why this passes the framework's discipline

- **Pure rendering of existing content.** The repo files
  (`framework.md`, `ADOPTING.md`, `agents/*.yaml`) are unchanged;
  the site is a view.
- **No new agents, rules, or governance changes.** No symmetric
  evidence concern.
- The walkthrough is generic-and-synthetic on purpose to avoid
  duplicating live adopter content; the orchestrator positioning
  draws on existing operational evidence (Claude Code is the
  framework's primary tested integration) and acknowledges gaps
  honestly (no first-party Cursor / LangGraph adopters yet).

### Deferred (per the framework's evidence discipline)

- **Auto-render open candidates from the GitLab API at build time.**
  Adds network + auth dependency at build time for thin benefit;
  current hand-maintained `docs/status/candidates.md` works. Per
  `feedback_provisional_shipping.md`, the Provisional bar is
  "concrete evidence + clear Provisional header"; here the evidence
  is "would be nice-to-have", not corroborated pain. Re-evaluate if
  the candidate list goes stale enough to mislead.

- **Filling the remaining skeleton pages** (other position/vs-*
  files, per-step pipeline narrative, principles deep-dive, etc.).
  Per `feedback_split_mrs_per_concern.md`, fill section by section
  as evidence shows them needed, not on a schedule.

## v0.36.0 (2026-04-27)

Resolves the long-standing concept-modelling proposal (#29) via the
agent → rule downgrade pattern. Closes #29.

- **`rules-template/workflow/concept-modelling-rule.md`** (new,
  Provisional): for every new or moved attribute on persisted data,
  a reviewer (Gate 1 or Gate 2) classifies it as Intrinsic (property
  of entity itself), Derived (depends on collection / queue /
  processing path), or Composite (smell — conflates two axes).
  Storage location must match classification. Per-change checklist
  + concrete walkthrough of silver-quill's `source_category` bug
  (was queue-provider-keyed; should have been venue-URL-keyed; not
  caught by code review, infosec, or data-quality reviewers).

- **Why a rule, not an agent.** Issue #29's original proposal was
  for a new `concept-modeller` agent definition. That artifact form
  is heavier (jurisdiction, communication, MANIFEST entries,
  framework_version tracking) and the framework's calibration since
  v0.31.0 is: rules ship on single-adopter evidence with Provisional
  headers; new agents wait for 2+ adopter evidence. The rule
  preserves silver-quill's value, gives a future second adopter a
  concrete pattern to corroborate, and keeps the door open to
  upgrade to an actual agent if rule-only enforcement proves
  insufficient.

- README: rule-template count refreshed inline 35 → 36 (avoiding a
  follow-up stats-only patch).

### Notable

- Issue #29 had been open 9 days as provisional single-adopter; the
  symmetric evidence discipline (v0.31.0) gates *promotion* to
  recommended, not initial *shipping*. Per memory
  `feedback_provisional_shipping.md`, single-adopter + Provisional
  header is the established pattern. The 9-day gap reflected the
  artifact-form question (agent vs rule), not an evidence-threshold
  question.

## v0.35.0 (2026-04-27)

blue-marlin contribution batch — first new adopter (14th project) +
two new provisional rules covering visible-artifact verification and
heterogeneous file-generator output. Also includes inline README/
ADOPTING.md stats refresh (avoiding another stats-only patch like
v0.34.1). Closes #39, #40, #41.

### New adopter: blue-marlin

- `learnings/blue-marlin.yaml` (new, integrated via MR !68): upstream
  snapshot for blue-marlin — a parametric CAD tooling project
  (Python + FreeCAD + CadQuery, Layout B). 7 entries: 4 documentary
  validation-class learnings + 3 significant gap-class learnings
  (#39 / #40 / #41 below). 14th adopter project in the corpus.

### New rules (both Provisional, single-adopter evidence)

- **`rules-template/workflow/visible-artifact-verification-rule.md`**
  (#39 + #40): two-part discipline for any task producing visible
  output (drawings, renderings, generated documents, UI screenshots,
  charts).
  - Part A — output-level pytest invariants (no-overlap, ordering,
    continuity, presence, bounded magnitude) on the *produced*
    output, not only on input parameters.
  - Part B — explicit Gate 2 visual-acceptance checklist item
    ("did a human or programmatic image-capable reviewer open the
    output and visually inspect it this iteration?").
  Issued together because the two patterns are structurally
  complementary — Part A catches what unit-math misses; Part B
  catches what Part A misses; skipping either leaves an invisible
  defect class.
  Evidence: blue-marlin (5 governance iterations passed pytest
  while the produced DXF / 3D rendering had a cross-brace
  projecting through the deck; after adding 2 output-level
  invariants, visual review caught 2 more pre-existing defects).

- **`rules-template/workflow/file-generator-rule.md`** (#41):
  lowest-common-denominator output principle for files consumed by
  heterogeneous viewer ecosystems. Two MUSTs: audit format usage
  tree EXHAUSTIVELY in one pass (avoid the iterate-per-defect
  anti-pattern), and test against the actual consumer (not only
  the format-spec linter). Generalises to SVG, PDF, CSV, JSON-LD,
  OOXML, GLB, STEP. Sibling to `always/environment-interrogation-
  rule.md` (the file-format analog of the deployment-target
  pattern).
  Evidence: blue-marlin DXF DIMENSION/TEXT entities invisible in
  3D-CAD viewers (Onshape / Fusion / SolidWorks); two iterations
  to land on plain LINE / LWPOLYLINE primitives. Parallel
  observation in Copilot 2026-04-26 sentinel review (cited as
  supporting context, not as adopter corroboration).

### Stats refresh (inline)

- README.md / ADOPTING.md: 13 → 14 projects, ~186 → ~194 learnings,
  33 → 35 rule templates. v0.34.1 was a dedicated stats-only
  release; this release refreshes inline as part of the same commit
  that adds the new content, avoiding another stats-only patch.

### Notable about this release

- **First non-self adopter contribution batch** — blue-marlin was
  filed via the framework's standard adopter-contribution channel
  (Learning issues + MR), and the maintainer-side workflow
  (issue-assessment-rule, symmetric evidence discipline, provisional
  shipping) handled it without exception.
- Two single-adopter rules ship as Provisional; promotion to
  recommended waits on second-adopter corroboration per the
  v0.31.0 discipline.

## v0.34.1 (2026-04-27)

README and ADOPTING.md statistics catch-up. No framework, rule,
or agent changes.

- **README.md**: learning-records count refreshed from `~177` to
  `~186`; rule-templates count from `26` to `33`; stack-specific
  examples from `6` to `7`. The 7 new rule templates added across
  v0.32.0..v0.34.0 (destructive-op-preflight, environment-
  interrogation, deputy-governor, audit-log-completeness,
  boundary-shape-testing, migration-generation, ci-trigger-scoping)
  were not reflected in the cited counts at release time;
  `agov-pre-tag-check` warned but did not block.
- **ADOPTING.md**: same `~177 learnings` reference updated.
- **No content changes** — purely numeric corrections to keep
  citation honesty consistent with what's actually in the repo.

Process note: `agov-pre-tag-check`'s stat validator currently warns
on mismatches rather than blocking. Treating stale counts as a tag
blocker is a candidate for a future hardening MR.

## v0.34.0 (2026-04-25)

silver-birch corroboration batch — first promotion-to-recommended
under the symmetric evidence discipline, plus four new provisional
rules from a single-adopter contribution. Closes #33, #34, #35, #36,
#37, #38.

### Promoted from provisional to recommended

- **framework.md — Multi-perspective review for non-trivial plans.**
  v0.30.1 / v0.30.2 shipped this on single-adopter evidence
  (iron-gate L043: 3 reviewers, 15 findings, zero overlap).
  silver-birch (issue #34, framework v0.33.0) provides second-adopter
  corroboration: 12 specialist agents, 22 findings (11 significant,
  3 critical), zero overlap, different domain (Elixir/Phoenix),
  different scale, different artifact type. The empirical signal is
  structural — driven by jurisdiction separation. **Status: recommended.**
  This is the first promotion under the symmetric evidence discipline
  added in v0.31.0 (issue-assessment-rule §Defending existing patterns).

### Documentation

- **ADOPTING.md Step 1.4b — slash-command install one-liner.**
  silver-birch (#35) hit the gap that copying `skills/` to
  `.claude/skills/` does not surface them as Claude Code `/<name>`
  slash commands; `SKILL.md` must also be in `.claude/commands/`.
  Step 1.4b now includes the explicit copy loop and a note about
  per-tool install paths (Cursor etc).

### New rules (all Provisional, single-adopter evidence)

- **`rules-template/always/audit-log-completeness-rule.md`** (#36):
  every CUD operation produces an audit trail. Audit calls go in the
  success branch (error-branch audits create false trails).
  Scope-specific function names over generic catch-all. Per-change
  Gate 2 check + per-milestone grep-based gap audit.
  Evidence: silver-birch (~65 missing audit calls across 33
  controllers; structurally invisible without dedicated agent).

- **`rules-template/workflow/boundary-shape-testing-rule.md`** (#37):
  external-data parsing tests must use the real shape (atom-vs-string
  key types, exact field names) plus a negative regression case.
  One canonical parser per boundary — duplication causes drift.
  Evidence: silver-birch OIDC group extraction silently returned `[]`
  at login because three duplicate parsers used different key types.

- **`rules-template/workflow/migration-generation-rule.md`** (#38a):
  never hand-write migration timestamps; always generate via the
  stack's tool (mix ecto.gen.migration, alembic revision, rails
  generate migration, knex migrate:make, etc.) then edit without
  renaming. Five-step ripple check after every migration: schema →
  validation → factories → seeds → integration tests.
  Evidence: silver-birch hit 9 schema-migration parity incidents in
  a 48-hour window before adopting tool-generation + ripple check.

- **`rules-template/workflow/ci-trigger-scoping-rule.md`** (#38b):
  long-running CI stages must be path-scoped on MRs. Governance-only
  changes (`agents/`, `learnings/`, `docs/`) must not trigger
  application-layer test suites. Operationalises the build-engineer
  agent's existing "right tests for right changes" design principle.
  Tag pipelines and manual triggers always run unconditionally.
  Evidence: silver-birch had a 75-minute Playwright suite firing on
  every MR including governance-only changes; path-scoping reduced
  governance-MR CI time from ~80 min to 1–2 min.

  Issue #38 contained two distinct rule proposals collapsed under one
  title. Split into rules 3 and 4 above per the issue body's explicit
  "Proposal 4" header.

### Learnings corpus

- `learnings/silver-birch.yaml`: three new entries appended
  (audit-log gap, parallel assessment corroboration, skills install
  gap). `sentinel_ack` for the cross-codename reference between
  silver-birch and iron-gate (the corroboration link is the learning
  value, not an information leak).

### Notable about this release

This release is the first time the symmetric evidence discipline
(v0.31.0) has been applied to **promote** an existing provisional
pattern to recommended via a documented second adopter — not just
to gate net-new structural additions. The four new rules
simultaneously demonstrate the *other* arm of the same discipline:
ship-as-provisional with single-adopter evidence, invite
corroboration in the rule headers, and wait for a second adopter
before promoting.

## v0.33.0 (2026-04-18)

Deputy Governor: time-bounded delegated approval as a provisional
workflow rule. Single-project evidence plus theoretical grounding in
existing consent-agenda and acting-authority governance patterns.

- **`rules-template/workflow/deputy-governor-rule.md`** (new):
  formalizes how approval authority at Gate 2 may be delegated to an
  agent acting in a Deputy Governor role, while the Human Governor
  retains ultimate authority.

  **Framework-default (load-bearing, generic):**
  - Red list of never-delegable categories: irreversible ops,
    security boundaries, architecture/data-model changes, critical
    severity, meta-governance, rules/agents definitions, compliance
    and licensing artifacts.
  - Delegation Artifact structural requirements: delegator, deputy,
    scope, severity ceiling, absolute expiry, audit trigger, and
    revocation conditions (auto-revoke on first pull, first challenge,
    expiry, or explicit revocation).
  - Deputy obligations: red-list check first, artifact check, severity
    ceiling (never critical), per-decision logging, stop-at-audit-
    trigger.
  - Consent-agenda pattern as the recommended entry-level shape —
    smallest blast radius (any pull auto-revokes the Artifact).
  - Periodic audit is mandatory, not decorative. Reversal at audit
    automatically revokes all active Delegation Artifacts and must
    produce a learning-record entry.

  **Per-project (opt-in, empty by default):**
  - Delegable categories — CUSTOMIZE block intentionally ships empty.
    Adopters opt in per category only after Step 0 assessment and a
    manual Gate 2 audit cycle producing evidence of routinely low-risk
    categories for their project.
  - Severity ceiling per category (never critical).
  - Project-specific red-list extensions.

  **Why opt-in by default.** Shipping a generic default list of
  delegable categories would repeat the Layout-B silent-failure mode
  (issue #22, fixed in v0.31.1) — adopters would accept the list
  without doing the risk-profile work. Empty CUSTOMIZE forces
  conscious opt-in.

- **Provisional status.** The structural requirements and red list are
  load-bearing regardless of domain, but the specific workflow wiring
  has only single-project evidence (this framework's own session
  cadence, 2026-04-18). Per v0.31.0's symmetric evidence discipline,
  the rule stays provisional until a second adopter corroborates.
  `framework.md` references and agent-YAML wiring are deferred until
  corroboration.

## v0.32.1 (2026-04-18)

Learnings corpus update — silver-quill adopter submissions
(2026-04-16..18) integrated into the upstream snapshot. No framework
or rules changes; no effect on dist archive contents (codename YAMLs
do not ship). Closes #24, #25, #26, #28.

- `learnings/silver-quill.yaml`: five new learnings appended
  covering parallel three-agent code review (validation), parallel
  specialist agents vs generalist assessment (validation), layered-
  architecture briefing gap in assessment agents (gap), concept-
  modeller agent missing from roster (gap), and pre-flight checker
  missing before destructive git ops (gap, critical — already
  addressed by v0.32.0 rules).
- `learnings/silver-quill.yaml`: `adoption.upgrade_history` records
  the v0.23.1 → v0.30.2 adopter upgrade (was previously empty).
- No rules, no framework.md, no ADOPTING.md, no agents/ changes.
- Related open item: #29 (concept-modeller rule proposal) stays
  open as provisional single-adopter evidence per v0.31.0's
  symmetric evidence discipline.

## v0.32.0 (2026-04-18)

Pre-flight preconditions: two new rules-template entries addressing
the recommendation-without-verification pattern. Corroborated by two
independent adopters. Closes #30, #31, #32.

- **`rules-template/workflow/destructive-op-preflight-rule.md`** (new):
  gates agent recommendations for git history rewrites, forced pushes,
  filesystem destruction, and destructive database ops. MUST-steps: list
  preconditions → verify each → block on failure. Concrete example
  walkthrough for `git filter-repo` with an unclean working tree.
  Evidence: silver-quill (2026-04-18, #30, critical severity) — agent
  recommended `git filter-repo` to purge committed credentials;
  8 uncommitted fixes silently discarded by the rewrite's fresh
  checkout.
- **`rules-template/always/environment-interrogation-rule.md`** (new):
  gates agent recommendations that assume target-host state. Mandatory
  probes: installed binary paths/versions (especially user-space package
  managers — `pipx`, `cargo install`, `npm -g`), API/token format
  variants, beneficiary-project config cross-check, filesystem
  constraints (bind mounts, tmpfs). Concrete example walkthrough for a
  `systemd` user service whose binary was pipx-installed.
  Evidence: amber-relay (2026-04-18, #32, significant severity) — agent
  generated four-step containerised-runner setup without interrogating
  the host; all four steps failed with knowable-in-advance errors.
- Both rules carry a **Provisional** header. The meta-pattern
  (preconditions matter) is corroborated by two adopters, but each
  specific checklist is still single-adopter evidence and invites
  corroborating learnings from future adopters.
- No new agent role this round. Grow-by-pain defers agent-splitting
  until rule-only enforcement proves insufficient.

## v0.31.1 (2026-04-17)

Adoption fast-path fix: layout choice is now mandatory before any
path-dependent step. Closes #22.

- **ADOPTING.md**: renamed and reordered "Where to put files" →
  "Choose your layout" and moved it *before* the Quickstart. The
  layout decision determines every file path that follows, so
  presenting it after the Quickstart was a silent-failure mode.
- **ADOPTING.md Quickstart**: declared Layout B assumption explicitly
  and made all paths self-consistent with it (previously the Quickstart
  extracted the archive as `agents-govern/` but wrote file paths at
  project root — Layout A-style — producing governance artifacts in
  the wrong location).
- **ADOPTING.md layout table**: now shows both Layout A and Layout B
  paths side-by-side for every artifact, instead of only showing the
  Layout B mapping.
- **README.md**: updated the "before creating governance files"
  callout to point at the renamed section and flag the silent-failure
  mode explicitly.
- Evidence: silver-quill learning (2026-04-16) — adoption placed
  GOVERNANCE.md, agents/, and learnings/ at project root instead of
  under agents-govern/. Error persisted through two framework upgrades
  before discovery.

## v0.31.0 (2026-04-16)

Entry-point scope sharpening and symmetric evidence discipline in
response to two external AI agent proposals (ChatGPT, Lumo AI) that
read the framework as a general AI/ML governance platform.
Closes MR !51.

- **README.md / ADOPTING.md**: added explicit "what this is, what this
  is not" block and Step 0 "confirm this is the right framework"
  callout. Non-claims cover general AI governance, ML governance, and
  agent-runtime use cases. Evidence: two independent external AI
  agents mis-scoped the framework at first read.
- **.claude/rules/workflow/issue-assessment-rule.md**: new
  **Defending existing patterns** section. Deflections-as-duplicate
  ("already implemented by X") must cite the corroborating learning
  record(s) or be marked provisional. Applies the framework's own
  evidence-from-2+-adopters discipline symmetrically to defenses of
  current structure, not just to additions.
- **profiles/research-lab.md** (new, UNTESTED skeleton): stub for an
  ML/research adopter profile. Non-negotiables locked (no auto-fix,
  no auto-block on adversarial scores, no speculative ML-wrapper
  plugin). Excluded from distribution archive until a real adopter
  tests it.
- **reviews/2026-04-16_external-ai-proposals-scope-misreading.md**
  (new): sentinel record of the external proposal exchange, matching
  the `2026-04-12-ChatGPT.md` precedent.

## v0.30.2 (2026-04-15)

Clarifies adopter vs framework-internal files for newcomers in
README and ADOPTING.md. Docs-only patch.

*Backfilled in v1.1.15 (2026-05-03) — original release-notes
placeholder was left unfilled at release time. Single commit:
`docs: clarify adopter vs framework-internal files for newcomers`.*

## v0.30.1 (2026-04-14)

Plan claim verification, multi-perspective review guidance.
229 tests. Closes #19, #20.

- framework.md: added **plan claim verification** gate — Fact-Checker
  verifies plan claims against actual codebase before Gate 1 approval.
  Gate 1 contributors updated from "—" to include Fact-Checker.
  Evidence: iron-gate L042 (7 wrong factual claims in one plan).
- framework.md: added **multi-perspective review** guidance — non-trivial
  plans require parallel review from three angles (factual, architectural,
  rule compliance). Evidence: iron-gate L043 (15 findings across 3
  reviewers, zero overlap).

## v0.30.0 (2026-04-14)

CLI namespace, shell wrapper removal, communication completeness,
agent topology fixes, agov-health, 4 new skills, dist integration tests.
229 tests. Closes #16, #17, #18.

### BREAKING CHANGES

- **CLI rename** (Closes #18): all commands renamed to `agov-` prefix.
  `validate-agents` → `agov-validate-agents`, `validate-learnings` →
  `agov-validate-learnings`, `validate-infosec` → `agov-validate-infosec`,
  `validate-contributions` → `agov-validate-contributions`,
  `validate-adoption` → `agov-validate-adoption`, `validate-readme-stats`
  → `agov-validate-readme-stats`, `pre-tag-check` → `agov-pre-tag-check`.
  Old names no longer exist.
- **Shell wrappers removed**: `scripts/*.sh` deleted. All CLI is
  `uv run agov-*` only. dist no longer ships shell scripts.
- **Agent communication changes**: academic-peer-reviewer,
  academic-visual-reviewer, fact-checker, capability-developer all have
  modified communication sections. Adopters who copied these must re-sync.

### Features

- **agov-health CLI** (`uv run agov-health`): periodic governance health
  check that runs all validators in sequence (adoption, agents strict,
  contributions, infosec) and produces a summary. Designed for pre-commit,
  weekly CI, or on-demand use. 3 tests.
- **Communication completeness check** (Closes #16): `agov-validate-agents`
  now warns when agents have sends_to but no receives_from (send-only) or
  vice versa (receive-only). Structurally silent agents are flagged. 5 tests.
- **--strict flag** (Closes #17): `agov-validate-agents --strict` treats
  communication asymmetries and completeness warnings as errors (exit 1).
  For zero-asymmetry enforcement in CI.
- **Dist integration tests**: 12 tests that build a dist archive, extract
  it, and verify shipped tools work against shipped content. Catches missing
  files, broken imports, and stale references after extraction.
- **4 new skills** (3→7 total): `health` (periodic governance check),
  `release` (gated release workflow), `contribution-audit` (adversarial
  review process), `capability-review` (agent evolution analysis round).
  All ship downstream. All pass contribution audit screening.

### Fixes

- academic-peer-reviewer: added communication section (was isolated — no
  sends_to or receives_from). Now sends review_report to governor-reviewer
  and content_context to academic-visual-reviewer.
- academic-visual-reviewer: added communication section (was isolated).
  Now sends visual_review_report to governor-reviewer, receives
  content_context from academic-peer-reviewer.
- capability-developer: removed stale `sentinel` reference (future Phase 4+
  agent, not yet implemented). Added comment for future reintroduction.
- fact-checker: removed stale `pre-tag-check.sh` reference (script, not
  agent). Converted to structured communication format.
- governor-reviewer: added sends_to/receives_from for academic-peer-reviewer
  and academic-visual-reviewer. Communication symmetry fully restored.

## v0.29.0 (2026-04-14)

Contribution Auditor agent, adversarial pattern detection, upstream scrutiny.
206 tests. Closes #15.

- **Contribution Auditor agent** (Closes #15): new Phase 1 cross-cutting agent
  that audits contributions for adversarial steering, unsound guidance, false
  precedent, and subtle capability drift. Six audit dimensions: plausibility,
  precedent safety, evidence quality, absence bias, boundary integrity, rule
  soundness. Mandatory from first contribution — not optional, not Phase 3+.
  Complements InfoSec Sentinel: that agent screens for information leaks; this
  agent screens for guidance integrity. Together they form the two-sided trust
  gate for incoming contributions.
- **Upstream scrutiny**: Contribution Auditor explicitly audits upstream
  framework updates on version upgrade. Same red flags apply regardless of
  source. Documented in framework.md and ADOPTING.md Section 6.4.
- **validate-contributions CLI** (`uv run agov-validate-contributions`): mechanical
  heuristics for absence-bias detection, governance relaxation detection, and
  evidence quality checking. 18 tests. Registered in pyproject.toml.
- **MR template checklists**: all three merge request templates (Learning Record,
  Rule or Agent, Default) now include guidance integrity checklist items from
  the Contribution Auditor's audit dimensions.
- framework.md: Contribution Auditor added to Phase 1 cross-cutting agents
  (always active). InfoSec Sentinel moved from "activate when needed" to
  "always active" alongside it. Full agent role description with upstream
  scrutiny clause.
- ADOPTING.md Step 1.3: contribution-auditor.yaml and infosec-sentinel.yaml
  added to mandatory Phase 1 copy list.
- governor-reviewer.yaml: added contribution-auditor to communication
  (receives audit_report, governance_relaxation_alert, block_recommendation;
  sends audit_request, red_flag_tuning). Communication symmetry restored.
- MANIFEST.yaml: contribution-auditor added as active.

## v0.28.0 (2026-04-13)

Version bump automation, graph coordination edges, versioned graph output,
design principles, ADR doc-gating. 188 tests. Closes #10, #11, #12, #13, #14.

- **agov-bump CLI tool** (`uv run agov-bump minor|patch|major|X.Y.Z`): automates
  version bumping across VERSION, CHANGELOG.md, README citation, and all agent
  framework_version fields. No carry between semver components (0.99.0 → 0.100.0).
  Registered in pyproject.toml as `agov-bump`.
  24 tests including no-carry edge cases. (Closes #13)
- **README citation auto-update**: `agov-bump` updates the APA and BibTeX citation
  version (major.minor only, per academic convention). `pre-tag-check` now validates
  citation matches the release — prevents the 4-release drift from v0.25 to v0.27.
- **Graph: coordination edges** (Closes #11): `coordinates_with`, `consults`, and
  `reports_to` fields now render as a distinct third edge style (blue dotted) in
  both Mermaid and Graphviz graphs, separate from symmetric (solid) and asymmetric
  (red dashed). Added `_extract_coordination_agents()` and `include_legacy`
  parameter to `_extract_comm_agents()`. `consults` added to symmetry checking.
  Legend updated with coordination edge style.
- **Graph: versioned output and --check** (Closes #12): `validate-agents --graph
  --output docs/` writes versioned markdown (`agent-communication-{version}.md`)
  with agent index table, summary statistics, and Mermaid graph. `--check` mode
  compares generated content against existing file and exits 1 if stale (CI-ready).
  `generate_versioned_markdown()` function for programmatic use.
- **design_principles field** (Closes #14): added `identity.design_principles`
  (recommended, not required) to agent definition template in framework.md. Added
  2 domain-specific principles to all 11 upstream agents. ADOPTING.md Step 1.3
  updated with guidance for adopters to add their own.
- **ADR doc-gating guidance** (Closes #10): added "Documentation gating by ADR
  status" to framework.md Architect section. Proposed ADRs → design docs only.
  Accepted ADRs gate user-facing documentation. Aspirational docs must carry
  "Not yet implemented" warning. Evidence: Phase 1 adopter had 4 docs describing
  unimplemented features from a proposed ADR.

## v0.27.0 (2026-04-13)

Gate-triggered review, agent manifest, graphviz diagrams, validate-adoption,
self-conformance, copyright enforcement. 151 tests. Closes #9.

- framework.md: added "Cross-Cutting Activation Modes" section to Pattern 5
  with four formalized modes (manual-review, checklist-in-governor,
  gate-triggered-review, standalone-agent). Documents trigger positioning
  trade-offs (MR creation vs pre-merge) and scoping guidance.
  Evidence: Phase 1 adopter had Security Guardian as manual-review; 2
  high-severity issues persisted across 2 releases until explicitly asked.
- ADOPTING.md Step 3.6: expanded with gate-triggered-review as recommended
  mode for projects with CI or AI tooling. Added CI job, AI prompt, and
  pre-merge hook examples. Added migration guidance from existing modes.
- learnings/_template.yaml: formalized activated_as vocabulary to four
  canonical values with migration notes for prior variants (per-change,
  standalone).

---

## v0.26.1 (2026-04-12)

Fix: Phase 1 cross-cutting agents missing from framework.md.

- framework.md: added separate "Phase 1 cross-cutting agents" table listing
  fact-checker, infosec-sentinel, academic-peer-reviewer, and
  academic-visual-reviewer with activation criteria. These were declared as
  phase: 1 in their YAML definitions but absent from the Phase 1 section.
- validate-agents: added phase distribution summary to cross-agent output,
  making drift between agent YAML phases and framework.md visible on every
  pre-commit and CI run.

---

## v0.26.0 (2026-04-12)

Build Engineer agent, capability-developer agent, local-only agent concept,
adversary-tester agent, 98 unit tests, security fix, cross-agent validation,
dual-license split.

- agents: new `build-engineer.yaml` — Phase 1 production agent that owns
  CI pipeline configuration, build automation, container definitions, and
  pipeline efficiency. Addresses corroborated gap: CI/build files were
  ungoverned in Phase 1 (evidence: copper-rapids, silver-birch). A separate
  finding (silver-birch: full Playwright e2e suite triggered when no code
  changed) demonstrated that this is a structurally distinct concern from
  code verification — a Coder-Tester does not naturally ask "should these
  tests have run at all?"
- agents: new `capability-developer.yaml` — Phase 4 meta agent that
  analyzes accumulated learnings and sentinel reviews to produce agent
  evolution proposals. Reactive-only (evidence-triggered, not speculative).
- agents: new `agents/local/` concept — local-only agents that exist
  exclusively within the framework's own development, excluded from
  distribution via `.distignore`
- agents: new `agents/local/adversary-tester.yaml` — local-only meta agent
  for adversarial security assessment of the framework's Python
  implementation
- agents: updated `coder-tester.yaml` — CI/build file ownership
  transferred to Build Engineer; added build-engineer to communication
- agents: updated `governor-reviewer.yaml` — added build-engineer,
  infosec-sentinel, fact-checker, and capability-developer to
  communication; added build/CI pipeline review checklist
- agents: updated `deployment-engineer.yaml` — standardized communication
  to receives_from/sends_to format; added Build Engineer merger notes;
  clarified jurisdiction boundary with Build Engineer
- agents: updated `planner-architect.yaml` — updated cannot_touch to
  reference Build Engineer
- framework.md: added Build Engineer to taxonomy (Section 2), verb table
  (Principle 1), artifact ownership table (Principle 2), Phase 1 table
  (Section 7); updated agent count from 19 to 20; updated Phase 3 to
  reference Build Engineer merger option
- ADOPTING.md: updated Step 1.3 to include build-engineer.yaml; updated
  minimum viable adoption to mention Build Engineer for projects with CI
- reviews: first capability-developer analysis report
  (2026-04-12_capability-developer-analysis-01.md) — 3 evolution proposals,
  4 structural issues, 2 watch items
- tests: added pytest infrastructure and 84 unit tests covering _yaml.py,
  validate_infosec.py (detection + evasion), and dist.py (.distignore
  handling, path traversal, symlinks). Framework now has non-zero test
  coverage for its own validation tools.
- **SECURITY FIX**: dist.py .distignore processing was vulnerable to path
  traversal — patterns containing `../` could delete files outside the
  distribution directory, and absolute path patterns caused unhandled
  exceptions. Fixed with pattern validation (reject `../` and absolute
  paths) and defense-in-depth resolve() check. Found by adversarial
  testing during this session.
- ADOPTING.md: added capability-developer activation guidance to Step 7;
  added Build Engineer to split signals table
- validate-agents: added cross-agent consistency checks — communication
  symmetry (sends_to/receives_from pairs) and artifact ownership overlap
  detection. Added --graph flag for Mermaid communication diagram.
- pre-tag-check: generates docs/agent-communication-{version}.md at
  release time as maturity evidence (included in release archive)
- pre-commit: hooks now use uv run directly instead of bash wrappers;
  added validate-agents and pytest hooks
- License split: Python tools (src/, tests/) now MIT (LICENSE-MIT);
  framework content (agents/, rules-template/, framework.md, docs/)
  remains CC-BY-SA-4.0 (LICENSE). Adopters can embed validation tools
  in proprietary CI pipelines without ShareAlike obligations.
- Version split: framework version (VERSION file) and tools version
  (pyproject.toml) are now tracked independently. They coincide at
  0.26.0 but will diverge when changes affect only one component.

---

## v0.25.5 (2026-04-12)

Platform-independent rules/skills, pre-tag validation, iron-gate learnings.

- framework.md: defined rules and skills as platform-independent concepts
  with tool mapping table (Claude Code, Cursor, generic)
- rules-template/README.md: reframed as tool-agnostic content
- uv run agov-pre-tag-check: validates VERSION, CHANGELOG, agent definitions,
  tag existence, and working tree before tagging
- learnings(iron-gate): 3 new entries (version-tag alignment, SPDX
  frontmatter, platform independence)

---

## v0.25.4 (2026-04-12)

Fix: skill frontmatter parsing broken by SPDX headers.

- skills: moved SPDX license headers from before the opening `---` fence
  to after the closing fence in all 3 SKILL.md files — Claude Code's
  parser requires `---` on line 1
- skills/infosec-screen: documented the frontmatter exception in Step 3
  (SPDX header verification) with example

---

## v0.25.3 (2026-04-12)

Version-tag alignment: VERSION file now matches the git tag at the
tagged commit (v0.25.2 tag was pushed before VERSION was bumped).

---

## v0.25.2 (2026-04-12)

README restructured to separate adopter and contributor audiences.

- README.md: added "Do not clone this repository" warning at top
- README.md: split into "For adopters" (release archive path, contents
  table with Action column, quick start) and "For framework contributors"
  (repo-only files: learnings, reviews, CI, dist.sh, .distignore)
- README.md: quick start heading now includes skills

---

## v0.25.1 (2026-04-12)

Documentation accuracy: refreshed stale project and learning counts.

- README.md: 10 → 13 projects, ~126 → ~143 learnings, 24 → 26 rules,
  added 3 skills to current state
- LESSONS-LEARNED.md: same count updates in narrative sections
- Protected `main` branch: direct push disabled, MR-only enforcement

---

## v0.25.0 (2026-04-12)

Skills distribution: reusable AI assistant workflows shipped alongside
agents, rules, and scripts. Copyright header fix across all files.

- New: `skills/` directory with three upstream skills (issue #8):
  - `skills/upgrade/SKILL.md` — 6-step gated upgrade workflow with two
    user confirmation gates (evidence: iron-gate v0.23.1 → v0.24.0
    upgrade, 33 files across 4 categories)
  - `skills/learning/SKILL.md` — learning record scaffolding with
    validation (evidence: iron-gate 30+ manually scaffolded entries)
  - `skills/infosec-screen/SKILL.md` — comprehensive leak screening
    wrapping validate-infosec.sh + pre-commit + SPDX + patterns
    (evidence: iron-gate project-specific patterns missed by script alone)
- New: `skills/README.md` — documents the `origin:` frontmatter
  convention (upstream vs project), promotion criteria, and the
  two-layer design (technology-agnostic framework + reference
  implementation targeting Claude Code)
- ADOPTING.md: added Step 1.4b (copy skills) with two-layer framing —
  explicitly acknowledges that skills are the implementation layer,
  adopters not using AI assistants can skip
- README.md: added `skills/` to repository contents table, ASCII
  adoption diagram, and quick start
- dist.sh: skills included in distribution archive and MANIFEST
- New: `.distignore` — pattern-based exclusion for repo-only files.
  `dist.sh` copies directories wholesale, then strips anything matching
  `.distignore` patterns before archiving. Keeps internal-only skills,
  dev scripts, and repo-specific rules out of the distribution without
  changing the directory structure
- Fix: copyright headers corrected from "Paijanen" to "Päijänen"
  across all files (79 files affected)

---

## v0.24.0 (2026-04-12)

Iron-gate upstream learnings: new-agent workflow, AI-assisted development appendix,
and version traceability in all GitLab templates.

- framework.md: added "New-Agent Workflow" subsection to Section 7 (Growth
  Phases) — 3-step checklist to keep cross-cutting concerns table in sync
  when creating agents (evidence: iron-gate issue #6 / UL-3, cross-cutting
  table stale for three agents over several days)
- framework.md: added "Appendix: AI-Assisted Development" — problem
  statement and 4 recommended adaptations for projects using AI coding
  assistants (evidence: iron-gate issue #6 / UL-4, AI assistant bypassed
  schema governance three times before CI gate added)
- GitLab templates: added `Framework version` field to Learning issue
  template, Rule Proposal issue template, and all 3 MR templates (Default,
  Learning Record, Rule or Agent) — Bug Report already had it
  (evidence: iron-gate issue #6 / UL-5, template consistency audit)

---

## v0.23.1 (2026-04-09)

Academic agents, frost-quill learnings, and MR template fix.

- New agents: academic-peer-reviewer (v0.2.0) and academic-visual-reviewer
  (v0.1.0) — domain-agnostic manuscript review agents promoted from
  frost-quill (evidence: 8 content review rounds + 1 visual round,
  issue #1, MR !22)
- learning(frost-quill): 4 new entries (contribution process friction,
  MR auto-close gap, dog-fooding validation, Web IDE YAML risk) +
  adoption metadata updated to v0.23.0 (evidence: issue #2)
- MR templates: added `Closes #` auto-close placeholder to all 3
  templates (evidence: frost-quill learning — issue #1 left open after
  MR !22 merge)
- Deleted `version-file` branch containing pre-anonymization learning
  records with real project names (infosec finding)

## v0.23.0 (2026-04-09)

Self-assessment and severity vocabulary standardisation.

- LEARNINGS.md: added Severity Levels table defining
  critical/significant/minor/informational with framework response
  guidance (evidence: 12 entries used non-vocabulary "moderate",
  triggered by MS Copilot external review)
- LEARNINGS.md: added Sentinel authority and succession paragraph —
  decision documentation, role transfer, multi-person handling
- Fixed 12 severity entries in pine-ridge and silver-index:
  "moderate" → "significant" (vocabulary standardisation)
- Self-assessment audit: fixed 10 self-conformance issues across
  framework.md, README.md, LESSONS-LEARNED.md, glass-key.yaml, and
  6 GitLab templates (missing SPDX headers, stale counts, missing
  Documentation Writer in taxonomy, Principle 7 violation)

## v0.22.0 (2026-04-09)

Stone-arch adoption learnings and ADOPTING.md improvements.

- New adopter: stone-arch (enterprise CRM, GDPR-native, 6 learnings at v0.21.0)
- ADOPTING.md Step 0.3: added verification guidance — assess security gaps
  against actual code/config, not secondary sources (evidence: stone-arch
  3/5 false-positive gaps from threat-analysis-based assessment)
- ADOPTING.md Step 1.4a (new): codebase-vs-rules conformance audit at
  adoption time — CONFORMANT/PARTIAL/GAP per rule (evidence: stone-arch
  discovered 8 PARTIAL + 1 GAP only after explicit post-adoption audit)
- ADOPTING.md Step 1.1: documented advisory-agent pattern for domain
  specialists during planning (evidence: stone-arch used UX Architect and
  CRM Domain Specialist at Gate 1)
- CONTRIBUTING.md: added step-by-step guide for submitting learning MRs

## v0.21.0 (2026-04-09)

ADR template and adoption guide for end-users.

- Added `docs/adr/_template.md`: copyable ADR template with all 8 required
  sections including Supersedes field and inline guidance comments
- Expanded `docs/adr/README.md`: quick start, format table, lifecycle
  rules, and index-driven supersession convention (ADR files are immutable;
  the README index carries supersession status)
- ADOPTING.md: new Step 2.4 — set up `docs/adr/` with template

## v0.20.0 (2026-04-09)

DISCLAIMER and git filter-repo safety.

- Added DISCLAIMER.md: no warranty, limitation of liability, advisory
  guidance
- Added git filter-repo pre-flight check to dist.sh
- Documented corroborated signals from adopter learnings
- New learning records: silver-index (fork-and-evaluate, 8 learnings),
  copper-rapids (task-assignment and rules upgrade, 7 learnings)

## v0.19.2 (2026-04-09)

InfoSec Sentinel bugfix and learning record redaction.

- validate-infosec.sh: fixed `extract_freetext()` to handle multi-line
  YAML values (`>` and `|` folded blocks) — Category D cumulative scoring
  was effectively disabled for any file using YAML folding
- validate-infosec.sh: finding IDs now use per-file counters instead of
  a global sequence — adding/removing files no longer renumbers findings
  in other files, making sentinel_ack entries stable
- Aggressively redacted all learning records: removed architecture
  fingerprints, specific tool/technology names, real institution names,
  cross-record links, and competitive positioning details
- learnings/_template.yaml: documented per-file finding ID scoping
- Updated sentinel_ack entries to match per-file IDs
- Backfilled CHANGELOG entries for v0.18.1–v0.19.1
- Evidence: 6 new learnings from iron-gate sentinel application

## v0.19.1 (2026-04-08)

Second history rewrite pass (janitor).

- Consultative guidelines added to infosec-sentinel agent definition

## v0.19.0 (2026-04-08)

InfoSec Sentinel agent for information leak screening.

- Added cross-cutting agent: `agents/infosec-sentinel.yaml`
- Added screening script: `uv run agov-validate-infosec` (6 detection
  categories: credentials, sensitive details, cross-field re-identification,
  cumulative leakage, cross-record linkage, semantic leaks)
- Added `sentinel_ack` mechanism to `learnings/_template.yaml`
- Updated MR templates with infosec sentinel validation step
- Added infosec lessons learned (v0.15.4–v0.19.0) to learning records
- Added sentinel_ack entries for 5 initial findings across 3 records

## v0.18.2 (2026-04-08)

- Fixed MANIFEST.md leaking CI token in Repository URL
- Added APA citation and BibTeX entry to README
- dist.sh: release archive now includes citation
- Added release checklist reminder to dist.sh

## v0.18.1 (2026-04-08)

- CHANGELOG, SECURITY.md, MR template updates

## v0.18.0 (2026-04-08)

Open-source readiness release.

- README: added ASCII lifecycle diagram showing the adoption and feedback loop
- CONTRIBUTING.md: contribution guidelines for learnings, rules, agents
- CODE_OF_CONDUCT.md: community standards emphasizing evidence and adopter privacy
- GitLab issue templates: Learning, Rule Proposal, Bug Report
- GitLab MR templates: Default, Learning Record, Rule or Agent
- dist.sh: release archive now includes all community files (59 files)
- README: fixed stale counts (5 agents, 24 rules, 6 stack-specific examples)
- CI: fixed SIGPIPE exit code 141 from tar | head truncation

## v0.17.0 (2026-04-07)

Complete anonymization of all narrative documentation.

- Scrubbed all real project/product names from LESSONS-LEARNED.md,
  GOVERNANCE.md, 5 sentinel review documents, and 5 learning records
- Git history rewritten with `git filter-repo` -- zero real names in any
  past commit message or file content
- Codename mapping: copper-rapids, silver-birch, iron-gate, frost-quill,
  north-wind, ember-clock, glass-key, quiet-river, pine-ridge
- Added adopter-validated rules from field evidence (Sentinel Review 5):
  - `rules-template/workflow/agent-proactive-analysis-rule.md`
  - `rules-template/workflow/testing-environments-rule.md`
  - `rules-template/docs/feature-flagged-docs-rule.md`
  - `rules-template/always/i18n-supported-locales-rule.md`
  - `rules-template/always/verifiable-claims-rule.md`
- Added Phase 3 agent definitions:
  - `agents/documentation-writer.yaml`
  - `agents/deployment-engineer.yaml`
- Added stack-specific examples:
  - `project-specific-examples/saas-multi-tenant/always/rls-and-tenancy-rule.md`
  - `project-specific-examples/tailwind-daisyui/ui/tailwind-daisyui-ui-rule.md`
- README: added end-user flow for proposing rules and agents to the framework

## v0.16.2 (2026-04-07)

Complete anonymization migration for all 8 adopters.

- Migrated remaining 7 learning records to codenames
- ANONYMIZATION-NOTICE.md updated: all 8 records show Migrated status
- 13 downstream projects commissioned to v0.16.1

## v0.16.1 (2026-04-07)

Documentation consistency fixes from framework self-assessment.

- Scrubbed all iron-gate references from iron-gate.yaml (added SPDX header)
- Renamed and anonymized ADR (iron-gate-adopter-feedback)
- Reconciled project count across README/GOVERNANCE/LESSONS-LEARNED (9)
- Fixed Sentinel review count in README (5, not 6)
- Fixed rule template count in README (19 generic + 4 stack-specific)
- Bumped agent definitions from v0.15.0 to v0.16.0
- Normalized learning filenames (dropped .learning/.learnings suffixes)
- Replaced placeholder codename in LESSONS-LEARNED.md

## v0.16.0 (2026-04-06)

Sentinel Review #5. Evidence: ~92 learnings across 10 projects.

- Added five-level domain maturity model to framework.md (Deployable /
  Validated / Instantiated / Schema-defined / Planned)
- Added `rules-template/docs/implementation-honesty-rule.md`
- License change CC BY-SA 4.0 confirmed (already shipped)
- 4 stale signals approved for retirement at Review 6
- LESSONS-LEARNED.md refreshed (52 to ~92 learnings, 6 to 10 projects)
- GOVERNANCE.md updated with Review 5 entry

## v0.15.4 (2026-04-06)

Mandatory anonymization policy for adopter submissions.

- Template enforces `project.codename` field (replaced `project.name`)
- Anonymization policy header in `_template.yaml`
- ADOPTING.md Step 0.1 mandates codename selection
- LEARNINGS.md: added Submission Privacy Policy section
- `validate-learnings` CI job gates MRs touching `learnings/`
- Pre-commit hook for local validation
- Migration notice for 8 existing adopters
- First migration: iron-gate record anonymized

## v0.15.3 (2026-04-05)

MR validation pipeline for distribution builds.

## v0.15.2 (2026-04-05)

MANIFEST quick-start for adopters in distribution archives.

## v0.15.1 (2026-04-05)

Release rule fix (missed v0.15.0 tag).

## v0.15.0 (2026-04-05)

First adopter-driven release. Iron-gate (cybersecurity exercise SDL)
generated 16 learnings over 3 days of adoption.

- Added VERSION file as single source of truth for framework version
- Added release-via-merge-request rule
- End-user flow for proposing rules to the framework
- Distribution archive builder (`uv run agov-dist`)

## v0.14.0 (2026-04-05)

ADRs, license change, release infrastructure.

- Architecture Decision Records (`docs/adr/`)
- License changed from CC BY-NC-SA 4.0 to CC BY-SA 4.0
- GitLab CI pipeline for builds and releases
