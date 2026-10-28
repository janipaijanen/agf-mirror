# Changelog

All notable repository-level changes to AGF-NG are recorded here.

The canonical adopter-facing AGF v2 changelog lives at
[`agf/CHANGELOG.md`](agf/CHANGELOG.md). The bootstrap reference changelog for
agents-govern 1.x lives at [`agents-govern/CHANGELOG.md`](agents-govern/CHANGELOG.md).

## [Unreleased]

Adopter-facing entries for the next release, with their `Adopter docs:` lines, are
in [`agf/CHANGELOG.md`](agf/CHANGELOG.md) under `[Unreleased]`; this section lists
repository-level changes only.

## [2.12.0] - 2026-10-28

**Documentation-focused minor release.** Standard current-main release from
reviewed construction base `fc98b4d1`. The canonical adopter changelog records
the tooling-coexistence guidance, homepage modernization, and documentation
corrections. This release adds no repository-operation change.

## [2.11.0] - 2026-10-26

**Minor release.** Standard-path release from reviewed `main` at
`24d098a1`. Full Rust workspace suite: 739 passed, 0 failed, 3 intentionally
ignored network/Podman tests; each framework archive passed all 28 shipped
tests. The adopter-facing release adds stricter MCP argument handling,
coordination lifecycle and publication controls, plan collision enforcement,
the optional Improvement Propeller skill, governance modes, and documentation
currency fixes. Repository operations also retire the unused Elixir and
Windows preview paths and route OCI work to the live Podman runner pool.

- Removed the retired Elixir release-payload guard script, its CI invocations and
  the `release:elixir-exclusion-selftest` job, and the Elixir toolchain pins from
  `mise.toml`/`mise.lock`. The generic forbidden-file mechanism in the
  release manifest is unchanged and still tested.
- Route two OCI pull/inspect CI jobs to the live Podman runner pool instead of the unavailable DinD pool.
- Retired the optional Windows `agov`/`agov-mcp` preview build and packaging
  path. Automation using that preview must move to Linux x86_64 or the OCI
  image; current CI and release tooling no longer produce Windows ZIPs.

## [2.10.1] - 2026-09-12

**Patch release, MINOR-shaped content.** Standard-path release: ships
all reviewed work merged to `main` since v2.10.0, not only the
customer-driven extension this entry leads with. The release content
is frozen from reviewed `main` at `90bb57a2`. Full workspace test
suite: 593 passed, 0 failed, 3 intentionally ignored network/podman
tests.

**Version-number note:** the customer-driven addition below is
additive/backward-compatible (new opt-in config block, new
subcommands, no existing behavior changed) and would conventionally be
a MINOR release on its own. This release ships as a PATCH number
(v2.10.1, not v2.11.0) at explicit governor direction, to keep the
customer-driven work a small, fast, clearly-scoped drop decoupled from
the separately paused v2.11/RUW work. See
`docs/plans/20260912T155648Z-v2101-ruw-generic-artifact-validation.md`
for the full record of that decision. The other changes below shipped
independently on `main` before this release cut and are included
because a standard-path release ships everything reviewed and merged
since the preceding release, not only its named headline feature.

### Added

- **Customer-driven addition — generic artifact/graph validation
  extension.** Developed for a prospective AGF adopter's multi-assistant
  project.

- `agov-config.yaml` gains an optional `artifact_sets` block: adopters
  can declare their own project-specific artifact types (name, path
  globs, JSON Schema, ID/reference JSON Pointers, uniqueness scope)
  without AGF hardcoding any adopter-specific paths or semantics.
  Absent this block, behavior is unchanged for every existing adopter.
  Adopter docs: updated (`agf/docs/site/reference/artifact-registry.md`).
- `agov artifact validate` and `agov artifact index` validate configured
  artifact sets against their registered schemas and produce a
  deterministic, byte-repeatable index (`id`, `kind`, `key`, `version`,
  path, JSON Pointer, digest, status). Enforces structural graph
  properties only: reference existence, dependency-graph acyclicity,
  and global ID uniqueness across all configured sets — no gate-
  ordering, reviewer-authority, or other governance-judgment semantics
  are inferred by this validator. Also exposed through `agov-mcp`.
- Filesystem containment for the new surface: every path component is
  checked for symlinks (not just the final component, and independently
  during glob expansion), paths are canonicalized and required to stay
  within the repository root, external schema `$ref` values are
  rejected recursively across the whole schema document, and reads are
  capped at 16 MiB with a bounded glob-traversal state limit. Reviewed
  by build-engineer and by an independent, non-author security review;
  see `docs/coordination/20260912T171830Z-build-engineer-n2ht.md` and
  `docs/coordination/20260912T173113Z-independent-implementation-reviewer-8kje.md`.
  Two accepted advisory limitations (a disclosed, out-of-threat-model
  TOCTOU gap under an actively hostile concurrent filesystem mutator,
  and pattern-count amplification bounded only by config file size) are
  recorded in those reviews, not fixed — both require an adversarial
  actor already outside this feature's stated "stable local working
  tree, opt-in, maintainer-authored config" threat model.
- **Safe multi-record coordination publication (CP-B-1).** `agov coord
  publication-check` now supports a candidate range that adds more than
  one coordination record, with byte-identical copy rejection checked
  against the union of the merge-base inventory, the pinned current
  target tip, and earlier candidate additions in the same range —
  closing a target-tip copy blind spot found in review. Target and
  candidate refs are each resolved once to an immutable commit before
  any ancestry, range, or content read, and every decisive Git read now
  fails closed instead of silently defaulting to an empty result on
  error. See `docs/plans/20260909T200622Z-cpb1-copied-record-contract-clarification.md`
  and its governing decisions for the full record.
- **Two CI reliability fixes**, unrelated to each other and to the
  customer-driven addition above: `commercial-corpus:validate` now sets
  `GIT_DEPTH: "0"` so its merge-base comparison does not fail on
  branches more than ~20 commits past their target; `rust:tooling:deny`
  now clears `$CARGO_HOME/advisory-dbs` before running `cargo deny
  check`, mitigating an observed crash where cargo-deny misparsed a
  git error message as a timestamp. The precise root cause of the
  latter is disclosed as unproven in
  `docs/plans/20260912T112252Z-cargo-deny-advisory-db-clean-clone-gate1.md`
  — the fix is an accepted, low-cost mitigation for the observed
  failure shape, not a demonstrated fix for a confirmed cause.
- **Improvement Propeller Stage 4 activated.** The bounded weekly
  opportunity/governance-friction scan, shipped planned/inactive in
  v2.10.0, is now active at Level 4 Validated per governor decision
  `20260909T043713Z-governor-reviewer-djh1`. Level 5 still requires a
  genuine unattended scheduled wake, not yet recorded.
- Coordination-channel tooling also gained advisory branch-handoff
  discovery, advisory plan-collision checks, and cross-session handoff
  addressing/reply-evidence annotations, merged to `main` since v2.10.0
  as part of ongoing governance-tooling work. These are internal
  coordination-workflow aids for AGF development sessions, not part of
  the customer-driven addition above; see the coordination channel
  (`docs/coordination/`) for the full record of each.

### Downstream impact

**ADDITIVE** for the customer-driven artifact/graph validation
extension: no existing coordination record schema or command behavior
changed by it, and adopters who do not add an `artifact_sets` block see
no behavior change from it at all. The two CI fixes above are
**PREVENTIVE** (foundry's own pipeline reliability only; adopters do
not consume this project's `.gitlab-ci.yml`). CP-B-1 and the
coordination-tooling items are **ADDITIVE** to the coordination-channel
tooling surface; no existing coordination record schema was changed.

## [2.10.0] - 2026-09-08

**Minor release.** Delivers autonomous AGF development operation through the
universal polling contract, the bounded Improvement Propeller, restored token
ROI estimates, and a fail-closed four-namespace release guard. The release
content is frozen from reviewed `main` at `fb118b89`. Reversible preflight
passed 506 Rust tests with 3 intentionally ignored network/podman tests; each
built framework archive also passed its 28 shipped tests. Adopter docs: updated.

### Added

- Universal polling and autonomous continuation now applies to every active
  development role: sessions discover the complete coordination, merge-request,
  issue, and awaited-state surface, then act only within their role, authority,
  approved scope, current gate, and collision-safe claim. The same rule adds
  bounded spare-work decomposition and the proposal-only capability-review
  trigger after 10 new valid learnings, with escalation by 15. Adopter docs:
  updated. **Downstream impact: BREAKING** for direct adopters because this adds
  a new MUST-level assistant duty, while the framework's backward-compatible
  functionality remains SemVer MINOR-shaped. Migration: record the last
  completed capability-review learning cursor and count, project the universal
  rule through assistant startup surfaces, configure a real wake schedule where
  supported, and document activation/resume polling where it is not.
- The Improvement Propeller role, canonical rule, documentation, and projection
  evidence provide a bounded weekly opportunity and governance-friction scan.
  It ships planned/inactive at **Level 4 Validated**; post-release issue #131
  owns activation, a genuine scheduler wake, and any later Level 5 maturity
  claim, while #132 owns the subsequent skill export. It cannot assign,
  prioritize, approve, implement, merge, or release work. Adopter docs: updated.
- `agov release namespace-check` and the mandatory
  `release:namespace-check` merge-request job verify authoritative absence of
  the candidate Git tag, GitLab Release, generic-package version, and derived
  OCI reference. Collection access, full package-status pagination, project
  identity, and target-head ambiguity all fail closed. The unchanged mandatory
  operator preflight separately verifies the exact protected-tag policy.
  Adopter docs: updated. **Downstream impact: BREAKING** for direct adopters
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
- The foundry learning corpus now records the end-to-end MCP-readiness failure:
  installing or building `agov-mcp` is not sufficient unless the assistant
  runtime actually loads the structured tools and a smoke test proves them
  callable. This is evidence for follow-up issue #116, not a claim that every
  runtime exposes MCP today.

### Fixed

- Restored automatic saved-token estimates for the documented `cold-start`,
  `session-resume:<role>`, and `next-handoff:<role>` workflows. CLI and MCP
  invocations share the same baseline arithmetic, and historical telemetry
  remains unchanged.

## [2.9.0] - 2026-09-06

**Minor release.** Ships the reviewed `main` line through `cab3c0d6`, adding
mechanical protection for changes to governance gates and strengthening the
release path against skipped checks, ambiguous changed-file ranges, OCI
reference drift, and vulnerabilities discovered only after tagging. Adopter
docs: updated.

### Added

- A versioned gate-registry schema, neutral example, and `agov gate
  change-check --base-ref <ref>` command. Registered gate surfaces now require
  a paired test-path change, gate IDs cannot silently disappear, and explicit
  retirement preserves the audit record.
- Merge-request-stage construction and Trivy scanning of the release OCI image,
  before an immutable release tag can be created.
- Single-source OCI reference commands and mismatch checks for release
  namespace reads and publication.

### Changed

- Gate self-test discipline now covers modifications and retirement as well as
  introductions; gate mechanics are ineligible for the fast-track waiver.
- Changed-file discovery fails closed on missing, malformed, or ambiguous
  comparison inputs instead of reporting an empty diff, with a repository test
  corpus covering its CI contract.
- Mandatory CI `needs` relationships are implication-tested across pipeline
  contexts so an acceptance job cannot be enabled while its prerequisite is
  skipped.
- Foundry orientation now documents building and registering the local
  `agov-mcp` binary alongside `agov`.
- Foundry CI now routes ordinary and nested-container workloads to their
  dedicated runners, and mandatory Linux release packaging uses native musl
  tooling instead of Docker-in-Docker.
- The capability-developer role is active again after the learning corpus
  exceeded its governed evidence threshold.

## [2.8.0] - 2026-08-28

**Minor release.** Ships the complete current `main` line through
`ca808607`, including issues #86, #87, #88, and #90 plus all repairs that
made the earlier frozen v2.7.0 reconstruction unsafe to publish. `v2.7.0`
is retired unused; immutable `v2.7.0-rc.1` remains an abandoned failed
candidate. Adopter docs: updated.

### Changed

- **BREAKING: rule-projection owner lifecycle and repository-contained path
  validation.** `normative_owner.stage` is now restricted to `existing` or
  `planned`; a planned entry must name at least one non-empty `path` or
  `paths` member. `agov plan projection-check --allow-planned` permits only a
  genuinely absent, repository-contained planned owner during Gate 1, while
  the default command remains strict Gate 2 evidence and every invocation
  rejects a stale planned marker once any declared owner exists. The new
  `projection-check-all` CI safety net checks the full manifest tree on every
  pipeline while keeping changed-page coverage diff-scoped. Migration is
  required for four previously accepted shapes: non-enum `stage` values;
  `stage: planned` on an already-existing owner; planned entries with neither
  a non-empty `path` nor non-empty `paths`; and any surface path that is
  absolute, contains `..`, or reaches outside the repository through a
  symlink. Replace them with the defined lifecycle value, clear stale planned
  metadata before Gate 2, name the proposed owner, and use repository-relative
  contained paths respectively.
- **BREAKING workflow relaxation:** an exact, advisory and reversible
  `docs/sources.md`/`docs/radar.md` research-maintenance class may skip the
  separate Gate 1 plan, contribution audit, and pre-change coordination MR.
  The canonical simplicity rule now owns strict semantic/path eligibility,
  disqualifiers, fail-closed per-commit and final-tree evidence, fresh
  target/open-MR collision checks, and escalation. The normal MR, green
  exact-head pipeline, protected-branch merge, and binding human Gate 2 remain.
  Generic documentation, ambiguous work, and any authority, implementation,
  integration, roadmap/resource, sensitive/legal/security/privacy, public AGF
  claim, governed-decision, or extra-path scope still require full Gate 1.
  Adopters opt in only when both files are genuinely non-normative research
  artifacts; stricter local policy remains valid. No file or data migration is
  required.
- **BREAKING: `agov validate learnings` now checks `checkout_role: live`
  records against `agf/VERSION`** (issue #112). A learning record's
  `adoption.framework_version` value that previously legitimately
  differed from `agf/VERSION` is now a blocking finding if that record
  is marked `adoption.checkout_role: live`. `checkout_role` is a new
  optional field: `live` (this checkout's own, currently-maintained
  adoption state) or `imported` (a reference copy of another adopter's
  or historical submission); a record with the field absent, or set to
  `imported`, is skipped by the drift check entirely — only an explicit
  opt-in via `checkout_role: live` activates blocking behavior.
  `agov validate learnings` also gains two always-on findings
  independent of any single record's own drift: an advisory when
  `agf/learnings/` has zero `checkout_role: live` records (detection is
  inactive for this checkout), and a blocking ambiguity finding when it
  has two or more. `agf/skills/upgrade/SKILL.md` gains a new Step 0 that
  identifies (never guesses) the authoritative live record before any
  changelog range is presented, and a bounded 24-hour
  `upgrade_in_progress` suppression window around the mid-upgrade
  version-mismatch that already existed on every upgrade. Migration: an
  adopter who already opts a record into `checkout_role: live` and has
  an already-drifted `framework_version` must escalate the historical
  gap for their own authorized decision-maker to dispose, then correct
  `checkout_role: live` and `framework_version` together, referencing
  the published disposition — see
  `docs/plans/20260823T203540Z-adoption-version-drift-check.md`'s
  "One-time reconciliation" procedure. No adopter is forced onto the
  blocking behavior without first opting a record into `checkout_role:
  live`.

### Added

- The `agov-mcp` server now exposes the coordination surfaces needed for
  cold start and cross-branch visibility, validates JSON-RPC 2.0 request
  versions and IDs before dispatch, and ships assistant-loader discovery plus
  a ready-to-copy Claude MCP configuration in tooling archives (#86, #87,
  #88, #90).
- Commercial-corpus and rule-projection manifests provide machine-readable
  ownership and projection contracts, with schema and CI validation across
  their governed lifecycles.
- **Level 4 validated candidate — bounded review convergence:** optional
  `convergence` metadata on coordination records, the normative
  `review-convergence-rule`, and `agov coord convergence --series <id>` add a
  per-invariant two-failure abstraction-reset boundary, one redesigned
  candidate and targeted review, deterministic duplicate/conflict handling,
  and a closed blocker-materiality vocabulary. The command validates declared
  bytes and transitions but does not infer materiality, accept risk, approve a
  gate, or merge. Historical records remain unchanged. This capability is not
  Level 5 deployable until it ships in a supported release artifact.
- `agov validate` and `agov session resume` now emit an advisory (never
  blocking) warning when the running `agov` binary's own build version
  differs from the commissioned `agf/VERSION` of the tree it is invoked
  against (issue #114). Neither an older binary run against a newer
  tree nor a newer binary run against an older tree was previously
  detectable — both could silently under- or over-validate. Absent
  `agf/VERSION` (not every tree `agov` runs against is AGF-adopted)
  produces no warning.

## [2.6.1] - 2026-08-17

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
  `agf/rules/workflow/release-via-merge-request-rule.md` (and its
  `.claude/rules/` projection): an optional, narrowly eligible path for
  promoting a previously published, immutable release-candidate commit
  to stable after the default branch has accumulated unrelated later
  work, without pulling that work into the promoted release. Requires
  a binding governor decision naming the exact candidate, its own
  Gate 1-approved release plan, Maintainer-level (40) protection on
  both the maintenance target and source refs, a matching protected-tag
  rule, and post-merge API proof that retained refs survive the merge.
  The standard current-main release workflow is unchanged and remains
  the default for every ordinary release. `agf/rules/workflow/branch-
  and-worktree-rule.md`'s prior "version bumps happen only ... on
  `main`" wording corrected to acknowledge this one exception.
  `agf/docs/site/docs/operate/review-and-release.md` and
  `agf/skills/release/SKILL.md` both updated: the release skill's new
  Step 0 fails closed to the standard path unless all three
  eligibility preconditions are already met. First intended use is a
  separately Gate 1-approved `v2.5.0` final release-publication plan;
  this change authorizes no release action by itself. See
  `20260809T163051Z-v250-maintenance-release-line-rule.md` and Gate 1
  approval `20260809T180533Z-governor-reviewer-sp7d.md`.
- Frozen-scope reconstruction exception in
  `agf/rules/workflow/release-via-merge-request-rule.md` (and its
  `.claude/rules/` projection): a second, rarer optional path for
  shipping a governor-frozen, named subset of already Gate 2-approved
  work as its own immutable release when that work was never combined
  into one reviewable commit and the default branch has since kept
  moving with other, unrelated approved work that must not enter the
  release. Requires a binding governor decision naming the frozen scope
  and stable base, its own Gate 1-approved release-publication plan, a
  three-part tree-delta proof (deterministic replay, path attribution,
  and independent whole-result content faithfulness — not scoped to
  conflict-marker regions), `push_access_levels: No one` protection on
  the dedicated reconstruction target, a two-baseline construction/
  frozen protection scheme for the reconstruction source, and post-merge
  parent/tree-OID verification before any tag is created. Two residual
  target-drift risks — inherent to the platform's non-atomic merge API —
  are each closed by a separate, narrowly scoped governor decision: the
  release-construction merge's own drift is accepted as
  fail-closed-before-publication detect-and-remediate
  (`20260814T074427Z-governor-reviewer-33br.md`), distinct from and not
  extending the metadata-forward-sync merge's own detect-and-remediate
  exception (`20260814T040118Z-governor-reviewer-apkm.md`) — neither
  exception treats the other as precedent, and neither extends to
  ordinary feature, governance, or standard release merges.
  `agf/docs/site/docs/operate/review-and-release.md` and
  `agf/skills/release/SKILL.md` both updated: the release skill's Step 0
  gains a third named path with its own fail-closed eligibility gate.
  First intended use is a separately Gate 1-approved `v2.6.0`
  coordination-reliability release-publication plan; this change
  authorizes no release action by itself. See
  `docs/plans/20260813T155743Z-frozen-scope-reconstruction-rule-amendment.md`,
  governor decision `20260813T154648Z-governor-reviewer-2r8r.md`
  (commissioning), and Gate 1 approval
  `20260814T074427Z-governor-reviewer-33br.md`.

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
  v2.5.0-rc.2 are unaffected.

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

## [2.5.0] - 2026-08-09

**Stable.** Metadata-only promotion of the exact `v2.5.0-rc.2` functional
candidate to stable, published through the protected maintenance-line
workflow. Zero functional, schema, documentation-content, migration, test,
dependency, CI, or artifact-layout change from RC2. Independently re-run:
340 passed, 0 failed, 3 ignored (matches RC2's own precedent exactly).
Adopter docs: not applicable — exact RC2 functional candidate; metadata-only
promotion. Full detail: `agf/CHANGELOG.md`.

## [2.5.0-rc.2] - 2026-08-04

**RC checkpoint; not final.** `v2.5.0` final ships the same functional
candidate as this release candidate unless a correction forces `rc.3`.

**SemVer classification: Additive minor, documentation-only.** One new
page, `agf/docs/site/acknowledgements.md`, crediting NASA's Lessons
Learned Information System (LLIS) and other named design influences
behind specific shipped AGF surfaces (the learning-record
`supersedes`/`status` fields; the `downstream-impact-rule.md`
classification model), plus a nav entry (`agf/docs/build/mkdocs.yml`)
and a cross-link section in
`agf/docs/site/docs/concepts/evidence-and-learnings.md`. No schema,
command, or validator behavior changes. Full detail:
`agf/CHANGELOG.md`.

### Added

- NASA LLIS and design-influence acknowledgements page
  (`docs/plans/nasa-llis-influence-attribution.md`, Gate 1 approved
  2026-08-03, Gate 2 approved). Merged and Level 4 Validated as of
  `rc.1`; reaches Level 5 Deployable via this publication.

## [2.5.0-rc.1] - 2026-08-03

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
  (`docs/plans/learning-supersession-status-fields.md`, Gate 1 approved,
  Path B). `superseded_by` is now deprecated (`x-deprecated: true`) but
  kept schema-legal; `supersedes` on the new entry replaces it and is
  enforced by `validate_learnings`. Full detail: `agf/CHANGELOG.md`.
- Upgrade-safe adopter role authority (issue #79,
  `docs/plans/v250-rc1-contract-core.md` work package 1). The shipped
  release archive no longer contains `agf/agents/MANIFEST.yaml` — a plain
  archive re-extraction over an adopter project could previously overwrite
  an adopter's own divergent roster with the framework's dogfooding
  manifest (reproduced concretely by issue #78). The archive now ships a
  neutral `agf/agents/MANIFEST.example.yaml` template instead, excluded
  from packaging by an explicit rule and never read as adopter state by
  `agov`. With the default path in effect and `MANIFEST.yaml` absent,
  `agov session resume`'s cold-start output states the missing path
  explicitly and names both remedies. Full detail: `agf/CHANGELOG.md`.
- `requires-ack` obligation semantics for non-handoff coordination records
  (issue #80, `docs/plans/v250-rc1-contract-core.md` work package 2).
  `requires-ack: true` was previously silently inert on every kind except
  `handoff`. The schema now rejects `kind: ack` + `requires-ack: true`
  (an ack cannot require its own ack), and a new `agov coord pending`
  command tracks every other kind's obligations via direct
  `references-to` resolution, with pre-cutoff records grandfathered.
  `handoff next`/`take`/`complete` and `coord graph-index` are unchanged.
  Full detail: `agf/CHANGELOG.md`.

Adopter docs: updated — see `agf/CHANGELOG.md`.

### Fixed

- `agf/rules/always/governance-file-layout-rule.md` and its
  `.claude/rules/` copy no longer present "Layout A"/"Layout B" as a live
  adopter choice. AGF v2.0 supports exactly one adoption layout, already
  stated in `agf/ADOPTING.md` and `agf/docs/site/docs/adopt/install-and-layouts.md`;
  the rule's stale `<!-- CUSTOMIZE -->` template contradicted both. Replaced
  with a single-layout ownership table (which paths are adopter-owned vs.
  upstream-owned) and corrected a stale header pointer and a pre-rename
  example path.

**Reclassified from Editorial to Additive** (Codex review, MR !916 on a
related plan, 2026-08-02): the removed dual-layout block was inert
(neither option was ever live), so the rule previously asserted no
concrete ownership guidance at all. The replacement table is new operative
content, not a restatement of prior behavior — Additive under
`downstream-impact-rule.md`'s own test, not Editorial. The table's
`agf/agents/*.yaml` row is explicitly marked as deciding issue #79's own
Gate 1 plan (`docs/plans/v250-rc1-contract-core.md`), not asserted
independently ahead of it — all other rows restate long-standing
convention already published in `ADOPTING.md`, unaffected by #79.

Adopter docs: not applicable — the table's content already exists in
`ADOPTING.md`'s "Confirm the layout" table (except the #79-pending row,
which is explicitly marked as such); no new adopter doc, template, or
behavior change as a result.

### Changed

- CI: the shared Rust cache (`.rust-base`) is now pull-only outside ordinary
  default-branch push pipelines — feature branches, merge requests, tags,
  schedules, API/web/trigger pipelines, and downstream pipelines no longer
  spend runner time archiving and uploading a cache most of them use once
  (issue #101, Slice A of `docs/plans/ci-rust-cache-policy-slice-a.md`,
  child scope of #53). A single explicit default-branch write context
  (`RUST_CACHE_POLICY: pull-push`) keeps the shared cache warm; every other
  context fails safe to `pull` via a top-level default, verified at runtime
  by `.gitlab/scripts/check-rust-cache-policy.sh assert-current`. The cache
  key moved to a versioned `rust-v2-main` namespace (with `rust-main`
  retained as fallback) so non-default consumers stop preferring a stale
  branch-specific cache they can no longer refresh. This does not eliminate
  concurrent default-branch writers uploading to the same key — that
  topology is separately gated as Slice B, per the plan's explicit
  boundary. No job, test coverage, or acceptance requirement changed; cache
  content remains build acceleration only, never test evidence.

Adopter docs: not applicable — repository-internal CI cost optimization,
not an AGF framework capability.

- CI: narrowed the default-branch Rust cache (`.rust-base`) from eight
  concurrent last-writer-wins uploaders to exactly one designated writer,
  `rust:tooling:test` (issue #101, Slice B of
  `docs/plans/ci-rust-cache-policy-slice-b.md`). Every other consumer —
  automatic, manual, and tag/release — is pull-only in every context,
  including an ordinary default-branch push, enforced by two separate
  variables (`RUST_CACHE_POLICY` for consumers, `RUST_CACHE_WRITER_POLICY`
  for the sole writer) so a new `.rust-base` inheritor is pull-only by
  construction rather than by remembering to opt out. The shared Rust job
  image moved from the floating `rust:1` tag to an immutable
  `rust:1.97.1@sha256:...` digest pin, and the cache key advanced to
  `rust-v3-main` (`rust-v2-main` and `rust-main` retained as fallbacks) so
  compiled output from the formerly floating image is never treated as the
  pinned toolchain's primary cache. `.gitlab/scripts/check-rust-cache-policy.sh`
  is now role-aware (`RUST_CACHE_ROLE=writer|consumer`) and fails closed on
  an unrecognised or missing role. A new `agov ci cache-topology-check`
  subcommand statically verifies a resolved CI configuration has exactly
  one writer, dynamically inventoried from GitLab CI Lint's merged YAML
  (not a hard-coded job list, so an indirect `.rust-base` inheritor cannot
  silently escape the check the way two jobs did during Slice A). This is
  a manually-invoked verification tool used for Gate 2 evidence, not a
  standing CI job — `CI_JOB_TOKEN` is not authorized against GitLab's CI
  Lint API on this project (confirmed via a real 403 at implementation
  time), and adding a scoped credential to enable automatic per-pipeline
  enforcement was deferred rather than done unilaterally. A pinned
  ShellCheck job (`docker.io/koalaman/shellcheck-alpine@sha256:...`) lints
  the policy script in CI. This trades away the small, inconsistent
  release-profile cache benefit `agov:governance:validate` sometimes got
  under the old last-writer-wins topology (see the plan's "why
  rust:tooling:test" section) — acceleration only, no correctness or
  acceptance-evidence effect. No job, test coverage, or acceptance
  requirement changed.

Adopter docs: not applicable — repository-internal CI cost optimization,
not an AGF framework capability.

### Validation

- Rust workspace: 340 passed, 0 failed, 3 ignored. Clippy clean with
  warnings denied.
- Strict `mkdocs build`: passed, no broken links.
- Framework-archive overlay tested against three published baselines:
  `v2.4.5`, `v2.4.8` (substituted for unpublished `v2.4.6` per governor
  decision `20260803T183732Z-governor-reviewer-rntt.md`), and `v2.4.11`.

## [2.4.11] - 2026-08-01

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

### Removed

- Removed `docs/academic/` (the "Governing the Governors" working paper,
  IEEE build files, figures, and academic/visual review records) after
  relocating it verbatim to `agf-promotional`, where the project's
  promotional and explanatory content about AGF lives. Historical
  coordination records, plans, and `agf/CHANGELOG.md` entries that
  reference the old path are preserved verbatim as governed evidence. This
  removal predates and is independent of the accessibility work above; it
  is included here because it was not yet part of a tagged release.

### Validation

- Rust workspace: 299 passed, 0 failed, 3 ignored (version-only bump; no
  Rust source changed in this release). Clippy clean with warnings denied.
- Strict `mkdocs build` including the new accessibility hook: passed.

Addresses issue #97. The issue closes only after the published v2.4.11
assets and site pass post-publication verification.

Adopter docs: updated

## [2.4.10] - 2026-07-31

Patch release making the repository-local `tooling/agov` launcher
source-compatible with its checkout.

### Fixed

- Removed the wrapper's handwritten command registry so the authoritative
  Rust parser receives the full command tree, including `coord
  publication-check` and all `dist` commands.
- Added a deterministic fingerprint of behavior-affecting tooling inputs and
  required compatible release/debug binaries before execution.
- Added one-shot verified release rebuilding with fail-closed behavior when
  Cargo or a compatible rebuilt binary is unavailable.
- Added actionable cross-worktree mismatch diagnostics that identify the
  correct checkout-local wrapper path. Standalone published binaries retain
  their existing behavior; this policy applies to the repository wrapper.

### Validation

- Rust workspace: 299 passed, 3 ignored; wrapper freshness, live-surface, and
  clean tar and zip framework archive acceptance suites passed.

Addresses issue #92. The issue closes only after the published v2.4.10 assets
and a fresh git checkout at the immutable tag pass post-publication
verification.

Adopter docs: updated

## [2.4.9] - 2026-07-31

Patch release removing stale live tooling surfaces after the Rust `agov`
migration.

### Fixed

- Removed the broken `bin/agf` wrapper, which delegated every command to a
  nonexistent Elixir/Mix project, and removed the dead Elixir parity harness.
- Converted current contributor, adopter, contract, governance, and
  decommission guidance to the supported Rust `agov` commands while preserving
  historical, generic ecosystem, and release-exclusion evidence.
- Added a fail-closed live-surface regression check with a negative self-test
  so supported entry points and current instructions cannot silently return to
  removed Mix, Elixir, legacy Python, or `bin/agf` paths.

### Validation

- Rust workspace: 299 passed, 3 ignored; clean tar and zip framework archive
  acceptance suites passed.

Addresses issue #93. The issue closes only after the published v2.4.9 assets
pass anonymous post-publication verification.

Adopter docs: updated

## [2.4.8] - 2026-07-30

Publication-completion recovery after the immutable v2.4.7 tag published its
OCI image but failed before creating the generic package and GitLab Release.

### Fixed

- `release:create` no longer passes the absent optional Windows `agov` ZIP to
  the strict Elixir-artifact exclusion checker. Framework archives and Linux
  tooling archives remain mandatory and are still checked unconditionally.
- The pre-tag release evidence now assembles the real framework and Linux
  tooling preview artifacts and exercises the exact downstream exclusion path
  both without an optional ZIP and with a temporary valid optional ZIP.

### Changed

- Release documentation records v2.4.7 as an immutable OCI-only partial
  publication and directs adopters to the complete v2.4.8 release.

## [2.4.7] - 2026-07-30

Tagged publication-recovery candidate superseding the unpublished v2.4.6
candidate. The v2.4.7 pipeline published the OCI image, but a downstream
release-creation check still required the optional Windows ZIP; consequently
no generic package or GitLab Release was created. Adopt v2.4.8 instead.

### Changed

- Tag pipelines now publish framework archives, Linux x86_64/aarch64 Rust
  tooling, their checksum and provenance sidecars, and OCI provenance without
  performing the Windows cross-build.
- Windows packaging remains available as optional, non-blocking manual preview
  evidence and is no longer part of the mandatory release-asset contract.
- Release documentation treats each GitLab Release page's actual asset links
  as the authoritative platform inventory.

## [2.4.4] - 2026-07-26

Patch release making the public documentation's mutable build identity
unambiguous relative to the latest immutable release. See
[`agf/CHANGELOG.md`](agf/CHANGELOG.md) for adopter-facing details.

### Fixed

- Public release information now separates the framework version, latest
  release tag and source commit, live documentation branch and commit, and
  whether the site is release or later development documentation.
- The AI evaluation dossier embeds the same full provenance tuple in visible,
  extractable text. The accessible page header visibly shows the framework
  version and docs-build commit; its accessible label also reports release
  status.
- Removed stale version-numbered lifecycle labels and added a strict source
  scanner that prevents their reintroduction except for an explicitly bounded
  historical qualifier.

### Changed

- Public status banners no longer couple page maturity to a historical release
  number.
- Added one governed public feature summary to the README and documentation
  homepage, and clarified that contributor setup belongs at the end of the
  adopter-facing README.
- Polling guidance now names incremental coordination listing and the optional
  structured MCP transport while retaining the Rust CLI fallback.

## [2.4.3] - 2026-07-25

Patch release combining three small adopter-facing fixes (issues #64, #65,
#66) with the v2.4.3 documentation-quality and information-architecture
backlog (MR !483/!527). See [`agf/CHANGELOG.md`](agf/CHANGELOG.md) for
adopter-facing details.

### Fixed

- Replaced all public Mermaid diagrams with openable, full-size SVG figures
  and removed the now-unused client-side Mermaid runtime.
- Corrected the Adopter FAQ's tooling-commissioning guidance to recommend
  installing and using `agov` from day one instead of after the first
  governed change.
- Explained that different authorized people may hold Gate 1 and Gate 2 on
  the same change, sequentially.
- Named `agf/VERSION` as the canonical source at all six hardcoded
  `agov:<version>` OCI-tag literals, for editor discoverability.
- Repointed two misdirected metadata links to their canonical slug.
- Reassigned the governance-loop figure's `learn` node from the "Reference /
  deferred content" palette category to "Shared concept," since learning
  records are authored by AI and by humans, not exclusively either.
- Aligned "Level 2" capability-maturity naming to "Surface-defined" across
  the internal rule file, matching the public site's existing usage.
- Documented `palette.md`'s bounded light-background assumption for seven
  named figure canvases, making no universal renderer/theme claim.
- Added `<figure>`/`<figcaption>` semantics to the seven existing reference
  figure embeds.

## [2.4.2] - 2026-07-25

Patch release for the public-experience and evidence-presentation work in MR
!488. See [`agf/CHANGELOG.md`](agf/CHANGELOG.md) for adopter-facing details.

### Fixed

- Corrected public governance-enforcement, provider-boundary, command-maturity,
  and adoption-exit claims.
- Aligned the adoption-exit figure's labels with its surrounding guidance.

### Changed

- Condensed the homepage tooling inventory and retained one canonical command
  reference.
- Added exact-build repository references to the AI evaluation dossier.
- Documented assistant/role bylines and improved progressive disclosure and
  next-action guidance across six evaluator-facing pages.

## [2.4.1] - 2026-07-24

Patch release closing the accessibility and prose-hardening work governed by
issues #51, #55, and #56 (MR !458). See [`agf/CHANGELOG.md`](agf/CHANGELOG.md)
for the adopter-facing details.

### Fixed

- Public documentation now precedes every figure with page-specific prose
  carrying its essential meaning, so the argument does not depend on perceiving
  the image alone.
- Mermaid diagrams now use a narrow-width horizontal-scroll presentation and
  expose a shared accessible name and description derived from their source;
  this closes the mobile and assistive-technology remediation deferred from
  v2.4.0.

### Changed

- The accessibility evidence records the exact browser, operating system, and
  assistive-technology test boundary. The result is Level 4 Validated and does
  not claim formal WCAG conformance or universal assistive-technology support.

## [2.4.0] - 2026-07-23

Minor release (see [`agf/CHANGELOG.md`](agf/CHANGELOG.md) `[2.4.0]` for the
adopter-facing feature changelog covering the four v2.4.0 workstreams).
Repository-level changes not already covered there:

### Fixed

- Documentation: kept the persistent `AGF / <page>` header and `AGF —
  <page>` browser-title identity visible on every page (MR !330).
- Documentation: corrected the "what ships and what you activate" page to
  match maintainer evidence rather than an earlier inaccurate framing
  (MR !328).
- Closed a tar-extraction ETXTBSY race by explicitly closing the writer
  handle before execution (MR !398).
- Closed a remaining ZIP/TAR extraction ETXTBSY race with a bounded
  executable-file-busy exec retry, on top of the MR !398 fix (MR !408).
- Fixed token-ROI log write-atomicity and a test-suite pollution issue
  (issue #50, MR !363).

### Changed

- CI: suppressed duplicate branch-push pipelines when an open MR already
  exists for the branch, cutting CI cost on routine coordination and seed
  commits (MR !410).

## [2.3.1] - 2026-07-21

Patch release recovering the repository's public release truth after v2.3.0
was published with this changelog still ending at 2.2.5. Tag `v2.3.0` remains
immutable; its historical entry was added on `main`, and v2.3.1 publishes that
correction under a new release identity.

### Added

- Proposed learnings from the v2.3.0 release now record consumer-layout host
  preflight and complete release truth-surface enumeration as future target
  states. Their generalized recommendations are not implemented by this patch.

### Fixed

- GitLab Release creation now exposes direct links for all individual checksum
  and provenance sidecars, verifies the mandatory archive/sidecar set before
  upload, and carries a negative self-test for a missing provenance record.
- Added the immutable recovery trail that supersedes premature v2.3.0 terminal
  completion claims without moving the published v2.3.0 source or OCI tags.

## [2.3.0] - 2026-07-21

Repository-level changes accompanying the v2.3.0 framework release
(see [`agf/CHANGELOG.md`](agf/CHANGELOG.md) `[2.3.0]` for the
adopter-facing feature changelog). Found and recorded during this
release's own public-truth verification pass — this repository-level
changelog had not been updated since 2.2.5.

### Fixed

- `agf/dist/release-manifest.yaml`: excluded `__pycache__/` and `.pyc`
  paths from the release package `exclude`/`forbidden` lists, closing a
  leak of stale Python bytecode into the framework release archives
  (MR !303).
- `.gitlab-ci.yml`: the `rust:tooling:preview`/`rust:tooling:release`
  checksum-generation step recorded a build-tree path
  (`../../dist/<archive>`) in each `.sha256` file instead of the archive
  basename, breaking the standard `sha256sum -c` consumer-verification
  flow. Fixed to match the basename-only convention `agov dist build`
  already used for framework archives (MR !308).

### Changed

- Enabled the project's GitLab Container Registry (previously disabled),
  required for the `agov:oci:release` tag-pipeline job to publish the
  `agov` OCI image. This had not been exercised end-to-end against a real
  tag before v2.3.0.

## [2.2.5] - 2026-06-25

This patch release completes AGF-NG's transition away from historical
agents-govern 1.x Python tooling and records the repository-level metadata
needed for the current Rust-only operator path.

### Changed

- AGF v2 contract documentation now marks v1.x Python-only tooling surfaces as
  retired or deferred before historical Python code removal. Binding validation
  is explicitly deferred in current Rust `agov`; `agov-validate-acronyms`,
  `agov-bump`, `agov --capabilities`, and `agov --contract-changelog` are
  retired; `agov-health` remains deferred.

### Removed

- Historical agents-govern 1.x Python package sources and package metadata are
  removed from `agents-govern/`; Rust `tooling/agov` is the only supported
  AGF-NG operator tooling.

### Added

- Root `LICENCE` file summarising the repository licence map:
  CC BY-SA 4.0 for framework/governance content, MIT for tooling, and
  separate commercial or other terms where expressly granted.
- Root `CHANGELOG.md` to make repository-level changes visible before readers
  enter the adopter-facing `agf/` distribution or bootstrap reference tree.

## [2.2.0-rc.2] - 2026-06-24

### Fixed

- Rust `agov` release archives now embed CI commit provenance in the binary
  itself, so `agov --version` and `agov version --format json` report the tag
  pipeline commit instead of `unknown dirty`.

## [2.2.0-rc.1] - 2026-06-24

### Added

- Readable coordination IDs are now the default format for
  `agov coord brief`.
- Rust `agov` reports version provenance through `agov --version` and
  `agov version --format json`.
- `agov coord graph-index` now has layered contract documentation covering
  human operation, AI cold-start use, output fields, and exit behavior.

### Changed

- RC-1 readiness audit completed: the Rust operator path was verified,
  inherited v1.x material in high-risk public paths was labelled, and release
  documentation was aligned with current tooling behavior.
