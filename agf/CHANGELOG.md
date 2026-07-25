# Changelog — agents-govern 2.0

v2.0 entries only. The v1.x changelog is not part of the `agf/`
distribution.

## [2.4.3] — 2026-07-25

Patch release improving existing public documentation figures, adoption
guidance, and documentation consistency. It adds no schema, role, tool, API,
CI, or adopter-contract surface.

### Fixed

- Replaced all five page-scale Mermaid diagrams with governed, openable SVG
  artifacts and removed the unused Mermaid runtime and CDN dependency.
- Corrected two legacy project links and aligned the internal Level 2 maturity
  label with the canonical “Surface-defined” vocabulary.
- Reclassified the governance-loop learning step as a shared human/AI concept
  and extended its legend with the existing palette treatment.

### Changed

- Updated the Adopter FAQ to recommend `agov` from the first evaluation or
  commissioning session while keeping framework files authoritative.
- Added bounded guidance for using different authorized people at Gate 1 and
  Gate 2 without creating simultaneous final decision-makers.
- Identified `agf/VERSION` beside all six OCI image pins and documented the
  accepted white/light figure-canvas assumption.
- Added descriptive alternative text and numbered captions to the seven
  existing reference figures.

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
