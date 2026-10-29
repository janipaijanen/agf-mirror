# Changelog — agents-govern 2.0

v2.0 entries only. The v1.x changelog is not part of the `agf/`
distribution.

## [Unreleased]

## [2.13.0] — 2026-10-29

Documentation paths in "Adopter docs" lines are relative to `agf/docs/site/` for
Pages and to `agf/` for rules, templates, schemas and skills.

### Added

- **Bounded record-only publication termination (!3354, `ee2a768a`).** An
  additive, authority-sensitive permission ends recursive decision-publication
  chains only when every stated condition holds: prior authority was published
  before the authorized action, all that authority is exhausted, records remain
  unchanged and create no new obligations, full-range publication and exact-head
  CI checks pass, and the exact merge tree and resulting parents are verified.
  Ordinary Human review remains valid; uncertainty returns to that path. This
  does not grant general self-merge, retrospective approval or new gate authority,
  and changes no schema, tooling or CI behavior. Capability: Level 5 Deployable in the v2.13.0
  distribution when using compatible artifacts. Evidence: Empirically informed
  by foundry use, without broad adopter validation. Adopter docs: updated
  (`rules/workflow/coordination-publication-rule.md`,
  `docs/concepts/coordination-protocol.md`, `templates/AGENTS.md`).

### Fixed

- **Urgency shipment status in the polling guide.** The heading and maturity
  sentence now match the already-shipped v2.11.0 Level 5 Deployable capability
  when using compatible artifacts. Urgency remains advisory and supplies no
  permission. This corrects a stale page found by the full-estate review; it
  changes no runtime behavior or governance authority. Adopter docs: updated
  (`docs/operate/poll-and-coordinate.md`).

## [2.12.0] — 2026-10-28

**Documentation-focused minor release.** Standard current-main release from
reviewed construction base `fc98b4d1`. It adds evidence-calibrated guidance for
using AGF alongside other assistant tooling, modernizes the public homepage's
canonical routes and accessibility presentation, and corrects shipped command
and v2.11 coordination-feature documentation. No runtime, schema, validator, or
governance-authority behavior changes. **Adopter docs: updated.**

### Added

- **Evidence-calibrated guidance for using AGF with other assistant tooling
  (b703704c, MR !3311).** A new adopter page defines authority, instruction,
  hook, and data-flow boundaries before tools are combined. The README and site
  navigation expose it, with bounded, dated Ponytail and Caveman examples and
  explicit Level 2 Surface-defined / Theoretical coexistence evidence. No
  integration, endorsement, or adopter-validated compatibility is claimed.
  Adopter docs: updated (`README.md`,
  `docs/adopt/third-party-plugin-coexistence.md`).

### Changed

- **Homepage routes readers to canonical guidance (dce4396f, MR !3314).** The
  public homepage now leads with Decide, Adopt, Operate, and Using AGF with
  other tools routes instead of duplicating detailed guidance. Route cards
  reflow from two columns to one, preserve visible keyboard focus, and use
  light/dark theme contrast reviewed at the exact implementation head. Adopter
  docs: updated (`index.md`, `stylesheets/agf.css`).

### Fixed

- **Shipped skills use current `agov` commands (dc5523f6, from MR !3098).** The
  `health` skill now describes the eleven checks that `agov validate` runs, and no
  longer mentions a strict mode that does not exist. The `infosec-screen` skill and
  the skills index now name `agov validate infosec` instead of `validate-infosec.sh`,
  which is not shipped. The landed change is wording only: no runtime, schema or
  validator behaviour changes. This entry is reconciled to the freeze under the
  Human decision to retain the landed change. Adopter docs: updated
  (`agf/skills/health/SKILL.md`, `agf/skills/infosec-screen/SKILL.md`,
  `agf/skills/README.md`).
- **Coordination feature pages reflect the published v2.11.0 artifacts.**
  Urgency ordering, challenge-response, and publication-hold fields no longer
  describe v2.11 as unreleased or awaiting shipped-artifact evidence. The
  compatible framework/tooling warning remains. Adopter docs: updated
  (`docs/adopt/mcp-server.md`, `docs/concepts/coordination-protocol.md`,
  `rules/workflow/coordination-publication-rule.md`).

## [2.11.0] — 2026-10-26

**Minor release.** Standard-path release from reviewed `main` at
`24d098a1`. Full Rust workspace suite: 739 passed, 0 failed, 3 intentionally
ignored network/Podman tests; each framework archive passed all 28 shipped
tests. It tightens MCP and polling contracts, adds explicit coordination
holds, challenge-response, supersession and plan-collision controls, exports
the optional Improvement Propeller method, and refreshes adopter documentation.
The breaking entries below include their required migrations; features that
need adopter-local authority remain inactive by default. **Adopter docs:
updated.**

### Changed (breaking)

- **MCP strict arguments (57527ede).** `agov-mcp` now rejects a tool call that
  carries a key the tool does not publish, with a JSON-RPC error naming the tool
  and the key. Before, such keys were silently dropped. Migration: compare the
  keys your client sends with `inputSchema.properties` from `tools/list` and
  remove the extras. Adopter docs: updated (`docs/adopt/mcp-server.md`).
- **Windows tooling preview support is retired (c2a73e5e).** Current
  `agov`/`agov-mcp` release tooling targets Linux x86_64; migrate Windows
  preview automation to a Linux host or the OCI image. Existing historical
  artifacts are unchanged. Adopter docs: updated (`docs/adopt/mcp-server.md`,
  `docs/adopt/tooling-and-tokens.md`, `docs/start/commission.md`,
  `reference/release-artifacts.md`).
- **Polling and timer wording follows runtime reality (8bfdc25f).** The universal
  polling rule's "Scheduling and notification honesty" section is now the single
  statement of timer qualification, expiry and re-arm reporting. The
  `poll-and-coordinate` skill's default idle window changes from 240 to 180
  minutes, and copied blanket bans on background processes are replaced by a
  pointer to that section. Migration: replace copied bans with the pointer; set
  an explicit idle window if you relied on the 240-minute default. Adopter docs:
  updated (`docs/operate/poll-and-coordinate.md`,
  `docs/adopt/assistant-loader-configuration.md`,
  `docs/concepts/coordination-protocol.md`, `templates/AGENTS.md`).

### Added

- **`agov coord list --ref` and the MCP `ref` argument (c1999c8e).** Reads the
  coordination channel from an already-fetched commit-ish, so a poll is not
  answered from a stale checkout; the server never fetches. With `ref`,
  `format: json` returns an object instead of an array, so strict clients must
  branch on whether they sent `ref`. Adopter docs: updated
  (`docs/adopt/mcp-server.md`, `docs/concepts/coordination-protocol.md`,
  `reference/tooling-conformance.md`).
- **Publication holds and `agov coord hold-status` (68003b56).** New optional
  coordination fields `urgency`, `publication_hold` and `deferred_by`, a
  `coordination-hold-status` schema, and an advisory command that reads pinned
  Git snapshots. Records that use the new fields are rejected by older tooling;
  upgrade `agov` and the schemas together before using them. Adopter docs:
  updated (`docs/concepts/coordination-protocol.md`,
  `docs/operate/poll-and-coordinate.md`, `docs/adopt/mcp-server.md`,
  `templates/AGENTS.md`).
- **Advisory challenge-response and urgency ordering (cbfa285b).** An optional
  `challenge` object on `review` records, a new workflow rule, and visibility of
  open challenges in the coordination tooling. Adopter docs: updated
  (`docs/concepts/coordination-protocol.md`, `docs/operate/poll-and-coordinate.md`,
  `rules/workflow/coordination-challenge-response-rule.md`).
- **Self-announcing supersession (78fc52a7).** New optional `supersedes` field on
  coordination records; a superseding record names the earlier records it
  replaces for the same audience, and a validly superseded record no longer
  appears as a pending acknowledgement. Adopter docs: updated
  (`rules/workflow/coordination-publication-rule.md`,
  `docs/concepts/coordination-protocol.md`).
- **`superseded` plan status (e9f7742d).** Gate 1 plans may now carry
  `status: superseded` with its required fields. Adopter docs: updated
  (`templates/gate1-plan.md`).
- **`pending` advisory consultation status (4a00dc6e).** A requested review that
  has not answered may be recorded as `pending`; a plan cannot become
  `approved`, `active` or `complete` while any declared row is still `pending`.
  No existing valid plan becomes invalid. Adopter docs: updated
  (`templates/gate1-plan.md`, `templates/AGENTS.md`).
- **Plan `depends_on` and optional blocking collision check (bcc76b04).** Plans
  may acknowledge a deliberate `affected_paths` overlap with `depends_on`. The
  new `agov plan collision-check --fail-on-unacknowledged-collision` flag blocks
  on unacknowledged overlaps; the default output severity stays advisory and the
  default collision exit code stays 1. Adopter docs: updated
  (`rules/workflow/development-workflow-rule.md`, `templates/gate1-plan.md`,
  `templates/AGENTS.md`).
- **Improvement Propeller skill (32fd0134).** Exports the Improvement Propeller
  method as an optional skill. It activates nothing: each adopter still needs its
  own governor decision naming the player, evidence surfaces, real wake
  mechanism, cadence and cursor state. Adopter docs: updated
  (`skills/improvement-propeller/SKILL.md`, `skills/README.md`).
- **Third-party plugin coexistence page (08d3a2ce).** Adopter docs: updated
  (`docs/adopt/third-party-plugin-coexistence.md`,
  `docs/adopt/assistant-loader-configuration.md`).
- **Governance mode LIGHT/NORMAL/STRICT (7efe8867).** The simplicity rule
  classifies each task by process weight before work starts. Adopter docs:
  updated (`rules/workflow/simplicity-rule.md`).
- **High-risk-operations domain customization (5216a2ef).** The rule's list is
  now a worked example that adopters replace with their own domain's areas.
  Adopter docs: updated (`rules/always/high-risk-operations-rule.md`).

### Changed

- **Release payload guard (7588a7c5).** The release manifest no longer lists
  Elixir and `mix` files as forbidden; the generic forbidden-file mechanism is
  unchanged. No adopter behavior changes. Adopter docs: not applicable, because
  the manifest controls the AGF release payload only.
- **Coordination schema compiled once per scan (d4cc426a).** Internal
  performance fix; no contract change. Adopter docs: not applicable.
- **Documentation currency fixes (046cec8a, bfc4d648, 7bd189b9, 4f7b6b95,
  2165b579, a507cadd, 52892835).** Role roster, integration-contract paths,
  governance page, decommission layout, environment-variable and figure
  references, an MCP tool row, and the roadmap. Wording and path corrections;
  no behavior change. Adopter docs: updated.

## [2.10.1] — 2026-09-12

**Patch release, MINOR-shaped content.** Standard-path release: ships
all reviewed work merged to `main` since v2.10.0, not only the
customer-driven extension this entry leads with. The release content
is frozen from reviewed `main` at `90bb57a2`. Full workspace test
suite: 593 passed, 0 failed, 3 intentionally ignored network/podman
tests. **Adopter docs: updated.**

**Version-number note:** the customer-driven addition below is
additive/backward-compatible and would conventionally be a MINOR
release on its own. This release ships as a PATCH number at explicit
governor direction, to keep the customer-driven work decoupled from
the separately paused v2.11 work. See
`docs/plans/20260912T155648Z-v2101-ruw-generic-artifact-validation.md`.
The other changes below shipped independently on `main` before this
release cut and are included because a standard-path release ships
everything reviewed and merged since the preceding release.

### Added

- **Customer-driven addition — generic artifact/graph validation
  extension.** `agov-config.yaml` gains an optional `artifact_sets`
  block: adopters can declare their own project-specific artifact types
  (name, path globs, JSON Schema, ID/reference JSON Pointers, uniqueness
  scope) without AGF hardcoding any adopter-specific paths or semantics.
  Absent this block, behavior is unchanged for every existing adopter.
  **Adopter docs: updated**
  (`agf/docs/site/reference/artifact-registry.md`).
- `agov artifact validate` and `agov artifact index` validate configured
  artifact sets against their registered schemas and produce a
  deterministic, byte-repeatable index. Enforces structural graph
  properties only — reference existence, dependency-graph acyclicity,
  global ID uniqueness — no governance-judgment semantics are inferred.
  Also exposed through `agov-mcp`.
- Filesystem containment (symlink rejection at every path component,
  canonicalized root containment, recursive external-`$ref` rejection,
  16 MiB read cap, bounded glob-traversal state) reviewed by
  build-engineer and by an independent, non-author security reviewer.
  Two advisory limitations accepted as outside the stated threat model
  (a disclosed TOCTOU gap requiring an actively hostile concurrent
  filesystem mutator; pattern-count amplification bounded only by
  config file size) — not fixed, both documented in the review record.
- **Safe multi-record coordination publication (CP-B-1).** `agov coord
  publication-check` supports a candidate range that adds more than one
  coordination record, with byte-identical copy rejection checked
  against the union of the merge-base inventory, the pinned current
  target tip, and earlier candidate additions — closing a target-tip
  copy blind spot found in review. Target and candidate refs are each
  resolved once to an immutable commit before any ancestry, range, or
  content read, and every decisive Git read fails closed instead of
  defaulting to an empty result on error. See
  `docs/plans/20260909T200622Z-cpb1-copied-record-contract-clarification.md`.
- **Two CI reliability fixes** to this project's own pipeline, unrelated
  to each other and to the customer-driven addition above:
  `commercial-corpus:validate` now sets `GIT_DEPTH: "0"` so its
  merge-base comparison does not fail on branches more than ~20 commits
  past their target; `rust:tooling:deny` now clears
  `$CARGO_HOME/advisory-dbs` before running `cargo deny check`,
  mitigating an observed crash. The root cause of the latter is
  disclosed as unproven, not a demonstrated fix; see
  `docs/plans/20260912T112252Z-cargo-deny-advisory-db-clean-clone-gate1.md`.
- **Improvement Propeller Stage 4 activated.** The bounded weekly
  opportunity/governance-friction scan, shipped planned/inactive in
  v2.10.0, is now active at Level 4 Validated for foundry per governor
  decision `20260909T043713Z-governor-reviewer-djh1`. Level 5 still
  requires a genuine unattended scheduled wake, not yet recorded.
  Adopter projects remain inactive until their own governor supplies an
  activation contract.
- Coordination-channel tooling also gained advisory branch-handoff
  discovery, advisory plan-collision checks, and cross-session handoff
  addressing/reply-evidence annotations, merged since v2.10.0 as
  ongoing governance-tooling work; see `docs/coordination/` for the
  full record of each.

### Downstream impact

**ADDITIVE** for the customer-driven artifact/graph validation
extension: no existing coordination record schema or command behavior
changed by it, and adopters who do not add an `artifact_sets` block see
no behavior change from it at all. The two CI fixes are **PREVENTIVE**
to foundry's own pipeline only — adopters do not consume this project's
`.gitlab-ci.yml`. CP-B-1 and the coordination-tooling items are
**ADDITIVE** to the coordination-channel tooling surface; no existing
coordination record schema changed.

## [2.10.0] — 2026-09-08

**Minor release.** Delivers autonomous AGF development operation through the
universal polling contract, the bounded Improvement Propeller, restored token
ROI estimates, and a fail-closed four-namespace release guard. The release
content is frozen from reviewed `main` at `fb118b89`. Reversible preflight
passed 506 Rust tests with 3 intentionally ignored network/podman tests; each
built framework archive also passed its 28 shipped tests. **Adopter docs:
updated.**

### Added

- Universal polling and autonomous continuation now applies to every active
  development role: sessions discover the complete coordination, merge-request,
  issue, and awaited-state surface, then act only within their role, authority,
  approved scope, current gate, and collision-safe claim. The same rule adds
  bounded spare-work decomposition and the proposal-only capability-review
  trigger after 10 new valid learnings, with escalation by 15. **Adopter docs:
  updated.** **Downstream impact: BREAKING** for direct adopters because this
  adds a new MUST-level assistant duty, while the framework's
  backward-compatible functionality remains SemVer MINOR-shaped. Migration:
  record the last completed capability-review learning cursor and count,
  project the universal rule through assistant startup surfaces, configure a
  real wake schedule where supported, and document activation/resume polling
  where it is not.
- The Improvement Propeller role, canonical rule, documentation, and projection
  evidence provide a bounded weekly opportunity and governance-friction scan.
  It ships planned/inactive at **Level 4 Validated**; post-release issue #131
  owns activation, a genuine scheduler wake, and any later Level 5 maturity
  claim, while #132 owns the subsequent skill export. It cannot assign,
  prioritize, approve, implement, merge, or release work. **Adopter docs:
  updated.**
- `agov release namespace-check` and the mandatory
  `release:namespace-check` merge-request job verify authoritative absence of
  the candidate Git tag, GitLab Release, generic-package version, and derived
  OCI reference. Collection access, full package-status pagination, project
  identity, and target-head ambiguity all fail closed. The unchanged mandatory
  operator preflight separately verifies the exact protected-tag policy.
  **Adopter docs: updated. Downstream impact: BREAKING** for direct adopters
  because the standard release path gains mandatory operator preflight and
  immediate pre-push checks. Migration: configure the release-project inputs
  and authorized token described by the release skill, adopt the CI job, and
  retain its JSON evidence.

### Changed

- Poll scheduling is explicitly non-reentrant and truthful. A real supported
  wake that arrives during active work coalesces into one cycle at the next safe
  boundary; the default 15-minute cadence expires after 180 quiet minutes, and
  unchanged state stays quiet. Assistants without a real wake path poll on
  activation or resume rather than claiming background execution.
- The foundry learning corpus records the end-to-end MCP-readiness failure:
  installing or building `agov-mcp` is not sufficient unless the assistant
  runtime actually loads the structured tools and a smoke test proves them
  callable. This is project evidence for follow-up issue #116, not a claim
  that every runtime exposes MCP today.

### Fixed

- Restored the documented automatic token-ROI baselines for `cold-start`,
  `session-resume:<role>`, and `next-handoff:<role>`. Direct CLI and
  `agov-mcp` calls now use the same savings arithmetic; transport affects
  attribution only. Existing telemetry is not rewritten, so the estimates
  appear only for successful calls recorded by a build containing this fix.
  **Adopter docs: not applicable** — the repair makes implementation match the
  existing documented baseline model and requires no operator or configuration
  change.

## [2.9.0] — 2026-09-06

**Minor release.** Adds mechanical gate-change accompaniment and strengthens
the foundry release path so required checks, changed-file discovery, OCI
namespace verification, and vulnerability scanning fail closed before an
immutable release tag is created. **Adopter docs: updated.**

### Added

- A shipped gate registry contract: `schemas/gate-registry.schema.json`, a
  neutral `examples/gates.yaml`, and `agov gate change-check --base-ref
  <ref>`. For each active registry entry, declared surfaces and tests must
  resolve; changing a surface requires a paired declared test-path change;
  and a gate ID cannot disappear. Retirement is explicit and retains the ID,
  reason, and date while permitting obsolete paths to be removed.
- Pre-tag container CVE coverage. `agov:oci:mr-build-scan` builds and
  Trivy-scans the release image at merge-request stage, reusing the existing
  `rust:tooling:preview` archive. Previously `agov:oci:release` was the only
  job that built and scanned the image and ran on tags only, so a container
  CVE could surface only after the tag was immutable — which is how
  `v2.7.0-rc.1` was burned.
- `agov-oci-image.sh image-ref`, `unpublished-ref`, and `verify-ref
  <reference>` expose and check the single published OCI reference, so a
  pre-tag namespace check cannot query a different name than publication
  uses.

### Changed

- `gate-self-test-rule.md` now applies to modification and retirement as well
  as gate introduction. Gate mechanics are explicitly excluded from the
  fast-track waiver, regardless of diff size. Existing projects that adopt
  the new mechanical command must create `.agf/gates.yaml`; the registry is
  not inferred automatically.
- Foundry CI now derives changed files through a fail-closed helper: absent,
  malformed, or ambiguous comparison inputs stop validation instead of
  appearing as an empty change set. Mandatory CI `needs` edges are also
  implication-tested across supported pipeline contexts.
- The capability-developer role is active again after the project learning
  corpus exceeded its governed evidence threshold. Activation remains
  assignment-bound and advisory; it does not gain gate authority.
- `agov-oci-image.sh` build and scan bodies are parameterised by image
  reference. The published path keeps its tag guard; the new unpublished
  path tags `localhost/agov-unpublished:$VERSION`, which is not a registry
  namespace, so no merge-request-stage path can name, reach, or overwrite a
  release namespace, and no registry credential is required.
- `agf/VERSION` added to `rust:tooling:preview`'s trigger paths: it names the
  produced archive that the new scan job consumes.

## [2.8.0] — 2026-08-28

**Minor release.** Ships the complete current `main` line through
`ca808607`, including issues #86, #87, #88, and #90 plus the post-v2.6.1
framework, schema, tooling, and governance corrections. `v2.7.0` is retired
unused; immutable `v2.7.0-rc.1` remains an abandoned failed candidate.
**Adopter docs: updated.**

### Added

- The `agov-mcp` server now exposes the coordination surfaces needed for
  cold start and cross-branch visibility, validates JSON-RPC 2.0 request
  versions and IDs before dispatch, and ships assistant-loader discovery plus
  a ready-to-copy Claude MCP configuration in tooling archives (#86, #87,
  #88, #90).
- Commercial-corpus and rule-projection manifests provide machine-readable
  ownership and projection contracts, with schema and CI validation across
  their governed lifecycles.
- Documented the GitLab first-push/MR-registration pipeline race at assistant,
  contributor, CI-maintainer, and public operator points of use. The guidance
  requires source/ref/status/job-coverage comparison before cancellation and
  does not claim that CI behavior itself has changed.

**Downstream impact: PREVENTIVE.** Corrected from an earlier EDITORIAL
classification per `agf/rules/always/downstream-impact-rule.md` — this
change adds mandatory operational checks (source/ref/status/job-coverage
comparison) and forbids same-SHA-alone cancellation, which is proactive
hardening of an implicit constraint, not a wording-only change. Full
three-circle impact statement in
`docs/plans/ci-first-push-expectation-guidance.md`.

### Changed

- **BREAKING workflow relaxation:** projects that retain `docs/sources.md`
  and/or `docs/radar.md` as non-normative research artifacts may let a narrowly
  eligible advisory, reversible maintenance MR skip a separate Gate 1 plan,
  contribution audit, and pre-change coordination MR. The canonical
  `agf/rules/workflow/simplicity-rule.md` now owns exact path and semantic
  eligibility, automatic disqualifiers, per-commit/final-tree evidence, fresh
  target/open-MR collision scans, escalation, and the retained human Gate 2
  boundary. Generic documentation does not inherit the exception. Before
  enabling it, confirm both artifacts are non-normative, keep protected-branch
  MR review and human Gate 2 enabled, and ensure assistant loaders point to the
  updated rule; projects without the artifacts or wanting stricter governance
  retain full Gate 1. No file or data migration is required. Adopter docs
  parity: updated in the day-to-day guide, situation map, and AGENTS template.

### Added

- `agov validate` and `agov session resume` now emit an advisory (never
  blocking) warning when the running `agov` binary's own build version
  differs from the commissioned `agf/VERSION` of the tree it is invoked
  against (issue #114). Neither an older binary run against a newer
  tree nor a newer binary run against an older tree was previously
  detectable — both could silently under- or over-validate. Absent
  `agf/VERSION` (not every tree `agov` runs against is AGF-adopted)
  produces no warning.

## [2.6.1] — 2026-08-17

**Minor release.** Corrects v2.6.0's incomplete publication: the release
tooling now reads one validated `tooling/rust/release-targets.txt`
manifest across the mandatory Linux build loop, release-asset contract,
and OCI archive lookup, closing the stale-target-list gap that failed
v2.6.0's `release:create` job after its OCI image had already published.
Ships the same coordination-reliability functional scope v2.6.0 targeted
(#76, #84, #85, #89), constructed directly from `main` rather than a
frozen line, per `docs/plans/20260816T174141Z-v261-release-publication.md`.

### Added

- New `web-ux-reviewer` agent role (`agf/agents/web-ux-reviewer.yaml`),
  activated in this foundry's `agf/agents/MANIFEST.yaml`. Reviews the
  rendered public Pages site (`agf/docs/site/`) for responsive layout,
  navigation/CTA correctness, dark/light theme rendering, WCAG AA
  accessibility (in situ, complementing the existing token-pair-only
  `check_palette_contrast.py`), typography, and overall visual
  impression — independent of the session/model that implemented the
  change under review. Distinct from `academic-visual-reviewer` (which
  stays skipped; scoped to compiled academic PDF output, not web/Pages
  content). Additive: no existing role, schema, or command behavior
  changes; adopters without a public Pages site can leave the role
  unstaffed or remove the definition file. See
  `20260808T161743Z-planner-architect-kpdj.md` finding 4a for the
  staffing-gap evidence that motivated this role.
- New `independent-implementation-reviewer` agent role
  (`agf/agents/independent-implementation-reviewer.yaml`), activated in
  this foundry's `agf/agents/MANIFEST.yaml`. Read-only, advisory review
  of a named implementation's exact Git head against its approved Gate 1
  plan, acceptance criteria, tests, and CI evidence, by a session and
  model distinct from the implementer. Named per non-trivial Gate 2
  candidate, not a mandatory fourth gate; `CLEAR`/`BLOCK` is evidence for
  `governor-reviewer`, never a Gate 2 approval or mechanically blocking
  signal (`tool-authority-boundary-rule.md`). Distinct from
  `contribution-auditor`, whose jurisdiction covers contribution/
  governance-guidance integrity, not exact-head implementation-diff
  review. Additive: no existing role, schema, or command behavior
  changes; adopters may leave the role unstaffed or remove the
  definition file. See `20260808T161743Z-planner-architect-kpdj.md`
  finding 4b and `20260808T171050Z-contribution-auditor-q1ge.md` for the
  audit-trail-ambiguity evidence that motivated this role.
- Maintenance-line release exception in
  `agf/rules/workflow/release-via-merge-request-rule.md`: an optional,
  narrowly eligible path for shipping a previously published,
  immutable release-candidate commit as a stable version after your
  default branch has kept moving with unrelated work — without pulling
  that work into the promoted release. Requires a binding governor
  decision naming the exact candidate, its own separately Gate
  1-approved release plan, Maintainer-level protection on the
  maintenance branches, a matching protected-tag rule, and post-merge
  proof that both refs survive the merge before anything is tagged.
  The standard release workflow (branch from your default branch, tag
  the merge commit) is unchanged and remains what every ordinary
  release uses — this exception exists for the specific, evidenced
  case where a shipped candidate must ship again exactly as-is, later.
  See [Review and release](docs/site/docs/operate/review-and-release.md#releases-the-maintenance-line-exception)
  for the adopter-facing explanation and the release skill's Step 0 for
  the fail-closed selection procedure. Additive: no existing release
  path changes; adopters who never need this exception see no
  difference in their normal release flow. **Adopter docs parity:
  updated.**
- Frozen-scope reconstruction exception in
  `agf/rules/workflow/release-via-merge-request-rule.md`: a second,
  rarer optional path for shipping a governor-frozen, named subset of
  already-approved work as its own immutable release when that work
  was never combined into one reviewable commit and your default
  branch has since kept moving with other, unrelated approved work
  that must not enter the release. Requires a binding governor decision
  naming the frozen scope and stable base, its own Gate 1-approved
  release plan, an independently reproducible proof that the
  reconstructed content is exactly the named scope, and post-merge
  proof before anything is tagged. The standard release workflow and
  the maintenance-line exception are both unchanged and remain the
  default — this exception exists for the specific, evidenced case
  where an approved-but-never-assembled scope must ship on its own.
  See [Review and release](docs/site/docs/operate/review-and-release.md#releases-the-frozen-scope-reconstruction-exception)
  for the adopter-facing explanation and the release skill's Step 0 for
  the fail-closed selection procedure. Additive: no existing release
  path changes; adopters who never need this exception see no
  difference in their normal release flow. **Adopter docs parity:
  updated.**

### Fixed

- Release tooling now reads one validated `tooling/rust/release-targets.txt`
  manifest for the mandatory Linux build loop, release-asset contract, and
  OCI archive lookup. Preview CI assembles real framework/tooling archives and
  checks the pre-OCI set; tag CI runs the same check before any OCI push. This
  prevents a target-list change from leaving the producer and mandatory-asset
  verifier out of sync. The immutable `v2.6.0` tag is retained as a failed
  publication attempt: no GitLab Release exists, while `agov:2.6.0` is
  failed-publication OCI residue and must not be treated as a usable release.
- `agov coord list --since` now accepts the compact filename-prefix
  timestamp (`20260729T195431Z`, no punctuation) in addition to
  RFC3339-with-punctuation and a full message id/filename (#84).
- `agov coord visibility` no longer treats sequence collisions confined
  to base-branch history, unrelated to the branch under inspection, as
  blocking. They are reported as a `legacy_base_collisions` summary
  (count + seqs) and no longer force a nonzero exit; a collision that
  involves the branch under inspection still blocks, unchanged (#85).
- `agov coord visibility`'s default (no `--branch`) scan now enumerates
  local branches and remote-tracking branches, deduplicated by commit,
  so a coordination file that exists only on a fetched-but-not-checked-
  out remote branch is discovered without the caller needing to already
  know to pass `--branch origin/<name>` explicitly. The base ref used
  for comparison also prefers a configured upstream's OID over a
  possibly-stale local ref when they diverge (#89).

### Changed

- `independent-implementation-reviewer` is **provisionally inactive
  (suspended)** in this foundry's `agf/agents/MANIFEST.yaml`, effective
  2026-08-09, per governor decision
  `20260809T073516Z-governor-reviewer-2gbg.md` accepting review
  `20260809T063108Z-independent-implementation-reviewer-12nr.md`'s six
  blocking findings against the role definition (incomplete both-gates
  prohibition, missing infosec-escalation route, and others). No work
  may rely on or commission the role until a corrective MR resolves the
  findings, an independent exact-head review returns COMPLETE/CLEAR, and
  a fresh binding Gate 2 decision reactivates it. This is an authority/
  status correction, not a rejection of the role program; see the same
  decision record for the full corrective-work sequence.
- **BREAKING**: `agov coord visibility`'s JSON output gains two
  always-present fields (`effective_base_oid`, `effective_base_source`)
  and one always-present summary object (`legacy_base_collisions`), and
  narrows `sequence_collisions` to branch-relevant collisions only
  (`base_duplicate_seq` entries move to `legacy_base_collisions`). Text
  output gains an always-first `effective-base: <oid> (<source>)` line.
  Any adopter script or CI job that branches on `agov coord
  visibility`'s exit code (0/1/2) may see a different code for the same
  repository state after this change — see `docs/plans/
  agov-coordination-read-path-reliability.md`'s migration note. Full
  field-by-field contract in `docs/reference/coord-visibility.md`.
- **BREAKING (Phase 0, no deprecation cycle — zero known external
  dependents on the aarch64-linux tooling archive today; see
  `workflow/deprecation-policy-rule.md`):** `aarch64-unknown-linux-musl`
  dropped from the mandatory Rust tooling target set in
  `.gitlab-ci.yml`. Cross-compilation for this target via Docker/QEMU
  repeatedly exhausted disk on the shared `saas-linux-small-amd64` CI
  runner class (`docs/plans/20260805T084100Z-drop-aarch64-linux-required-target.md`).
  Future tag releases publish only the `linux-x86_64` Rust tooling
  archive until CI runner capacity for this target is sponsored.
  Already-published `linux-aarch64` archives on releases v2.4.7 through
  v2.5.0-rc.2 are unaffected. `agf/docs/site/reference/
  release-artifacts.md` updated to match.

### Performance

- `agov coord visibility` reads coordination files for each candidate
  branch via one batched `git cat-file --batch` process instead of one
  `git show` process per file, cutting Git process count from O(branches
  × files) to O(branches) (#76). A representative fixture with 15
  branches, equal-OID aliases, a diverged remote ref, and 225+
  coordination files dropped from ~500ms to ~95ms wall-clock in local
  testing.

Adopter docs: waived — `docs/reference/coord-visibility.md` was
rewritten in full to cover the new default branch scope, base-ref
freshness, legacy-collision handling, the exit-code table, and the JSON
field contract; it is not under this validator's recognized
`agf/docs/site/docs/{adopt,operate,advanced}/` paths, so it does not
clear the automated check on its own, but adopter documentation for this
change does exist.

## [2.5.0] — 2026-08-09

**Stable.** Metadata-only promotion of the exact `v2.5.0-rc.2` functional
candidate (tag object `2bdcbb76716ba84ffa78e792b380b7befdd9340e`, peeled
commit `3874a5b92b006fc867a4cedf359cfc17c6dfbae9`) to stable, published
through the protected maintenance-line workflow
(`agf/rules/workflow/release-via-merge-request-rule.md`'s maintenance-line
exception). Zero functional, schema, documentation-content, migration,
test, dependency, CI, or artifact-layout change from RC2 — only the current-
version surfaces and this changelog changed. Independently re-run against
the identical source tree: 340 passed, 0 failed, 3 ignored (matches RC2's
own precedent exactly).

**Adopter docs: not applicable — exact RC2 functional candidate;
metadata-only promotion.**

### Governance

- Governor decision `20260809T171844Z-governor-reviewer-sq5j.md`'s external
  evaluation disposition stands unchanged: `sjqm` findings 2 (unauthenticated
  coordination `from:`), 3 (mutation-served-as-authentic / tamper-evidence
  gap), and 4 (zero local technical Gate enforcement) remain unresolved and
  are deferred through this stable release to separate post-2.5.0 hardening
  work, per governor decision `rq2n`'s original disposition — a scope/timing
  disposition, not an acceptance of the underlying risk, and not a defect
  introduced by this promotion.
- Plan `docs/plans/20260809T194404Z-v250-final-release-publication.md`,
  Gate 1 approved by decision-record
  `docs/coordination/20260809T200252Z-governor-reviewer-h4wn.md`.

## [2.5.0-rc.2] — 2026-08-04

**RC checkpoint; not final.** `v2.5.0` final ships the same functional
candidate as this release candidate unless a correction forces `rc.3`.

**SemVer classification: Additive minor, documentation-only.** One new
page (`agf/docs/site/acknowledgements.md`) crediting NASA's Lessons
Learned Information System (LLIS) and other named design influences
behind specific shipped AGF surfaces (the learning-record
`supersedes`/`status` fields; the `downstream-impact-rule.md`
classification model). No schema, command, or validator behavior
changes.

### Added

- `agf/docs/site/acknowledgements.md`: new page attributing design
  influences, most notably NASA LLIS, behind specific shipped AGF
  surfaces (`docs/plans/nasa-llis-influence-attribution.md`, Gate 1
  approved 2026-08-03 `ts77`, Gate 2 approved `k2bk` evidenced by
  fact-checker `sppt`'s CLEAR verdict).
- `agf/docs/build/mkdocs.yml`: nav entry for the new acknowledgements
  page.
- `agf/docs/site/docs/concepts/evidence-and-learnings.md`: cross-link
  section referencing the new acknowledgements page.

### Governance

- `sjqm`'s external evaluation (Grok, Lumo) against published
  `v2.5.0-rc.1` classified: findings 2 (unauthenticated coordination
  `from:`), 3 (mutation-served-as-authentic / tamper-evidence gap), and
  4 (zero local technical Gate enforcement) remain unresolved and are
  deferred through stable `v2.5.0` to separate post-2.5.0 hardening
  work, per governor decision `rq2n` — a scope/timing disposition, not
  an acceptance of the underlying risk. None of the three is introduced
  by, or remediable through, this candidate's shipped documentation
  paths.

## [2.5.0-rc.1] — 2026-08-03

**RC checkpoint; not final.** `v2.5.0` final ships the same functional
candidate as this release candidate unless a correction forces `rc.2`.

**SemVer classification: Additive minor.** Two new optional learning-record
schema fields, one new fail-closed adopter-role-authority default, one new
non-`handoff` obligation-tracking command, one rules-document correction
(inert content replaced with new operative guidance), and one new packaged
CI tooling command. Existing command **syntax** is unchanged, and behavior
for an adopter with a valid, present `agf/agents/MANIFEST.yaml` is
unchanged. One existing command's **output** does change: with the
default path and no `MANIFEST.yaml` present, `agov session resume` (#79)
now reports the missing role-authority path and both remedies explicitly,
in place of the prior generic "not found" message. No other existing
command, schema field, or validator behavior changed for an adopter not
opting into the new fields/commands.

### Added

- Learning-record `status` and `supersedes` fields
  (`docs/plans/learning-supersession-status-fields.md`, Gate 1 approved
  2026-08-03, Path B). `agf/schemas/learning.schema.json` gains two new
  optional fields: `status` (`proposed | implemented`) and `supersedes`
  (a stable learning id, set on the *new* superseding entry, never on the
  entry being replaced). `supersedes` is enforced by `validate_learnings`
  (`tooling/rust/crates/agov-validators/src/learnings.rs`): the
  referenced id must exist in the same file and must precede the
  referencing entry's position — both requirements were previously only
  claimed by `agf/schemas/invariants.yaml`'s `learnings.append_only_supersession`
  invariant, not actually implemented. The reverse view (what superseded
  id X) is derived at read time, never stored, avoiding the
  bidirectional-consistency drift AGF-C4 already identified for the
  proposed `related_learnings` field. `status` is optional; existing
  records with no `status` remain valid and are not rewritten.

  `superseded_by` is now marked `x-deprecated: true` in its schema
  description: it required editing already-published entries to
  populate, contradicting append-only operation, and was never enforced
  or actually used across the corpus. It remains schema-legal —
  outright removal (Path A) requires a separate, future governor factual
  determination that the field has zero external production dependents;
  absent that, this is Path B (deprecate, don't remove), Additive.

- Upgrade-safe adopter role authority (issue #79,
  `docs/plans/v250-rc1-contract-core.md` work package 1). The shipped
  release archive no longer contains `agf/agents/MANIFEST.yaml` — a plain
  archive re-extraction over an adopter project could previously overwrite
  an adopter's own divergent roster with the framework's dogfooding
  manifest (reproduced concretely by issue #78: `coder-tester` and
  `build-engineer` silently reactivated, `documentation-writer` and
  `capability-developer` silently dropped). The archive now ships a
  neutral `agf/agents/MANIFEST.example.yaml` template instead, matched by
  the same include glob but excluded by an explicit packaging rule;
  `agov` never reads it as adopter state. `AGOV_MANIFEST_PATH`'s existing
  override precedence is unchanged. With the default path in effect and
  `agf/agents/MANIFEST.yaml` absent, `agov session resume`'s cold-start
  output now states the missing path explicitly and names both remedies
  (copy the example, or set `AGOV_MANIFEST_PATH`) rather than a generic
  "not found" message.

Adopter docs: updated — `ADOPTING.md` §1.3 and
`docs/site/docs/adopt/install-and-layouts.md` now describe copying
`MANIFEST.example.yaml` to `MANIFEST.yaml` as the commissioning step,
replacing the inline roster example.

- `requires-ack` obligation semantics for non-handoff coordination records
  (issue #80, `docs/plans/v250-rc1-contract-core.md` work package 2).
  Previously, `requires-ack: true` was schema-legal on every message kind
  but silently inert except on `handoff` — `find_pending()` only ever
  considered `kind: handoff`, so a `requires-ack: true` `review`, `note`,
  `claim`, `completion`, `decision-record`, `fast-track-waiver`, or
  `lesson-proposal` created no tracked obligation at all (reproduced
  independently twice: the original finding and again against the
  published v2.4.5 archive). Two changes close this:
  - The schema now rejects `kind: ack` records that also set
    `requires-ack: true` — an acknowledgement cannot require its own
    acknowledgement, so the combination is a schema error, not a silent
    no-op.
  - A new `agov coord pending` command lists pending (unresolved)
    `requires-ack: true` records of every kind other than `handoff`
    (unchanged, owned exclusively by `handoff next`/`take`/`complete`)
    and `ack`. Resolution is a direct reference — a record is pending
    until any later record's `references-to` names its file — not
    `handoff`'s transitive descendant-chain resolution. Records published
    before `2026-08-02T22:01:54Z` are grandfathered and never appear, so
    the pre-existing non-handoff `requires-ack: true` records already
    in this project's own coordination channel are not retroactively
    treated as a pending backlog. **Correction (found while resolving
    this merge, 2026-08-03): the cutoff stated here previously read
    `2026-08-02T18:10:18Z`, an earlier draft default the governor
    replaced before merge — see decision-record
    `20260802T220154Z-governor-reviewer-sejb` — but this changelog entry
    was never updated to match. The shipped default
    (`tooling/rust/crates/agov/src/main.rs`) and the rule docs
    (`docs/coordination/README.md`,
    `agf/docs/site/docs/concepts/coordination-protocol.md`) already used
    the corrected value; only this changelog entry had drifted.**
  `handoff next`, `handoff take`, `handoff complete`, and `coord
  graph-index` are unchanged — existing tests for all four pass unmodified.

Adopter docs: updated — `docs/coordination/README.md` and
`agf/docs/site/docs/concepts/coordination-protocol.md` both state the
same permitted-kind matrix, cutoff, and resolution rule.

- Added `agov ci cache-topology-check` (issue #101), a packaged tooling
  command that statically validates a shared CI cache's writer/consumer
  topology against GitLab CI Lint's merged YAML — confirming exactly one
  authorized writer job and that every writer and consumer job references
  the expected cache-policy CI variable. It was built for this project's
  own CI cost problem and carries no documented adopter workflow today,
  but is not adopter-inert: it accepts arbitrary merged CI YAML plus
  configurable cache-key, writer-job, and policy-variable names as input,
  so an adopter with a similar shared-cache topology could point it at
  their own CI configuration. It has no `requires-ack`/obligation
  semantic and does not participate in the coordination protocol.

Adopter docs: not applicable — no dedicated how-to page exists yet; this
entry is this feature's only current adopter-facing documentation.

### Fixed

- `agf/rules/always/governance-file-layout-rule.md` no longer presents
  "Layout A"/"Layout B" as a live adopter choice. AGF v2.0 supports exactly
  one adoption layout — already stated in `agf/ADOPTING.md` and
  `agf/docs/site/docs/adopt/install-and-layouts.md` — and the rule's
  `<!-- CUSTOMIZE -->` template contradicted both. Replaced with a
  single-layout ownership table (adopter-owned vs. upstream-owned paths)
  and corrected a stale `ADOPTING.md` header pointer and a pre-rename
  example path.

**Reclassified from Editorial to Additive** (Codex review, MR !916 on a
related plan, 2026-08-02): the removed block was inert, so the rule
previously asserted no concrete ownership guidance. The table is new
operative content, Additive under `downstream-impact-rule.md`'s own test.
Its `agf/agents/*.yaml` row is explicitly marked as deciding issue #79's
own Gate 1 plan (`docs/plans/v250-rc1-contract-core.md`), not asserted
independently ahead of it.

Adopter docs: not applicable — the table's content already exists in
`ADOPTING.md`'s "Confirm the layout" table (except the #79-pending row,
which is explicitly marked as such); no new adopter doc, template, or
behavior change as a result.

### Validation

- Rust workspace: 340 passed, 0 failed, 3 ignored. Clippy clean with
  warnings denied.
- Strict `mkdocs build`: passed, no broken links.
- Framework-archive overlay tested against three published baselines:
  `v2.4.5`, `v2.4.8` (substituted for unpublished `v2.4.6` per governor
  decision `20260803T183732Z-governor-reviewer-rntt.md`), and `v2.4.11`.

## [2.4.11] — 2026-08-01

Documentation and accessibility patch.

### Added

- Added a visible AI-generation disclosure and descriptive alternative text
  to every public figure embed in `README.md` and the public documentation
  site (33 embeds), and an AI-assistance/human-responsibility acknowledgement
  in `README.md` and the generated release-information page.
- Added a strict-build accessibility hook
  (`agf/docs/build/hooks/figure_accessibility.py`) that enforces the above
  going forward. An adopter who reuses the supplied MkDocs configuration
  unmodified inherits this same check over their own content.
- Added title/description metadata to all 13 public SVG figure pairs.
- Stated the disclosure measure as supporting Article 50 (EU AI Act)
  transparency and accessibility objectives, without claiming legal
  certification. Academic paper PDF figures remain outside this release's
  accessibility scope, as a separately tracked follow-up.

### Fixed

- Corrected the copyable commissioning prompt's stale hard-coded version pin
  (issue #97).
- Corrected public compatibility prose to describe sustained author-operated
  Claude and Codex integration use in Visual Studio Code, attributed
  explicitly to the project author, without asserting independent-adopter or
  universal-compatibility coverage. The contribution page's request for
  independent corroboration is unchanged.
- Fixed the figure renderer's relative-path invocation
  (`agf/docs/figures/render_svg.py`), which previously misrouted the public
  site copy when run via its documented invocation.

### Validation

- Rust workspace: 299 passed, 0 failed, 3 ignored (version-only bump; no
  Rust source changed in this release). Clippy clean with warnings denied.
- Strict `mkdocs build` including the new accessibility hook: passed.

Addresses issue #97. The issue closes only after the published v2.4.11
assets and site pass post-publication verification.

## [2.4.10] — 2026-07-31

### Fixed

- Made repository-local `tooling/agov` forward the full authoritative Rust
  command tree instead of maintaining a handwritten command registry.
- Added deterministic source fingerprinting, compatible-binary selection, one
  verified release rebuild, and fail-closed recovery for behavior-affecting
  tooling changes.
- Added correct cross-worktree diagnostics while leaving standalone published
  binary behavior unchanged.

Validation: Rust workspace 299 passed, 3 ignored; wrapper freshness,
live-surface, and clean tar and zip framework archive acceptance suites passed.

Issue: #92. Closure follows successful verification of the published assets
and a fresh git checkout at the immutable tag.

Adopter docs: updated

## [2.4.9] — 2026-07-31

### Fixed

- Removed the broken `bin/agf` Mix wrapper and dead Elixir parity harness;
  `tooling/agov` remains the single supported repository entry point.
- Updated current contributor and adopter instructions, contracts, governance
  descriptions, and decommission guidance to executable Rust `agov` commands
  without rewriting historical or generic ecosystem evidence.
- Added a fail-closed live-surface scanner and negative self-test covering
  removed Mix, Elixir, legacy Python, and `bin/agf` paths.

Validation: Rust workspace 299 passed, 3 ignored; clean tar and zip framework
archive acceptance suites passed.

Issue: #93. Closure follows successful verification of the published assets.

Adopter docs: updated

## [2.4.8] — 2026-07-30

### Fixed

- Completed release publication after the immutable v2.4.7 tag published its
  OCI image but failed before generic-package upload and GitLab Release
  creation. The strict exclusion check now requires framework and Linux
  tooling archives while checking an optional Windows ZIP only when present.
- Added pre-tag evidence using the real assembled framework and Linux tooling
  preview artifacts, covering both absence and presence of the optional ZIP.

Adopter docs: updated

## [2.4.7] — 2026-07-30

**Publication status:** tagged and partially published. The immutable tag
published OCI image
`registry.gitlab.com/janipaijanen/agents-govern-framework/agov:2.4.7`, but no
generic package or GitLab Release was created. Adopt v2.4.8, which completes
the publication recovery.

### Changed

- Recovered release publication after the v2.4.6 tag pipeline exhausted
  GitLab runner disk space during its mandatory Windows cross-build. Tag
  releases now require framework archives, Linux x86_64/aarch64 Rust tooling,
  checksum and provenance sidecars, and OCI publication without building a
  Windows archive.
- Retained Windows packaging as optional, non-blocking manual preview evidence.
  The selected GitLab Release page's actual assets are the authoritative
  platform inventory.

Adopter docs: updated

## [2.4.6] — 2026-07-30

**Publication status:** tagged, but never published as a GitLab Release. The
tag is immutable; no generic package or OCI release was published for this
version. Adopt v2.4.8, which supersedes both incomplete recovery candidates for
adoption.

### Added

- Added a version-controlled release roadmap page to the documentation site,
  discoverable from the top-level navigation, the documentation overview, and
  the repository README.

### Fixed

- Corrected stale release-version metadata across packaged agent definitions,
  adopter examples, manifests, roadmap guidance, and Rust package metadata.

- Clean-archive validation now runs every Python test shipped in both the
  `.tar.gz` and `.zip` distributions. Tests that depend on repository-only
  fixtures remain in the foundry test suite instead of shipping as broken
  adopter-facing tests.
- Completed the remaining legacy GitLab Pages URL corrections in the
  governance documentation.
- Restored resolution of legacy 20-character Crockford coordination
  references and deduplicated equivalent resolver results.
- Plan validation now distinguishes known inactive advisory roles from
  unknown role names and reports their current status.

Adopter docs: updated

## [2.4.5] — 2026-07-28

### Added

- Added a canonical operations and continuity runbook to GitLab Pages, with
  graceful-checkpoint, abrupt-loss recovery, and complete
  poll-and-coordinate procedures. GitLab and Claude instructions remain
  removable adapters, and `/compact-agf` no longer describes validated
  local-only state as a durable cross-machine handoff.
- Added `agov coord publication-check`, which mechanically verifies that a
  coordination-only commit range is genuinely coordination-only: resolvable
  base ancestry, append-only paths under `docs/coordination/*.md`, no merge
  or imported commits, the protected fast-path commit-subject convention
  enforced across the full range (not just the tip), and schema validation
  of every newly added record. Wired into CI as a lightweight fast path so
  expensive implementation jobs can still skip for genuinely
  coordination-only changes, while the check itself no longer trusts a
  commit title or `[ci skip]` marker. See the new
  `agf/rules/workflow/coordination-publication-rule.md`.
- Added `agov plan projection-check`, which validates the structured
  six-surface rule-projection evidence manifests
  (`docs/reviews/rule-projection/*.yaml`) required for any plan or Gate 2
  review that adds or changes normative guidance in a Pages page or
  runbook: schema shape, changed-file completeness, declared-path
  resolvability, and distribution completeness (every changed file that
  ships via a release-manifest glob must be declared or explicitly marked
  not applicable). See the new
  `agf/rules/workflow/documentation-rule-projection-rule.md`.

### Fixed

- `[ci skip]`/`[skip ci]` on coordination-titled commits is now documented
  and enforced as forbidden, not merely insufficient. These are
  platform-level pipeline-*creation* skips that no `.gitlab-ci.yml` rule
  can override, and previously could silently defeat governance validation
  of a "coordination-only" claim -- the root cause class behind a prior
  incident where a branch titled and described as coordination-only
  carried unreviewed implementation history onto `main`.

### Changed

- This project's GitLab merge settings now enable
  `only_allow_merge_if_pipeline_succeeds` and
  `allow_merge_on_skipped_pipeline: false`, so a merge request with no
  pipeline result, a failed pipeline, or a skipped (zero-job) pipeline
  cannot merge. This is a partial mitigation for revalidating a candidate
  when its target branch advances after the last passing pipeline; full
  mechanical enforcement (via merge-train/merged-results pipelines) is
  confirmed unavailable on this project's current GitLab plan tier and is
  tracked as a governed, documented limitation rather than claimed as
  fully closed.

Adopter docs: updated

## [2.4.4] — 2026-07-26

Documentation-provenance and release-hygiene patch. It adds no schema, role,
Rust CLI/MCP implementation, governance authority, API, or adopter-contract
surface.

### Fixed

- Generated release information now derives the latest immutable `vX.Y.Z`
  tag and peeled source commit from Git while separately reporting the live
  documentation branch and exact build commit.
- Documentation status distinguishes an exact release build from later or
  divergent development documentation without fabricating unavailable tag or
  ancestry data.
- The public release-information page and AI evaluation dossier expose the same
  full release/build provenance tuple in visible, extractable text. The
  accessible header visibly shows the framework version and docs-build commit;
  its accessible label also reports release status.
- Removed the remaining current-looking versioned lifecycle labels and added a
  strict documentation-source check that permits only the bounded form
  `Status (vX.Y, historical)`.

### Changed

- Public page status banners use non-versioned lifecycle wording instead of
  coupling maturity to the documentation version in which a page appeared.
- Added a governed summary of AGF's human gates, named authority, durable
  records, evidence labels, reviewed learning, and repository-native exit
  boundary to the README and public homepage.
- Contributor setup is identified as foundry-specific and placed after the
  adopter-facing README material.

### Validation boundary

- Exact-SHA CI, strict MkDocs generation, real shallow-checkout ancestry
  tests, stale-label adversarial tests, rendered HTML/plain-text extraction,
  live Pages verification, and independent fact-checking passed before the
  release branch was cut.
- Mutable Pages remains development documentation; the tagged source and
  release artifacts are the immutable release authority.

Adopter docs: updated

## [2.4.3] — 2026-07-25

Patch release fixing three adopter-facing documentation defects and closing
an 11-item documentation-quality and information-architecture backlog. It
adds no schema, role, tool, API, CI, or adopter-contract surface.

### Fixed

- Public Mermaid diagrams are now openable, full-size SVG figures; the
  unused client-side Mermaid runtime is removed.
- The Adopter FAQ now recommends installing and using `agov` from day one,
  instead of deferring it until after the first governed change.
- The Adopter FAQ now explains that different authorized people may hold
  Gate 1 and Gate 2 sequentially on the same change.
- Six hardcoded `agov:<version>` OCI-tag literals now carry an editor
  comment naming `agf/VERSION` as the canonical source.
- Two misdirected metadata links now point to their canonical page.
- The governance-loop figure's `learn` node now uses the "Shared concept"
  palette category, correcting a prior "AI action" mislabel.
- Capability-maturity Level 2 naming ("Surface-defined") is now consistent
  across public pages and the internal rule file.
- `palette.md` documents its light-background assumption for seven named
  figure canvases explicitly, without a universal renderer/theme claim.
- The seven existing reference-figure embeds now carry
  `<figure>`/`<figcaption>` semantics.

### Validation boundary

- Exact-head CI, strict site generation, palette/contrast validation,
  desktop and 390px rendered inspection, Chromium accessibility-tree
  inspection, and bounded fact-checking passed. The fact-check disposition was
  ADOPT for all four reviewed claim groups with zero ADAPT or REJECT findings.
- The project had no access to macOS/Safari/VoiceOver or
  Windows/Firefox/NVDA. The human governor explicitly accepted this residual
  assistive-technology coverage gap for Gate 2; those combinations are not
  represented as tested or passing.

Adopter docs: updated

## [2.4.2] — 2026-07-25

Patch release improving the existing public documentation's first-contact
clarity, factual boundaries, and evidence navigation. It adds no schema, role,
tool, API, CI, or adopter-contract surface.

### Fixed

- Corrected public wording about structural versus participant-enforced
  governance, external AI-provider data boundaries, supported command
  maturity, and adoption exit terminology.
- Aligned the adoption-exit figure with the page's Pause, Downgrade, and Remove
  vocabulary.

### Changed

- Replaced the homepage's drifting command inventory with a compact,
  outcome-led summary linked to the canonical tooling reference.
- Added exact-source repository links to the AI evaluation dossier while
  preserving the public Flow 07 corpus and copied Evaluate self-link.
- Added concise identity-and-role byline guidance for assistant-authored GitLab
  activity without presenting shared-account bylines as authentication.
- Revised six evaluator-facing entry pages to lead with ordinary language,
  preserve maturity and evidence limits, and give readers a concrete next
  action.

### Validation boundary

- The implementation passed strict site generation, adopter-documentation and
  release-stat validation, rendered desktop/narrow checks, link verification,
  and exact-text fact-checking. The fact-check disposition was ADOPT for all
  five implementation commits with zero ADAPT or REJECT findings.

Adopter docs: updated

## [2.4.1] — 2026-07-24

Patch release correcting the accessibility and narrow-width presentation of
the public documentation's existing figures and Mermaid diagrams. This closes
the Mermaid remediation deferred from v2.4.0; it adds no schema, role, tool,
API, or adopter-contract surface.

### Fixed

- Added page-specific explanatory prose before all 22 public figure embeds so
  each page communicates the figure's purpose, relationships, and essential
  takeaway without requiring the image to be perceived.
- Added responsive horizontal scrolling for all five public Mermaid diagrams
  at narrow widths while preserving normal desktop presentation and keyboard
  reachability.
- Added accessible names and descriptions to rendered Mermaid SVGs from their
  source definitions, with equivalent surrounding prose retained as the
  primary explanation.

### Validation boundary

- The shared rendering pattern and all five diagrams were reviewed at desktop
  and 390px widths, including zoom/reflow, keyboard reachability, DOM and
  accessibility-tree inspection, and an Orca 46.1/Chromium 150.0.7871.124
  smoke test on Linux Mint 22.3. This is Level 4 Validated evidence, not a claim of formal
  WCAG conformance or universal assistive-technology compatibility.

Adopter docs: updated

## [2.4.0] — 2026-07-23

Minor release covering four independently governed workstreams: a breaking
consumer-facing release-asset rename, a unified release-provenance
contract, MCP-vs-direct-CLI telemetry transport attribution, and a
documentation figure/responsibility-label system. See
`docs/plans/v2.4.0-release.md` for the full umbrella plan and per-workstream
child plans.

### Changed (breaking)

- Release archive and checksum/provenance sidecar filenames now use public
  platform slugs instead of raw Rust target triples:
  `x86_64-unknown-linux-musl` → `linux-x86_64`,
  `aarch64-unknown-linux-musl` → `linux-aarch64`,
  `x86_64-pc-windows-gnu` → `windows-x86_64`. No old-name aliases are
  published. Rust target triples are retained in build configuration and
  provenance metadata; only the public-facing filename changed. Historical
  release assets, tags, changelog entries, and coordination records keep
  their original names. **Phase 0 breaking change** (no deprecation
  cycle): zero known external production dependents on the release-asset
  filename surface as of this release.

### Added

- Unified release-provenance contract: `agf/schemas/release-provenance.schema.json`
  covers framework source archives, platform tooling archives, and OCI
  image evidence with one schema version, closed field allowlist, and
  SHA-256/byte-size linkage to the public artifact. Pre-v2.4 provenance
  records are classified against a separate
  `release-provenance-legacy.schema.json` rather than silently treated as
  v2.4-shaped.
- MCP-vs-direct-CLI telemetry transport attribution: token-ROI and usage
  records now carry a `transport: cli` / `transport: mcp` field
  identifying the trusted local `agov-mcp` adapter as the invocation path,
  without persisting any MCP client, session, or request identifier.
  Existing JSONL readers remain compatible; the field is optional.
- Documentation figure and responsibility-label system: one semantic
  palette (orange for human decisions, blue for AI-executed work, green
  for automated checks/validated outcomes, outlined neutral for shared
  concepts, grey for reference/deferred content) applied across public
  documentation figures, inline Mermaid diagrams, and academic SVG/PDF
  figures. Added restrained responsibility-labelled text blocks (Human
  decision / AI action / Automated check / Shared concept) to the
  first-governed-change walkthrough, used only where responsibility
  changes or ambiguity is likely.
- Removed the full-size "AGF in a Nutshell" flyer from the documentation
  overview reading flow; it remains available as a labelled standalone
  downloadable orientation artifact.

## [2.3.1] — 2026-07-21

This patch recovers the public release truth after v2.3.0 was published with
an outdated repository-root changelog. The immutable v2.3.0 tag and OCI tag
remain unchanged; the corrected source history is published under v2.3.1.

### Added

- Added two proposed, single-adopter learning records from the v2.3.0 release:
  release-critical CI needs host preflight from the consumer layout, and
  release completion needs an enumerated public-truth surface. Their
  recommendations remain proposed target states; this patch does not claim
  the generalized validators are implemented.

### Fixed

- GitLab Release creation now links every framework and tooling archive,
  individual checksum, and provenance sidecar directly from the Release page.
  It verifies the exact mandatory 14-file set before upload, with a CI
  self-test proving that a missing provenance sidecar blocks publication and
  names the omitted file.
- Reconciled adopter-facing release history with the repaired repository-level
  v2.3.0 history. Earlier terminal completion records are preserved as
  immutable historical evidence and explicitly superseded by the v2.3.1
  recovery trail.

## [2.3.0] — 2026-07-21

**2.2.6 was prepared and merged on 2026-06-26 but was not published as a
tagged release. Its changes are incorporated into 2.3.0** (see the annotated
`[2.2.6]` entry below). The last published tag before this release is
`v2.2.5`; this entry covers the full diff from `v2.2.5` to `v2.3.0`,
including the 2.2.6 content.

### Added

- Added coordination write primitives to `agov`: `agov coord post` writes and
  validates a coordination message in one step (`--from`, `--to`, `--kind`,
  `--topic`, `--ref`, `--artifact`, `--task`, `--model`), and `agov coord
  brief`/`agov coord list`/`agov coord status` generate skeletons, list
  messages since an id/timestamp, and summarize channel/role activity. Agents
  no longer need to hand-construct coordination front matter.
- Added an `agov` OCI image release job: `.gitlab-ci.yml` builds and publishes
  a container image on tag pipelines from `tooling/rust/Containerfile.agov`
  via `tooling/rust/scripts/agov-oci-image.sh`, with a version/runtime smoke
  test and a Trivy vulnerability scan gating on fixed HIGH/CRITICAL findings.
  Cross-compiled release binaries now carry correct embedded build
  provenance (`tooling/rust/Cross.toml` passes `AGOV_BUILD_COMMIT` /
  `AGOV_BUILD_DIRTY` through the cross-build container, which previously
  silently fell back to `unknown`/`dirty: true`); the scan path's
  image-reference and format bugs are fixed and the container base is
  digest-pinned to a patched Alpine base.
- Added `agov-mcp`, a local stdio Model Context Protocol server exposing
  `agov` workflow commands as MCP tools. Built into the release archive by
  CI. Documented at `agf/docs/site/docs/adopt/mcp-server.md`, status
  **Level 4 Validated** (not yet Level 5 Deployable — see the known
  path-contract limitation on that page and the tracking plan at
  `docs/plans/mcp-coordination-dir-contract-fix.md`).
- Added an adopter applicability catalog (`agf/docs/site/docs/adopt/applicability-catalog.md`),
  classifying a first representative slice of 8 governed artifacts (of 58
  total) with a per-artifact activation condition and removal consequence.
  New optional schema fields: `activation_condition`, `safe_to_remove_when`,
  `removal_forfeits`, `depends_on` on rules (`agf/schemas/rule.schema.json`);
  `activation_condition`, `deactivation.local_deactivation`,
  `deactivation.definition_removal` on agents (`agf/schemas/agent.schema.json`).
  All new fields are optional; no existing rule or agent file becomes
  invalid. Also corrects an internal inconsistency in `framework.md` §7 and
  `ADOPTING.md`'s "signals to grow" table, which had described Build
  Engineer, Contribution Auditor, and InfoSec Sentinel as unconditionally
  "always active" in one place while stating a specific activation
  condition elsewhere in the same documents — both properties (library-core
  status and activation condition) are now stated explicitly rather than
  conflated. The remaining 50 governed artifacts are not yet classified;
  tracked as an explicit follow-up, not silently deferred (see
  `docs/coordination/20260721T063606Z-coder-tester-pjms.md`). Contribution
  Auditor's `activation_condition` reads "first contribution to the
  framework or an adopter project" (settled wording; see
  `docs/coordination/20260721T071957Z-governor-reviewer-rts3.md`).
- Added `deprecation-policy-rule`, defining a two-phase deprecation policy:
  Phase 0 (pre-customer, current state) permits disclosed breaking changes
  with no deprecation cycle; Phase 1 (real external adopters) requires a
  minimum support window (2 minor releases or 60 calendar days by default,
  whichever is longer, modeled on Kubernetes' published API deprecation
  policy). Motivated by the `agov-mcp` `dir` field rename
  (`docs/plans/mcp-coordination-dir-contract-fix.md`), which is a Phase 0
  hard rename under this rule rather than a repeat of the coordination
  schema's never-actually-sunset `seq` field pattern.
- Added `agent-improvement-proposal-rule`, a SHOULD-strength norm that lets
  agents surface workflow friction and improvement ideas without expanding the
  active MR or taking approval authority.
- `agov coord brief` now accepts `--model <name>` and uses `AGOV_MODEL` as
  the default model provenance when set, so generated coordination front matter
  can record the assistant/runtime used without manual editing.
- `agov validate adopter-docs` now checks release-range diffs for
  adopter-relevant framework/tooling changes and requires downstream-facing
  docs/templates or an explicit changelog waiver.
- Added a new Concepts page, "Philosophical lenses on AI governance"
  (`agf/docs/site/docs/concepts/philosophical-lenses.md`), presenting six
  consultative governance-critique lenses (legibility, accountability
  sinks, automation bias, Goodhart's law, psychological safety vs.
  surveillance, proceduralism vs. outcome-based regulation) as review
  prompts applicable to any governance design, explicitly not new AGF
  rules or guarantees. A second set of six learning-focused lenses (OODA,
  PDCA/PDSA, single- vs. double-loop learning, espoused theory vs.
  theory-in-use, Cynefin, SECI/*ba*) is Gate 1 approved
  (`docs/plans/philosophical-lenses-learning-additions.md`) but explicitly
  **not** implemented in this release; scheduled as a post-2.3.0 follow-up.

### Changed

- The **Learning loop and evidence** concept page now identifies its
  double-loop learning, PDSA, CAST, and ISO/IEC 42001 foundations, explicitly
  labels them as conceptual prior art rather than empirical validation, and
  shows causal analysis and governed approval before implementation in the
  learning-loop figure.
- Public landing, adoption, evaluation, dossier, and installation guidance now
  distinguishes the complete shipped governance library from the smaller role
  and rule subset a project activates. The proposed reduced starter product is
  labelled Level 1 Planned rather than implied to ship.
- Downstream-impact and release rules now require an `Adopter docs parity`
  note for adopter-relevant work before feature-ready or release-ready claims.
- Adopter-facing documentation now explains the docs parity gate, the
  enterprise/multi-repository boundary, reviewer evidence sources, schema
  claim hygiene, and the current plan-to-diff / binding-traceability limits.
- External-review triage records now label the OpenAI web-based AI assessment
  accurately and route fact-checking / advisory claims through durable
  coordination notes.
- Homepage and public documentation now consistently use "AI assistant" (with
  bare "assistant" as shorthand within a section) for the tool a person runs,
  reserving "agent" for a governed role under this framework.

### Learning

- Added a proposed learning record for using formal advisory roles as
  auditable evidence producers during external-review claim assessment.
- Added `20260721T073035Z-bt1s`: a mixed push whose HEAD commit is
  `coordination:`-titled can skip CI for the whole push under
  `.gitlab-ci.yml`'s title-only workflow rule, silently including an
  untested content commit. Structural fix (checking the full commit range)
  is deferred pending a `gate-self-test-rule` negative test; the immediate
  discipline is committing coordination-only changes separately from content
  changes.
- Added `20260721T003902Z-4j3j`: merged-plan Gate 1 metadata drift and a
  same-role self-ack are not governor-reviewer authority; corrections need an
  actual `governor-reviewer` coordination record.

Adopter docs: updated

## [2.2.6] — 2026-06-26 (prepared and merged; not tagged or published)

This patch content was prepared and merged to `main` at commit
`320405f38457f4890ba16fc25f952b4a8c6d1307` but was never tagged or
published as a release. It is not a SemVer prerelease. Its changes are
incorporated into [2.3.0](#230--2026-07-21) above; do not look for a
`v2.2.6` tag. This patch content publishes the external-review hardening
work that followed the v2.2.5 AI evaluation dossier rollout.

### Changed

- The AI evaluation dossier now includes the full generated docs corpus with
  dossier-local links, source-flow markers, and subsection-preserving anchors
  so external AI reviewers can evaluate a single page without losing source
  structure.
- Public docs clarify shipped coordination tooling versus the remaining
  semantic-validation gap, including validator-surface and CI policy-profile
  summaries.
- Adoption guidance now includes privacy/labor/provider-boundary checks,
  enterprise boundary positioning, adoption pattern examples, governor
  delegation guidance, evidence-quality labels, anti-ceremony metrics,
  customization-zone summaries, and conformance fixture families.
- Homepage status wording now points evaluators to the generated framework
  content version instead of stale documentation-line labels.

## [2.2.5] — 2026-06-25

This patch release completes the historical Python tooling removal sequence
after the Rust `agov` path became the only supported AGF v2 operator tooling.
It also ships follow-up documentation and schema alignment for deferred or
retired Python-only surfaces.

### Changed

- Contract and schema documentation now mark v1.x Python-only tooling surfaces
  as retired or deferred before historical Python code removal. Binding
  validation is explicitly deferred in current Rust `agov`;
  `agov-validate-acronyms`, `agov-bump`, `agov --capabilities`, and
  `agov --contract-changelog` are retired; `agov-health` remains deferred.

### Removed

- Historical agents-govern 1.x Python package sources and package metadata are
  removed from the repository. Current AGF v2 operation uses Rust `agov`.

## [2.2.4] — 2026-06-24

### Removed

- `tooling/elixir/` — the Elixir reference implementation is removed.
  The Rust `agov` binary is the sole tooling implementation going forward.
- `elixir:test` and `rust:tooling:parity` CI jobs removed; the parity job
  had no purpose without the Elixir implementation.

### Changed

- `agf/docs/contracts/tooling-conformance.md`: Rust `agov` is now the
  reference implementation; Elixir reference row removed from the
  implementation registry.
- `agf/ADOPTING.md`: removed Elixir/Mix forward-references and the
  Elixir licensing clause; updated validator instructions to use `agov`.
- `agf/dist/README.md`: dist commands updated to `cargo run -p agov`.

## [2.2.3] — 2026-06-24

### Changed

- `evaluate.md`: version and commit hash are now mentioned explicitly in the
  identity check table, pointing reviewers to the page header badge.
- `evaluate.md`: `agf/schemas/` in the technical reading list is now a
  clickable link to the GitLab repository browser.

### CI

- `rust:tooling:parity` is now manual / on-demand only (`when: manual`); it
  no longer runs automatically on every MR touching `tooling/rust/`. The job
  caches its Rust build artifacts via a dedicated `rust-parity-*` cache key so
  repeated manual runs on the same branch are faster.
- `rust:tooling:deny`: dropped `--force` from `cargo install cargo-deny`; the
  binary is now served from the `.cargo/bin` cache on subsequent runs.
- `.rust-tooling-package`: `cross` is now installed from crates.io
  (`--locked --version "^0.2"`) instead of from git source, saving 2–3 minutes
  per release build.
- `.rust-base` cache now declares `fallback_keys: [rust-main]` so new branches
  warm from the main branch cache instead of starting cold.

## [2.2.2] — 2026-06-24

### Fixed

- All Rust CI jobs now set `AGOV_BUILD_COMMIT`/`AGOV_BUILD_DIRTY` via the
  `.rust-base` template `before_script`. Previously only the dist/release job
  set these; test/clippy/deny/parity jobs fell back to git at compile time,
  and a warm cache could serve a stale binary with `(unknown dirty)` provenance.

## [2.2.1] — 2026-06-24

### Fixed

- Rust workspace version (`tooling/rust/Cargo.toml`) was `0.1.0` and had not
  been synced to the AGF framework version. `agov --version` now reports
  `2.2.1` matching `agf/VERSION`.

## [2.2.0] — 2026-06-24

### Added

- **Readable coordination IDs** — `agov coord brief` now defaults to the
  readable `YYYYMMDDThhmmssZ-<role-slug>-<4char>` format. The explicit
  `--id-format readable` flag remains valid. ULID format is still accepted
  for legacy compatibility.

- **Version provenance** — `agov --version` now includes the git commit SHA
  and dirty-flag at build time. `agov version --format json` emits a
  machine-readable provenance record. Cross-compilation CI jobs embed
  `AGOV_BUILD_COMMIT`/`AGOV_BUILD_DIRTY` to produce correct provenance in
  release archives.

- **`agov coord graph-index`** — read-only command that emits a JSON graph
  of the coordination channel: nodes, causal edges (`references-to`),
  pending handoffs, and invalid files.

- **`agf/templates/AGENTS.md`** — adopter template for the project root
  `AGENTS.md` loader (Codex / GitHub Copilot). Includes cold-start commands,
  an AGF version reference comment, rule orientation, coordination channel
  guidance, and CUSTOMIZE markers. Copy to project root after adoption.

- **`agov.validate.assistant_loader`** — new advisory in the umbrella
  `agov validate`. Warns when no loader file exists (`AGENTS.md` or
  `.claude/CLAUDE.md`) and when a loader's `<!-- AGF: X.Y.Z -->` version
  comment is stale relative to `agf/VERSION`. Advisory only; never blocks.

### Changed

- `agf/ADOPTING.md` §1.4 step 2 now names specific starter templates per
  assistant surface: Codex → `agf/templates/AGENTS.md`; Claude Code →
  `agf/project-specific-examples/python-flask/CLAUDE.md`.

- `ADOPTING.md` §2.4 prescribes the `YYYYMMDD-` filename prefix convention
  for ADR files.

- Parity test exclusion list extended to cover `assistant_loader` (Rust-only,
  parallel to the existing `assistant_drift` exclusion).

### Documentation

- RC-1 readiness audit completed: operator path verified, all v1.x legacy
  material labelled, three-layer contract documented for `agov coord
  graph-index`.

- Adopter learnings from amber-register (issues #9 and #10) incorporated into
  `agf/learnings/foundry.yaml`.

## [2.1.6] — 2026-06-22

### Fixed

- `agov validate` now reports advisory drift when copied `.claude/rules/` or
  `.claude/skills/` trees differ from canonical `agf/rules/` or `agf/skills/`
  content, including summarized output for large drift sets and remediation
  guidance that preserves `agov` as optional tooling.
- Adoption and upgrade documentation now explain the loader migration path for
  existing copied assistant-rule adopters: keep `.claude` files as small
  loaders, remove copied AGF content, and move intentional project-specific
  assistant rules outside copied AGF trees.
- `agov validate` now includes a coverage note clarifying that plan and
  coordination records are validated separately with `agov plan validate` and
  `agov coord validate`, or by equivalent manual review.

## [2.1.5] — 2026-06-22

### Added

- GitLab tag release pipelines now publish a Windows x64 Rust `agov` archive
  (`agov-X.Y.Z-x86_64-pc-windows-gnu.zip`) alongside the existing Linux
  tooling archives, with checksum and provenance files.

### Changed

- Release creation now uploads and links `agov` zip archives as first-class
  GitLab release assets.
- The release-artifacts reference page now documents the Windows zip package
  and distinguishes Linux `.tar.gz` tooling archives from Windows `.zip`
  tooling archives.

## [2.1.4] — 2026-06-22

### Changed

- Framework source release archives are now built by the Rust `agov` CLI via
  `agov dist validate`, `agov dist build`, and `agov dist check-tag`.
- GitLab `agf:dist:*` CI jobs now use the Rust tooling path for manifest
  validation, preview archives, and tag-only release archives.
- The release-artifacts reference page now describes framework source archives
  as the canonical versioned framework snapshot and Rust `agov` archives as the
  execution surface.

### Fixed

- Release documentation no longer presents Mix-backed source packaging as the
  current release implementation.

## [2.1.3] — 2026-06-22

### Changed

- Public README and documentation homepage now provide a three-minute entry
  path: problem, mechanism, cost, exit path, and one next step before deeper
  tooling and evidence material.
- The external evaluation page now treats the homepage as the first-scan entry
  and separates first-scan, quick, and thorough assessment tracks.
- CI now rebuilds the public documentation site when release metadata changes
  and runs Rust `agov validate` as release-facing framework validation.

### Fixed

- Release-tag CI coverage now emphasizes Rust tooling checks and no longer runs
  the Elixir test suite for release tags or CI-only changes.

## [2.1.2] — 2026-06-22

### Added

- Rust `agov` tooling release archives for supported targets, with GitLab
  release links and package-registry upload wiring.
- Public documentation for the first-run assistant loader, tooling and token
  savings, coordination protocol, released-artifact references, and
  principal-bound AI delegation as a v2.x docs-only convention.
- Readable coordination IDs and supporting `agov coord brief` guidance for new
  AI-authored coordination messages.
- Dependency CVE scanning for Rust tooling with `cargo-deny`.

### Changed

- Public documentation now presents the v2.1 line as released, clarifies the
  canonical GitLab Pages URL, and incorporates external-review follow-up from
  the Lumo review path.
- Coordination, learning, and release guidance now emphasize timestamp-based
  identifiers, session-resume hygiene, and the release-via-MR tag workflow.
- Adoption guidance now better separates advisory governance gates from
  structural CI/tooling gates and gives more realistic governor-effort ranges.

### Fixed

- Session-resume invalid-file summaries and learning ID handling were repaired
  across Elixir and Rust tooling paths.
- Documentation build and release metadata behavior were tightened, including
  release-info git environment fallbacks and stricter public-doc wording.

### Note

- `2.1.1` was merged as a release bump but no `v2.1.1` GitLab release was
  created. `2.1.2` is the next immutable release and includes the merged
  `2.1.1` changelog entry plus subsequent main-branch work.

## [2.1.1] — 2026-06-18

### Changed

- `release-via-merge-request-rule.md`: steps 7–8 (tag + push) now explicitly
  require no additional confirmation — the merged release MR is the authorization.
- Public README and documentation pages now describe v2.1 as the released
  documentation line while keeping evidence-limited rule status visible where
  it applies.

## [2.1.0] — 2026-06-18

### Added

- Coordination validator now emits an advisory WARN when a message contains
  the deprecated `seq` field. Detection is schema-driven via `x-deprecated: true`
  on the `seq` property — any future deprecated field needs only a schema change.
- `agf/schemas/coordination-message.schema.json`: `id` (ULID) is now required;
  `seq` marked deprecated. Conflict-free `COORD-YYYYMMDD-XXX` filename convention
  adopted for all coordination templates and documentation (issue #2).
- GitLab issue templates (Bug Report, External Review, Learning, Rule Proposal)
  and MR templates (Default, Learning Record, Rule or Agent) adopted from v1.

## [2.0.4] — 2026-06-18

### Fixed

- CI: `release:create` job failed with `curl: not found` on Alpine sh.
  The `release-cli:latest` image does not include `curl`; added
  `apk add --no-cache curl` as the first script step.

## [2.0.3] — 2026-06-18

### Fixed

- CI: `release:create` job failed with `syntax error: unexpected redirection`
  on Alpine sh. Removed `2>/dev/null` from the `for`-`in` glob list; the
  `[ -f "$f" ] || continue` guard inside the loop already handles unmatched
  globs.

## [2.0.2] — 2026-06-18

### Fixed

- Test: `DistManifestTest` integration test hardcoded `"2.0.0"` as the
  expected version, causing `elixir:test` to fail on every version bump.
  Fixed by reading `agf/VERSION` dynamically at test time.

## [2.0.1] — 2026-06-17

### Fixed

- CI: add `release:create` job to upload release archives to the GitLab generic
  package registry and create a GitLab release entry with download links.
  `v2.0.0` shipped without this job; archives were only accessible as CI job
  artifacts.

## [2.0.0] — 2026-06-16

First release of AGF v2. Restructures the framework into the canonical
`agf/` adopter-facing directory and ships a GitLab CI/CD pipeline with
framework source archives, a linux-x86_64 tooling binary, and a combined
release bundle.

### Added — CI/CD and release artifacts

- Root `.gitlab-ci.yml` with six jobs:
  - `elixir:test` — compile with `--warnings-as-errors` and run `mix test`
    on every MR, branch, and tag.
  - `agf:dist:validate` — validate the manifest-driven release manifest.
  - `agf:dist:preview` — build framework source archives as review artifacts
    on MRs and the default branch (1-week expiry).
  - `agf:dist:release` — build `agf-2.0.0.{tar.gz,zip}` with SHA-256
    checksums and provenance JSON on tag pipelines only; tag/VERSION
    mismatch blocks production before any upload.
  - `agov:tooling:preview` / `agov:tooling:release` — build the linux-x86_64
    Mix release archive (`agov-2.0.0-linux-x86_64-otp-29.tar.gz`) with
    SHA-256 and provenance. Release job runs on tags only; guarded by
    `mix agov.dist.check_tag`.
  - `agf:bundle:preview` / `agf:bundle:release` — assemble
    `agf-2.0.0-bundle.tar.gz` wrapping the framework archives and tooling
    binary with `SHA256SUMS` and `bundle-provenance.json`. Release job
    runs on tags only, after both `agf:dist:release` and
    `agov:tooling:release`.
- `mix agov.tooling.build` — builds the ERTS-bundled linux-x86_64 Mix
  release; derives OTP major version at build time; checks for forbidden
  paths (`agf/`, `.env`, `.git/`, `tmp/`, `agents-govern/`) before
  archiving.
- `mix agov.bundle.build` — assembles the bundle archive from framework
  archives already in `dist/`; fails cleanly if tooling archive is missing.
- Tooling binary provenance includes `otp_version`, `elixir_version`,
  `target_platform`, and `packaging` fields.

### Added — framework content

- `docs/site/` — 16-page documentation site (MkDocs, strict mode):
  marketing lane (landing, value proposition, use cases with explicit
  non-use cases) and a progressive learning path
  (start → adopt → concepts → operate → advanced → evolution), including
  pre-adoption `downstream-impact` and `exit-and-decommission` pages.
- `docs/adopt/decommission.md` — full downgrade/pause/remove procedure
  with verification steps.
- `docs/reference/quick-reference.md` — one-page roles, pipeline,
  coordination kinds, and key rules summary.
- `docs/academic/governing-the-governors.tex` — working paper describing
  the framework's design science foundations, formative evaluation, and
  design goals.
- New rules in `agf/rules/`: `tool-authority-boundary-rule`,
  `active-branch-preflight-rule`, `answer-only-rule`,
  `coordination-sequence-guard-rule`, `coordination-message-rule`,
  `deputy-governor-rule`, `downstream-impact-rule`, `gate-self-test-rule`,
  `visible-artifact-verification-rule`, `file-generator-rule`,
  `boundary-shape-testing-rule`, `concept-modelling-rule`,
  `ci-trigger-scoping-rule`, `migration-generation-rule`,
  `merge-request-description-rule`, `release-via-merge-request-rule`,
  `merge-request-workflow-rule`.
- New agents: `academic-peer-reviewer`, `academic-visual-reviewer`,
  `build-engineer`, `capability-developer`, `contribution-auditor`,
  `deployment-engineer`, `documentation-writer`, `fact-checker`.
- New skills: `capability-review`, `contribution-audit`, `health`,
  `infosec-screen`, `learning`, `release`, `upgrade`.

### Changed (structure only — no rule or agent behaviour changes)

- Repository root reduced to essential files; governance documents moved
  under `docs/governance/`.
- Contract documents consolidated under `docs/contracts/`.
- Adoption material consolidated under `docs/adopt/`.
- `framework_version` in all agent definitions and `contract_version` in
  `framework.yaml` set to `2.0.0`.

### Deferred (not in 2.0.0 scope)

- macOS, Windows, and ARM tooling binaries.
- Package registry publishing, release signing, SBOM.
- Design system, semantic colour palette, and Graphviz figure sources.
